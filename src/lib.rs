#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![doc(html_root_url = "https://docs.rs/prime_bag/0.4.0")]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(warnings, dead_code, unused_imports, unused_mut)]
#![warn(clippy::pedantic)]

//! # prime bag
//! ![GITHUB](https://img.shields.io/github/last-commit/wainwrightmark/prime_bag)
//!
//! A bag datatype that used unsigned integers for storage.
//! This works by assigning each possible item a prime number.
//! The contents of the bag is represented by the product of those prime numbers.
//! This works best if the set of possible items is constrained and some items are much more common than others.
//! To maximize the possible size of the bag, assign lower prime numbers to more common items.
//!
//! Using prime bags, certain operations can be done very efficiently:
//! Adding an element or combining two bags is achieved by multiplication.
//! Removing an element or bag of elements is achieved by division.
//! Testing for the presence of an element is achieved by modulus.
//!
//! |    Set Operation    |     Math Operation     |
//! | :-----------------: | :--------------------: |
//! |   Insert / Extend   |     Multiplication     |
//! |       Remove        |        Division        |
//! | Contains / Superset |        Modulus         |
//! |    Intersection     | Greatest Common Factor |
//!
//! Elements of the Bag must implement `PrimeBagElement`
//! Currently only 128 different element values are supported, but if necessary I could increase this
//!
//!
//! ## Getting started
//!
//! ```rust
//! use prime_bag::*;
//!
//! #[derive(Debug)]
//! pub struct MyElement(usize);
//!
//! impl PrimeBagElement for MyElement {
//!     fn to_prime_index(&self) -> usize {
//!         self.0
//!     }
//!
//!     fn from_prime_index(value: usize) -> Self {
//!         Self(value)
//!     }
//! }
//!
//! let bag = PrimeBag16::<MyElement>::try_from_iter([MyElement(1), MyElement(2), MyElement(2)]).unwrap();
//! let bag2 = bag.try_extend([MyElement(3), MyElement(3), MyElement(3)]).unwrap();
//!
//! let items : Vec<(MyElement, core::num::NonZeroUsize)> = bag2.iter_groups().collect();
//! let inner_items: Vec<(usize, usize)> = items.into_iter().map(|(element, count)|(element.0, count.get())).collect();
//!
//! assert_eq!(inner_items, vec![(1,1), (2,2), (3,3)])
//! ```

#[macro_use]
extern crate static_assertions;

/// Iterator of groups of elements
pub mod group_iter;
mod helpers;
/// Iterator of elements
pub mod iter;

use core::marker::PhantomData;
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
use group_iter::{
    PrimeBagGroupIter128, PrimeBagGroupIter16, PrimeBagGroupIter32, PrimeBagGroupIter64,
    PrimeBagGroupIter8,
};

use crate::{
    helpers::{Helpers128, Helpers16, Helpers32, Helpers64, Helpers8},
    iter::{PrimeBagIter128, PrimeBagIter16, PrimeBagIter32, PrimeBagIter64, PrimeBagIter8},
};

/// Indicates a type that can be put into a Prime Bag
/// To implement correctly, every possible value of this type must map to a unique number
/// And that number must map back to that element.
/// To maximize possible bag size and performance, use the lowest numbers possible and assign lower numbers to more common elements.
/// The element which maps to `0` will be able to use compiler intrinsics for some operations, particularly `count_instances` making them much faster
pub trait PrimeBagElement {    
    /// The index of this element.
    /// This should be a different value for each element
    /// Only values in the range `0..32` are valid unless the `primes256` feature is specified, in which case the range in `0..256`
    /// Please contact me if you need larger values and I will add a feature for them.
    fn to_prime_index(&self) -> usize;

    /// Creates an element from a prime index.
    /// If you are using this crate as intended, this will only be called on values produced by `to_prime_index`
    /// But it is possible to get a different value (e.g. by deserialization) so you must also handle this case
    fn from_prime_index(value: usize) -> Self;
}

