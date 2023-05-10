use core::{marker::PhantomData, num::NonZeroU8};

use crate::{backing::Backing, prime_bag_iter::PrimeBagIter};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct PrimeBag<B: Backing, E: Into<usize> + From<usize>>(B, PhantomData<E>);

impl<B: Backing, E: Into<usize> + From<usize>> IntoIterator for PrimeBag<B, E> {
    type Item = E;

    type IntoIter = PrimeBagIter<B, E>;

    fn into_iter(self) -> Self::IntoIter {
        PrimeBagIter::new(self.0)
    }
}





assert_eq_size!(PrimeBag<NonZeroU8, usize>, u8);
assert_eq_size!(Option<PrimeBag<NonZeroU8, usize>>, u8);

impl<B: Backing, E: Into<usize> + From<usize>> Default for PrimeBag<B, E> {
    fn default() -> Self {
        Self(B::ONE, Default::default())
    }
}

impl<B: Backing, E: Into<usize> + From<usize>> PrimeBag<B, E> {

    /// Try to extend the bag with elements from an iterator.
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2]).unwrap();
    ///
    /// let bag2 = bag.try_with_extended([3,3,3]).unwrap();
    /// assert_eq!(bag.count_instances(3), 0);
    /// assert_eq!(bag2.count_instances(3), 3);
    /// ```
    #[must_use]
    pub fn try_with_extended<T: IntoIterator<Item = E>>(&self, iter: T) -> Option<Self> {
        let mut b = self.0;
        for t in iter {
            let u: usize = t.into();
            let p = B::get_prime(u)?;
            b = b.checked_mul(p)?;
        }

        Some(Self(b, self.1))
    }

    /// Creates a bag from an iterator
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    ///
    /// let elements: Vec<_> = bag.into_iter().collect();
    /// assert_eq!(elements, [1,2,2,3,3,3]);
    /// ```
    #[must_use]
    pub fn try_from_iter<T: IntoIterator<Item = E>>(iter: T) -> Option<Self> {
        Self::default().try_with_extended(iter)
    }

    /// Returns the number of instances of value in the bag.
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    /// assert_eq!(bag.count_instances(0), 0);
    /// assert_eq!(bag.count_instances(1), 1);
    /// assert_eq!(bag.count_instances(2), 2);
    /// assert_eq!(bag.count_instances(3), 3);
    /// ```
    #[must_use]
    pub fn count_instances(&self, value: E) -> usize {
        let u: usize = value.into();
        // todo use binary search

        if let Some(p) = B::get_prime(u) {
            let mut n: usize = 0;
            let mut b = self.0;

            while let Some(new_b) = b.div_exact(p) {
                n += 1;
                b = new_b;
            }

            return n;
        }
        return 0;
    }

    /// Returns whether this is a superset of the `rhs` bag.
    /// This is true if every element in the `rhs` bag is contained at least as many times in this.
    /// Note that this will also return true if the two bags are equal.
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let super_bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    /// let sub_bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,3]).unwrap();
    ///
    /// assert!(super_bag.is_superset(&sub_bag));
    /// assert!(super_bag.is_superset(&super_bag));
    /// assert!(!sub_bag.is_superset(&super_bag));
    /// ```
    #[must_use]
    pub fn is_superset(&self, rhs: &Self) -> bool {
        self.0.is_multiple(rhs.0)
    }

    /// Returns whether this is a subset of the `rhs` bag.
    /// This is true if every element in this bag is contained at least as many times in `rhs`.
    /// Note that this will also return true if the two bags are equal.
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let super_bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    /// let sub_bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,3]).unwrap();
    ///
    /// assert!(sub_bag.is_subset(&super_bag));
    /// assert!(sub_bag.is_subset(&sub_bag));
    /// assert!(!super_bag.is_subset(&sub_bag));
    /// ```
    #[must_use]
    pub fn is_subset(&self, rhs: &Self) -> bool {
        rhs.is_superset(self)
    }

    /// Returns whether the bag contains a particular `value`.
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    /// assert!(bag.contains(2));
    /// assert!(!bag.contains(4));
    /// ```
    #[must_use]
    pub fn contains(&self, value: E) -> bool {
        let u: usize = value.into();
        if let Some(p) = B::get_prime(u) {
            return self.0.is_multiple(p);
        }
        false
    }

    /// Returns whether the bag contains a particular `value` at least `n` times.
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    /// assert!(bag.contains_at_least(2, 2));
    /// assert!(!bag.contains_at_least(2,3));
    /// ```
    #[must_use]
    pub fn contains_at_least(&self, value: E, n: u32) -> bool {
        let u: usize = value.into();
        if let Some(p) = B::get_prime(u) {
            if let Some(b) = p.checked_pow(n) {
                return self.0.is_multiple(b);
            }
        }
        false
    }

    /// Returns whether the bag contains zero elements.
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    /// assert!(!bag.is_empty());
    /// assert!(PrimeBag::<NonZeroU16, usize>::default().is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0 == B::ONE
    }

    /// Try to create a new bag with the `value` inserted.
    /// Does not modify the existing bag.
    /// Returns `None` if the bag does not have enough space.
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    /// //Note: the original bag is almost full - it has space for a 0 but not a 4
    /// let expected_bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3, 0]).unwrap();
    /// assert_eq!(bag.try_insert(0), Some(expected_bag));
    /// assert_eq!(bag.try_insert(4), None);
    /// ```
    #[must_use]
    pub fn try_insert(&self, value: E) -> Option<Self> {
        let u: usize = value.into();
        let p = B::get_prime(u)?;
        let b = self.0.checked_mul(p)?;
        Some(Self(b, self.1))
    }

    /// Try to create a new bag with the `value` inserted `n` times.
    /// Does not modify the existing bag.
    /// Returns `None` if the bag does not have enough space.
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2]).unwrap();
    /// //Note: the original bag has space to add 3 copies of 3 but not 4 copies
    /// let expected_bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    /// assert_eq!(bag.try_insert_many(3,3), Some(expected_bag));
    /// assert_eq!(bag.try_insert_many(3,4), None);
    /// ```
    #[must_use]
    pub fn try_insert_many(&self, value: E, count: u32) -> Option<Self> {
        let u: usize = value.into();
        let p = B::get_prime(u)?;
        let p2 = p.checked_pow(count)?;
        let b = self.0.checked_mul(p2)?;
        Some(Self(b, self.1))
    }

    /// Try to create the union of this bag and `rhs`.
    /// Does not modify the existing bags.
    /// Returns `None` if the resulting bag would be too large.
    /// The union contains each element that is present in either bag a number of times equal to the total count of that element in both bags combined.
    ///
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,3,3]).unwrap();
    /// let bag2 = PrimeBag::<NonZeroU16, usize>::try_from_iter([2,3]).unwrap();
    ///
    /// let expected_bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    /// assert_eq!(bag.try_union(&bag2), Some(expected_bag));
    /// assert_eq!(expected_bag.try_union(&expected_bag), None); //The bag created would be too big
    /// ```
    #[must_use]
    pub fn try_union(&self, rhs: &Self) -> Option<Self> {
        let b = self.0.checked_mul(rhs.0)?;
        Some(Self(b, self.1))
    }

    /// Try to create the difference (or complement) of this bag and `rhs`.
    /// Does not modify the existing bags.
    /// Returns `None` if this bag is not a superset of `rhs`.
    /// The difference contains each element in the first bag a number of times equal to the number of times it appears in the first bag minus the number of times it appears in `rhs`
    ///
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag1 = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,2,3,3,3]).unwrap();
    /// let bag2 = PrimeBag::<NonZeroU16, usize>::try_from_iter([2,3]).unwrap();
    ///
    /// let expected_bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2,3,3]).unwrap();
    /// assert_eq!(bag1.try_difference(&bag2), Some(expected_bag));
    /// assert_eq!(bag2.try_difference(&bag1), None); //bag2 is not a superset of bag1
    /// ```
    #[must_use]
    pub fn try_difference(&self, rhs: &Self) -> Option<Self> {
        let b = self.0.div_exact(rhs.0)?;
        Some(Self(b, self.1))
    }

    /// Create the intersection of this bag and `rhs`.
    /// Does not modify the existing bags.
    /// The intersection contains each element which appears in both bags a number of times equal to the minimum number of times it appears in either bag.
    /// ```
    /// use core::num::NonZeroU16;
    /// use prime_bag::prime_bag::PrimeBag;
    /// let bag_1_1_3 = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,1,3]).unwrap();
    /// let bag_1_2 = PrimeBag::<NonZeroU16, usize>::try_from_iter([1,2]).unwrap();
    ///
    /// let expected_bag = PrimeBag::<NonZeroU16, usize>::try_from_iter([1]).unwrap();
    /// assert_eq!(bag_1_1_3.intersection(&bag_1_2), expected_bag);
    /// ```
    #[must_use]
    pub fn intersection(&self, rhs: &Self) -> Self {
        let gcd =self.0.gcd(rhs.0);
        Self(gcd, Default::default())
    }
}
