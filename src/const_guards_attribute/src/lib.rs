use std::cmp::Ordering;

use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::{
    parse,
    parse::ParseStream,
    token::{Brace, Where},
    Block, ConstParam, Expr, GenericParam, Generics, Item, ItemEnum, ItemFn, ItemStruct, Token,
    TraitItem, TraitItemMethod, TraitItemType, TypeParam,
};

const INVALID_ITEM: &str = "guarded items need to support `where` clauses";
const INVALID_GUARD: &str = "guard must be an expression or polymorphic block";
const GUARD_FAILED: &str = "guard evaluated to false";

/// Represents some guarded `syn::Item`.
struct GuardItem {
    /// Identifier, i.e it's name
    ident: Ident,
    /// Declaration until the beginning of the where clause
    decl: proc_macro2::TokenStream,
    /// Generic parameters
    generics: Generics,
    /// Declaration after the where clause
    cont: proc_macro2::TokenStream,
}

/// Represents a list of statements with optional outer generic parameters
#[derive(Parse)]
struct PolyBlock {
    /// Generic parameters
    #[peek(Token![<])]
    generics: Option<Generics>,
    /// Function-like block
    block: Block,
}

// Represents a guard expression or poly block
#[derive(Parse)]
enum Guard {
    #[peek_with(|input: ParseStream<'_>| input.peek(Token![<]) || input.peek(Brace), name = "PolyBlock")]
    PolyBlock(PolyBlock),
    #[peek_with(|input: ParseStream<'_>| !(input.peek(Token![<]) || input.peek(Brace)), name = "Expr")]
    Expr(Expr),
}

