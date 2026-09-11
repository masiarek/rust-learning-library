//! Which signature makes the caller pay — counted, not asserted.
//!
//!   rustc --edition 2024 string_api_design.rs -o /tmp/sad && /tmp/sad
//!
//! A counting allocator wraps the system one, so every claim below about who
//! allocates is a number this program measured. Each input is built OUTSIDE the
//! measured closure, so only the call itself is counted.

use std::alloc::{GlobalAlloc, Layout, System};
use std::borrow::Cow;
use std::fmt::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

/// Counts every allocation and every reallocation — both are somebody paying.
struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Runs `f`, and returns its result with the number of allocations inside it.
fn counted<T>(f: impl FnOnce() -> T) -> (T, usize) {
    let before = ALLOCS.load(Relaxed);
    let out = f();
    (out, ALLOCS.load(Relaxed) - before)
}

// ---- the signatures being compared -----------------------------------------

/// The default: borrow. Every caller that has text can pass it for free.
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// The anti-pattern: owning a value the function only reads.
fn count_words_owned(s: String) -> usize {
    s.split_whitespace().count()
}

/// Any kind of text, with one compiled copy of the function per kind.
fn count_words_any<T: AsRef<str>>(s: T) -> usize {
    s.as_ref().split_whitespace().count()
}

/// Which instantiation a call selects.
fn which<T: AsRef<str>>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

/// Where `AsRef<str>` earns its keep: a slice of either kind of text.
fn total_words<S: AsRef<str>>(items: &[S]) -> usize {
    items.iter().map(|s| count_words(s.as_ref())).sum()
}

struct User {
    name: String,
}

impl User {
    /// Keeps the text, so it asks for anything that can BECOME a `String`: a
    /// caller holding one moves it in, a caller holding a `&str` pays once.
    fn new(name: impl Into<String>) -> User {
        User { name: name.into() }
    }

    /// The same constructor taking `&str`: now even a caller with a `String`
    /// to spare watches it be copied.
    fn from_borrowed(name: &str) -> User {
        User { name: name.to_owned() }
    }
}

/// Borrow when nothing changes; allocate only on the write.
fn tidy(s: &str) -> Cow<'_, str> {
    if s.contains('\t') { Cow::Owned(s.replace('\t', " ")) } else { Cow::Borrowed(s) }
}

/// Returning text, the first shape that works: a new `String` every call.
fn label(n: u32) -> String {
    format!("row {n}")
}

/// The second: write into a buffer the caller lends, and reuses.
fn label_into(n: u32, out: &mut String) {
    out.clear();
    write!(out, "row {n}").expect("writing into a String cannot fail");
}

fn main() {
    let owned = String::from("Ada Lovelace wrote the first program");

    println!("1. The default: take &str");
    let (_, lit) = counted(|| count_words("one two three"));
    let (_, by_ref) = counted(|| count_words(&owned));
    let (_, slice) = counted(|| count_words(&owned[..12]));
    println!("   count_words(\"one two three\")   {lit} allocations");
    println!("   count_words(&owned)            {by_ref} allocations   (&String coerced to &str)");
    println!("   count_words(&owned[..12])      {slice} allocations");

    println!();
    println!("2. The anti-pattern: take String by value");
    let (_, from_lit) = counted(|| count_words_owned("one two three".to_string()));
    let (_, from_clone) = counted(|| count_words_owned(owned.clone()));
    let spare = owned.clone();
    let (_, moved) = counted(move || count_words_owned(spare));
    println!("   a caller with a literal        {from_lit} allocation    (.to_string() first)");
    println!("   a caller keeping its String    {from_clone} allocation    (.clone() first)");
    println!("   a caller finished with its own {moved} allocations   (moved in)");
    println!("   Only the last caller got it free, and the function never needed to own");
    println!("   the text: it reads it and drops it.");

    println!();
    println!("3. impl AsRef<str>: every kind of text, one copy of the function per kind");
    let spare = owned.clone();
    let (_, a) = counted(|| count_words_any("one two"));
    let (_, b) = counted(|| count_words_any(&owned));
    let (_, c) = counted(move || count_words_any(spare));
    println!("   &str {a}, &String {b}, String {c} allocations");
    println!("   the three instantiations: {}, {}, {}", which(&"x"), which(&&owned), which(&owned));
    println!("   Three types in, three compiled copies of count_words_any out. The cost is");
    println!("   code size, paid once per type — never per call.");
    let strs = ["one two", "three"];
    let strings = vec![String::from("one two"), String::from("three")];
    let (x, n1) = counted(|| total_words(&strs));
    let (y, n2) = counted(|| total_words(&strings));
    println!("   total_words(&strs)      {x} words, {n1} allocations   (a &[&str])");
    println!("   total_words(&strings)   {y} words, {n2} allocations   (a &[String])");
    println!("   The slice is where it earns its keep. A &[&str] parameter would refuse");
    println!("   &strings with E0308: a &String coerces to &str, but a slice of one never");
    println!("   coerces to a slice of the other. PathBuf is refused outright (E0277): a");
    println!("   path is not promised to be UTF-8, so it is AsRef<Path> and AsRef<OsStr>,");
    println!("   never AsRef<str>.");

    println!();
    println!("4. impl Into<String>: when the function is going to keep the text");
    let (u1, via_literal) = counted(|| User::new("Ada"));
    let name = String::from("Ada");
    let (u2, via_move) = counted(move || User::new(name));
    let name = String::from("Ada");
    let (u3, via_borrow) = counted(|| User::from_borrowed(&name));
    assert!(u1.name == u2.name && u2.name == u3.name);
    println!("   User::new(\"Ada\")             {via_literal} allocation    (the &str is copied once, as it must be)");
    println!("   User::new(name)              {via_move} allocations   (the caller's String moves in)");
    println!("   User::from_borrowed(&name)   {via_borrow} allocation    (a String existed, and was copied anyway)");

    println!();
    println!("5. Cow<str> as the return type: borrow unless something changed");
    for s in ["no tabs here", "one\ttab"] {
        let (out, n) = counted(|| tidy(s));
        let kind = match out {
            Cow::Borrowed(_) => "Borrowed",
            Cow::Owned(_) => "Owned",
        };
        println!("   {:<22} -> {kind:<8} {n} allocation(s)", format!("tidy({s:?})"));
    }

    println!();
    println!("6. Returning text: a new String, or a buffer the caller lends you");
    let (_, fresh) = counted(|| {
        for n in 1..=3 {
            std::hint::black_box(label(n));
        }
    });
    let mut buf = String::new();
    let (_, reused) = counted(|| {
        for n in 1..=3 {
            label_into(n, &mut buf);
        }
    });
    println!("   label(n), three calls                 {fresh} allocations   (one String each)");
    println!("   label_into(n, &mut buf), three calls  {reused} allocation    (one buffer, reused: {buf:?})");
    println!("   The third shape, a &str into a String made inside the function, never");
    println!("   compiles. With no input to borrow from it is E0106 first; write 'static");
    println!("   as rustc suggests and it becomes E0515, because the String dies at the");
    println!("   closing brace and the reference may not outlive it.");
}
