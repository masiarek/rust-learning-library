//! Collect the iterator into a `Vec` — and the questions that need no `Vec` at all.
//!
//!   rustc --edition 2024 collect_into_a_vec.rs -o /tmp/civ && /tmp/civ

use std::cell::Cell;

fn main() {
    // ---------------------------------------------------------------- 1
    let s = "a:b:c";

    println!("1. One iterator, two ways to spend it");
    let r: Vec<&str> = s.split(':').collect();
    println!("   collect  -> {r:?}");
    let mut walked = String::new();
    for part in s.split(':') {
        walked.push_str(part);
        walked.push(' ');
    }
    println!("   for loop -> {}", walked.trim_end());
    println!("   Same three pieces either way. The first line built a Vec to");
    println!("   hold them; the second never built a collection at all.");
    println!();

    // ---------------------------------------------------------------- 2
    let line = "cara:ada:ben:ada";
    let parts: Vec<&str> = line.split(':').collect();

    println!("2. What the Vec buys — five things the iterator cannot do");
    println!("   len          -> {}", parts.len());
    println!("   index [1]    -> {}", parts[1]);
    let mut sorted = parts.clone();
    sorted.sort_unstable();
    println!("   sorted       -> {sorted:?}");
    println!("   join         -> {:?}", parts.join(", "));
    let longest = parts.iter().map(|p| p.len()).max().unwrap();
    let total: usize = parts.iter().map(|p| p.len()).sum();
    println!("   two passes   -> longest {longest}, {total} letters in all");
    println!("   `sort` and `join` are SLICE methods, not iterator ones: neither");
    println!("   can start until the last item has arrived, so neither can be an");
    println!("   adapter. Two passes need the pieces to still be somewhere.");
    println!();

    // ---------------------------------------------------------------- 3
    let calls = Cell::new(0usize);
    let tally = |p| {
        calls.set(calls.get() + 1);
        p
    };
    let reset = || calls.set(0);

    println!("3. What it costs — and the questions that need no Vec");
    reset();
    let n = line.split(':').map(tally).collect::<Vec<_>>().len();
    println!("   collect().len() = {n}   {} pieces built, 1 Vec allocated", calls.get());
    println!("   .count()        = {}   the same answer, no allocation", line.split(':').count());
    reset();
    let first = line.split(':').map(tally).next().unwrap();
    println!("   .next()         = {first:?}  {} closure call", calls.get());
    reset();
    let found = line.split(':').map(tally).find(|p| p.starts_with('b'));
    println!("   .find(b…)       = {found:?}  {} closure calls, then it stopped", calls.get());
    reset();
    let dup = line.split(':').map(tally).any(|p| p == "ada");
    println!("   .any(== ada)    = {dup}   {} closure calls, then it stopped", calls.get());
    reset();
    let where_ben = line.split(':').map(tally).position(|p| p == "ben");
    println!("   .position(ben)  = {where_ben:?} {} closure calls", calls.get());
    println!("   .max_by_key(len)= {:?}", line.split(':').max_by_key(|p| p.len()));
    println!("   Only the first line allocated. A `collect` written to answer one");
    println!("   of the other six is a Vec built, read once, and dropped.");
    println!();

    // ---------------------------------------------------------------- 4
    println!("4. The pieces are slices OF the original, not copies of it");
    let base = line.as_ptr() as usize;
    let offsets: Vec<usize> = parts.iter().map(|p| p.as_ptr() as usize - base).collect();
    println!("   {line:?}");
    println!("   byte offsets of the pieces -> {offsets:?}");
    println!("   Not one character was copied: the Vec holds four (pointer, len)");
    println!("   pairs aimed into the string it came from. Cheap — and the reason");
    println!("   a Vec<&str> can never outlive the string it was split from.");
    println!();

    // ---------------------------------------------------------------- 5
    println!("5. Borrowed or owned — one annotation apart");
    let borrowed: Vec<&str> = line.split(':').collect();
    let owned: Vec<String> = line.split(':').map(String::from).collect();
    println!("   Vec<&str>   -> {borrowed:?}  (1 allocation: the Vec)");
    println!("   Vec<String> -> {owned:?}  (5: the Vec, and one per piece)");
    let survivor: Vec<String> = {
        let temporary = String::from("cara:ada:ben");
        temporary.split(':').map(String::from).collect()
    };
    println!("   the owned one outlives its source -> {survivor:?}");
    println!("   The same block returning Vec<&str> is E0515: the pieces would");
    println!("   point into a String that was dropped one line earlier.");
    println!();

    // ---------------------------------------------------------------- 6
    println!("6. How many pieces? The answer is never zero");
    for input in ["", "a", "a::b", "a:b:"] {
        let v: Vec<&str> = input.split(':').collect();
        let call = format!("{input:?}.split(':')");
        println!("   {call:<30} -> len {}  {v:?}", v.len());
    }
    let terminated: Vec<&str> = "a:b:".split_terminator(':').collect();
    println!("   {:<30} -> len {}  {terminated:?}", "\"a:b:\".split_terminator(':')", terminated.len());
    let blank: Vec<&str> = "".split_whitespace().collect();
    println!("   {:<30} -> len {}  {blank:?}", "\"\".split_whitespace()", blank.len());
    println!("   `split` yields one more piece than there are separators, always,");
    println!("   so the empty string gives one empty piece and the Vec is never");
    println!("   empty. `is_empty()` on it answers a question you did not ask.");
    println!();

    // ---------------------------------------------------------------- 7
    println!("7. For a key=value line you want neither");
    for setting in ["port=8080", "debug"] {
        let pieces: Vec<&str> = setting.splitn(2, '=').collect();
        println!("   {setting:?}");
        println!("      splitn(2, '=').collect() -> {pieces:?}, and pieces.get(1) = {:?}", pieces.get(1));
        println!("      split_once('=')          -> {:?}", setting.split_once('='));
    }
    println!("   `pieces[1]` on the second line is a panic, and the type system");
    println!("   said nothing: indexing a Vec is where a missing separator turns");
    println!("   into a crash. `split_once` returns Option<(&str, &str)>, so the");
    println!("   same mistake is a `None` the compiler makes you handle.");
}
