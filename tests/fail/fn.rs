#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
extern crate const_guards;
use const_guards::guard;

fn main() {
    f::<0>()
}

#[guard(N > 0)]
fn f<const N: usize>() {}
