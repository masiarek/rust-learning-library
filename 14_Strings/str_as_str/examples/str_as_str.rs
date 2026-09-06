//! `str::as_str`: why `s.as_str()` fails on a `&str`, what to write instead on
//! the smart pointers that actually need it, and the method-resolution rule
//! that got the method stabilized and then taken back again.
//!
//!   rustc --edition 2024 str_as_str.rs -o /tmp/sas && /tmp/sas

use std::any::type_name_of_val as type_of;
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

/// A pixel that carries its three components, plus a trait that hands them out
/// as a slice. This is the shape of the `rgb` crate, reduced to what matters.
struct Pixel([u8; 3]);

trait ComponentSlice {
    fn as_mut_slice(&mut self) -> &mut [u8];
}

impl ComponentSlice for Pixel {
    fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// ...and here is the standard library adding an *inherent* method with a name
/// the trait already used. Nothing was removed and nothing conflicts, but every
/// `p.as_mut_slice()` in the program below now means something else.
impl Pixel {
    fn as_mut_slice(&mut self) -> &mut [u8; 3] {
        &mut self.0
    }
}

fn main() {
    println!("1. On a &str you already have a &str");
    let s: &str = "Hello";
    let whole = &s[..];
    let deref = &*s;
    println!("   s          {s:?}");
    println!("   &s[..]     {whole:?}     same pointer: {}", std::ptr::eq(s, whole));
    println!("   &*s        {deref:?}     same pointer: {}", std::ptr::eq(s, deref));
    println!("   All three are one borrow of one buffer -- not copies that");
    println!("   happen to be equal. There is no conversion left to perform,");
    println!("   which is why the method you reached for is not there.");

    println!();
    println!("2. The error, on stable");
    println!("   let u = s.as_str();");
    println!("     error[E0658]: use of unstable library feature `str_as_str`");
    println!("       |     let u = s.as_str();");
    println!("       |               ^^^^^^");
    println!("       = note: see issue #130366 for more information");
    println!("   Note what it is NOT: not `no method named`, not a missing");
    println!("   trait import. The method exists, is written, and is fenced");
    println!("   off behind #![feature(str_as_str)] -- a nightly-only door.");

    println!();
    println!("3. Where the method would have earned its place");
    let boxed: Box<str> = "Hello".into();
    let counted: Rc<str> = "Hello".into();
    let shared: Arc<str> = "Hello".into();
    let maybe: Cow<'_, str> = Cow::Borrowed("Hello");
    println!("   Box<str>   &*b          -> {}", type_of(&&*boxed));
    println!("   Rc<str>    &*r          -> {}", type_of(&&*counted));
    println!("   Arc<str>   &*a          -> {}", type_of(&&*shared));
    println!("   Cow<str>   &*c          -> {}", type_of(&&*maybe));
    println!("   None of these is a String, so String::as_str is unavailable,");
    println!("   and `&*` is doing real work rather than nothing. That is the");
    println!("   gap the feature was proposed to fill: one spelling, `as_str`,");
    println!("   that reads the same on every owner of some text.");

    println!();
    println!("4. Why the obvious workaround is the disputed one");
    let arr: [u8; 3] = [1, 2, 3];
    println!("   [u8; 3].as_ref()        -> {}", type_of(&arr.as_ref()));
    println!("   Not &[u8; 3]. AsRef is NOT reflexive: there is no blanket");
    println!("   `impl<T> AsRef<T> for T`, so `.as_ref()` on an array finds");
    println!("   nothing at that rung, derefs to [u8], and answers as a slice.");
    println!("   So `.as_ref()` means 'whatever the deref chain offers first',");
    println!("   which is a different question from 'the same thing, as a view'.");
    println!("   Reach for `&*x` when you mean the second one.");

    println!();
    println!("5. And why the method was taken back");
    let mut p = Pixel([1, 2, 3]);
    println!("   p.as_mut_slice()                     -> {}", type_of(&p.as_mut_slice()));
    println!("   ComponentSlice::as_mut_slice(&mut p) -> {}", type_of(&ComponentSlice::as_mut_slice(&mut p)));
    println!("   One call site, two answers. An inherent method beats a trait");
    println!("   method at the same rung, so adding one to a type thousands of");
    println!("   crates already extend is a silent, source-breaking change --");
    println!("   the calls still compile, and hand back a different type.");
    println!("   That is exactly what happened when the feature grew from str");
    println!("   to [T]: the new inherent [T]::as_mut_slice shadowed the rgb");
    println!("   crate's trait method, and the stabilization was reverted.");

    println!();
    println!("6. So, today");
    println!("   &str            nothing -- you already have one");
    println!("   Box/Rc/Arc<str> &*x, or &x[..]");
    println!("   Cow<str>        &*x");
    println!("   String          x.as_str()   (stable since 1.7, and unaffected)");
}
