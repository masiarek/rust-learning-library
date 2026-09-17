//! An array is its elements, so it lives wherever its owner lives.
//!
//!   rustc --edition 2024 where_an_array_lives.rs -o /tmp/wal && /tmp/wal
//!
//! Section 5 runs this same binary again as a child process, because a
//! stack overflow aborts the whole process and cannot be caught from inside it.

use std::hint::black_box;
use std::mem::{size_of, size_of_val};
use std::process::Command;
use std::thread;

const BIG: usize = 1_000_000; // one megabyte of u8
const SMALL_STACK: usize = 256 * 1024; // a thread asked for 256 KiB

static TABLE: [i32; 3] = [10, 20, 30];

fn table_from_elsewhere() -> &'static [i32; 3] {
    &TABLE
}

struct Reading {
    id: u16,
    samples: [i32; 3],
}

fn local_array() -> u8 {
    let buf = [7u8; BIG];
    black_box(&buf)[BIG - 1]
}

fn boxed_array() -> u8 {
    let buf = Box::new([7u8; BIG]);
    black_box(&buf)[BIG - 1]
}

fn boxed_slice_from_vec() -> u8 {
    let buf: Box<[u8]> = vec![7u8; BIG].into_boxed_slice();
    black_box(&buf)[BIG - 1]
}

fn boxed_array_from_vec() -> u8 {
    let buf: Box<[u8; BIG]> = vec![7u8; BIG].into_boxed_slice().try_into().unwrap();
    black_box(&buf)[BIG - 1]
}

fn on_small_stack(f: fn() -> u8) -> u8 {
    thread::Builder::new()
        .name("small".into())
        .stack_size(SMALL_STACK)
        .spawn(f)
        .expect("spawn")
        .join()
        .expect("join")
}

/// Runs this binary again with `mode`, and reports how the child ended.
fn child(mode: &str) -> String {
    let exe = std::env::current_exe().expect("current_exe");
    let out = Command::new(exe).arg(mode).output().expect("spawn child");
    let stderr = String::from_utf8_lossy(&out.stderr);
    if out.status.success() {
        format!("finished, printed {}", String::from_utf8_lossy(&out.stdout).trim())
    } else if stderr.contains("has overflowed its stack") {
        "aborted: thread 'small' has overflowed its stack".to_string()
    } else {
        format!("failed some other way: {stderr}")
    }
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("local") => return println!("{}", on_small_stack(local_array)),
        Some("boxed") => return println!("{}", on_small_stack(boxed_array)),
        Some("slice") => return println!("{}", on_small_stack(boxed_slice_from_vec)),
        Some("via-vec") => return println!("{}", on_small_stack(boxed_array_from_vec)),
        _ => {}
    }

    println!("1. No header: the value is the elements");
    println!("   size_of::<[i32; 3]>()        = {}", size_of::<[i32; 3]>());
    println!("   size_of::<Reading>()         = {}  (a u16 and the three i32s, inline)", size_of::<Reading>());
    let r = Reading { id: 1, samples: [4, 5, 6] };
    println!("   Reading {{ id: {}, samples: {:?} }}", r.id, r.samples);

    println!();
    println!("2. Four owners, four places");
    let local = [1, 2, 3];
    let boxed = Box::new([1, 2, 3]);
    let in_vec: Vec<[i32; 3]> = vec![[1, 2, 3], [4, 5, 6]];
    println!("   local:  the array is the stack slot; &local[0] is the slot's address: {}",
             std::ptr::eq(&local[0], local.as_ptr()));
    println!("   Box:    8 bytes on the stack, and the box's pointer is the first element: {}",
             std::ptr::eq(&*boxed as *const [i32; 3] as *const i32, &boxed[0]));
    let flat = in_vec.as_flattened();
    println!("   Vec:    the rows sit end to end in the Vec's one buffer: flat[3] is in_vec[1][0]: {}",
             std::ptr::eq(&flat[3], &in_vec[1][0]));
    println!("   static: &TABLE is one address, whichever function asks: {}",
             std::ptr::eq(&TABLE, table_from_elsewhere()));

    println!();
    println!("3. Box<[i32; 3]> and Box<[i32]>");
    let thin: Box<[i32; 3]> = Box::new([1, 2, 3]);
    let fat: Box<[i32]> = Box::new([1, 2, 3]);
    println!("   size_of_val(&thin) = {}  the length is in the type", size_of_val(&thin));
    println!("   size_of_val(&fat)  = {} the length rides beside the pointer", size_of_val(&fat));
    println!("   size_of_val(&*fat) = {} the three i32s on the heap", size_of_val(&*fat));
    let n = black_box(4);
    let run_time: Box<[i32]> = vec![0; n].into_boxed_slice();
    let collected: Box<[i32]> = (1..=n as i32).collect();
    println!("   a run-time length: vec![0; n].into_boxed_slice() = {run_time:?}");
    println!("                      (1..=n).collect::<Box<[i32]>>() = {collected:?}");
    let back: Box<[i32; 4]> = collected.try_into().unwrap();
    println!("   and back to a length in the type: Box<[i32; 4]> = {back:?}");

    println!();
    println!("4. Moving an array moves its bytes; moving a Box moves a pointer");
    let plain = [9u8; 64];
    let in_box = Box::new([9u8; 64]);
    let heap_addr = &in_box[0] as *const u8;
    let moved_plain = plain;
    let moved_box = in_box;
    println!("   size_of_val(&moved_plain) = {} bytes copied by `let moved_plain = plain;`", size_of_val(&moved_plain));
    println!("   size_of_val(&moved_box)   = {}  bytes copied by `let moved_box = in_box;`", size_of_val(&moved_box));
    println!("   the elements did not move: {}", std::ptr::eq(heap_addr, &moved_box[0]));

    println!();
    println!("5. A megabyte on a {} KiB thread", SMALL_STACK / 1024);
    println!("   let buf = [7u8; 1_000_000];              {}", child("local"));
    println!("   Box::new([7u8; 1_000_000]), debug build  {}", child("boxed"));
    println!("   vec![7u8; 1_000_000].into_boxed_slice()  {}", child("slice"));
    println!("   ... .try_into::<Box<[u8; 1_000_000]>>()  {}", child("via-vec"));
}
