//! Kata solution: a slice type that promises its order. `Sorted<T>` is the
//! borrowed half over `[T]`, `SortedVec<T>` the owned half over `Vec<T>` —
//! the `str`/`String` pattern again, one level more generic.
//!
//!   rustc --edition 2024 implementing_to_owned_kata.rs -o /tmp/itok && /tmp/itok

use std::borrow::{Borrow, Cow};

use sorted::{Sorted, SortedVec};

mod sorted {
    use std::borrow::Borrow;

    #[repr(transparent)]
    pub struct Sorted<T> {
        items: [T],
    }

    pub struct SortedVec<T> {
        items: Vec<T>,
    }

    impl<T> Sorted<T> {
        fn from_slice_unchecked(items: &[T]) -> &Sorted<T> {
            // SAFETY: `Sorted<T>` is `#[repr(transparent)]` over `[T]`, so the
            // pointers share a layout and the element count travels with them.
            unsafe { &*(items as *const [T] as *const Sorted<T>) }
        }

        pub fn as_slice(&self) -> &[T] {
            &self.items
        }

        /// Allowed to binary-search, because the type promises order.
        pub fn contains(&self, x: &T) -> bool
        where
            T: Ord,
        {
            self.items.binary_search(x).is_ok()
        }

        /// A copy with `x` inserted where it belongs.
        pub fn with(&self, x: T) -> SortedVec<T>
        where
            T: Ord + Clone,
        {
            let mut items = self.items.to_vec();
            let at = items.partition_point(|y| *y < x);
            items.insert(at, x);
            SortedVec { items }
        }
    }

    impl<'a, T: Ord> TryFrom<&'a [T]> for &'a Sorted<T> {
        /// The index of the first element smaller than the one before it.
        type Error = usize;

        fn try_from(items: &'a [T]) -> Result<Self, usize> {
            match items.windows(2).position(|w| w[0] > w[1]) {
                Some(i) => Err(i + 1),
                None => Ok(Sorted::from_slice_unchecked(items)),
            }
        }
    }

    // No bound on T: lending the slice copies nothing.
    impl<T> Borrow<Sorted<T>> for SortedVec<T> {
        fn borrow(&self) -> &Sorted<T> {
            Sorted::from_slice_unchecked(&self.items)
        }
    }

    // T: Clone, because making the owned twin copies every element.
    impl<T: Clone> ToOwned for Sorted<T> {
        type Owned = SortedVec<T>;

        fn to_owned(&self) -> SortedVec<T> {
            SortedVec { items: self.items.to_vec() }
        }
    }

    /// The door the second question is about: public here only so `main` can
    /// show what walks through it.
    pub fn from_vec_unchecked<T>(items: Vec<T>) -> SortedVec<T> {
        SortedVec { items }
    }
}

/// Borrowed when 0 is already there; one new `SortedVec` when it is not.
fn ensure_zero(s: &Sorted<i32>) -> Cow<'_, Sorted<i32>> {
    if s.contains(&0) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.with(0))
    }
}

fn main() {
    println!("1. TryFrom is the checked door");
    for items in [&[-2, 0, 5, 9][..], &[1, 4, 3, 8][..]] {
        match <&Sorted<i32>>::try_from(items) {
            Ok(s) => println!("   {items:?} -> Ok, contains(&5) = {}", s.contains(&5)),
            Err(i) => println!("   {items:?} -> Err, order breaks at index {i}"),
        }
    }

    println!();
    println!("2. ToOwned and Borrow, round trip");
    let s: &Sorted<i32> = (&[1, 2, 3][..]).try_into().unwrap();
    let owned: SortedVec<i32> = s.to_owned();
    let back: &Sorted<i32> = owned.borrow();
    println!("   s.to_owned().borrow() = {:?}", back.as_slice());

    println!();
    println!("3. Cow over Sorted<i32>");
    for items in [&[-1, 0, 4][..], &[-1, 4][..]] {
        let s: &Sorted<i32> = items.try_into().unwrap();
        match ensure_zero(s) {
            Cow::Borrowed(b) => println!("   ensure_zero({items:?}) -> Borrowed {:?}", b.as_slice()),
            Cow::Owned(o) => {
                let lent: &Sorted<i32> = o.borrow();
                println!("   ensure_zero({items:?}) -> Owned    {:?}", lent.as_slice());
            }
        }
    }

    println!();
    println!("4. Skip the check and the damage is a wrong answer, not undefined behaviour");
    let liar = sorted::from_vec_unchecked(vec![3, 2, 1]);
    let lent: &Sorted<i32> = liar.borrow();
    println!("   {:?}.contains(&3) = {}   (3 is right there)", lent.as_slice(), lent.contains(&3));
}
