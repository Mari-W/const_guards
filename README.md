# Const Guards [[docs.rs](https://docs.rs/const-guards)]
With `const_guards` you can express certain compile time constraints on rust's [`const_generics`](https://github.com/rust-lang/rust/issues/44580) using the unstable [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560) feature.

## Documentation
For documentation visit [docs.rs](https://docs.rs/const-guards).

## Motivation
Consider the following usage of the `first` method on slices from the standard library: 
```rust
let slice: [(); 1] = [(); 1];
let head: Option<&()> = slice.first();
``` 
Would it be nice if we could just write
```rust
let head: &() = slice.first();
```
since the compiler should know this slice has length `1` at this point.
With const guards we can express such as follows:
```rust
#[guard(N > 0)]
fn first<'a, T, const N: usize>(slice: &'a [T; N]) -> &'a T {
    &slice[0]
}
```
The index call on the slice `&slice[0]` cannot possible fail because we enforced the length of the slice to be `> 0` at compile time. We could now call it as follows 
```rust
let slice: [(); 1] = [(); 1];
let head: &() = first(&slice);
```
while the case where the slice is actually empty would _fail to compile_:
```
let slice: [(); 0] = [(); 0];
let head: &() = first(&slice);
```
Finally we could even express this as a trait to make it more accessable:
```rust
trait SliceHead<T, const N: usize> {
    #[guard(<const N: usize> { N > 0 })]
    fn head(&self) -> &T;
}

impl<T, const N: usize> SliceHead<T, N> for [T; N] {
    fn head(&self) -> &T {
        &self[0]
    }
}

fn main() {
    let slice: &[(); 1] = &[(); 1];
    let head: &() = slice.head();
}
```
Though, as you can see, we need to introduce generics not introduced by the guarded item explicitly.

## Implementation
Consider this simple example of a const guard:
```rust
fn main() {
    f::<0>()
}

#[guard(N > 0)]
fn f<const N: usize>() {
    todo!()
}
```
and have a look at the expanded form:
```rust
struct Guard<const U: bool>;

trait Protect {}
impl Protect for Guard<true> {}

fn main() {
    f::<0>()
}

fn f<const N: usize>()
where
    Guard<{
        const fn _f_guard<const N: usize>() -> bool {
            if !N > 0 {
                panic!("guard evaluated to false")
            }
            true
        }
        _f_guard::<N>()
    }>: Protect,
{
    todo!()
}
```

## Todo

- [ ] Improve error messages
- [ ] Write more tests