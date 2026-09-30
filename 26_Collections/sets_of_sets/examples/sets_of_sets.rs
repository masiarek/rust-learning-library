// Sets of sets: why Rust needs no frozenset.
//
// Python has two set types because a `set` can change after it is stored, so
// it cannot be hashed; `frozenset` gives up every mutating method to become
// hashable. Rust has one `HashSet` and one `BTreeSet`, because mutability
// belongs to the binding, and because no safe, stable API hands out `&mut` to
// an element that is inside a set.
//
// Every HashSet here is printed through a BTreeSet (see `show`), except in
// section 3, where the iteration order is the point and a fixed hasher makes
// it the same on every run.

use std::cell::Cell;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::hash::{BuildHasherDefault, DefaultHasher, Hash, Hasher};

fn show<T: Ord + Debug>(items: impl IntoIterator<Item = T>) -> String {
    format!("{:?}", items.into_iter().collect::<BTreeSet<T>>())
}

fn hash_of<T: Hash>(value: &T) -> u64 {
    let mut h = DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}

fn add_one(mut s: BTreeSet<i32>) -> BTreeSet<i32> {
    s.insert(1);
    s
}

fn power_set(base: &BTreeSet<i32>) -> BTreeSet<BTreeSet<i32>> {
    let mut all = BTreeSet::from([BTreeSet::new()]);
    for &x in base {
        let with_x: Vec<BTreeSet<i32>> = all
            .iter()
            .map(|s| {
                let mut t = s.clone();
                t.insert(x);
                t
            })
            .collect();
        all.extend(with_x);
    }
    all
}

fn main() {
    println!("1. A set whose members are sets: the inner set is a BTreeSet");
    let family: BTreeSet<BTreeSet<i32>> = BTreeSet::from([
        BTreeSet::from([1, 2]),
        BTreeSet::from([2, 3]),
        BTreeSet::from([2, 1]),
    ]);
    println!("   BTreeSet<BTreeSet<i32>> from [{{1, 2}}, {{2, 3}}, {{2, 1}}] = {family:?}");
    println!("   len = {}   ({{1, 2}} and {{2, 1}} are one member)", family.len());
    println!("   contains {{2, 1}}? {}", family.contains(&BTreeSet::from([2, 1])));
    println!("   contains {{1, 3}}? {}", family.contains(&BTreeSet::from([1, 3])));
    let mut hashed: HashSet<BTreeSet<i32>> = HashSet::new();
    hashed.insert(BTreeSet::from([1, 2]));
    hashed.insert(BTreeSet::from([2, 1]));
    hashed.insert(BTreeSet::from([2, 3]));
    println!("   HashSet<BTreeSet<i32>> gets the same {} members: {}", hashed.len(), show(hashed.iter()));
    println!();

    println!("2. Built in two orders: equal, and they hash the same");
    let x = BTreeSet::from([3, 1, 2]);
    let mut y = BTreeSet::new();
    for n in [2, 3, 1] {
        y.insert(n);
    }
    println!("   x from [3, 1, 2] = {x:?}   y inserted 2, 3, 1 = {y:?}");
    println!("   x == y: {}   hash(x) == hash(y): {}   x.cmp(&y): {:?}", x == y, hash_of(&x) == hash_of(&y), x.cmp(&y));
    let empty: BTreeSet<BTreeSet<i32>> = BTreeSet::new();
    let holds_empty: BTreeSet<BTreeSet<i32>> = BTreeSet::from([BTreeSet::new()]);
    println!("   ∅ has {} members; {{∅}} has {}: {holds_empty:?}", empty.len(), holds_empty.len());
    let p = power_set(&BTreeSet::from([1, 2, 3]));
    println!("   the power set of {{1, 2, 3}} has {} members:", p.len());
    println!("   {p:?}");
    println!();

    println!("3. Why HashSet is not Hash: equal sets, different iteration orders");
    type Fixed = BuildHasherDefault<DefaultHasher>;
    let items = [10, 20, 30, 40, 50];
    let mut small: HashSet<i32, Fixed> = HashSet::with_hasher(Fixed::default());
    let mut large: HashSet<i32, Fixed> = HashSet::with_capacity_and_hasher(100, Fixed::default());
    small.extend(items);
    large.extend(items);
    let small_order: Vec<i32> = small.iter().copied().collect();
    let large_order: Vec<i32> = large.iter().copied().collect();
    println!("   same five members, one fixed hasher, capacities {} and {}", small.capacity(), large.capacity());
    println!("   small == large: {}", small == large);
    println!("   small iterates {small_order:?}");
    println!("   large iterates {large_order:?}");
    println!("   hashed in iteration order, equal? {}", hash_of(&small_order) == hash_of(&large_order));
    println!();

    println!("4. Mutability belongs to the binding, not to the set");
    let frozen = BTreeSet::from([2, 3]);
    // frozen.insert(4);   // E0596: cannot borrow `frozen` as mutable
    let mut thawed = frozen; // the same value, moved to a `let mut` name
    thawed.insert(4);
    println!("   let frozen = {{2, 3}}; let mut thawed = frozen; thawed.insert(4) -> {thawed:?}");
    let refrozen = thawed; // and back: no `mut`, no more changes
    println!("   let refrozen = thawed -> {refrozen:?}, and refrozen cannot change");
    println!("   fn add_one(mut s: BTreeSet<i32>) on it -> {:?}", add_one(refrozen));
    println!();

    println!("5. No &mut to a member: take it out, change it, put it back");
    let mut groups: BTreeSet<BTreeSet<i32>> = BTreeSet::from([BTreeSet::from([1, 2]), BTreeSet::from([5])]);
    println!("   before: {groups:?}");
    // for g in groups.iter() { g.insert(3); }   // E0596: `g` is a `&` reference
    let mut g = groups.take(&BTreeSet::from([1, 2])).expect("was there");
    g.insert(3);
    groups.insert(g);
    println!("   after take, insert(3), insert: {groups:?}");
    println!("   iter() yields &T, get() returns Option<&T>: {:?}", groups.get(&BTreeSet::from([5])));
    println!();

    println!("6. The gap: Cell is Ord, not Hash, so it can sit in a BTreeSet");
    let cells = BTreeSet::from([Cell::new(1), Cell::new(2), Cell::new(3)]);
    cells.first().expect("three members").set(5); // a write through `&Cell`
    let now: Vec<i32> = cells.iter().map(Cell::get).collect();
    println!("   first member set to 5 through &Cell; members now {now:?}");
    println!("   contains(2)? {}   contains(5)? {}", cells.contains(&Cell::new(2)), cells.contains(&Cell::new(5)));
    println!("   (a logic error by std's docs: the result is unspecified; clippy::mutable_key_type warns)");
    println!();

    println!("7. Freezing a HashSet: collect it into a BTreeSet");
    let pairs = [("ann", "bob"), ("bob", "ann"), ("bob", "cy")];
    let mut meetings: HashSet<BTreeSet<&str>> = HashSet::new();
    for (a, b) in pairs {
        let working: HashSet<&str> = HashSet::from([a, b]);
        meetings.insert(working.into_iter().collect::<BTreeSet<&str>>());
    }
    println!("   pairs {pairs:?}");
    println!("   as unordered pairs: {} -> {}", meetings.len(), show(meetings.iter()));
}
