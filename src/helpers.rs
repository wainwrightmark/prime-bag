use core::num::NonZeroU16;

pub struct Helpers16;

const_assert_eq!(Helpers16::PRIMES[0].get(), 2u16);
const_assert_eq!(Helpers16::PRIMES[1].get(), 3u16);
const_assert_eq!(Helpers16::PRIMES[53].get(), 251u16);

impl Helpers16 {
    const PRIMES: [NonZeroU16; Self::NUM_PRIMES] = {
        let mut current: u16 = 2;
        let mut arr = [1u16; Self::NUM_PRIMES];
        let mut index: usize = 0;

        while index < arr.len() {
            let mut sieve_index = 0;
            let mut factor_found = false;
            while sieve_index < index {
                let factor: u16 = arr[sieve_index];
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
            let nz = unsafe { NonZeroU16::new_unchecked(u) };
            arr1[index] = nz;
            index += 1;
        }

        arr1
    };

    pub const NUM_PRIMES: usize = 54;

    pub const fn get_prime(i: usize) -> Option<NonZeroU16> {
        if i < Self::PRIMES.len() {
            let p = Self::PRIMES[i];
            Some(p)
        } else {
            None
        }
    }

    pub const ONE: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(1) };

    pub const fn div_exact(x: NonZeroU16, other: NonZeroU16) -> Option<NonZeroU16> {
        let x: u16 = x.get();
        let other = other.get();

        let rem = x % other;
        let quo = x / other;

        if rem == 0 {
            return NonZeroU16::new(quo); //quo must be non zero here because math
        } else {
            return None;
        }
    }

    pub const fn is_multiple(x: NonZeroU16, other: NonZeroU16) -> bool {
        let x: u16 = x.get();
        let other: u16 = other.get();

        let rem = x % other;
        rem == 0
    }
}
