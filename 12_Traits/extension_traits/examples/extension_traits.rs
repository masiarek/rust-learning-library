//! Extension traits: giving a method to a type you did not write.
//!
//! Everything here is the half that works. The six compile errors this page
//! discusses live on the page itself, where a broken program belongs.

// ---------------------------------------------------------------- 1. the shape
// `str` belongs to the standard library. `Shout` belongs to us, and that is the
// entire permission slip: the orphan rule asks that the TRAIT or the TYPE be
// local, and here the trait is.
trait Shout {
    fn shout(&self) -> String;
}

impl Shout for str {
    fn shout(&self) -> String {
        let mut s = self.to_uppercase();
        s.push('!');
        s
    }
}

// -------------------------------------------------------- 2. one impl, every type
// `Tally` is implemented once, for everything that iterates — including iterators
// that did not exist when this line was written. The body lives in the trait as a
// default, so the impl block is empty; `Sized` is required because `count(self)`
// takes the iterator by value.
trait Tally: Iterator + Sized {
    fn tally(self) -> usize {
        self.count()
    }
}

impl<I: Iterator> Tally for I {}

// ------------------------------------------------- 3. the one from the talk
// Will Crichton's progress bar, with the terminal-clearing removed so the frames
// print in order. `Progress` wraps an iterator and is itself an iterator, so it
// drops into a `for` loop with nothing else changed.
const WIDTH: usize = 3;

struct Progress<I> {
    iter: I,
    i: usize,
}

impl<I: Iterator> Iterator for Progress<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // `saturating_sub`, not `-`: an iterator longer than WIDTH would
        // otherwise underflow a `usize` and panic mid-bar.
        println!(
            "    [{}{}]",
            "*".repeat(self.i.min(WIDTH)),
            " ".repeat(WIDTH.saturating_sub(self.i))
        );
        self.i += 1;
        self.iter.next()
    }
}

// The extension trait is four lines and is the only reason `.progress()` can be
// written with a dot. Narrowed to iterators on purpose — see the page for what
// the unnarrowed version puts the method on.
trait ProgressIteratorExt: Iterator + Sized {
    fn progress(self) -> Progress<Self> {
        Progress { iter: self, i: 0 }
    }
}

impl<I: Iterator> ProgressIteratorExt for I {}

// ------------------------------------------------------------- 4. the silent trap
// `len` collides with the inherent `[T]::len`. `second` does not collide with
// anything. Both are declared here; only one of them is reachable with a dot.
trait SliceExt<T> {
    fn len(&self) -> usize;
    fn second(&self) -> Option<&T>;
}

impl<T> SliceExt<T> for [T] {
    fn len(&self) -> usize {
        999
    }
    fn second(&self) -> Option<&T> {
        self.get(1)
    }
}

fn main() {
    println!("1. A local trait, implemented for `str`:");
    println!("   {}", "hello".shout());
    println!("   {}", String::from("owned too").shout());

    println!("\n2. One blanket impl reaches every iterator:");
    println!("   vec into_iter  -> {}", vec![1, 2, 3].into_iter().tally());
    println!("   str split      -> {}", "a b c d".split(' ').tally());
    println!("   filtered range -> {}", (0..10).filter(|n| n % 3 == 0).tally());

    println!("\n3. `.progress()` on a plain range, then on a `map`:");
    for _ in (0..3).progress() {}
    for _ in ["x", "y", "z"].iter().map(|s| s.len()).progress() {}

    println!("\n4. The inherent method wins, and nothing warns:");
    let v = [10, 20, 30];
    println!("   v.len()             = {}", v.len());
    println!("   SliceExt::len(&v)   = {}", SliceExt::len(&v[..]));
    println!("   v.second()          = {:?}", v.second());
}
