use core::marker::PhantomData;
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

use crate::helpers::{Helpers128, Helpers16, Helpers32, Helpers64, Helpers8};
use crate::PrimeBagElement;

macro_rules! prime_bag_iter {
    ($iter_x: ident, $helpers_x: ty, $nonzero_ux: ty) => {
        /// Iterate through elements of a prime bag
        #[derive(Debug, Clone)]
        pub struct $iter_x<E: PrimeBagElement> {
            chunk: $nonzero_ux,
            prime_index: usize,
            phantom: PhantomData<E>,
        }

        impl<E: PrimeBagElement> $iter_x<E> {
            pub(crate) const fn new(chunk: $nonzero_ux) -> Self {
                Self {
                    chunk,
                    prime_index: 0,
                    phantom: PhantomData,
                }
            }
        }

        impl<E: PrimeBagElement> Iterator for $iter_x<E> {
            type Item = E;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.chunk == <$helpers_x>::ONE {
                    return None;
                }

                loop {
                    let prime = <$helpers_x>::get_prime(self.prime_index)?;
                    if let Some(new_chunk) = <$helpers_x>::div_exact(self.chunk, prime) {
                        self.chunk = new_chunk;
                        return Some(E::from_prime_index(self.prime_index));
                    }
                    self.prime_index += 1;
                }
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let (twos, chunk, prime_index) = if self.prime_index == 0 {
                    let tz = self.chunk.get().trailing_zeros();

                    let new_chunk = self.chunk.get() >> tz;
                    if let Ok(new_chunk) = <$nonzero_ux>::try_from(new_chunk) {
                        (tz as usize, new_chunk, 1usize)
                    } else {
                        return (tz as usize, Some(tz as usize));
                    }
                } else {
                    (0usize, self.chunk, self.prime_index)
                };

                if chunk.get() == 1 {
                    return (twos, Some(twos));
                }

                let Some(prime) = <$helpers_x>::get_prime(prime_index) else {
                    return (twos, None);
                };

                let log = chunk.get().ilog(prime.get()) as usize;

                (twos + 1, Some(twos + log))
            }

            //Don't implement min and max as we do not know the ordering of the prime bag elements

            #[inline]
            fn last(mut self) -> Option<Self::Item>
            where
                Self: Sized,
            {
                DoubleEndedIterator::next_back(&mut self)
            }

            fn count(self) -> usize
            where
                Self: Sized,
            {
                <$helpers_x>::count_chunk(self.chunk, self.prime_index)
            }

            fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
                if self.prime_index == 0 {
                    let tz = self.chunk.trailing_zeros();

                    match n.checked_sub(tz as usize) {
                        Some(new_n) => {
                            n = new_n;
                            self.chunk = <$nonzero_ux>::new(self.chunk.get() >> tz)
                                .unwrap_or(<$nonzero_ux>::MIN);
                            self.prime_index = 1;
                        }
                        None => {
                            self.chunk = <$nonzero_ux>::new(self.chunk.get() >> (n as u32 + 1))
                                .unwrap_or(<$nonzero_ux>::MIN);

                            return Some(E::from_prime_index(0));
                        }
                    }
                }

                while n > 0 {
                    let _ = self.next()?;
                    n -= 1;
                }

                self.next()
            }
        }

        impl<E: PrimeBagElement> core::iter::FusedIterator for $iter_x<E> {}

        impl<E: PrimeBagElement> DoubleEndedIterator for $iter_x<E> {
            //todo rfold, nth_back

            /// Note the performance of this is not great if called repeatedly - we have to do a bitshift and a binary search every time
            fn next_back(&mut self) -> Option<Self::Item> {
                if self.chunk == <$nonzero_ux>::MIN {
                    return None;
                }

                let (start_index, chunk) = if self.prime_index == 0 {
                    let chunk = self.chunk.get() >> self.chunk.trailing_zeros();

                    let chunk = <$nonzero_ux>::try_from(chunk).unwrap_or(<$nonzero_ux>::MIN);

                    if chunk == <$nonzero_ux>::MIN {
                        self.chunk = <$nonzero_ux>::try_from(self.chunk.get() / 2)
                            .unwrap_or(<$nonzero_ux>::MIN);
                        return Some(Self::Item::from_prime_index(0));
                    }
                    (1, chunk)
                } else {
                    (self.prime_index, self.chunk)
                };

                let mut prime_index =
                    match <$helpers_x>::find_largest_possible_prime(start_index, chunk) {
                        Ok(index) => {
                            self.chunk = <$nonzero_ux>::try_from(self.chunk.get() / chunk)
                                .unwrap_or(<$nonzero_ux>::MIN);

                            return Some(Self::Item::from_prime_index(index));
                        }
                        Err(index) => index,
                    };

                loop {
                    prime_index = prime_index.checked_sub(1)?;
                    let prime = <$helpers_x>::get_prime(prime_index)?;

                    if chunk.get() % prime == 0 {
                        self.chunk = <$nonzero_ux>::try_from(self.chunk.get() / prime)
                            .unwrap_or(<$nonzero_ux>::MIN);
                        return Some(Self::Item::from_prime_index(prime_index));
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