#[proc_macro_attribute]
pub fn guard(attr: TokenStream, stream: TokenStream) -> TokenStream {
    let GuardItem {
        ident,
        decl,
        generics,
        cont,
    } = GuardItem::from(stream);

    let guard_ident = format_ident!("_{ident}_guard");
    let where_ext = where_ext(&generics);

    let (guard, generics, param_idents) = match Guard::from(attr) {
        Guard::PolyBlock(PolyBlock {
            generics: guard_generics,
            block,
        }) => {
            let params = if let Some(guard_generics) = guard_generics {
                merge_generic_params(generics, guard_generics)
            } else {
                generics.params.into_iter().collect::<Vec<GenericParam>>()
            };
            let generics = quote! {< #(#params),* >};
            (quote!(#block), generics, param_idents(&params))
        }
        Guard::Expr(expr) => {
            let params = generics.params.into_iter().collect::<Vec<GenericParam>>();
            let generics = quote! {< #(#params),* >};
            (quote!((#expr)), generics, param_idents(&params))
        }
    };

    let tokens = quote! {
        #decl #where_ext const_guards::Guard<{
            const fn #guard_ident #generics() -> bool {
                if !#guard {
                    panic!(#GUARD_FAILED)
                }
                true
            }
            #guard_ident::<#param_idents>()
        }>: const_guards::Protect #cont
    };
    
    TokenStream::from(tokens)
}

fn param_idents(params: &[GenericParam]) -> proc_macro2::TokenStream {
    let idents = params
        .iter()
        .filter_map(param_ident)
        .collect::<Vec<&Ident>>();
    quote! {#(#idents),*}
}

fn where_ext(generics: &Generics) -> Option<proc_macro2::TokenStream> {
    generics
        .where_clause
        .as_ref()
        .map(|wc| {
            if wc.predicates.trailing_punct() {
                Some(quote! {,})
            } else {
                None
            }
        })
        .or_else(|| {
            let kw_where = Where(Span::call_site());
            Some(Some(quote!(#kw_where)))
        })
        // unwrap because its either Some(Some(",")), Some(None) or Some(Some("where"))
        .unwrap()
}

fn merge_generic_params(left: Generics, right: Generics) -> Vec<GenericParam> {
    let mut left_params = left.params.into_iter().collect::<Vec<GenericParam>>();
    left_params.extend(right.params.into_iter().collect::<Vec<GenericParam>>());
    let mut params = left_params
        .into_iter()
        .filter(|param| {
            matches!(param, GenericParam::Type(_)) | matches!(param, GenericParam::Const(_))
        })
        .collect::<Vec<GenericParam>>();
    params.sort_by(compare_params);
    params.dedup_by(|left, right| compare_params(left, right).is_eq());
    params
}

fn compare_params(left: &GenericParam, right: &GenericParam) -> Ordering {
    match (param_ident(left), param_ident(right)) {
        (Some(left), Some(right)) => left.cmp(right),
        // ruled out by filtering for anything other than
        // `syn::GenericParam::Type` and `sny::GenericParam::Const`
        _ => unreachable!(),
    }
}

fn param_ident(param: &GenericParam) -> Option<&Ident> {
    match param {
        syn::GenericParam::Type(TypeParam { ident, .. }) => Some(ident),
        syn::GenericParam::Const(ConstParam { ident, .. }) => Some(ident),
        _ => None,
    }
}

impl From<TokenStream> for GuardItem {
    fn from(stream: TokenStream) -> Self {
        // unwrap because it cannot fail, since it will result in `Item::Verbatim(stream)`
        let item = parse::<Item>(stream).unwrap();
        if let Item::Verbatim(stream) = item {
            // unwrap because it cannot fail, since it will result in `TraitItem::Verbatim(stream)`
            GuardItem::from(parse::<TraitItem>(TokenStream::from(stream)).unwrap())
        } else {
            GuardItem::from(item)
        }
    }
}

impl From<TokenStream> for Guard {
    fn from(stream: TokenStream) -> Self {
        parse::<Guard>(stream).expect(INVALID_GUARD)
    }
}

impl From<Item> for GuardItem {
    fn from(item: Item) -> Self {
        let (decl, ident, generics, cont) = match item {
            Item::Enum(ItemEnum {
                attrs,
                vis,
                enum_token,
                ident,
                generics,
                variants,
                ..
            }) => (
                quote! {#(#attrs)* #vis #enum_token #ident #generics},
                ident,
                generics,
                quote! {{ #variants }},
            ),
            Item::Fn(ItemFn {
                attrs,
                vis,
                sig,
                block,
            }) => (
                quote! {#(#attrs)* #vis #sig},
                sig.ident,
                sig.generics,
                quote! {#block},
            ),
            Item::Impl(_) => todo!(),
            Item::Struct(ItemStruct {
                attrs,
                vis,
                struct_token,
                ident,
                generics,
                fields,
                semi_token,
            }) => (
                quote! {#(#attrs)* #vis #struct_token #ident #generics #fields},
                ident,
                generics,
                quote! { #semi_token},
            ),
            Item::Trait(_) => todo!(),
            Item::Type(_) => todo!(),
            Item::Union(_) => todo!(),
            _ => panic!("{INVALID_ITEM}"),
        };

        GuardItem {
            ident,
            decl,
            generics,
            cont,
        }
    }
}

impl From<TraitItem> for GuardItem {
    fn from(item: TraitItem) -> Self {
        let (decl, ident, generics, cont) = match item {
            TraitItem::Method(TraitItemMethod {
                attrs,
                sig,
                default,
                semi_token,
            }) => (
                quote! {#(#attrs)* #sig},
                sig.ident,
                sig.generics,
                quote! {#default #semi_token},
            ),
            TraitItem::Type(TraitItemType {
                attrs,
                type_token,
                ident,
                generics,
                colon_token,
                bounds,
                default,
                semi_token,
            }) => {
                let cont = if let Some((_, ty)) = default {
                    quote! { #colon_token #bounds = #ty #semi_token}
                } else {
                    quote! { #colon_token #bounds #semi_token}
                };
                (
                    quote! {#(#attrs)* #type_token #ident},
                    ident,
                    generics,
                    cont,
                )
            }
            _ => panic!("{INVALID_ITEM}"),
        };

        GuardItem {
            ident,
            decl,
            generics,
            cont,
        }
    }
}