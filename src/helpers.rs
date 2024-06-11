use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

macro_rules! helpers {
    ($helpers_x: ident, $nonzero_ux: ty, $ux: ty, $num_primes: expr, $gcd_func: expr) => {
        pub(crate) struct $helpers_x;

        impl $helpers_x {
            pub(crate) const PRIMES: [$nonzero_ux; Self::NUM_PRIMES] = {
                let mut current: $nonzero_ux = <$nonzero_ux>::MIN.saturating_add(1);
                let mut arr: [$nonzero_ux; Self::NUM_PRIMES] =
                    [<$nonzero_ux>::MIN; Self::NUM_PRIMES];
                let mut index: usize = 0;

                while index < arr.len() {
                    let mut sieve_index = 0;
                    let mut factor_found = false;
                    while sieve_index < index {
                        let factor: $nonzero_ux = arr[sieve_index];
                        if current.get().rem_euclid(factor.get()) == 0 {
                            factor_found = true;
                            break;
                        }
                        sieve_index += 1;
                    }
                    if !factor_found {
                        arr[index] = current;
                        index += 1;
                    }
                    current = current.saturating_add(1);
                }

                let mut arr1 = [Self::ONE; Self::NUM_PRIMES];
                let mut index: usize = 0;
                while index < arr.len() {
                    let u = arr[index];
                    arr1[index] = u;
                    index += 1;
                }

                arr1
            };

            pub const NUM_PRIMES: usize = $num_primes;

            #[inline]
            pub const fn get_prime(i: usize) -> Option<$nonzero_ux> {
                if i < Self::PRIMES.len() {
                    let p = Self::PRIMES[i];
                    Some(p)
                } else {
                    None
                }
            }

            pub const ONE: $nonzero_ux = <$nonzero_ux>::MIN;

            #[inline]
            pub const fn div_exact(x: $nonzero_ux, other: $nonzero_ux) -> Option<$nonzero_ux> {
                let x: $ux = x.get();
                let other = other.get();

                let rem = x % other;
                let quo = x / other;

                if rem == 0 {
                    return <$nonzero_ux>::new(quo); //quo must be non zero here because math
                }
                None
            }

            #[inline]
            pub(crate) const fn is_multiple(x: $nonzero_ux, other: $nonzero_ux) -> bool {
                let x: $ux = x.get();
                let other: $ux = other.get();

                let rem = x % other;
                rem == 0
            }

            #[inline]
            pub(crate) const fn gcd(lhs: $nonzero_ux, rhs: $nonzero_ux) -> $nonzero_ux {
                $gcd_func(lhs, rhs)
            }

            #[inline]
            pub(crate) const fn lcm(lhs: $nonzero_ux, rhs: $nonzero_ux) -> Option<$nonzero_ux> {
                let gcd = Self::gcd(lhs, rhs);

                let Some(divided) = Self::div_exact(lhs, gcd) else {
                    return None;
                };
                let Some(lcm) = rhs.checked_mul(divided) else {
                    return None;
                }; // Note LCM is a*b / gcd

                Some(lcm)
            }

            #[inline]
            pub(crate) const fn count_chunk(chunk: $nonzero_ux, mut prime_index: usize) -> usize {
                let mut count = 0usize;

                let mut chunk = if prime_index == 0 {
                    let tz = chunk.trailing_zeros();

                    count += tz as usize;
                    prime_index = 1;
                    chunk.get() >> tz
                } else {
                    chunk.get()
                };

                let mut prime = if let Some(prime) = Self::get_prime(prime_index) {
                    prime.get()
                } else {
                    core::debug_assert!(false, "Prime index is out of range");
                    return count;
                };

                if chunk == 1 {
                    return count;
                }

                loop {
                    if chunk % prime == 0 {
                        chunk /= prime;
                        count += 1;
                        if chunk == 1 {
                            return count;
                        }
                    } else {
                        prime_index += 1;
                        prime = if let Some(prime) = Self::get_prime(prime_index) {
                            prime.get()
                        } else {
                            core::debug_assert!(false, "Prime index is out of range");
                            return count;
                        };
                    }
                }
            }

            /// Search for the largest prime greater than or equal to number, skipping the first `skip` primes
            /// Returns `Ok(index)` if the number is prime, where `index` is the index of that prime
            /// Returns `Err(index)` if the number is not prime, where `index` is the index of the next prime after `number`
            #[inline]
            pub(crate) fn find_largest_possible_prime(
                skip: usize,
                number: $nonzero_ux,
            ) -> Result<usize, usize> {
                match Self::PRIMES[skip..].binary_search(&number) {
                    Ok(offset) => Ok(offset + skip),
                    Err(offset) => Err(offset + skip),
                }
            }
        }
    };
}

