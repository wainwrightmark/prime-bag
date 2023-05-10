use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

macro_rules! helpers {
    ($helpers_x: ident, $nonzero_ux: ty, $ux: ty, $num_primes: expr, $gcd_func: expr) => {
        pub(crate) struct $helpers_x;

        impl $helpers_x {
            const PRIMES: [$nonzero_ux; Self::NUM_PRIMES] = {
                let mut current: $ux = 2;
                let mut arr: [$ux; Self::NUM_PRIMES] = [1; Self::NUM_PRIMES];
                let mut index: usize = 0;

                while index < arr.len() {
                    let mut sieve_index = 0;
                    let mut factor_found = false;
                    while sieve_index < index {
                        let factor: $ux = arr[sieve_index];
                        if current.rem_euclid(factor) == 0 {
                            factor_found = true;
                            break;
                        }
                        sieve_index += 1;
                    }
                    if !factor_found {
                        arr[index] = current;
                        index += 1;
                    }
                    current += 1;
                }

                let mut arr1 = [Self::ONE; Self::NUM_PRIMES];
                let mut index: usize = 0;
                while index < arr.len() {
                    let u = arr[index];
                    let nz = unsafe { <$nonzero_ux>::new_unchecked(u) };
                    arr1[index] = nz;
                    index += 1;
                }

                arr1
            };

            pub const NUM_PRIMES: usize = $num_primes;

            pub const fn get_prime(i: usize) -> Option<$nonzero_ux> {
                if i < Self::PRIMES.len() {
                    let p = Self::PRIMES[i];
                    Some(p)
                } else {
                    None
                }
            }

            pub const ONE: $nonzero_ux = unsafe { <$nonzero_ux>::new_unchecked(1) };

            pub const fn div_exact(x: $nonzero_ux, other: $nonzero_ux) -> Option<$nonzero_ux> {
                let x: $ux = x.get();
                let other = other.get();

                let rem = x % other;
                let quo = x / other;

                if rem == 0 {
                    return <$nonzero_ux>::new(quo); //quo must be non zero here because math
                } else {
                    return None;
                }
            }

            pub(crate) const fn is_multiple(x: $nonzero_ux, other: $nonzero_ux) -> bool {
                let x: $ux = x.get();
                let other: $ux = other.get();

                let rem = x % other;
                rem == 0
            }

            pub(crate) const fn gcd(lhs: $nonzero_ux, rhs: $nonzero_ux) -> $nonzero_ux {
                $gcd_func(lhs, rhs)
            }
        }
    };
}

helpers!(Helpers8, NonZeroU8, u8, 54, gcd::binary_nonzero_u8);
helpers!(Helpers16, NonZeroU16, u16, 128, gcd::binary_nonzero_u16);
helpers!(Helpers32, NonZeroU32, u32, 128, gcd::binary_nonzero_u32);
helpers!(Helpers64, NonZeroU64, u64, 128, gcd::binary_nonzero_u64);
helpers!(Helpers128, NonZeroU128, u128, 128, gcd::binary_nonzero_u128);

const_assert_eq!(Helpers8::PRIMES[0].get(), 2u8);
const_assert_eq!(Helpers8::PRIMES[1].get(), 3u8);
const_assert_eq!(Helpers8::PRIMES[53].get(), 251u8);

const_assert_eq!(Helpers16::PRIMES[0].get(), 2u16);
const_assert_eq!(Helpers16::PRIMES[1].get(), 3u16);
const_assert_eq!(Helpers16::PRIMES[127].get(), 719u16);

const_assert_eq!(Helpers16::PRIMES[127].get(), 719u16);
const_assert_eq!(Helpers32::PRIMES[127].get(), 719u32);
const_assert_eq!(Helpers64::PRIMES[127].get(), 719u64);
const_assert_eq!(Helpers128::PRIMES[127].get(), 719u128);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_abc() {
        for x in Helpers16::PRIMES {
            std::println!("{x}")
        }
    }
}
