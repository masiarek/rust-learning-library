//! Printing borrows. Every formatting macro takes a reference to each of its
//! arguments, so a value can be printed any number of times and its owner
//! still has it afterwards. `Ticket` announces its own drop, so the output
//! shows where a move ends a value's life, and that printing never does.
//!
//!   rustc --edition 2024 printing_borrows.rs -o /tmp/pb && /tmp/pb

use std::fmt::{self, Write};

/// Neither `Copy` nor `Clone`, and it prints a line when it is dropped.
struct Ticket(u32);

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ticket #{}", self.0)
    }
}

impl Drop for Ticket {
    fn drop(&mut self) {
        println!("  ~ ticket #{} dropped", self.0);
    }
}

/// Takes the ticket by value: calling this is a move, and the ticket is
/// dropped when this function returns.
fn take(t: Ticket) {
    println!("  take() owns {t} now");
}

/// Also takes the ticket by value, and hands back a `String` to print.
fn label(t: Ticket) -> String {
    format!("{t}, labelled")
}

fn rule(title: &str) {
    println!("\n──── {title}");
}

fn main() {
    rule("The two spellings from the question");
    let a = "foo".to_string();
    let b = String::from("foo");
    println!("  {a} {a}"); // foo foo
    println!("  {b} {b}"); // foo foo
    println!("  a == b: {}", a == b); // true

    rule("Every formatting macro borrows: t is used again after each one");
    let t = Ticket(7);
    println!("  println!   {t}");
    print!("  print!     {t}");
    println!();
    let formatted = format!("{t}");
    println!("  format!    {formatted}");
    let mut buf = String::new();
    write!(buf, "{t}").unwrap();
    println!("  write!     {buf}");
    assert!(t.0 == 7, "unexpected {t}");
    println!("  assert!    passed, so its message was never formatted");
    if t.0 != 7 {
        panic!("{t}");
    }
    println!("  panic!     compiled, in a branch that did not run");
    assert_eq!(a, b);
    println!("  assert_eq! passed, and a and b are still here: {a} {b}");
    println!("  no drop line yet: t still owns {t}");

    rule("A function that takes it by value: the move");
    take(t);
    println!("  back in main; t was moved, so this line cannot name it");

    rule("An argument that consumes: the move happens before println! runs");
    let u = Ticket(8);
    println!("  {}", label(u));
    println!("  the drop line came first: label(u) returned before the print began");
}
