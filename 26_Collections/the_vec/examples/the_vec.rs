//! A `Vec` is three numbers on the stack and one allocation on the heap.
//!
//!   rustc --edition 2024 the_vec.rs -o /tmp/vec && /tmp/vec

use std::any::type_name_of_val;
use std::fmt::Display;

fn total(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

/// One type for a column that may hold three — the closed-set answer to
/// `Vec<T>` holding exactly one `T`.
enum Value {
    Int(i64),
    Text(String),
    Real(f64),
}

impl Value {
    fn render(&self) -> String {
        match self {
            Value::Int(n) => format!("Int({n})"),
            Value::Text(s) => format!("Text({s:?})"),
            Value::Real(x) => format!("Real({x})"),
        }
    }
}

fn main() {
    println!("1. Three numbers: pointer, length, capacity");
    println!("   size_of::<Vec<u32>>()  = {}  (three usize)", size_of::<Vec<u32>>());
    println!("   size_of::<Vec<[u8; 999]>>() = {}  — the same three, whatever T is",
             size_of::<Vec<[u8; 999]>>());
    println!("   The elements are not in the Vec. The Vec is a receipt for them.");

    println!();
    println!("2. One type per Vec — and the compiler works out which");
    let inferred = vec![1, 2, 3];
    let suffixed = vec![1, 2, 3u64];
    let annotated: Vec<f64> = vec![1.0, 2.0, 3.0];
    println!("   vec![1, 2, 3]           -> Vec<{}>", type_name_of_val(&inferred[0]));
    println!("   vec![1, 2, 3u64]        -> Vec<{}>   — the LAST element decided",
             type_name_of_val(&suffixed[0]));
    println!("   let a: Vec<f64> = ...   -> Vec<{}>   — the annotation decided",
             type_name_of_val(&annotated[0]));
    println!("   `vec![1, \"two\", 3.0]` is error[E0308], and it names the second");
    println!("   element: one type per Vec, and rustc stops at the first quarrel.");
    println!("   Two ways to hold a mixture, both of which give every element");
    println!("   ONE type:");
    let closed = [Value::Int(7), Value::Text("seven".into()), Value::Real(7.5)];
    let rendered: Vec<String> = closed.iter().map(Value::render).collect();
    println!("   enum, closed set:   {}", rendered.join(", "));
    let open: Vec<Box<dyn Display>> = vec![Box::new(7), Box::new("seven"), Box::new(7.5)];
    let shown: Vec<String> = open.iter().map(|c| c.to_string()).collect();
    println!("   Box<dyn>, open set: {}", shown.join(", "));

    println!();
    println!("3. Growth is amortised doubling, and you can watch it");
    let mut v: Vec<u32> = Vec::new();
    println!("   Vec::new()            len {} cap {}", v.len(), v.capacity());
    for n in 1..=9u32 {
        let before = v.capacity();
        v.push(n);
        let after = v.capacity();
        if after != before {
            println!("   push({n}) reallocated   len {} cap {before} -> {after}", v.len());
        }
    }
    println!("   after 9 pushes        len {} cap {}", v.len(), v.capacity());
    println!("   Nine pushes, three allocations. Doubling is why pushing n items");
    println!("   costs O(n) in total rather than O(n^2), and the exact sequence is");
    println!("   this std's choice, not a promise in the language.");

    println!();
    println!("4. If you know the size, say so");
    let mut sized: Vec<u32> = Vec::with_capacity(9);
    let start = sized.capacity();
    for n in 1..=9u32 {
        sized.push(n);
    }
    println!("   with_capacity(9): cap {start} -> {} — no reallocation at all", sized.capacity());
    println!("   Same nine values, one allocation instead of three, and no copying");
    println!("   of the old contents.");

    println!();
    println!("5. What `growable` costs");
    println!("   [f64; 4]        {} bytes, all payload, in one place",
             size_of::<[f64; 4]>());
    println!("   Vec<f64> of 4   {} header + {} heap = {} bytes, in two",
             size_of::<Vec<f64>>(), 4 * size_of::<f64>(),
             size_of::<Vec<f64>>() + 4 * size_of::<f64>());
    let wave: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let elements_were = wave.as_ptr();
    let moved = wave;
    println!("   moving it copied the {}-byte header and not one element: {}",
             size_of::<Vec<f64>>(), moved.as_ptr() == elements_were);
    let mut grow: Vec<u32> = Vec::new();
    let mut allocations = 0;
    for n in 1..=100u32 {
        let cap = grow.capacity();
        grow.push(n);
        if grow.capacity() != cap {
            allocations += 1;
        }
    }
    println!("   100 pushes = 100 capacity checks and {allocations} allocator calls");
    println!("   So the bookkeeping is paid at the EDGES — allocate, grow, free —");
    println!("   and per element it is one pointer hop the optimizer hoists out of");
    println!("   a loop. Which is why `use a Vec` is still the default advice.");

    println!();
    println!("6. A Vec derefs to a slice, so slice methods just work");
    println!("   total(&v) = {} — `total` takes &[u32] and was handed a &Vec<u32>",
             total(&v));
    println!("   v.first() = {:?}, v.contains(&5) = {}", v.first(), v.contains(&5));
    println!("   v.iter().rev().take(3): {:?}", v.iter().rev().take(3).collect::<Vec<_>>());
    let heap: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let stack: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let from_vec: &[f64] = &heap;
    let from_array: &[f64] = &stack;
    println!("   &Vec<f64>   as &[f64]: {from_vec:?}");
    println!("   &[f64; 4]   as &[f64]: {from_array:?}");
    println!("   Two owners, two storage stories, ONE borrowed type: {} bytes of",
             size_of_val(&from_vec));
    println!("   pointer-and-length from the heap, {} from the stack — the same",
             size_of_val(&from_array));
    println!("   type, so the same function takes either.");
    println!("   Write &[T] in a signature and both callers work. Write &Vec<T>");
    println!("   and you have refused arrays, slices and everything borrowed.");

    println!();
    println!("7. Removing: the one that keeps order, and the one that is fast");
    let mut a: Vec<char> = "abcdef".chars().collect();
    let mut b = a.clone();
    let removed = a.remove(1);
    let swapped = b.swap_remove(1);
    println!("   remove(1)      -> {removed}, left {a:?}   (everything after shifts down)");
    println!("   swap_remove(1) -> {swapped}, left {b:?}   (the last element fills the hole)");
    println!("   O(n) versus O(1). If you are about to sort anyway, take the O(1).");

    let mut scores = vec![5, 3, 0, 4, 0, 2];
    scores.retain(|&s| s > 0);
    println!("   retain(|s| s > 0) on [5, 3, 0, 4, 0, 2] -> {scores:?}");
    println!("   One pass, one shift per survivor — not one `remove` per zero.");
}
