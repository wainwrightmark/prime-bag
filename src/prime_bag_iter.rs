use core::marker::PhantomData;

use crate::backing::Backing;

#[derive(Debug, Clone)]
pub struct PrimeBagIter<B : Backing, E : From<usize>>{
    chunk: B,
    prime_index: usize,
    phantom: PhantomData<E>,
}

impl<B: Backing, E: From<usize>> PrimeBagIter<B, E> {
    pub fn new(chunk: B) -> Self { Self { chunk, prime_index: 0, phantom: PhantomData } }
}

//TODO double ended iterator etc
impl<B: Backing, E: From<usize>> Iterator for PrimeBagIter<B, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk == B::ONE{
            return None;
        }

        loop {
            let prime = B::get_prime(self.prime_index)?;
            if let Some(new_chunk) = self.chunk.div_exact(prime){
                self.chunk = new_chunk;
                return Some(E::from(self.prime_index));
            }
            else{
                self.prime_index += 1;
            }
        }

    }
}