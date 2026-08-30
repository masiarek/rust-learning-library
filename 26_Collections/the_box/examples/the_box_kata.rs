//! Kata solution: a list that ends, and the three sizes that explain it.
//!
//!   rustc --edition 2024 the_box_kata.rs -o /tmp/bk && /tmp/bk

/// A singly linked list of scores. `None` is the end.
#[derive(Debug)]
struct Node {
    score: u32,
    next: Option<Box<Node>>,
}

fn from_slice(scores: &[u32]) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    for &score in scores.iter().rev() {
        head = Some(Box::new(Node { score, next: head }));
    }
    head
}

fn total(node: &Option<Box<Node>>) -> u32 {
    match node {
        None => 0,
        Some(n) => n.score + total(&n.next),
    }
}

fn to_vec(node: &Option<Box<Node>>) -> Vec<u32> {
    let mut out = Vec::new();
    let mut cursor = node;
    while let Some(n) = cursor {
        out.push(n.score);
        cursor = &n.next;
    }
    out
}

/// Prints when it is dropped, so drop order is observable.
struct Loud(&'static str);
impl Drop for Loud {
    fn drop(&mut self) {
        println!("   dropping {}", self.0);
    }
}

fn main() {
    println!("1. The list");
    let list = from_slice(&[5, 3, 0, 4]);
    println!("   scores : {:?}", to_vec(&list));
    println!("   total  : {}", total(&list));

    println!();
    println!("2. Why it costs nothing to say \"or nothing\"");
    println!("   size_of::<Box<Node>>()         = {}", size_of::<Box<Node>>());
    println!("   size_of::<Option<Box<Node>>>() = {}", size_of::<Option<Box<Node>>>());
    println!("   size_of::<Node>()              = {}", size_of::<Node>());
    println!("   The Option is the same size as the Box. A Box can never be null,");
    println!("   so the compiler uses the all-zero bit pattern to mean `None` —");
    println!("   the null-pointer optimisation. `Option<Box<T>>` is a nullable");
    println!("   pointer with the null checked by the type system.");

    println!();
    println!("3. What the recursion costs");
    println!("   `total` calls itself once per node, so a list of 100_000 nodes");
    println!("   would need 100_000 stack frames and overflow the stack. The");
    println!("   `while let` in `to_vec` walks the same list in constant stack:");
    println!("   to_vec = {:?}", to_vec(&list));
    println!("   Rust does not promise tail-call elimination, so the iterative");
    println!("   form is the one to write for a list of unknown length.");

    println!();
    println!("4. Drop order, which the list makes visible");
    {
        let _outer = Loud("outer");
        let _inner = Loud("inner");
        println!("   two values, declared outer then inner:");
    }
    println!("   Last declared, first dropped. The default recursive drop of a");
    println!("   long list has the same stack problem as `total` — which is why");
    println!("   real list types implement Drop by hand, popping in a loop.");

    println!();
    println!("5. The whole point, in one line");
    println!("   Take the Box out of `next: Option<Box<Node>>` and rustc says");
    println!("   \"recursive type `Node` has infinite size\" [E0072], and offers the");
    println!("   fix itself: \"insert some indirection (e.g., a `Box`, `Rc`, or");
    println!("   `&`) to break the cycle\". The type needs a size; a pointer has one.");
}
