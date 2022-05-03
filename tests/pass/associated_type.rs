#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

extern crate const_guards;
use const_guards::guard;

fn main() {
    let _ = B;
}

trait A {
    #[guard(N > 0)]
    type B<T, const N: usize> = [T; N];
}
struct B;

impl A for B {}
