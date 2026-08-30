//! A tuple is a struct whose fields are numbered instead of named.
//!
//!   rustc --edition 2024 tuples.rs -o /tmp/tuples && /tmp/tuples

// A function that has two things to say returns both, with no type to declare.
fn min_max(scores: &[i32]) -> (i32, i32) {
    let mut lo = scores[0];
    let mut hi = scores[0];
    for &s in scores {
        if s < lo {
            lo = s;
        }
        if s > hi {
            hi = s;
        }
    }
    (lo, hi)
}

// The same thing, named. Compare the two call sites below.
struct Range {
    lowest: i32,
    highest: i32,
}

fn range_of(scores: &[i32]) -> Range {
    let (lo, hi) = min_max(scores);
    Range { lowest: lo, highest: hi }
}

fn main() {
    let scores = [5, 3, 0, 4, 2];

    println!("1. Two values out of one function, with no type declared");
    let (lo, hi) = min_max(&scores);
    println!("   min_max({scores:?}) = ({lo}, {hi})");
    let both = min_max(&scores);
    println!("   or keep the pair whole: both.0 = {}, both.1 = {}", both.0, both.1);
    println!("   The type is written the way the value is: (i32, i32).");

    println!();
    println!("2. Length and element types are part of the type");
    let mixed: (u8, char, bool) = (5, 'A', true);
    println!("   (u8, char, bool) = ({}, {}, {})", mixed.0, mixed.1, mixed.2);
    println!("   size_of::<(u8, char, bool)>() = {}", size_of::<(u8, char, bool)>());
    println!("   size_of::<(i32, i32)>()       = {}", size_of::<(i32, i32)>());
    println!("   A tuple has no header and no indirection: it is its fields,");
    println!("   laid out next to each other, in an order the compiler picks.");

    println!();
    println!("3. The two odd arities");
    let unit: () = ();
    let one = (7,);
    println!("   () is the empty tuple, size {} — what every function without", size_of::<()>());
    println!("      a `->` returns, and what a `;` turns an expression into.");
    println!("   (7,) is a ONE-tuple, and the trailing comma is load-bearing:");
    println!("      (7) is just 7 in parentheses. one.0 = {}, unit = {unit:?}", one.0);

    println!();
    println!("4. Comparison is field by field, left to right");
    let mut rounds = [(2, "Cara"), (1, "Ben"), (2, "Ada")];
    rounds.sort();
    println!("   sorted: {rounds:?}");
    println!("   (1, ..) < (2, ..) settles the first two without reading the name;");
    println!("   the tie between the 2s is broken by \"Ada\" < \"Cara\".");
    println!("   Ordering a tuple is a free sort key — put the field you want");
    println!("   to sort by first.");

    println!();
    println!("5. Where a tuple stops being readable");
    let r = range_of(&scores);
    println!("   tuple:  let (lo, hi) = min_max(&scores);        -> ({lo}, {hi})");
    println!("   struct: range_of(&scores).lowest / .highest   -> {} / {}", r.lowest, r.highest);
    println!("   Two fields you destructure on the spot: tuple. Three or more,");
    println!("   or a value that travels, or `.2` appearing in another function:");
    println!("   name the fields. `.0` is a comment nobody wrote.");
}
