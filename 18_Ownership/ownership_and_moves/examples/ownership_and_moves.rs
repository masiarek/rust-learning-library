//! Ownership and moves: who is responsible for freeing this value?
//!
//! A move is not a copy and not a free. It is a transfer of RESPONSIBILITY —
//! the bytes stay where they are, and what changes is who will drop them.
//!
//! Every step is made visible by a type that announces its own destruction, so
//! you can see exactly when each value dies rather than taking it on trust.
//!
//!   rustc --edition 2024 ownership_and_moves.rs -o /tmp/oam && /tmp/oam

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

/// Prints when dropped. Ownership is invisible until something says goodbye.
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("      · drop({})", self.0);
    }
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() {
    banner(1, "Rule 3, made visible: the owner's scope end frees the value");

    println!("  entering an inner block");
    {
        let _inner = Noisy("inner");
        println!("  inside — inner is alive");
    }
    println!("  the block ended, and inner was freed just above");

    let _outer = Noisy("outer");
    println!("  outer lives until step1 returns");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "A move transfers RESPONSIBILITY, not bytes");

    let first = Noisy("the-value");
    println!("  `first` owns it");
    {
        let _second = first; // ← the move
        println!("  moved to `second`, inside the block");
    }
    println!("  the block ended — and it was freed THERE, not here");
    println!("      The bytes never travelled. What changed is who frees it, and so");
    println!("      when. And `first` is not merely stale: it is unusable, by name.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "Copy types are duplicated instead of moved");

    let a = 5;
    let b = a;
    println!("  i32     a = {a}, b = {b}        both still usable");

    let flag = true;
    let same = flag;
    println!("  bool    flag = {flag}, same = {same}");

    let s = String::from("hi");
    let t = s;
    println!("  String  t = {t:?}              `s` is gone: reading it is error[E0382]");

    println!("      A type is Copy when duplicating it means copying its bytes and");
    println!("      nothing else. A String owns a heap buffer, so a byte copy would");
    println!("      leave TWO owners of one allocation — and two frees. Moves exist");
    println!("      to make that unrepresentable, not merely discouraged.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn consume(n: Noisy) {
    println!("  consume() owns {} now, and will free it on return", n.0);
}

fn inspect(n: &Noisy) {
    println!("  inspect() only borrows {}", n.0);
}

fn hand_back(n: Noisy) -> Noisy {
    println!("  hand_back() received {} and returns it", n.0);
    n
}

fn step4() {
    banner(4, "Passing by value moves; borrowing does not");

    let a = Noisy("a");
    inspect(&a);
    inspect(&a);
    println!("  a survived two borrows");
    consume(a);
    println!("  consume() has returned, and a is already gone");

    let b = Noisy("b");
    let b = hand_back(b);
    println!("  b was moved out and moved back: still owned here as {}", b.0);
}

// ─────────────────────────────────────────────────────────── Step 5
struct Person {
    name: String,
    age: u8,
}

fn step5() {
    banner(5, "Partial moves: ownership is tracked per FIELD");

    let p = Person { name: String::from("Ada"), age: 36 };
    let name = p.name; // moves one field out, leaving the struct half-empty
    println!("  moved out   name = {name:?}");
    println!("  still fine  p.age = {}", p.age);
    println!("      But `p` as a whole is gone — passing it anywhere is");
    println!("      error[E0382] 'use of partially moved value'. The compiler");
    println!("      is tracking each field separately, not the variable.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn step6() {
    banner(6, "Asking for a second one, and getting one out of a collection");

    let original = String::from("parcel");
    let copy = original.clone();
    println!("  clone     original = {original:?}, copy = {copy:?}   (two allocations)");

    let v = vec![String::from("x"), String::from("y")];
    let borrowed = &v[0];
    println!("  &v[0]     {borrowed:?}   — indexing yields a place, not a value:");
    println!("            `let first = v[0];` is error[E0507], cannot move out of index");

    let mut v = v;
    let owned = v.remove(0);
    println!("  remove    owned = {owned:?}, v is now {v:?}");

    let mut slot = String::from("here");
    let taken = std::mem::take(&mut slot);
    println!("  take      taken = {taken:?}, slot left at Default = {slot:?}");
    println!("      Each of these is the same question answered differently: the");
    println!("      collection still owes one free per element, so it will not let");
    println!("      you walk off with an element unless something replaces it.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    println!();
}
