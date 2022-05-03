#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
extern crate const_guards;
use const_guards::guard;

fn main() {
    let slice: &[(); 1] = &[(); 1];
    let _: &() = slice.head();
}

trait SliceHead<'a, T, const N: usize> {
    #[guard(<const N: usize> { N > 0 })]
    fn head(&self) -> &'a T;
}

impl<'a, T, const N: usize> SliceHead<'a, T, N> for &'a [T; N] {
    fn head(&self) -> &'a T {
        &self[0]
    }
}
