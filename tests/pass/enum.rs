#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
extern crate const_guards;
use const_guards::guard;

fn main() {
    let _ = A::<(), 1>::B([(); 1]);
}

#[guard(N > 0)]
pub enum A<T, const N: usize>
where
    T: Eq,
{
    B([T; N]),
}
