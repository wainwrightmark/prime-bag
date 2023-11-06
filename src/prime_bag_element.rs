use core::usize;

pub trait PrimeBagElement{
    fn into_prime_index(&self)-> usize;

    fn from_prime_index(value: usize)-> Self;
}