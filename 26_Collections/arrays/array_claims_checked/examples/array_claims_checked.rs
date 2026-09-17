//! Claims about arrays from books, docs, answers and chat, each run.
//!
//!   rustc --edition 2024 array_claims_checked.rs -o /tmp/acc && /tmp/acc
//!
//! Claim 3 runs this binary again as a child, to read the real panic text.

use std::any::type_name_of_val;
use std::cell::Cell;
use std::collections::HashSet;
use std::hint::black_box;
use std::mem::{size_of, size_of_val};
use std::process::Command;

static TABLE: [u64; 1000] = [0; 1000];

struct Loud(&'static str);

impl Drop for Loud {
    fn drop(&mut self) {
        println!("   dropped Loud({})", self.0);
    }
}

fn loud(name: &'static str) -> Loud {
    println!("   evaluated loud({name})");
    Loud(name)
}

/// The child half of claim 3: index a three-element array with 99.
fn out_of_bounds() {
    let v = [1, 2, 3];
    let i = black_box(99);
    println!("{}", v[i]);
}

/// The panic lines the child printed, with the two run-specific parts replaced:
/// the OS thread id in parentheses, and the path the source was compiled from.
fn panic_lines() -> Vec<String> {
    let exe = std::env::current_exe().expect("current_exe");
    let out = Command::new(exe).arg("oob").output().expect("spawn child");
    let stderr = String::from_utf8_lossy(&out.stderr);
    stderr
        .lines()
        .filter(|l| !l.trim().is_empty())
        .take(2)
        .map(|l| {
            if let Some(rest) = l.strip_prefix("thread 'main' (") {
                let after_id = &rest[rest.find(')').unwrap() + 1..];
                let location = after_id.rsplit(['/', '\\']).next().unwrap();
                let location = location.trim_start_matches(" panicked at ");
                format!("thread 'main' (<id>) panicked at <dir>/{location}")
            } else {
                l.to_string()
            }
        })
        .collect()
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("oob") {
        out_of_bounds();
        return;
    }

    println!("1. \"Arrays are stack allocated.\"");
    let boxed = Box::new([0u64; 1000]);
    let rows: Vec<[u64; 4]> = vec![[1, 2, 3, 4]; 2];
    println!("   a local [u64; 1000] is {} bytes of stack frame", size_of::<[u64; 1000]>());
    println!("   Box<[u64; 1000]> holds it on the heap; the Box is {} bytes", size_of_val(&boxed));
    println!("   Vec<[u64; 4]> holds its rows on the heap: {} rows, {} bytes each", rows.len(), size_of_val(&rows[0]));
    println!("   a static [u64; 1000] is in the binary's data, {} bytes", size_of_val(&TABLE));
    println!("   Verdict: only when the owner is a local. It lives where its owner does.");

    println!();
    println!("2. \"Out-of-bounds indexing on an array is a compile-time error.\"");
    println!("   Only with a constant index: `xs[3]` on [i32; 3] trips the deny-by-default");
    println!("   lint unconditional_panic. With #![allow(unconditional_panic)] it compiles,");
    println!("   and any index the compiler cannot see is checked at run time:");
    let xs = [1, 2, 3];
    let i = black_box(3);
    println!("   xs.get(i) = {:?}; xs[i] would panic", xs.get(i));
    println!("   Verdict: a compile error only for a constant index; otherwise a panic.");

    println!();
    println!("3. The panic text, on 1.98.0");
    for line in panic_lines() {
        println!("   {line}");
    }
    println!("   Location first, message on its own line: the format since Rust 1.73.");
    println!("   Older transcripts put the message first, in quotes, then the location.");

    println!();
    println!("4. \"Arrays are limited to 32 elements.\"");
    let big = [7u8; 33];
    let copy = big;
    let mut seen = HashSet::new();
    seen.insert(big);
    let total: u32 = big.into_iter().map(u32::from).sum();
    println!("   [u8; 33]: Copy {}, PartialEq {}, Hash {}, IntoIterator sum {}, Debug {:?}..",
             copy == big, big == [7; 33], seen.contains(&copy), total, &big[..2]);
    let small: [u8; 32] = Default::default();
    println!("   Default: [u8; 32] works ({} zeros); [u8; 33] is E0277", small.len());
    let t12 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("   tuples: a 12-tuple is Debug {t12:?}; a 13-tuple is E0277");
    let from_tuple: [i32; 3] = (1, 2, 3).into();
    println!("   From<(T, T, T)> for [T; 3]: {from_tuple:?} (tuples of 1 to 12)");
    println!("   Verdict: true before const generics; on 1.98.0 only Default stops at 32.");

    println!();
    println!("5. Rust by Example's comments");
    let rbe: [i32; 5] = [1, 2, 3, 4, 5];
    let unannotated = [1, 2, 3, 4, 5];
    println!("   \"type signature is superfluous\": unannotated is {} - true", type_name_of_val(&unannotated));
    println!("   size_of_val(&xs) = {} - true: five i32s, no header", size_of_val(&rbe));
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);
    println!("   assert_eq!(&empty_array, &[]) and &[][..]: both pass");
    let got: Vec<Option<&i32>> = (0..rbe.len() + 1).map(|i| rbe.get(i)).collect();
    println!("   0..xs.len() + 1 with .get(i): {got:?}");
    println!("   The +1 is on purpose: the last get is the out-of-range one, and it is None.");
    println!("   \"Out of bound indexing on slice causes runtime error\": for `slice[i]`");
    println!("   yes; the example shows `.get`, which is the version that does not.");

    println!();
    println!("6. assert_eq!([1, 2], &array[1..])");
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;
    assert_eq!([1, 2], &array[1..]);
    print!("   passes, and `for x in array` prints: ");
    for x in array {
        print!("{x} ");
    }
    println!();
    println!("   PartialEq::<&[i32]>::eq(&[1, 2], &&array[1..]) = {}", PartialEq::<&[i32]>::eq(&[1, 2], &&array[1..]));
    println!("   It is an impl, `impl PartialEq<&[U]> for [T; N]`, not a coercion.");

    println!();
    println!("7. \"The value in [value; N] must be Copy or a constant.\"");
    let one: [Loud; 1] = [loud("one"); 1];
    println!("   [loud(\"one\"); 1] compiled, len {}: no copy to make", one.len());
    #[allow(clippy::zero_repeat_side_effects)] // the side effect is what is being checked
    let none: [Loud; 0] = [loud("zero"); 0];
    println!("   [loud(\"zero\"); 0] compiled, len {}: evaluated, then dropped at once", none.len());
    println!("   Verdict: true for N of 2 or more.");
    drop(one);

    println!();
    println!("8. \"[true; 10000] is an array of 10,000 bools\" beside code saying 1000");
    println!("   size_of::<[bool; 1000]>() = {}, size_of::<[bool; 10000]>() = {}", size_of::<[bool; 1000]>(), size_of::<[bool; 10000]>());
    println!("   Verdict: each is right about its own number; read the code, not the prose.");

    println!();
    println!("9. \"Vec::new() does not allocate until elements are pushed.\"");
    let mut v: Vec<[i32; 3]> = Vec::new();
    let before = v.capacity();
    v.push([1, 2, 3]);
    println!("   capacity before push {before}, after one push {}", v.capacity());
    println!("   Verdict: true.");

    println!();
    println!("10. \"Iterators are lazy and do nothing unless consumed.\"");
    let calls = Cell::new(0);
    let numbers = [1, 2, 3, 2];
    let pending = numbers.into_iter().filter(|&n| {
        calls.set(calls.get() + 1);
        n == 2
    });
    println!("   after building the filter: closure ran {} times", calls.get());
    let twos: Vec<i32> = pending.collect();
    println!("   after collect: {twos:?}, closure ran {} times", calls.get());
    println!("   Verdict: true.");

    println!();
    println!("11. A 2015 answer: filter an array with |&&x|");
    let by_ref: Vec<&i32> = numbers.iter().filter(|&&x| x == 2).collect();
    println!("   numbers.iter().filter(|&&x| x == 2) = {by_ref:?} - still right for iter()");
    let by_value: Vec<i32> = numbers.into_iter().filter(|&x| x == 2).collect();
    println!("   numbers.into_iter().filter(|&x| x == 2) = {by_value:?} - edition 2021 on");
    println!("   With into_iter() on 2021, |&&x| is E0308 and |&x| *x == 2 is E0614.");

    println!();
    println!("12. \"The useful methods are on slices, not arrays.\"");
    let doubled = array.map(|x| x * 10);
    let refs: [&i32; 3] = array.each_ref();
    println!("   array.map(|x| x * 10) = {doubled:?}, array.each_ref() = {refs:?}");
    println!("   array.as_slice().len() = {}", array.as_slice().len());
    println!("   Verdict: mostly still true; arrays have gained a few of their own.");
}
