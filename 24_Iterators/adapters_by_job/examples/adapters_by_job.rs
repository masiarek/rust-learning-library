//! Choosing an adapter: the job you have, and the one that does it.
//!
//!   rustc --edition 2024 adapters_by_job.rs -o /tmp/abj && /tmp/abj

fn main() {
    let scores = [5, 3, 0, 4, 2, 1];
    let rows = ["5", "3", "no", "4"];

    println!("1. Keep some — and the two ways to stop, which are not the same");
    println!("   .filter(< 5)          {:?}", scores.iter().filter(|s| **s < 5).collect::<Vec<_>>());
    println!("   .take_while(< 5)      {:?}", scores.iter().take_while(|s| **s < 5).collect::<Vec<_>>());
    println!("   .skip_while(< 5)      {:?}", scores.iter().skip_while(|s| **s < 5).collect::<Vec<_>>());
    println!("   `filter` tests every item. `take_while` stops at the FIRST item");
    println!("   that fails and never looks again — on a sorted sequence that is");
    println!("   the point, and on an unsorted one it is a bug that returns a");
    println!("   plausible prefix. The 5 at the front ends take_while immediately.");

    println!();
    println!("2. Transform-and-keep in one step");
    let ok: Vec<i32> = rows.iter().filter_map(|s| s.parse().ok()).collect();
    println!("   .filter_map(parse.ok())  {ok:?}");
    println!("   `filter_map` is `map` whose closure returns Option: Some keeps and");
    println!("   transforms, None drops. Writing it as .map(..).filter(..).map(..)");
    println!("   parses twice or unwraps.");

    println!();
    println!("3. One item in, many out — and the silent-loss trap");
    let words = ["Ada Lovelace", "Ben Carter"];
    let names: Vec<&str> = words.iter().flat_map(|w| w.split(' ')).collect();
    println!("   .flat_map(split)         {names:?}");
    let parsed: Vec<i32> = rows.iter().flat_map(|s| s.parse::<i32>()).collect();
    println!("   .flat_map(parse)         {parsed:?}");
    println!("   The second one is the trap. A Result IS an iterator of length 0 or");
    println!("   1, so flat_map over it flattens the Ok values and DROPS the Err —");
    println!("   quietly, with the same shape as the successful case. Three rows in,");
    println!("   three out; four rows in, three out, and nothing says which vanished.");
    println!("   When the failure matters, collect into Result instead.");

    println!();
    println!("4. Split into two, keeping both halves");
    let (small, large): (Vec<i32>, Vec<i32>) = scores.iter().partition(|s| **s < 3);
    println!("   .partition(< 3)          small {small:?}  large {large:?}");
    let pairs = [("Ada", 5), ("Ben", 3)];
    let (names, points): (Vec<&str>, Vec<i32>) = pairs.into_iter().unzip();
    println!("   .unzip()                 {names:?}  {points:?}");
    println!("   `partition` splits one stream by a predicate; `unzip` splits a");
    println!("   stream of pairs by position. Neither is lazy: both consume.");

    println!();
    println!("5. Carry state along the chain");
    let running: Vec<i32> = scores
        .iter()
        .scan(0, |total, s| {
            *total += s;
            Some(*total)
        })
        .collect();
    println!("   .scan(0, running total)  {running:?}");
    println!("   `scan` is a fold that yields every intermediate rather than only");
    println!("   the last, and its closure returns Option — returning None ends the");
    println!("   iterator, which is how you write a stateful `take_while`.");

    println!();
    println!("6. Look ahead without consuming");
    let mut it = scores.iter().peekable();
    let first = *it.peek().copied().unwrap();
    let taken: Vec<&i32> = it.by_ref().take(2).collect();
    println!("   peek() saw {first}, then take(2) got {taken:?}");
    println!("   `peek` takes &mut self even though it consumes nothing, because it");
    println!("   has to pull the item and hold it. That is why a peeked iterator");
    println!("   needs `let mut`, and why peeking inside a `while let` over the same");
    println!("   iterator needs care about where the borrow ends.");

    println!();
    println!("7. Join, repeat, and step");
    println!("   .chain(other)            {:?}", scores.iter().take(2).chain([9, 9].iter()).collect::<Vec<_>>());
    println!("   .cycle().take(8)         {:?}", [1, 2, 3].iter().cycle().take(8).collect::<Vec<_>>());
    println!("   .step_by(2)              {:?}", scores.iter().step_by(2).collect::<Vec<_>>());
    println!("   `cycle` is endless, so it only makes sense with something that");
    println!("   stops — laziness is what keeps it from hanging.");

    println!();
    println!("8. The three that are NOT iterator adapters");
    let v = vec![1, 2, 3, 4];
    println!("   slice::windows(2)        {:?}", v.windows(2).collect::<Vec<_>>());
    println!("   slice::chunks(3)         {:?}", v.chunks(3).collect::<Vec<_>>());
    let mut d = vec![1, 1, 2, 2, 3];
    d.dedup();
    println!("   Vec::dedup()             {d:?}");
    println!("   `windows` and `chunks` are methods on a SLICE, not on Iterator —");
    println!("   they need to look at several items at once, which an iterator that");
    println!("   has handed you an item can no longer do. `dedup` is a Vec method");
    println!("   and only removes CONSECUTIVE duplicates; deduplicating a whole");
    println!("   sequence is a collect into a HashSet, or a sort first.");
}
