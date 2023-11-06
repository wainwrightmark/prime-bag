#![cfg_attr(not(any(test, feature = "std")), no_std)]
#[macro_use]
extern crate static_assertions;

pub mod iter;
pub mod prime_bag_element;
pub mod group_iter;
mod helpers;

use core::marker::PhantomData;
use core::num::*;

use group_iter::*;

use crate::{helpers::*, iter::*, prime_bag_element::*};

macro_rules! prime_bag {
    ($bag_x: ident, $helpers_x: ty, $nonzero_ux: ty, $ux: ty) => {
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct $bag_x<E>($nonzero_ux, PhantomData<E>);

        assert_eq_size!($bag_x<usize>, $ux);
        assert_eq_size!(Option<$bag_x<usize>>, $ux);

        impl<E> Default for $bag_x<E> {
            fn default() -> Self {
                Self(<$helpers_x>::ONE, PhantomData)
            }
        }

        impl<E: PrimeBagElement> $bag_x<E> {
            /// Try to extend the bag with elements from an iterator.
            /// Does not modify this bag.
            /// Returns `None` if the resulting bag would be too large
            #[must_use]
            pub fn try_extend<T: IntoIterator<Item = E>>(&self, iter: T) -> Option<Self> {
                let mut b = self.0;
                for e in iter {
                    let u: usize = e.into_prime_index();
                    let p = <$helpers_x>::get_prime(u)?;
                    b = b.checked_mul(p)?;
                }

                Some(Self(b, PhantomData))
            }

            /// Tries to create a bag from an iterator of values.
            /// Returns `None` if the resulting bag would be too large.
            #[must_use]
            pub fn try_from_iter<T: IntoIterator<Item = E>>(iter: T) -> Option<Self> {
                Self::default().try_extend(iter)
            }

            /// Returns the number of instances of `value` in the bag.
            #[must_use]
            pub fn count_instances(&self, value: E) -> usize {
                let u: usize = value.into_prime_index();
                // todo use binary search

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
            pub fn contains(&self, value: E) -> bool {
                let u: usize = value.into_prime_index();
                if let Some(p) = <$helpers_x>::get_prime(u) {
                    return <$helpers_x>::is_multiple(self.0, p);
                }
                false
            }

            /// Returns whether the bag contains a particular `value` at least `n` times.
            #[must_use]
            pub fn contains_at_least(&self, value: E, n: u32) -> bool {
                let u: usize = value.into_prime_index();
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
            pub fn try_insert(&self, value: E) -> Option<Self> {
                let u: usize = value.into_prime_index();
                let p = <$helpers_x>::get_prime(u)?;
                let b = self.0.checked_mul(p)?;
                Some(Self(b, PhantomData))
            }

            /// Try to remove `value` from this bag
            /// Returns `None` if the bag does not contain `value`
            pub fn try_remove(&self, value: E)-> Option<Self>{
                let u: usize = value.into_prime_index();
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
            pub fn try_insert_many(&self, value: E, count: u32) -> Option<Self> {
                let u: usize = value.into_prime_index();
                let p = <$helpers_x>::get_prime(u)?;
                let p2 = p.checked_pow(count)?;
                let b = self.0.checked_mul(p2)?;
                Some(Self(b, PhantomData))
            }
        }

        impl<E> $bag_x<E> {
            /// Returns whether this is a superset of the `rhs` bag.
            /// This is true if every element in the `rhs` bag is contained at least as many times in this.
            /// Note that this will also return true if the two bags are equal.
            #[must_use]
            pub const fn is_superset(&self, rhs: &Self) -> bool {
                <$helpers_x>::is_multiple(self.0, rhs.0)
            }

            /// Returns whether this is a subset of the `rhs` bag.
            /// This is true if every element in this bag is contained at least as many times in `rhs`.
            /// Note that this will also return true if the two bags are equal.
            #[must_use]
            pub const fn is_subset(&self, rhs: &Self) -> bool {
                rhs.is_superset(self)
            }

            /// Returns whether the bag contains zero elements.
            #[must_use]
            pub const fn is_empty(&self) -> bool {
                self.0.get() == <$helpers_x>::ONE.get()
            }

            /// Try to create the sum of this bag and `rhs`.
            /// Returns `None` if the resulting bag would be too large.
            /// The sum contains each element that is present in either bag a number of times equal to the total count of that element in both bags combined.
            #[must_use]
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
            pub const fn try_union(&self, rhs: &Self) -> Option<Self> {
                let gcd = <$helpers_x>::gcd(self.0, rhs.0);

                let Some(divided) = <$helpers_x>::div_exact(self.0, gcd) else {return None}; // Note: this should never fail
                let Some(lcm) = rhs.0.checked_mul(divided)  else {return None}; // Note LCM is a*b / gcd



                Some(Self(lcm, PhantomData))


            }

            /// Try to create the difference (or complement) of this bag and `rhs`.
            /// Returns `None` if this bag is not a superset of `rhs`.
            /// The difference contains each element in the first bag a number of times equal to the number of times it appears in `self` minus the number of times it appears in `rhs`
            #[must_use]
            pub const fn try_difference(&self, rhs: &Self) -> Option<Self> {
                match <$helpers_x>::div_exact(self.0, rhs.0) {
                    Some(b) => Some(Self(b, PhantomData)),
                    None => None,
                }
            }



            /// Create the intersection of this bag and `rhs`.
            /// The intersection contains each element which appears in both bags a number of times equal to the minimum number of times it appears in either bag.
            #[must_use]
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
            pub fn iter_groups(&self) -> impl Iterator<Item = (E, usize)> {
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

    impl PrimeBagElement for usize{
        fn into_prime_index(&self)-> usize {
            *self
        }

        fn from_prime_index(value: usize)-> Self {
            value
        }
    }

    #[test]
    fn test_iter_groups() {
        let bag = PrimeBag32::<usize>::try_from_iter([1,1,1, 3, 3, 4,4,4]).unwrap();
        let v: Vec<_> = bag.iter_groups().collect();

        assert_eq!(v, [(1, 3), (3,2), (4, 3)])
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
    pub fn test_try_union(){
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
}
