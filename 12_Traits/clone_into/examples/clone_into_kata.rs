//! Kata solution: which of these four loops actually reuses the buffer?
//!
//!   rustc --edition 2024 clone_into_kata.rs -o /tmp/cik && /tmp/cik

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);

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
        REALLOCS.fetch_add(1, Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

fn measure<T>(label: &str, work: impl FnOnce() -> T) -> T {
    let (a0, r0) = (ALLOCS.load(Relaxed), REALLOCS.load(Relaxed));
    let out = work();
    let (a1, r1) = (ALLOCS.load(Relaxed), REALLOCS.load(Relaxed));
    println!("   {label:<38} alloc {:<3} realloc {}", a1 - a0, r1 - r0);
    out
}

const ROWS: [&str; 5] = ["Ada", "Ben", "Cara", "Dev", "Eve"];

fn main() {
    println!("1. Four loops. Two of them reuse the buffer, two only look like it.");

    let mut a = String::with_capacity(16);
    measure("buf = row.to_owned()", || {
        for row in ROWS {
            a = row.to_owned();
            let _ = a.len();
        }
    });

    let mut b = String::with_capacity(16);
    measure("row.clone_into(&mut buf)", || {
        for row in ROWS {
            row.clone_into(&mut b);
            let _ = b.len();
        }
    });

    let mut c = String::with_capacity(16);
    measure("buf.clear(); buf.push_str(row)", || {
        for row in ROWS {
            c.clear();
            c.push_str(row);
            let _ = c.len();
        }
    });

    let mut d = String::with_capacity(16);
    measure("buf = String::from(row)", || {
        for row in ROWS {
            d = String::from(row);
            let _ = d.len();
        }
    });

    println!("   Assignment REPLACES the String — the old buffer is freed and a new");
    println!("   one bought. Only 2 and 3 write into the buffer that is already there,");
    println!("   and 3 is literally what str's clone_into impl does:");
    println!("       fn clone_into(&self, target: &mut String) {{");
    println!("           target.clear();");
    println!("           target.push_str(self);");
    println!("       }}");

    println!();
    println!("2. `mut` on the binding is not reuse of the buffer");
    // The thing that makes loop 1 look like it should be free is `let mut`.
    // But `mut` is about the NAME being rebindable; the heap buffer behind it
    // is a separate question, and `=` answers it the expensive way.
    println!("   loop 1 and loop 4 both wrote through `let mut`, and both paid.");
    println!("   the buffer is reused by the METHOD you call, not by the binding.");

    println!();
    println!("3. The trap: a derived Clone gives you none of this");
    #[derive(Clone)]
    struct Derived {
        name: String,
    }
    struct Tuned {
        name: String,
    }
    impl Clone for Tuned {
        fn clone(&self) -> Self {
            Tuned { name: self.name.clone() }
        }
        fn clone_from(&mut self, source: &Self) {
            self.name.clone_from(&source.name);
        }
    }

    let long = "a name long enough to live on the heap".to_string();
    let src_d = Derived { name: long.clone() };
    let mut dst_d = Derived { name: String::with_capacity(64) };
    measure("derived:      dst.clone_from(&src)", || dst_d.clone_from(&src_d));

    let src_t = Tuned { name: long.clone() };
    let mut dst_t = Tuned { name: String::with_capacity(64) };
    measure("hand-written: dst.clone_from(&src)", || dst_t.clone_from(&src_t));

    println!("   both landed the same text: {:?}", dst_d.name);
    println!("   #[derive(Clone)] emits `clone` and nothing else, so `clone_from`");
    println!("   falls back to the default `*self = source.clone()` and allocates.");
    println!("   The 64 bytes sitting in dst were never touched.");
}
