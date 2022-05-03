#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
extern crate const_guards;
use const_guards::guard;

fn main() {
    let array: &[(); 1] = &[(); 1];
    let _: &() = array.head();
}

trait ArrayHead<T, const N: usize> {
    #[guard(<const N: usize> { N > 0 })]
    fn head(&self) -> &T;
}

impl<T, const N: usize> ArrayHead<T, N> for [T; N] {
    fn head(&self) -> &T {
        &self[0]
    }
}
