//! Set operations: the methods borrow both sets and return lazy iterators,
//! the operators on `&a` and `&b` build a new set, and the predicates build
//! nothing at all.
//!
//!   rustc --edition 2024 set_operations.rs -o /tmp/setops && /tmp/setops

use std::any::type_name_of_val;
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// A HashSet iterates in a different order in every process, so every
/// HashSet on this page is printed through a BTreeSet.
fn show(set: &HashSet<i32>) -> BTreeSet<i32> {
    set.iter().copied().collect()
}

fn hs<const N: usize>(items: [i32; N]) -> HashSet<i32> {
    HashSet::from(items)
}

fn bs<const N: usize>(items: [i32; N]) -> BTreeSet<i32> {
    BTreeSet::from(items)
}

fn main() {
    let a = hs([1, 2, 3, 4]);
    let b = hs([3, 4, 5]);

    println!("1. Methods: borrow both sets, return a lazy iterator of &T");
    println!("   a = {:?}   b = {:?}", show(&a), show(&b));
    let u = a.union(&b);
    println!("   a.union(&b) is a {}", type_name_of_val(&u));
    let row = |name: &str, v: BTreeSet<i32>| println!("   a.{name:<28} {v:?}");
    row("union(&b)", a.union(&b).copied().collect());
    row("intersection(&b)", a.intersection(&b).copied().collect());
    row("difference(&b)", a.difference(&b).copied().collect());
    row("symmetric_difference(&b)", a.symmetric_difference(&b).copied().collect());
    println!("   b.difference(&a)               {:?}   <- not the same question",
             b.difference(&a).copied().collect::<BTreeSet<i32>>());
    println!("   a.intersection(&b).count() = {}   (counted, nothing collected)",
             a.intersection(&b).count());

    println!();
    println!("2. Operators on references: a new, owned set every time");
    let or = &a | &b;
    println!("   &a | &b is a {}", type_name_of_val(&or));
    println!("   &a | &b = {:?}", show(&or));
    println!("   &a & &b = {:?}", show(&(&a & &b)));
    println!("   &a - &b = {:?}", show(&(&a - &b)));
    println!("   &a ^ &b = {:?}", show(&(&a ^ &b)));
    println!("   a and b are still usable afterwards: {} and {} members", a.len(), b.len());
    println!("   BTreeSet has the same four: {:?}", &bs([1, 2, 3, 4]) - &bs([3, 4, 5]));

    println!();
    println!("3. Yes-or-no questions build nothing");
    let small = hs([3, 4]);
    let empty: HashSet<i32> = HashSet::new();
    println!("   small.is_subset(&a)       = {}", small.is_subset(&a));
    println!("   a.is_superset(&small)     = {}", a.is_superset(&small));
    println!("   a.is_subset(&a)           = {}   (subset, not proper subset)", a.is_subset(&a));
    println!("   proper: small ⊂ a         = {}", small.is_subset(&a) && small != a);
    println!("   empty.is_subset(&small)   = {}", empty.is_subset(&small));
    println!("   a.is_disjoint(&b)         = {}", a.is_disjoint(&b));
    println!("   a.is_disjoint(&hs([9]))   = {}", a.is_disjoint(&hs([9])));
    println!("   a.contains(&5)            = {}", a.contains(&5));

    println!();
    println!("4. insert and remove answer with a bool");
    let mut s: HashSet<i32> = HashSet::new();
    let first = s.insert(7);
    let again = s.insert(7);
    println!("   insert(7) = {first}   insert(7) again = {again}   len = {}", s.len());
    let gone = s.remove(&7);
    let gone_again = s.remove(&7);
    println!("   remove(&7) = {gone}   remove(&7) again = {gone_again}   len = {}", s.len());
    s.insert(8);
    println!("   take(&8) = {:?}   take(&8) again = {:?}", s.take(&8), s.take(&8));

    println!();
    println!("5. The element type decides what a set can hold");
    println!("   HashSet<T> needs T: Eq + Hash; BTreeSet<T> needs T: Ord.");
    println!("   f64 is neither: NaN != NaN, so HashSet<f64>::insert does not compile.");
    let bits: HashSet<u64> = [0.0_f64, -0.0, f64::NAN, f64::NAN].iter().map(|x| x.to_bits()).collect();
    println!("   0.0 == -0.0 is {}, but as to_bits() keys: {} members from [0.0, -0.0, NaN, NaN]",
             0.0_f64 == -0.0, bits.len());
    let ints: BTreeSet<i64> = [1, 1, 1].into_iter().collect();
    println!("   [1, 1, 1] -> {ints:?}   (1, 1.0 and true are three types, not three spellings of one value)");

    println!();
    println!("6. Chained symmetric difference: members of an ODD number of sets");
    let x = bs([0, 1, 2, 3, 4]);
    let y = bs([2, 3, 4]);
    let z = bs([2, 5]);
    println!("   {x:?} ^ {y:?} ^ {z:?} = {:?}", &(&x ^ &y) ^ &z);
    let four = [bs([1, 2, 3]), bs([3, 4, 5]), bs([5, 6, 7]), bs([7, 8, 1])];
    let chained = four.iter().fold(BTreeSet::new(), |acc, set| &acc ^ set);
    println!("   {:?} ^ {:?} ^ {:?} ^ {:?} = {chained:?}", four[0], four[1], four[2], four[3]);
    let mut counts: BTreeMap<i32, usize> = BTreeMap::new();
    for set in &four {
        for &n in set {
            *counts.entry(n).or_insert(0) += 1;
        }
    }
    println!("   how many sets hold each: {counts:?}");
    let odd: BTreeSet<i32> = counts.iter().filter(|&(_, c)| c % 2 == 1).map(|(&n, _)| n).collect();
    println!("   the odd counts: {odd:?}   same set: {}", odd == chained);

    println!();
    println!("7. Two laws from the algebra of sets, checked on these sets");
    let c = hs([4, 5, 6]);
    println!("   a - (b | c) == (a - b) & (a - c)   {}", &a - &(&b | &c) == &(&a - &b) & &(&a - &c));
    println!("   a ^ b == (a | b) - (a & b)         {}", &a ^ &b == &(&a | &b) - &(&a & &b));

    println!();
    println!("8. Building, comparing, filtering in place");
    let forwards = hs([1, 2, 3]);
    let backwards: HashSet<i32> = [3, 2, 1, 2].into_iter().collect();
    println!("   {{1, 2, 3}} == collect([3, 2, 1, 2]) is {}   (order and repeats ignored)", forwards == backwards);
    let mut grow = hs([1, 2]);
    grow.extend([2, 3, 4]);
    println!("   {{1, 2}}.extend([2, 3, 4]) -> {:?}", show(&grow));
    grow.retain(|n| n % 2 == 0);
    println!("   retain(even)             -> {:?}", show(&grow));

    println!();
    println!("9. Deduplicating a Vec: two answers, two orders");
    let visits = vec![3, 1, 1, 2, 3, 1];
    let mut seen: HashSet<i32> = HashSet::new();
    let first_seen: Vec<i32> = visits.iter().copied().filter(|n| seen.insert(*n)).collect();
    println!("   {visits:?}");
    println!("   seen.insert as the filter -> {first_seen:?}   first-seen order");
    let mut sorted = visits.clone();
    sorted.sort();
    sorted.dedup();
    println!("   sort then dedup           -> {sorted:?}   sorted order");
    let mut unsorted = visits.clone();
    unsorted.dedup();
    println!("   dedup without the sort    -> {unsorted:?}   adjacent repeats only");
}
