//! Kata solution: pick the pointer.
//!
//! 1. Only reads a config        -> `&Config`: borrows, drops nothing, and accepts
//!                                  a `Config` owned any way.
//! 2. Parent owns its children   -> `Option<Box<Node>>`: one owner, dropped with the
//!                                  parent. `Option<Node>` is E0072, recursive type
//!                                  has infinite size.
//! 3. A cache several pages hold -> `Rc<Cache>`: the last holder frees it, whoever
//!                                  that is. With `cache: &'a Cache`, dropping the
//!                                  cache while a page still reads it is E0505.
//! 4. C's memchr                 -> `*const c_void` in, `*mut c_void` out. A `&[u8]`
//!                                  parameter compiles with an `improper_ctypes`
//!                                  warning: slices have no C equivalent.
//!
//! The two errors and the warning are rustc 1.98.0 output.
//!
//!   rustc --edition 2024 pointer_types_compared_kata.rs -o /tmp/pointer_types_compared_kata && /tmp/pointer_types_compared_kata

use std::ffi::{c_int, c_void};
use std::mem::size_of;
use std::rc::Rc;

// 1. &T
struct Config {
    host: &'static str,
    retries: u32,
}

fn describe(config: &Config) -> String {
    format!("{} x{}", config.host, config.retries)
}

// 2. Box<T>
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("   drop node {}", self.value);
    }
}

fn sum(node: &Node) -> i32 {
    node.value + node.left.as_deref().map_or(0, sum) + node.right.as_deref().map_or(0, sum)
}

// 3. Rc<T>
struct Cache {
    statuses: Vec<(&'static str, u16)>,
}

impl Drop for Cache {
    fn drop(&mut self) {
        println!("   drop cache");
    }
}

struct Page {
    path: &'static str,
    cache: Rc<Cache>,
}

impl Page {
    fn status(&self) -> u16 {
        self.cache.statuses.iter().find(|(p, _)| *p == self.path).map_or(404, |(_, s)| *s)
    }
}

// 4. *const T / *mut T
unsafe extern "C" {
    // void *memchr(const void *s, int c, size_t n);
    fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
}

/// The index of the first `byte` in `bytes`, found by C.
fn find_byte(bytes: &[u8], byte: u8) -> Option<(usize, u8)> {
    // SAFETY: `s` and `n` come from one live slice, so memchr reads only bytes
    // that exist, and nothing writes `bytes` while it runs (it is a `&[u8]`).
    let hit = unsafe { memchr(bytes.as_ptr().cast(), c_int::from(byte), bytes.len()) };
    if hit.is_null() {
        return None; // the table's "can be null: yes"
    }
    let at: *const u8 = hit.cast_const().cast();
    // SAFETY: non-null means memchr found the byte inside `bytes[..len]`, and
    // `bytes` is still borrowed, so the allocation is live and unchanged.
    let found = unsafe { *at };
    Some((at.addr() - bytes.as_ptr().addr(), found))
}

fn main() {
    let word = size_of::<usize>();

    println!("1. &Config: one signature for any owner");
    let config = Config { host: "localhost", retries: 3 };
    let boxed = Box::new(Config { host: "boxed", retries: 1 });
    let counted = Rc::new(Config { host: "counted", retries: 2 });
    println!("   describe(&config)              -> {}", describe(&config));
    println!("   describe(&boxed)               -> {}", describe(&boxed));
    println!("   describe(&counted)             -> {}", describe(&counted));
    println!("   config.host after the call     -> {}", config.host);

    println!();
    println!("2. Option<Box<Node>>: the parent owns its children");
    let root = Node {
        value: 1,
        left: Some(Box::new(Node { value: 2, left: None, right: None })),
        right: Some(Box::new(Node { value: 3, left: None, right: None })),
    };
    println!("   sum(&root)                     -> {}", sum(&root));
    println!("   Option<Box<Node>>, words       -> {}", size_of::<Option<Box<Node>>>() / word);
    println!("   drop(root)                     -> three destructors, parent first");
    drop(root);

    println!();
    println!("3. Rc<Cache>: whoever holds it last frees it");
    let cache = Rc::new(Cache { statuses: vec![("/", 200), ("/old", 301)] });
    let home = Page { path: "/", cache: Rc::clone(&cache) };
    let old = Page { path: "/old", cache: Rc::clone(&cache) };
    println!("   holders                        -> {}", Rc::strong_count(&cache));
    drop(cache); // the builder goes first
    println!("   builder dropped, home reads    -> {}", home.status());
    drop(home);
    println!("   home dropped, old reads        -> {}", old.status());
    println!("   holders now                    -> {}", Rc::strong_count(&old.cache));
    println!("   drop(old)                      -> the last holder frees it");
    drop(old);

    println!();
    println!("4. *const c_void into C, *mut c_void back");
    let header = b"Content-Length: 42";
    println!("   find ':'                       -> {:?}", find_byte(header, b':').map(|(i, b)| (i, b as char)));
    println!("   find '#'                       -> {:?}   (memchr returned null)", find_byte(header, b'#'));
}
