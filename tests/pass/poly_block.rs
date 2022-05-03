#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
extern crate const_guards;
use const_guards::guard;

fn main() {
    let array: &[usize; 1] = &[0; 1];
    let _: &usize = head(array);
}

#[guard(<const N: usize> {N > 0})]
fn head<'a, T, const N: usize>(array: &'a [T; N]) -> &'a T {
    &array[0]
}