//! `&'a mut Node<'a>` borrows the node for the rest of its life. This program
//! shows the signature that does not, what the one that does still allows, and
//! the destructor it forbids.
//!
//!   rustc --edition 2024 borrowing_forever.rs -o /tmp/bf && /tmp/bf

use std::mem::ManuallyDrop;

#[derive(Debug)]
struct Node<'a>(&'a str);

/// Two lifetimes: the borrow of the node (elided) and the text it holds (`'_`).
/// The borrow ends when the call returns.
fn rename(node: &mut Node<'_>, name: &'static str) {
    node.0 = name;
}

/// One lifetime for both: the borrow of the node must last as long as `'a`,
/// and `'a` has to cover every later use of the node.
fn rename_forever<'a>(node: &'a mut Node<'a>, name: &'static str) -> &'a mut Node<'a> {
    node.0 = name;
    node
}

/// The shared version. `Node` is covariant in `'a`, so the compiler shrinks
/// `'a` to the call and nothing stays locked.
fn look<'a>(node: &'a Node<'a>) -> usize {
    node.0.len()
}

struct Loud<'a>(&'a str);

impl Drop for Loud<'_> {
    fn drop(&mut self) {
        println!("   drop runs for Loud({:?})", self.0);
    }
}

fn touch(loud: &mut Loud<'_>) {
    loud.0 = "touched";
}

fn touch_forever<'a>(loud: &'a mut Loud<'a>) {
    loud.0 = "touched forever";
}

fn main() {
    let leaf = String::from("leaf");

    println!("1. &mut Node<'_>: the borrow ends at the call");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    rename(&mut node, "branch");
    println!("   renamed twice, then printed: {node:?}");

    println!();
    println!("2. &'a mut Node<'a>: the borrow lasts as long as the node");
    let mut locked = Node(&leaf);
    let handle = rename_forever(&mut locked, "twig");
    handle.0 = "branch";
    println!("   everything goes through the handle now: {handle:?}");
    println!("   `locked` itself may not be named again: printing it, calling a");
    println!("   method, taking & or &mut, or moving it are all refused.");

    println!();
    println!("3. &'a Node<'a>: shared, so the same shape costs nothing");
    let shared = Node(&leaf);
    println!("   look = {}, look again = {}", look(&shared), look(&shared));
    println!("   and still usable: {shared:?}");

    println!();
    println!("4. A destructor needs the value at the closing brace");
    {
        let mut loud = Loud(&leaf);
        touch(&mut loud);
        println!("   &mut Loud<'_> compiles, and the drop is allowed to run:");
    }
    let mut kept = ManuallyDrop::new(Loud(&leaf));
    touch_forever(&mut kept);
    println!("   &'a mut Loud<'a> compiles only when no drop will run,");
    println!("   here inside ManuallyDrop, whose destructor never calls Loud's.");
    println!("   `kept` is locked like `locked` above: not even kept.0 may be read.");
}