macro_rules! prime_bag {
    ($bag_x: ident, $helpers_x: ty, $nonzero_ux: ty, $ux: ty) => {
        /// Represents a bag (multi-set) of elements
        /// The bag will have a maximum capacity
        /// Use larger sized bags (e.g. `PrimeBag64`, `PrimeBag128`) to store more elements
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct $bag_x<E>($nonzero_ux, PhantomData<E>);

        assert_eq_size!($bag_x<usize>, $ux);
        assert_eq_size!(Option<$bag_x<usize>>, $ux);

        impl<E> Default for $bag_x<E> {
            #[inline]
            fn default() -> Self {
                Self(<$helpers_x>::ONE, PhantomData)
            }
        }

        impl<E: PrimeBagElement> $bag_x<E> {
            

            /// Try to extend the bag with elements from an iterator.
            /// Does not modify this bag.
            /// Returns `None` if the resulting bag would be too large
            #[must_use]
            #[inline]
            pub fn try_extend<T: IntoIterator<Item = E>>(&self, iter: T) -> Option<Self> {
                let mut b = self.0;
                for e in iter {
                    let u: usize = e.to_prime_index();
                    let p = <$helpers_x>::get_prime(u)?;
                    b = b.checked_mul(p)?;
                }

                Some(Self(b, PhantomData))
            }

            /// Tries to create a bag from an iterator of values.
            /// Returns `None` if the resulting bag would be too large.
            #[must_use]
            #[inline]
            pub fn try_from_iter<T: IntoIterator<Item = E>>(iter: T) -> Option<Self> {
                Self::default().try_extend(iter)
            }

            /// Returns the number of instances of `value` in the bag.
            #[must_use]
            #[inline]
            pub fn count_instances(&self, value: E) -> usize {
                let u: usize = value.to_prime_index();
                // todo use binary search

                if u == 0 {
                    return self.0.trailing_zeros() as usize;
                }

                if let Some(p) = <$helpers_x>::get_prime(u) {
                    let mut n: usize = 0;
                    let mut b = self.0;

                    while let Some(new_b) = <$helpers_x>::div_exact(b, p) {
                        n += 1;
                        b = new_b;
                    }

                    return n;
                }
                return 0;
            }

            /// Returns whether the bag contains a particular `value`.
            #[must_use]
            #[inline]
            pub fn contains(&self, value: E) -> bool {
                let u: usize = value.to_prime_index();
                if let Some(p) = <$helpers_x>::get_prime(u) {
                    return <$helpers_x>::is_multiple(self.0, p);
                }
                false
            }

            /// Returns whether the bag contains a particular `value` at least `n` times.
            #[must_use]
            #[inline]
            pub fn contains_at_least(&self, value: E, n: u32) -> bool {
                let u: usize = value.to_prime_index();
                if let Some(p) = <$helpers_x>::get_prime(u) {
                    if let Some(b) = p.checked_pow(n) {
                        return <$helpers_x>::is_multiple(self.0, b);
                    }
                }
                false
            }

            /// Try to create a new bag with the `value` inserted.
            /// Does not modify the existing bag.
            /// Returns `None` if the bag does not have enough space.
            #[must_use]
            #[inline]
            pub fn try_insert(&self, value: E) -> Option<Self> {
                let u: usize = value.to_prime_index();
                let p = <$helpers_x>::get_prime(u)?;
                let b = self.0.checked_mul(p)?;
                Some(Self(b, PhantomData))
            }

            /// Try to remove `value` from this bag
            /// Returns `None` if the bag does not contain `value`
            #[inline]
            pub fn try_remove(&self, value: E) -> Option<Self> {
                let u: usize = value.to_prime_index();
                let p = <$helpers_x>::get_prime(u)?;

                match <$helpers_x>::div_exact(self.0, p) {
                    Some(b) => Some(Self(b, PhantomData)),
                    None => None,
                }
            }

            /// Try to create a new bag with the `value` inserted `n` times.
            /// Does not modify the existing bag.
            /// Returns `None` if the bag does not have enough space.
            #[must_use]
            #[inline]
            pub fn try_insert_many(&self, value: E, count: u32) -> Option<Self> {
                let u: usize = value.to_prime_index();
                let p = <$helpers_x>::get_prime(u)?;
                let p2 = p.checked_pow(count)?;
                let b = self.0.checked_mul(p2)?;
                Some(Self(b, PhantomData))
            }
        }

        impl<E> $bag_x<E> {

            /// An empty bag
            pub const EMPTY: Self = Self(<$nonzero_ux>::MIN, PhantomData);

            /// Create a bag from the inner value
            /// This can be used to convert a bag from one type to another or to enable serialization
            #[inline]
            #[must_use]
            pub const fn from_inner(inner: $nonzero_ux) -> Self {
                Self(inner, PhantomData)
            }

            /// Convert the bag to the inner value
            /// This can be used to convert a bag from one type to another or to enable serialization
            #[inline]
            #[must_use]
            pub const fn into_inner(self) -> $nonzero_ux {
                self.0
            }

            /// Returns whether this is a superset of the `rhs` bag.
            /// This is true if every element in the `rhs` bag is contained at least as many times in this.
            /// Note that this will also return true if the two bags are equal.
            #[must_use]
            #[inline]
            pub const fn is_superset(&self, rhs: &Self) -> bool {
                <$helpers_x>::is_multiple(self.0, rhs.0)
            }

            /// Returns whether this is a subset of the `rhs` bag.
            /// This is true if every element in this bag is contained at least as many times in `rhs`.
            /// Note that this will also return true if the two bags are equal.
            #[must_use]
            #[inline]
            pub const fn is_subset(&self, rhs: &Self) -> bool {
                rhs.is_superset(self)
            }

            /// Returns whether the bag contains zero elements.
            #[must_use]
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0.get() == <$helpers_x>::ONE.get()
            }

            /// Try to create the sum of this bag and `rhs`.
            /// Returns `None` if the resulting bag would be too large.
            /// The sum contains each element that is present in either bag a number of times equal to the total count of that element in both bags combined.
            #[must_use]
            #[inline]
            pub const fn try_sum(&self, rhs: &Self) -> Option<Self> {
                match self.0.checked_mul(rhs.0) {
                    Some(b) => Some(Self(b, PhantomData)),
                    None => None,
                }
            }

            /// Try to create the union of this bag and `rhs`.
            /// Returns `None` if the resulting bag would be too large.
            /// The union contains each element that is present in either bag a number of times equal to the maximum count of that element in either bag.
            #[must_use]
            #[inline]
            pub const fn try_union(&self, rhs: &Self) -> Option<Self> {
                let Some(lcm) = <$helpers_x>::lcm(self.0, rhs.0) else {
                    return None;
                };

                Some(Self(lcm, PhantomData))
            }

            /// Try to create the difference (or complement) of this bag and `rhs`.
            /// Returns `None` if this bag is not a superset of `rhs`.
            /// The difference contains each element in the first bag a number of times equal to the number of times it appears in `self` minus the number of times it appears in `rhs`
            #[must_use]
            #[inline]
            pub const fn try_difference(&self, rhs: &Self) -> Option<Self> {
                match <$helpers_x>::div_exact(self.0, rhs.0) {
                    Some(b) => Some(Self(b, PhantomData)),
                    None => None,
                }
            }

            /// Create the intersection of this bag and `rhs`.
            /// The intersection contains each element which appears in both bags a number of times equal to the minimum number of times it appears in either bag.
            #[must_use]
            #[inline]
            pub const fn intersection(&self, rhs: &Self) -> Self {
                let gcd = <$helpers_x>::gcd(self.0, rhs.0);
                Self(gcd, PhantomData)
            }
        }
    };
}

