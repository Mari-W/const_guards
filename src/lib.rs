//! `const_guards` is a simple attribute macro that makes it easy to express certain
//! compile time constraints on [`const_generics`] using [`generic_const_exprs`].
//!
//! [`const_generics`]: https://rust-lang.github.io/rfcs/2000-const-generics.html
//! [`generic_const_exprs`]: https://github.com/rust-lang/rust/issues/76560
//!
//! ## Example
//!
//! ```rust
//! # #![allow(incomplete_features)]
//! #![feature(generic_const_exprs)]
//! use const_guards::guard;
//!
//! #[guard(N > 0)]
//! fn f<const N: usize>() { todo!() }
//! ```
//! ```compile_fail
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs)]
//! # use const_guards::guard;
//! #
//! # #[guard(N > 0)]
//! # fn f<const N: usize>() { () }
//! # fn main() {
//! f::<0>()
//! # }
//! ```
//! ```rust
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs)]
//! # use const_guards::guard;
//! #
//! # #[guard(<const N: usize> {N > 0})]
//! # fn f<const N: usize>() { () }
//! # fn main() {
//! f::<1>()
//! # }
//! ```
//!
//! ## Guards
//! Guards can have either a expression or a polymorphic block as argument.
//!
//! #### Expression Guard
//! The expression guard can only use const generics and normal generics introduced
//! by the item and is limited to one [rust expression] allowed in [`const fn`].
//!
//! [rust expression]: https://doc.rust-lang.org/reference/expressions.html
//! [`const fn`]: https://doc.rust-lang.org/reference/const_eval.html#const-functions
//!
//! ```rust
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs)]
//! # use const_guards::guard;
//! #
//! #[guard(PREFIX == '_' || PREFIX == '#')]
//! struct Label<const PREFIX: char> (&'static str);
//! ```
//! ```compile_fail
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs)]
//! # use const_guards::guard;
//! #
//! # #[guard(PREFIX == '_' || PREFIX == '#')]
//! # struct Label<const PREFIX: char>(&'static str);
//! # fn main() {
//! let label = Label::<'!'>("important");
//! # }
//! ```
//! ```rust
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs)]
//! # use const_guards::guard;
//! #
//! # #[guard(PREFIX == '_' || PREFIX == '#')]
//! # struct Label<const PREFIX: char>(&'static str);
//! # fn main() {
//! let label = Label::<'_'>("unused");
//! # }
//! ```
//!
//! #### Polymorphic Block Guard
//! The polymorphic block is denoted `<const A: Type, const B: Type, .., T, V, ..> { .. }`
//! and can introduce const generics and normal generics from the outer scope
//! to the block in curly brackets. The generics are optional and an polymorphic block
//! without the need of outer generics can be denoted as `{ .. }`
//!
//! Inside the block the same kind of logic that can be used inside [`const fn`] is allowed.
//!
//! [`const fn`]: https://doc.rust-lang.org/reference/const_eval.html#const-functions
//!
//! ```rust
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs)]
//! # use const_guards::guard;
//! #
//! trait ArrayHead<T, const N: usize> {
//!   #[guard(<const N: usize> { N > 0 })]
//!   fn head(&self) -> &T;
//! }
//!
//! impl<T, const N: usize> ArrayHead<T, N> for [T; N] {
//!   fn head(&self) -> &T {
//!     &self[0]
//!   }
//! }
//! ```
//! ```compile_fail
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs)]
//! # use const_guards::guard;
//! #
//! # trait ArrayHead<T, const N: usize> {
//! #  #[guard(<const N: usize> { N > 0 })]
//! #  fn head(&self) -> &T;
//! # }
//! #
//! # impl<T, const N: usize> ArrayHead<T, N> for [T; N] {
//! #  fn head(&self) -> &T {
//! #    &self[0]
//! #   }
//! # }
//! # fn main() {
//!   let array = &[(); 0];
//!   let head: &() = array.head();
//! # }
//! ```
//! ```rust
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs)]
//! # use const_guards::guard;
//! #
//! # trait ArrayHead<T, const N: usize> {
//! #  #[guard(<const N: usize> { N > 0 })]
//! #  fn head(&self) -> &T;
//! # }
//! #
//! # impl<T, const N: usize> ArrayHead<T, N> for &[T; N] {
//! #  fn head(&self) -> &T {
//! #    &self[0]
//! #   }
//! # }
//! # fn main() {
//!   let array = &[(); 1];
//!   let head: &() = array.head();
//! # }
//! ```
//!
//! ## Items
//! Guards are allowed to be an attribute of the following rust items:
//!
//! * `fn`
//! * `enum`
//! * `struct`
//! * `impl`
//!
//! ## Advanced
//!
//! #### Custom Error Messages
//! Raise an custom error by `panic!`-ing inside the guard.
//! This will be rendered as compile error with help of rust's [`const_panic`].
//! With that in mind we could modify the trait definition from
//! [Polymorphic Block Guard](#polymorphic-Block-guard).
//!
//! [`const_panic`]: https://rust-lang.github.io/rfcs/2345-const-panic.html
//!
//! ```rust
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs)]
//! # use const_guards::guard;
//! #
//! trait ArrayHead<T, const N: usize> {
//!   #[guard(<const N: usize> {
//!     if N == 0 {
//!         panic!("expected at least one item in array")
//!     } else {
//!         true
//!     }
//!   })]
//!   fn head(&self) -> &T;
//! }
//! ```
//!
//! #### Const Functions
//! It's possible to outsource logic by calling other `const fn`.
//! With that in mind we could modify the trait definition from
//! [Polymorphic Block Guard](#polymorphic-Block-guard).
//!
//! ```rust
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs)]
//! # use const_guards::guard;
//! #
//! const fn is_not_empty<const N: usize>() -> bool {
//!   N > 0
//! }
//!
//! trait ArrayHead<T, const N: usize> {
//!   #[guard(<const N: usize> { is_not_empty::<N>() })]
//!   fn head(&self) -> &T;
//! }
//! ```

extern crate const_guards_attribute;
pub use const_guards_attribute::guard;

pub struct Guard<const U: bool>;

pub trait Protect {}
impl Protect for Guard<true> {}
