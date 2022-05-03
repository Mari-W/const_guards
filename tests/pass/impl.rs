#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

extern crate const_guards;
use const_guards::guard;

fn main() {
    let _ = B([0; 1]);
}

unsafe trait A<const N: usize> {}
struct B<T, const N: usize>([T; N]);

#[guard(N > 0)]
default unsafe impl<T, const N: usize> A<N> for B<T, N> where T: Eq {}