prime_bag!(PrimeBag8, Helpers8, NonZeroU8, u8);
prime_bag!(PrimeBag16, Helpers16, NonZeroU16, u16);
prime_bag!(PrimeBag32, Helpers32, NonZeroU32, u32);
prime_bag!(PrimeBag64, Helpers64, NonZeroU64, u64);
prime_bag!(PrimeBag128, Helpers128, NonZeroU128, u128);

macro_rules! into_iterator {
    ($bag_x: ty, $iter_x: ty) => {
        impl<E: PrimeBagElement> IntoIterator for $bag_x {
            type Item = E;
            type IntoIter = $iter_x;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                Self::IntoIter::new(self.0)
            }
        }
    };
}

into_iterator!(PrimeBag8<E>, PrimeBagIter8<E>);
into_iterator!(PrimeBag16<E>, PrimeBagIter16<E>);
into_iterator!(PrimeBag32<E>, PrimeBagIter32<E>);
into_iterator!(PrimeBag64<E>, PrimeBagIter64<E>);
into_iterator!(PrimeBag128<E>, PrimeBagIter128<E>);

macro_rules! from_bag_to_bag {
    ($t_from: ty, $t_into: ty) => {
        impl<E> From<$t_from> for $t_into {
            #[inline]
            fn from(value: $t_from) -> Self {
                Self(value.0.into(), PhantomData)
            }
        }
    };
}

