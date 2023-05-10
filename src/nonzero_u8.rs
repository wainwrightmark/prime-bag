use core::num::NonZeroU8;
use crate::backing::Backing;

const_assert_eq!(PRIMES_NONZERO_U8[0].get(), 2u8);
const_assert_eq!(PRIMES_NONZERO_U8[1].get(), 3u8);
const_assert_eq!(PRIMES_NONZERO_U8[53].get(), 251u8);

const PRIMES_NONZERO_U8: [NonZeroU8; NonZeroU8::NUM_PRIMES] = {
    let mut current: u8 = 2;
    let mut arr = [1u8; NonZeroU8::NUM_PRIMES];
    let mut index: usize = 0;

    while index < arr.len() {
        let mut sieve_index = 0;
        let mut factor_found = false;
        while sieve_index < index {
            let factor: u8 = arr[sieve_index];
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

    let mut arr1 = [NonZeroU8::ONE; NonZeroU8::NUM_PRIMES];
    let mut index: usize = 0;
    while index < arr.len() {
        let u = arr[index];
        let nz = unsafe { NonZeroU8::new_unchecked(u) };
        arr1[index] = nz;
        index += 1;
    }

    arr1
};


impl Backing for NonZeroU8 {
    const ONE: Self = { unsafe { NonZeroU8::new_unchecked(1) } };

    const NUM_PRIMES: usize = 54;

    fn get_prime(i: usize) -> Option<Self> {
        PRIMES_NONZERO_U8.get(i).map(|x| *x)
    }

    fn checked_mul(self, other: Self) -> Option<Self> {
        self.checked_mul(other)
    }

    fn checked_pow(self, other: u32) -> Option<Self> {
        self.checked_pow(other)
    }

    fn div_exact(self, other: Self) -> Option<Self> {
        let s: u8 = self.into();

        let rem = s % other;
        let quo = s / other;

        if rem == 0 {
            return Some(quo.try_into().unwrap()); //quo must be non zero here because math
        } else {
            return None;
        }
    }

    fn is_multiple(self, other: Self) -> bool {
        let s: u8 = self.into();

        let rem = s % other;
        rem == 0
    }
}