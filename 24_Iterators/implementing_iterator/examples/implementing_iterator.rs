//! Implementing Iterator: one method, and what does and does not come free.
//!
//!   rustc --edition 2024 implementing_iterator.rs -o /tmp/ii && /tmp/ii

/// A sequence with no collection behind it at all: the state IS the iterator.
struct Countdown {
    n: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.n == 0 {
            return None;
        }
        self.n -= 1;
        Some(self.n + 1)
    }
}

/// The same thing, with the one optional method that costs `collect` real money.
struct Hinted {
    n: u32,
}

impl Iterator for Hinted {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.n == 0 {
            return None;
        }
        self.n -= 1;
        Some(self.n + 1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.n as usize, Some(self.n as usize))
    }
}

/// The mistake: a COLLECTION that implements Iterator. It works once.
struct OneShot {
    rows: Vec<u32>,
    at: usize,
}

impl Iterator for OneShot {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let row = self.rows.get(self.at).copied();
        self.at += 1;
        row
    }
}

/// The pattern std actually uses: the collection is not an iterator, it HANDS
/// OUT iterators — one per kind of access.
struct Roster {
    rows: Vec<String>,
}

impl Roster {
    fn new(rows: &[&str]) -> Self {
        Roster { rows: rows.iter().map(|r| r.to_string()).collect() }
    }
    fn iter(&self) -> std::slice::Iter<'_, String> {
        self.rows.iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, String> {
        self.rows.iter_mut()
    }
}

impl IntoIterator for Roster {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl<'a> IntoIterator for &'a Roster {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter()
    }
}

impl<'a> IntoIterator for &'a mut Roster {
    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter_mut()
    }
}

fn main() {
    println!("1. Write `next`, and seventy-five more methods arrive");
    println!("   Countdown {{ n: 5 }}.collect()      {:?}", Countdown { n: 5 }.collect::<Vec<_>>());
    println!("   .sum()                            {}", Countdown { n: 5 }.sum::<u32>());
    println!("   .filter(odd).take(3).collect()    {:?}",
             Countdown { n: 9 }.filter(|n| n % 2 == 1).take(3).collect::<Vec<_>>());
    println!("   .zip([a, b, c]).collect()         {:?}",
             Countdown { n: 3 }.zip(["a", "b", "c"]).collect::<Vec<_>>());
    println!("   .max()  .last()                   {:?} {:?}",
             Countdown { n: 4 }.max(), Countdown { n: 4 }.last());
    println!("   for n in Countdown {{ n: 3 }}        <- works too: every Iterator");
    for n in (Countdown { n: 3 }) {
        print!("     {n}");
    }
    println!();

    println!();
    println!("2. What the trait actually asked for");
    println!("   type Item = u32;                     the element type");
    println!("   fn next(&mut self) -> Option<Item>;  advance, or say you are done");
    println!("   That is all. In 1.98.0's source, `Iterator` declares 76 methods");
    println!("   (a handful still unstable) and exactly one of them — `next` — has");
    println!("   no default body. Everything in section 1 is written in terms of it.");

    println!();
    println!("3. Three things that are NOT free");
    println!("   .rev()   needs DoubleEndedIterator  (you must also write next_back)");
    println!("   .len()   needs ExactSizeIterator");
    println!("   Both are compile errors on Countdown, not runtime surprises.");
    println!("   And the default size_hint costs allocations:");
    let no_hint: Vec<u32> = Countdown { n: 9 }.collect();
    let hinted: Vec<u32> = Hinted { n: 9 }.collect();
    println!("     default size_hint()  {:<12} collect capacity {}",
             format!("{:?}", Countdown { n: 9 }.size_hint()), no_hint.capacity());
    println!("     size_hint written    {:<12} collect capacity {}",
             format!("{:?}", Hinted { n: 9 }.size_hint()), hinted.capacity());
    println!("   Same nine items. Without the hint, collect grew the Vec by doubling");
    println!("   and overshot to 16; with it, one allocation of exactly nine.");

    println!();
    println!("4. The mistake: implementing Iterator ON a collection");
    let mut once = OneShot { rows: vec![5, 3, 0], at: 0 };
    let first: Vec<u32> = once.by_ref().collect();
    let second: Vec<u32> = once.collect();
    println!("   first pass over the collection:  {first:?}");
    println!("   second pass over the SAME value: {second:?}");
    println!("   An iterator is single-use by construction — `next` takes &mut self");
    println!("   and there is no rewind. So a collection that IS an iterator can be");
    println!("   read once, and `for row in collection` consumes it. Nobody wants a");
    println!("   Vec that empties itself when you look at it.");

    println!();
    println!("5. What std does instead: hand out an iterator per kind of access");
    let mut roster = Roster::new(&["Ada", "Ben", "Cara"]);
    println!("   roster.iter().count()          {}", roster.iter().count());
    println!("   roster.iter().count() again    {}", roster.iter().count());
    for row in roster.iter_mut() {
        row.push('!');
    }
    println!("   after iter_mut:                {:?}", roster.iter().collect::<Vec<_>>());
    print!("   for row in &roster            ");
    for row in &roster {
        print!(" {row}");
    }
    println!();
    let longest = (&roster).into_iter().max_by_key(|r| r.len()).cloned();
    println!("   adapters work on the borrow:   longest = {longest:?}");
    let owned: Vec<String> = roster.into_iter().collect();
    println!("   for row in roster (by value)   {owned:?}");
    println!("   Three IntoIterator impls — for Roster, &Roster and &mut Roster —");
    println!("   are what make all three `for` spellings work, and the collection");
    println!("   itself stays re-readable because it never became the iterator.");

    println!();
    println!("6. The borrowing iterators carry a lifetime, and it is load-bearing");
    println!("   fn iter(&self) -> std::slice::Iter<'_, String>");
    println!("   The '_ ties the iterator to the borrow of the Roster, so the");
    println!("   compiler refuses an iterator that outlives what it reads:");
    println!("     error[E0515]: cannot return value referencing local variable `v`");
    println!("     help: use `.collect()` to allocate the iterator");
    println!("   That help line is the whole decision: hand back a borrow tied to");
    println!("   data the caller owns, or hand back owned data.");
}
