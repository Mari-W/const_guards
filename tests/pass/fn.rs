#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
extern crate const_guards;
use const_guards::guard;

fn main() {
    f::<(), 1>();
}

#[guard(N > 0)]
pub fn f<T, const N: usize>() -> [T; N]
where
    T: Default + Copy,
{
    [T::default(); N]
}
