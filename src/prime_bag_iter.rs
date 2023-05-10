use core::{marker::PhantomData, num::NonZeroU16};

use crate::helpers::Helpers16;


#[derive(Debug, Clone)]
pub struct PrimeBagIter16< E : From<usize>>{
    chunk: NonZeroU16,
    prime_index: usize,
    phantom: PhantomData<E>,
}

impl<E: From<usize>> PrimeBagIter16< E> {
    pub fn new(chunk: NonZeroU16) -> Self { Self { chunk, prime_index: 0, phantom: PhantomData } }
}

//TODO double ended iterator etc
impl<E: From<usize>> Iterator for PrimeBagIter16< E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk == Helpers16::ONE{
            return None;
        }

        loop {
            let prime = Helpers16::get_prime(self.prime_index)?;
            if let Some(new_chunk) = Helpers16::div_exact(self.chunk, prime){
                self.chunk = new_chunk;
                return Some(E::from(self.prime_index));
            }
            else{
                self.prime_index += 1;
            }
        }

    }
}