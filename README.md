# Const Guards [[docs.rs](https://docs.rs/const-guards)]
With `const_guards` you can express certain compile time constraints on rust's [`const_generics`](https://github.com/rust-lang/rust/issues/44580) using the unstable [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560) feature.

## Documentation
For documentation visit [docs.rs](https://docs.rs/const-guards).

## Motivation
Consider the following usage of the `first` method on arrays from the standard library: 
```rust
let array: [(); 1] = [(); 1];
let head: Option<&()> = array.first();
``` 
Would it be nice if we could just write
```rust
let head: &() = array.first();
```
since the compiler should know this array has length `1` at this point.
With const guards we can express such as follows:
```rust
#[guard(N > 0)]
fn first<'a, T, const N: usize>(array: &'a [T; N]) -> &'a T {
    &array[0]
}
```
The index call on the array `&array[0]` cannot possible fail because we enforced the length of the array to be `> 0` at compile time. We could now call it as follows 
```rust
let array: [(); 1] = [(); 1];
let head: &() = first(&array);
```
while the case where the array is actually empty would _fail to compile_:
```
let array: [(); 0] = [(); 0];
let head: &() = first(&array);
```
Finally we could even express this as a trait to make it more accessable:
```rust
trait ArrayHead<T, const N: usize> {
    #[guard(<const N: usize> { N > 0 })]
    fn head(&self) -> &T;
}

impl<T, const N: usize> ArrayHead<T, N> for [T; N] {
    fn head(&self) -> &T {
        &self[0]
    }
}

fn main() {
    let array: &[(); 1] = &[(); 1];
    let head: &() = array.head();
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