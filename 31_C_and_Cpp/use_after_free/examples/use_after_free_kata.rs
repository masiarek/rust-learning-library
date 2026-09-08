//! Kata solution: write the C bug in Rust, and read the error instead.
//!
//!   rustc --edition 2024 use_after_free_kata.rs -o /tmp/uaf && /tmp/uaf

fn main() {
    println!("THE C SHAPE");
    println!("  char *p = malloc(16); strcpy(p, \"secret\");");
    println!("  free(p);");
    println!("  printf(\"%s\", p);        <- reads a block the allocator now owns");
    println!("  It usually prints something. That is the instructive part: a");
    println!("  freed block is not blanked, it is marked AVAILABLE, so the read");
    println!("  returns whatever lives there now -- often another allocation's");
    println!("  data, which is how a use-after-free becomes an information leak");
    println!("  rather than a crash.");
    println!();

    println!("THE SAME PROGRAM IN RUST");
    println!("  let s = String::from(\"secret\");");
    println!("  drop(s);");
    println!("  println!(\"{{s}}\");        <- E0382: borrow of moved value: `s`");
    println!();
    println!("  drop() is not a special form. It is a function taking its");
    println!("  argument BY VALUE, so calling it moves the String in -- and the");
    println!("  compiler already tracks that `s` no longer owns anything. The");
    println!("  free happens when drop's own parameter goes out of scope, one");
    println!("  line later, and the check that catches the misuse is the same");
    println!("  move checker that catches passing a value to any function twice.");
    println!();

    println!("SO WHAT ACTUALLY HAPPENS HERE");
    let s = String::from("secret");
    let len = s.len();
    drop(s);
    println!("  s was dropped; its length, copied out first, is {len}");
    println!("  There is no way to write the read. Not 'it panics' -- the");
    println!("  program does not exist.");
    println!();

    println!("THE HARDER CASE: A REFERENCE THAT OUTLIVES ITS OWNER");
    println!("  let r;");
    println!("  {{ let v = vec![1, 2, 3]; r = &v[0]; }}   <- v dropped here");
    println!("  println!(\"{{r}}\");                       <- E0597: `v` does not");
    println!("                                            live long enough");
    println!("  That is the borrow checker rather than the move checker, and it");
    println!("  is the one that has no C equivalent at all: C will happily give");
    println!("  you a pointer into a block that is about to be freed and say");
    println!("  nothing, at any warning level.");
    println!();

    let r;
    let v = vec![10, 20, 30];
    r = &v[0];
    println!("  the version that compiles: v outlives r, so r = {r}");
    println!("  Moving `let v` inside a block would break it, and the error");
    println!("  names both the borrow and the drop.");
    println!();

    println!("WHAT THIS COSTS AT RUNTIME");
    println!("  Nothing. There is no free-list check, no tombstone, no refcount");
    println!("  -- the analysis happened at compile time and the generated code");
    println!("  is the same malloc/free pair C would emit. That is the whole");
    println!("  claim: not a safer allocator, an earlier question.");

    assert_eq!(len, 6);
    assert_eq!(*r, 10);
}