from_bag_to_bag!(PrimeBag8<E>, PrimeBag16<E>);
from_bag_to_bag!(PrimeBag8<E>, PrimeBag32<E>);
from_bag_to_bag!(PrimeBag8<E>, PrimeBag64<E>);
from_bag_to_bag!(PrimeBag8<E>, PrimeBag128<E>);

from_bag_to_bag!(PrimeBag16<E>, PrimeBag32<E>);
from_bag_to_bag!(PrimeBag16<E>, PrimeBag64<E>);
from_bag_to_bag!(PrimeBag16<E>, PrimeBag128<E>);

from_bag_to_bag!(PrimeBag32<E>, PrimeBag64<E>);
from_bag_to_bag!(PrimeBag32<E>, PrimeBag128<E>);

from_bag_to_bag!(PrimeBag64<E>, PrimeBag128<E>);

macro_rules! group_iterator {
    ($bag_x: ty, $iter_x: ty) => {
        impl<E: PrimeBagElement> $bag_x {
            /// Iterate through groups of elements, each item of the iterator will be the element and its count.
            /// Elements which are not present are skipped.
            #[inline]
            pub fn iter_groups(&self) -> impl Iterator<Item = (E, NonZeroUsize)> {
                <$iter_x>::new(self.0)
            }
        }
    };
}

group_iterator!(PrimeBag8<E>, PrimeBagGroupIter8<E>);
group_iterator!(PrimeBag16<E>, PrimeBagGroupIter16<E>);
group_iterator!(PrimeBag32<E>, PrimeBagGroupIter32<E>);
group_iterator!(PrimeBag64<E>, PrimeBagGroupIter64<E>);
group_iterator!(PrimeBag128<E>, PrimeBagGroupIter128<E>);

#[cfg(test)]
mod tests {

    use super::*;

    impl PrimeBagElement for usize {
        fn to_prime_index(&self) -> usize {
            *self
        }

        fn from_prime_index(value: usize) -> Self {
            value
        }
    }

    #[test]
    fn test_inner() {
        let bag = PrimeBag8::<usize>::try_from_iter([1, 1, 2]).unwrap();

        let inner = bag.into_inner();

        assert_eq!(inner.get(), 45);

        let bag = PrimeBag8::<usize>::from_inner(NonZeroU8::new(45).unwrap());

        let v: Vec<_> = bag.iter_groups().collect();

        assert_eq!(
            v,
            [
                (1, NonZeroUsize::new(2).unwrap()),
                (2, NonZeroUsize::new(1).unwrap())
            ]
        );
    }

    #[test]
    fn test_iter_groups_8() {
        let bag = PrimeBag8::<usize>::try_from_iter([1, 1, 2]).unwrap();
        let v: Vec<_> = bag.iter_groups().collect();

        assert_eq!(
            v,
            [
                (1, NonZeroUsize::new(2).unwrap()),
                (2, NonZeroUsize::new(1).unwrap())
            ]
        );
    }

