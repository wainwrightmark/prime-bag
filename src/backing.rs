pub trait Backing: Eq + PartialEq + Ord + PartialOrd + Sized + Copy + gcd::Gcd {
    const ONE: Self;

    const NUM_PRIMES: usize;

    fn get_prime(i: usize) -> Option<Self>;

    fn checked_mul(self, other: Self) -> Option<Self>;

    fn checked_pow(self, other: u32) -> Option<Self>;

    /// Returns None if other is not a factor, otherwise Some(self / other)
    fn div_exact(self, other: Self) -> Option<Self>;

    fn is_multiple(self, other: Self) -> bool;
}