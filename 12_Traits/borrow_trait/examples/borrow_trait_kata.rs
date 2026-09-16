//! Kata solution: one lookup for every kind of key, and a count proving that
//! none of the three calls built an owned key to search with.
//!
//!   rustc --edition 2024 borrow_trait_kata.rs -o /tmp/btk && /tmp/btk

use std::alloc::{GlobalAlloc, Layout, System};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

struct Counting;

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// `K: Borrow<Q>` lets the map compare its owned keys with the borrowed one.
/// `Q: Hash + Eq` is what the map needs to find the bucket from `key` alone.
/// `?Sized` because all three borrowed forms below — `str`, `Path`, `[u8]` —
/// are unsized. Delete it and the calls stop compiling, with E0277: "the size
/// for values of type `str` cannot be known at compilation time".
fn count_for<K, Q>(map: &HashMap<K, u32>, key: &Q) -> u32
where
    K: Borrow<Q> + Hash + Eq,
    Q: Hash + Eq + ?Sized,
{
    map.get(key).copied().unwrap_or(0)
}

fn main() {
    let by_name: HashMap<String, u32> = HashMap::from([(String::from("Ada"), 3)]);
    let by_path: HashMap<PathBuf, u32> = HashMap::from([(PathBuf::from("notes.txt"), 7)]);
    let by_bytes: HashMap<Vec<u8>, u32> = HashMap::from([(b"GET".to_vec(), 12)]);

    let before = ALLOCS.load(Relaxed);
    let name = count_for(&by_name, "Ada");
    let path = count_for(&by_path, Path::new("notes.txt"));
    let bytes = count_for(&by_bytes, b"GET".as_slice());
    let missing = count_for(&by_name, "Ben");
    let allocs = ALLOCS.load(Relaxed) - before;

    println!("1. One function, three kinds of key");
    println!("   String  keys, &str  lookup: {name}");
    println!("   PathBuf keys, &Path lookup: {path}");
    println!("   Vec<u8> keys, &[u8] lookup: {bytes}");
    println!("   a missing key:              {missing}");
    println!("   heap allocations across the four calls: {allocs}");

    println!();
    println!("2. The call that does not compile");
    // count_for(&by_name, b"Ada".as_slice());
    //   E0277: the trait bound `String: Borrow<[u8]>` is not satisfied.
    // A str and a [u8] with the same bytes hash differently, so std never
    // wrote that impl; String lends out its bytes through AsRef<[u8]> instead.
    println!("   count_for(&by_name, b\"Ada\".as_slice()) is E0277: String is not Borrow<[u8]>");
}
