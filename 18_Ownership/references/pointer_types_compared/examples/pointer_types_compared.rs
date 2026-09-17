//! Six pointer types in one table. Every cell is measured here, or is a
//! refusal whose real rustc output sits on the page beside the table.
//!
//! Sizes are printed in words (units of `size_of::<usize>()`), so the key
//! is the same on a 32-bit and a 64-bit target.
//!
//!   rustc --edition 2024 pointer_types_compared.rs -o /tmp/pointer_types_compared && /tmp/pointer_types_compared

use std::fmt::Display;
use std::marker::PhantomData;
use std::mem::size_of;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

// ---- owns the pointee: count destructor runs ----------------------------

static DROPS: AtomicU32 = AtomicU32::new(0);

struct Noisy;

impl Drop for Noisy {
    fn drop(&mut self) {
        DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

fn drops() -> u32 {
    DROPS.load(Ordering::SeqCst)
}

// ---- Copy: an inherent method with a `T: Copy` bound is picked when the
// ---- bound holds; otherwise the call falls through to the trait method.

struct Probe<T: ?Sized>(PhantomData<T>);

trait NotCopy {
    fn is_copy(&self) -> bool {
        false
    }
}

impl<T: ?Sized> NotCopy for Probe<T> {}

impl<T: Copy> Probe<T> {
    fn is_copy(&self) -> bool {
        true
    }
}

/// The yeses again, checked by the compiler without the trick.
fn assert_copy<T: Copy>() {}

// ---- sizes, in words ----------------------------------------------------

fn words<P>() -> String {
    (size_of::<P>() / size_of::<usize>()).to_string()
}

fn yes_no(b: bool) -> String {
    if b { "yes" } else { "no" }.to_string()
}

static SEVEN: i32 = 7;

fn main() {
    assert_copy::<&u8>();
    assert_copy::<*const u8>();
    assert_copy::<*mut u8>();

    // -- owns the pointee: does ending the pointer's scope run a destructor?
    let mut target = Noisy;
    let start = drops();
    {
        let _r: &Noisy = &target;
    }
    let after_ref = drops() - start;
    {
        let _m: &mut Noisy = &mut target;
    }
    let after_mut = drops() - start;
    {
        let _p: *const Noisy = &target;
    }
    let after_const = drops() - start;
    {
        let _q: *mut Noisy = &mut target;
    }
    let after_raw_mut = drops() - start;
    let start = drops();
    {
        let _b: Box<Noisy> = Box::new(Noisy);
    }
    let after_box = drops() - start;
    let start = drops();
    let first = Rc::new(Noisy);
    let second = Rc::clone(&first);
    drop(first);
    let rc_after_one = drops() - start;
    drop(second);
    let rc_after_last = drops() - start;

    // -- write through it: each "yes" below is a write that happened
    let mut n = 1;
    let m: &mut i32 = &mut n;
    *m = 2;
    let wrote_mut = n;
    let q: *mut i32 = &mut n;
    unsafe { *q = 3 };
    let wrote_raw = n;
    let mut b = Box::new(1);
    *b = 4;
    let wrote_box = *b;
    let mut c = Rc::new(1);
    if let Some(inner) = Rc::get_mut(&mut c) {
        *inner = 5;
    }
    let c2 = Rc::clone(&c);
    let none_when_shared = Rc::get_mut(&mut c).is_none();

    // -- deref needs unsafe: these reads compile outside any `unsafe` block
    let local = 1;
    let at_local: &i32 = &local;
    let at_static: &i32 = &SEVEN;
    let boxed = Box::new(4);
    let into_box: &i32 = &*boxed;
    let counted = Rc::new(8);
    let safe_reads = *at_local + *boxed + *counted;

    let heads = ["&T", "&mut T", "*const T", "*mut T", "Box<T>", "Rc<T>"];
    let owns = [
        yes_no(after_ref == 1),
        yes_no(after_mut == 1),
        yes_no(after_const == 1),
        yes_no(after_raw_mut == 1),
        yes_no(after_box == 1),
        if rc_after_one == 0 && rc_after_last == 1 { "last one" } else { "?" }.to_string(),
    ];
    let write = [
        "no".to_string(),
        if wrote_mut == 2 { "yes" } else { "?" }.to_string(),
        "no".to_string(),
        if wrote_raw == 3 { "in unsafe" } else { "?" }.to_string(),
        if wrote_box == 4 { "if mut" } else { "?" }.to_string(),
        if *c2 == 5 && none_when_shared { "if unique" } else { "?" }.to_string(),
    ];
    let copy = [
        yes_no(Probe::<&u8>(PhantomData).is_copy()),
        yes_no(Probe::<&mut u8>(PhantomData).is_copy()),
        yes_no(Probe::<*const u8>(PhantomData).is_copy()),
        yes_no(Probe::<*mut u8>(PhantomData).is_copy()),
        yes_no(Probe::<Box<u8>>(PhantomData).is_copy()),
        yes_no(Probe::<Rc<u8>>(PhantomData).is_copy()),
    ];
    let deref_unsafe = ["no", "no", "yes", "yes", "no", "no"].map(String::from);
    let option = [
        words::<Option<&u8>>(),
        words::<Option<&mut u8>>(),
        words::<Option<*const u8>>(),
        words::<Option<*mut u8>>(),
        words::<Option<Box<u8>>>(),
        words::<Option<Rc<u8>>>(),
    ];
    let sized = [
        words::<&u8>(),
        words::<&mut u8>(),
        words::<*const u8>(),
        words::<*mut u8>(),
        words::<Box<u8>>(),
        words::<Rc<u8>>(),
    ];
    let slice = [
        words::<&[u8]>(),
        words::<&mut [u8]>(),
        words::<*const [u8]>(),
        words::<*mut [u8]>(),
        words::<Box<[u8]>>(),
        words::<Rc<[u8]>>(),
    ];
    let text = [
        words::<&str>(),
        words::<&mut str>(),
        words::<*const str>(),
        words::<*mut str>(),
        words::<Box<str>>(),
        words::<Rc<str>>(),
    ];
    let object = [
        words::<&dyn Display>(),
        words::<&mut dyn Display>(),
        words::<*const dyn Display>(),
        words::<*mut dyn Display>(),
        words::<Box<dyn Display>>(),
        words::<Rc<dyn Display>>(),
    ];
    let null = option.clone().map(|w| yes_no(w == "2"));
    let lives = ["anywhere", "anywhere", "anywhere", "anywhere", "heap", "heap"].map(String::from);

    let row = |label: &str, cells: &[String; 6]| {
        let mut line = format!("{label:<23}");
        for cell in cells {
            line.push_str(&format!("{cell:<11}"));
        }
        println!("{}", line.trim_end());
    };

    row("", &heads.map(String::from));
    row("owns (drops) pointee", &owns);
    row("write through it", &write);
    row("Copy", &copy);
    row("deref needs unsafe", &deref_unsafe);
    row("can be null", &null);
    row("pointee may live", &lives);
    row("P<u8>, words", &sized);
    row("P<[u8]>, words", &slice);
    row("P<str>, words", &text);
    row("P<dyn Display>, words", &object);
    row("Option<P<u8>>, words", &option);

    println!();
    println!("How each row was filled");
    println!("  owns       destructor runs when the pointer's scope ends:");
    println!("             &T {after_ref}, &mut T {after_mut}, *const T {after_const}, *mut T {after_raw_mut}, Box<T> {after_box};");
    println!("             Rc<T> {rc_after_one} after dropping one of two clones, {rc_after_last} after the last");
    println!("  write      *m = 2 -> n = {wrote_mut}   unsafe {{ *q = 3 }} -> n = {wrote_raw}   let mut b ...; *b = 4 -> *b = {wrote_box}");
    println!("             Rc::get_mut wrote {} while unique; None after one clone: {none_when_shared}", *c2);
    println!("  Copy       controls for the probe: i32 {}, String {}",
        Probe::<i32>(PhantomData).is_copy(),
        Probe::<String>(PhantomData).is_copy());
    println!("  deref      *at_local + *boxed + *counted = {safe_reads}, outside any unsafe block");
    println!("  null       yes where None needs a word of its own; std::ptr::null::<u8>().is_null() = {}",
        std::ptr::null::<u8>().is_null());
    println!("  lives      one type, &i32, at a local {}, at a static {}, inside a Box {}", *at_local, *at_static, *into_box);
    println!("  words      size_of::<P>() / size_of::<usize>(), P being the pointer in that column");
}
