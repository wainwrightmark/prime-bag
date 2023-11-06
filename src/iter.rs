use core::marker::PhantomData;
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

use crate::helpers::*;
use crate::prime_bag_element::PrimeBagElement;

macro_rules! prime_bag_iter {
    ($iter_x: ident, $helpers_x: ty, $nonzero_ux: ty) => {
        #[derive(Debug, Clone)]
        pub struct $iter_x<E: PrimeBagElement> {
            chunk: $nonzero_ux,
            prime_index: usize,
            phantom: PhantomData<E>,
        }

        impl<E: PrimeBagElement> $iter_x<E> {
            pub const fn new(chunk: $nonzero_ux) -> Self {
                Self {
                    chunk,
                    prime_index: 0,
                    phantom: PhantomData,
                }
            }
        }

        //TODO double ended iterator etc
        impl<E: PrimeBagElement> Iterator for $iter_x<E> {
            type Item = E;

            fn next(&mut self) -> Option<Self::Item> {
                if self.chunk == <$helpers_x>::ONE {
                    return None;
                }

                loop {
                    let prime = <$helpers_x>::get_prime(self.prime_index)?;
                    if let Some(new_chunk) = <$helpers_x>::div_exact(self.chunk, prime) {
                        self.chunk = new_chunk;
                        return Some(E::from_prime_index(self.prime_index));
                    } else {
                        self.prime_index += 1;
                    }
                }
            }
        }
    };
}

prime_bag_iter!(PrimeBagIter8, Helpers8, NonZeroU8);
prime_bag_iter!(PrimeBagIter16, Helpers16, NonZeroU16);
prime_bag_iter!(PrimeBagIter32, Helpers32, NonZeroU32);
prime_bag_iter!(PrimeBagIter64, Helpers64, NonZeroU64);
prime_bag_iter!(PrimeBagIter128, Helpers128, NonZeroU128);
