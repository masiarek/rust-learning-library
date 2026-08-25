//! Kata solution: a list that ends, built out of a pointer that may be absent.
//!
//!   rustc --edition 2024 nullable_pointers_kata.rs -o /tmp/npk && /tmp/npk

use std::mem::size_of;

/// `next: Node` would be a type of infinite size. `Box` gives it a known size
/// (a pointer), and `Option` is how the last node says "the list stops here" —
/// a job other languages give to a null pointer.
#[derive(Debug)]
struct Node {
    score: u8,
    next: Option<Box<Node>>,
}

struct Stack {
    head: Option<Box<Node>>,
}

impl Stack {
    fn new() -> Self {
        Stack { head: None }
    }

    fn push(&mut self, score: u8) {
        // take() leaves None behind, so the old head can be moved into the new
        // node while `self` is only borrowed.
        let old = self.head.take();
        self.head = Some(Box::new(Node { score, next: old }));
    }

    fn pop(&mut self) -> Option<u8> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.score)
    }

    fn scores(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut cursor = &self.head;
        while let Some(node) = cursor {
            out.push(node.score);
            cursor = &node.next;
        }
        out
    }
}

fn main() {
    let mut stack = Stack::new();
    for s in [5, 3, 0] {
        stack.push(s);
    }

    println!("A list whose end is a None rather than a null:");
    println!("  contents -> {:?}", stack.scores());
    println!("  pop      -> {:?}", stack.pop());
    println!("  contents -> {:?}", stack.scores());
    println!("  pop, pop -> {:?}, {:?}", stack.pop(), stack.pop());
    println!("  pop on empty -> {:?}", stack.pop());

    println!("\nAnd the safety costs nothing:");
    println!("  size_of::<Box<Node>>()         = {}", size_of::<Box<Node>>());
    println!("  size_of::<Option<Box<Node>>>() = {}", size_of::<Option<Box<Node>>>());
    println!("      Same width. None is stored as the one bit pattern a Box can");
    println!("      never hold — the null — so the tag is free. That is why a");
    println!("      linked list in Rust is not paying for its safety.");
}
