//! Kata solution: responsibility as a property of the value.
//!
//!   rustc --edition 2024 double_free_kata.rs -o /tmp/dfk && /tmp/dfk

struct Tracked(&'static str);
impl Drop for Tracked {
    fn drop(&mut self) { println!("    freeing {}", self.0); }
}

fn consume(t: Tracked) { println!("    consume() took {}", t.0); }

fn main() {
    println!("THE C SHAPE");
    println!("  free(p); ... free(p);");
    println!("  Two calls, one block. The second one corrupts the allocator's");
    println!("  bookkeeping, and the crash -- if there is one -- happens in some");
    println!("  unrelated malloc much later. The bug is not really about");
    println!("  memory: it is that TWO PIECES OF CODE both believed they were");
    println!("  responsible for the same block.");
    println!();

    println!("RUST MAKES RESPONSIBILITY A PROPERTY OF THE VALUE");
    println!("  let a = Tracked(\"one\");");
    let a = Tracked("one");
    println!("  consume(a);          <- moves it: consume is now responsible");
    consume(a);
    println!("  consume(a);          <- E0382: use of moved value");
    println!();
    println!("  Exactly one binding owns the value at a time, and passing it by");
    println!("  value transfers that. So the second call is not a runtime");
    println!("  double-free, it is a compile error naming the line that took");
    println!("  ownership -- which is also the line a C reviewer would have had");
    println!("  to notice by reading the callee.");
    println!();

    println!("AND THE DROP HAPPENS EXACTLY ONCE, WHEREVER RESPONSIBILITY ENDED");
    println!("  three values, three scopes:");
    {
        let _b = Tracked("two (block scope)");
        println!("    inside the block");
    }
    let c = Tracked("three (moved into a Vec)");
    let v = vec![c];
    println!("    the Vec owns it now, and owns the freeing");
    drop(v);
    let d = Tracked("four (end of main)");
    println!("    d will go last");
    println!();

    println!("WHAT ABOUT Rc?");
    println!("  Rc<T> shares ownership between several holders and frees when");
    println!("  the LAST one goes. That is a different answer to the same");
    println!("  question -- responsibility held jointly, counted at runtime --");
    println!("  and it still cannot double-free, because no holder can free on");
    println!("  its own. You pay a counter for it, which is why it is not the");
    println!("  default.");
    println!();

    println!("THE PART WORTH CARRYING TO C");
    println!("  The C fix is a convention: 'the caller frees', written in a");
    println!("  comment. Rust's fix is the same convention, moved into the type");
    println!("  system where the compiler can check it -- and the reason it can");
    println!("  is that a value has exactly one owner unless you ask otherwise.");
    println!();
    println!("  end of main:");
    let _ = &d;
}
