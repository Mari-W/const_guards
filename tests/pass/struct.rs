#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
extern crate const_guards;
use const_guards::guard;

fn main() {
    let _ = A::<(), 1>([(); 1]);
    let _ = B::<(), 1> { b: [(); 1] };
}

#[guard(N > 0)]
pub struct A<T, const N: usize>([T; N])
where
    T: Eq;

#[guard(N > 0)]
pub struct B<T, const N: usize>
where
    T: Eq,
{
    b: [T; N],
}