// todo I believe the euclid algorithm is faster than the binary for u8/u16/u32 but slower otherwise

#[cfg(not(feature = "primes256"))]
helpers!(Helpers8, NonZeroU8, u8, 32, gcd::binary_nonzero_u8);
#[cfg(not(feature = "primes256"))]
helpers!(Helpers16, NonZeroU16, u16, 32, gcd::binary_nonzero_u16);
#[cfg(not(feature = "primes256"))]
helpers!(Helpers32, NonZeroU32, u32, 32, gcd::binary_nonzero_u32);
#[cfg(not(feature = "primes256"))]
helpers!(Helpers64, NonZeroU64, u64, 32, gcd::binary_nonzero_u64);
#[cfg(not(feature = "primes256"))]
helpers!(Helpers128, NonZeroU128, u128, 32, gcd::binary_nonzero_u128);

#[cfg(feature = "primes256")]
helpers!(Helpers8, NonZeroU8, u8, 54, gcd::binary_nonzero_u8);
#[cfg(feature = "primes256")]
helpers!(Helpers16, NonZeroU16, u16, 256, gcd::binary_nonzero_u16);
#[cfg(feature = "primes256")]
helpers!(Helpers32, NonZeroU32, u32, 256, gcd::binary_nonzero_u32);
#[cfg(feature = "primes256")]
helpers!(Helpers64, NonZeroU64, u64, 256, gcd::binary_nonzero_u64);
#[cfg(feature = "primes256")]
helpers!(Helpers128, NonZeroU128, u128, 256, gcd::binary_nonzero_u128);

const_assert_eq!(Helpers8::PRIMES[0].get(), 2u8);
const_assert_eq!(Helpers8::PRIMES[1].get(), 3u8);
const_assert_eq!(Helpers8::PRIMES[31].get(), 131u8);
#[cfg(feature = "primes256")]
const_assert_eq!(Helpers8::PRIMES[53].get(), 251u8);

const_assert_eq!(Helpers16::PRIMES[0].get(), 2u16);
const_assert_eq!(Helpers16::PRIMES[1].get(), 3u16);
const_assert_eq!(Helpers16::PRIMES[31].get(), 131u16);
#[cfg(feature = "primes256")]
const_assert_eq!(Helpers16::PRIMES[255].get(), 1619u16);

const_assert_eq!(Helpers32::PRIMES[31].get(), 131u32);
#[cfg(feature = "primes256")]
const_assert_eq!(Helpers32::PRIMES[255].get(), 1619u32);

const_assert_eq!(Helpers64::PRIMES[31].get(), 131u64);
#[cfg(feature = "primes256")]
const_assert_eq!(Helpers64::PRIMES[255].get(), 1619u64);

const_assert_eq!(Helpers128::PRIMES[31].get(), 131u128);
#[cfg(feature = "primes256")]
const_assert_eq!(Helpers128::PRIMES[255].get(), 1619u128);

#[cfg(test)]
mod tests {

    //use super::*;

    // #[test]
    // fn test_abc() {
    //     for x in Helpers16::PRIMES {
    //         std::println!("{x}")
    //     }
    // }
}
