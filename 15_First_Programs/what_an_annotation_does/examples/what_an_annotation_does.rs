//! What a type annotation actually does — four answers, one per expression shape.
//!
//!   rustc --edition 2024 what_an_annotation_does.rs -o /tmp/waad && /tmp/waad

use std::collections::BTreeSet;
use std::mem::size_of_val;
use std::ops::Deref;

/// The static type of whatever you hand it a reference to.
/// `type_name` erases lifetimes, so `&'static str` prints as `&str`.
fn ty<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn shout(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    println!("1. A string literal — the annotation changes nothing");
    let a = "a";
    let b: &str = "a";
    let c: &'static str = "a";
    println!("   let a = \"a\";                 {}", ty(&a));
    println!("   let b: &str = \"a\";           {}", ty(&b));
    println!("   let c: &'static str = \"a\";   {}", ty(&c));
    println!("   same value in all three? {}", a == b && b == c);
    println!("   A literal has exactly one possible type, so there is nothing for the");
    println!("   annotation to decide. (type_name erases lifetimes; all three are");
    println!("   &'static str — the lifetime half is the static_str lesson.)");

    println!("\n2. An integer literal — the annotation picks the type");
    let n = 1;
    let m: u8 = 1;
    let f: f64 = 1.0;
    println!("   let n = 1;         {:<4} size {}   <- the i32 fallback", ty(&n), size_of_val(&n));
    println!("   let m: u8 = 1;     {:<4} size {}", ty(&m), size_of_val(&m));
    println!("   let f: f64 = 1.0;  {:<4} size {}", ty(&f), size_of_val(&f));
    println!("   One literal, three types. i32 is what Rust falls back to when nothing");
    println!("   else decides — and an annotation is one of the things that can decide.");
    println!("   Load-bearing, not decorative:");
    println!("      let big: u8 = 1_000_000;   // error: literal out of range for `u8`");

    println!("\n3. A borrow — the annotation performs a coercion");
    let owned = String::from("hi");
    let r = &owned;
    let s: &str = &owned;
    println!("   let r = &owned;        {}", ty(&r));
    println!("   let s: &str = &owned;  {}   <- deref coercion, asked for by the annotation", ty(&s));
    println!("   The expression `&owned` only ever produces a &String. The annotation");
    println!("   asks for a &str, which is a different type — and String's");
    println!("   `impl Deref<Target = str>` is the permission slip that lets the");
    println!("   compiler bridge them by inserting a call. Four spellings, one result:");
    let by_coercion: &str = &owned;
    let by_deref_star: &str = &*owned;
    let by_method_call: &str = Deref::deref(&owned);
    let by_as_str: &str = owned.as_str();
    println!("      let s: &str = &owned;               the coercion (compiler-inserted)");
    println!("      let s: &str = &*owned;              what it desugars to");
    println!("      let s: &str = Deref::deref(&owned); what the `*` calls");
    println!("      let s: &str = owned.as_str();       the same thing, written out");
    println!(
        "   same pointer in all four? {}",
        [by_deref_star, by_method_call, by_as_str]
            .iter()
            .all(|v| v.as_ptr() == by_coercion.as_ptr())
    );
    println!(
        "   String is {} bytes on the stack (ptr, len, cap); &str is {} (ptr, len).",
        size_of_val(&owned),
        size_of_val(&s)
    );
    println!("   The coercion copies no text and allocates nothing — it forgets the");
    println!("   capacity field and keeps pointing at the same bytes.");
    let v1 = vec![&owned];
    let v2: Vec<&str> = vec![&owned];
    println!("   vec![&owned]                     {}", ty(&v1));
    println!("   let _: Vec<&str> = vec![&owned]; {}", ty(&v2));
    println!("   A call site coerces too — shout(r) compiles and gives {:?} — so", shout(r));
    println!("   the annotation only earns its keep where nothing else offers to coerce:");
    println!("   inside a Vec, a tuple, a struct field, a return type.");

    println!("\n4. No answer without one — the annotation drives inference");
    let parsed: i32 = "42".parse().unwrap();
    let wide: i64 = "42".parse().unwrap();
    println!("   let parsed: i32 = \"42\".parse().unwrap();  {:<4} {}", ty(&parsed), parsed);
    println!("   let wide:   i64 = \"42\".parse().unwrap();  {:<4} {}", ty(&wide), wide);
    println!("      let x = \"42\".parse().unwrap();   // error[E0284]: type annotations needed");

    let letters = ['R', 'u', 's', 't', 'a', 'c', 'e', 'a', 'n'];
    let joined: String = letters.iter().collect();
    let listed: Vec<char> = letters.iter().copied().collect();
    let unique: BTreeSet<char> = letters.iter().copied().collect();
    println!("   One expression, three results — only the annotation differs:");
    println!("      let _: String        = letters.iter().collect();   {:?}", joined);
    println!("      let _: Vec<char>     = ...collect();               {} items", listed.len());
    println!("      let _: BTreeSet<char> = ...collect();              {} items, {:?}", unique.len(), unique);

    println!("\nThe rule");
    println!("   Annotate when the expression is ambiguous (a numeric literal, parse,");
    println!("   collect, into) or when you want a coercion. On \"a\" it is neither, so");
    println!("   `let s = \"a\";` and `let s: &str = \"a\";` are the same program.");
}