    #[test]
    fn test_iter_groups_16() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 1, 2]).unwrap();
        let v: Vec<_> = bag.iter_groups().collect();

        assert_eq!(
            v,
            [
                (1, NonZeroUsize::new(2).unwrap()),
                (2, NonZeroUsize::new(1).unwrap())
            ]
        );
    }

    #[test]
    fn test_iter_groups_32() {
        let bag = PrimeBag32::<usize>::try_from_iter([1, 1, 1, 3, 3, 4, 4, 4]).unwrap();
        let v: Vec<_> = bag.iter_groups().collect();

        assert_eq!(
            v,
            [
                (1, NonZeroUsize::new(3).unwrap()),
                (3, NonZeroUsize::new(2).unwrap()),
                (4, NonZeroUsize::new(3).unwrap())
            ]
        );
    }

    #[test]
    fn test_iter_groups_64() {
        let bag = PrimeBag64::<usize>::try_from_iter([1, 1, 1, 3, 3, 4, 4, 4]).unwrap();
        let v: Vec<_> = bag.iter_groups().collect();

        assert_eq!(
            v,
            [
                (1, NonZeroUsize::new(3).unwrap()),
                (3, NonZeroUsize::new(2).unwrap()),
                (4, NonZeroUsize::new(3).unwrap())
            ]
        );
    }

    #[test]
    fn test_iter_groups_128() {
        let bag = PrimeBag128::<usize>::try_from_iter([1, 1, 1, 3, 3, 4, 4, 4]).unwrap();
        let v: Vec<_> = bag.iter_groups().collect();

        assert_eq!(
            v,
            [
                (1, NonZeroUsize::new(3).unwrap()),
                (3, NonZeroUsize::new(2).unwrap()),
                (4, NonZeroUsize::new(3).unwrap())
            ]
        );
    }

    #[test]
    fn test_from_bag_to_bag() {
        let b8 = PrimeBag8::<usize>::try_from_iter([1, 2, 3]).unwrap();

        let b16: PrimeBag16<usize> = b8.into();
        let b32: PrimeBag32<usize> = b8.into();
        let b64: PrimeBag64<usize> = b8.into();
        let b128: PrimeBag128<usize> = b8.into();

        assert_eq!(b16, PrimeBag16::<usize>::try_from_iter([1, 2, 3]).unwrap());
        assert_eq!(b32, PrimeBag32::<usize>::try_from_iter([1, 2, 3]).unwrap());
        assert_eq!(b64, PrimeBag64::<usize>::try_from_iter([1, 2, 3]).unwrap());
        assert_eq!(
            b128,
            PrimeBag128::<usize>::try_from_iter([1, 2, 3]).unwrap()
        );
    }

    #[test]
    fn test_try_extend() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2]).unwrap();
        let bag2 = bag.try_extend([3, 3, 3]).unwrap();
        assert_eq!(bag.count_instances(3), 0);
        assert_eq!(bag2.count_instances(3), 3);
    }

    #[test]
    fn test_try_from_iter() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        let elements: Vec<_> = bag.into_iter().collect();
        assert_eq!(elements, [1, 2, 2, 3, 3, 3]);
    }

    #[test]
    fn test_count_instances() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        assert_eq!(bag.count_instances(0), 0);
        assert_eq!(bag.count_instances(1), 1);
        assert_eq!(bag.count_instances(2), 2);
        assert_eq!(bag.count_instances(3), 3);
        assert_eq!(bag.count_instances(1000), 0);
    }

    #[test]
    fn test_count_instances_of_zero() {
        let bag = PrimeBag16::<usize>::try_from_iter([0, 0, 0, 1, 2, 3]).unwrap();
        assert_eq!(bag.count_instances(0), 3);
    }

    #[test]
    fn test_contains() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        assert!(bag.contains(2));
        assert!(!bag.contains(4));
        assert!(!bag.contains(1000)); // it is impossible for the bag to contain this value
    }

    #[test]
    fn test_contains_at_least() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        assert!(bag.contains_at_least(2, 2));
        assert!(!bag.contains_at_least(2, 3));
        assert!(!bag.contains_at_least(1000, 1)); // it is impossible for the bag to contain this value
    }

    #[test]
    pub fn test_try_insert() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        //Note: the original bag is almost full - it has space for a 0 but not a 4
        let expected_bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3, 0]).unwrap();
        assert_eq!(bag.try_insert(0), Some(expected_bag));
        assert_eq!(bag.try_insert(4), None);
    }

    #[test]
    pub fn test_try_remove() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2]).unwrap();
        //Note: the original bag is almost full - it has space for a 0 but not a 4
        let expected_bag = PrimeBag16::<usize>::try_from_iter([1, 2]).unwrap();
        assert_eq!(bag.try_remove(2), Some(expected_bag));
        assert_eq!(bag.try_remove(3), None);
    }

    #[test]
    pub fn test_try_insert_many() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2]).unwrap();
        //Note: the original bag has space to add 3 copies of 3 but not 4 copies
        let expected_bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        assert_eq!(bag.try_insert_many(3, 3), Some(expected_bag));
        assert_eq!(bag.try_insert_many(3, 4), None);
    }

    #[test]
    pub fn test_is_superset() {
        let super_bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        let sub_bag = PrimeBag16::<usize>::try_from_iter([1, 2, 3]).unwrap();
        assert!(super_bag.is_superset(&sub_bag));
        assert!(super_bag.is_superset(&super_bag));
        assert!(!sub_bag.is_superset(&super_bag));
    }

    #[test]
    pub fn test_is_subset() {
        let super_bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        let sub_bag = PrimeBag16::<usize>::try_from_iter([1, 2, 3]).unwrap();
        assert!(sub_bag.is_subset(&super_bag));
        assert!(sub_bag.is_subset(&sub_bag));
        assert!(!super_bag.is_subset(&sub_bag));
    }

    #[test]
    pub fn test_is_empty() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        assert!(!bag.is_empty());
        assert!(PrimeBag16::<usize>::default().is_empty());
    }

    #[test]
    pub fn test_try_union() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 3, 3]).unwrap();
        let bag2 = PrimeBag16::<usize>::try_from_iter([2, 3, 4]).unwrap();

        let expected_bag = PrimeBag16::<usize>::try_from_iter([1, 2, 3, 3, 4]).unwrap();
        assert_eq!(bag.try_union(&bag2), Some(expected_bag));

        let friend = PrimeBag16::<usize>::try_from_iter([5]).unwrap();

        assert_eq!(expected_bag.try_union(&friend), None); //The bag created would be too big
    }

    #[test]
    pub fn test_try_sum() {
        let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 3, 3]).unwrap();
        let bag2 = PrimeBag16::<usize>::try_from_iter([2, 3]).unwrap();
        let expected_bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        assert_eq!(bag.try_sum(&bag2), Some(expected_bag));
        assert_eq!(expected_bag.try_sum(&expected_bag), None); //The bag created would be too big
    }

    #[test]
    pub fn test_intersection() {
        let bag_1_1_3 = PrimeBag16::<usize>::try_from_iter([1, 1, 3]).unwrap();
        let bag_1_2 = PrimeBag16::<usize>::try_from_iter([1, 2]).unwrap();
        let expected_bag = PrimeBag16::<usize>::try_from_iter([1]).unwrap();
        assert_eq!(bag_1_1_3.intersection(&bag_1_2), expected_bag);
    }

    #[test]
    pub fn test_try_difference() {
        let bag1 = PrimeBag16::<usize>::try_from_iter([1, 2, 2, 3, 3, 3]).unwrap();
        let bag2 = PrimeBag16::<usize>::try_from_iter([2, 3]).unwrap();
        let expected_bag = PrimeBag16::<usize>::try_from_iter([1, 2, 3, 3]).unwrap();
        assert_eq!(bag1.try_difference(&bag2), Some(expected_bag));
        assert_eq!(bag2.try_difference(&bag1), None); //bag2 is not a superset of bag1
    }

    #[test]
    pub fn test_iter_size_hint() {
        let mut bag = PrimeBag16::<usize>::default();

        assert_eq!((0, Some(0)), bag.into_iter().size_hint());
        let mut expected_count = 0;
        assert_eq!(expected_count, bag.into_iter().count());

        for (to_add, min, max) in [
            (0, 1, 1),
            (0, 2, 2),
            (1, 3, 3),
            (1, 3, 4),
            (2, 3, 5),
            (4, 3, 7),
        ] {
            bag = bag.try_insert(to_add).unwrap();

            assert_eq!((min, Some(max)), bag.into_iter().size_hint());

            expected_count += 1;
            assert_eq!(expected_count, bag.into_iter().count());
        }

        let mut iter = bag.into_iter();

        for ec in (0..=expected_count).rev() {
            assert_eq!(ec, iter.clone().count());
            iter.next();
        }
    }



    #[test]
    pub fn test_iter_reverse() {
        let expected: Vec<usize> = vec![0, 0, 0, 1, 1, 2, 2, 3, 3, 5, 7, 13, 19];
        let bag = PrimeBag128::<usize>::try_from_iter(expected.clone()).unwrap();

        let mut actual: Vec<usize> = bag.into_iter().rev().collect();
        actual.reverse();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_iter_nth() {
        let expected: Vec<usize> = vec![0, 0, 0, 1, 1, 2, 2, 3, 3, 5, 7, 13, 19];
        let bag = PrimeBag128::<usize>::try_from_iter(expected.clone()).unwrap();

        for n in 0..=expected.len() {
            let e = expected.iter().nth(n).copied();
            let a = bag.into_iter().nth(n);

            assert_eq!(e, a);
        }
    }

    #[test]
    pub fn test_iter_last() {
        let expected: Vec<usize> = vec![0, 0, 0, 1, 1, 2, 2, 3, 3, 5, 7, 13, 19];
        let bag = PrimeBag128::<usize>::try_from_iter(expected.clone()).unwrap();

        assert_eq!(expected.last().copied(), bag.into_iter().last());
    }

    #[test]
    pub fn test_empty(){
        let bag = PrimeBag128::<usize>::EMPTY;

        assert!(bag.is_empty());
    }
}
