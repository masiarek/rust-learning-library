//! The claims around the table: where a pointer sits versus where its
//! pointee sits, `Rc` written through, raw pointers cast both ways, moving
//! out through a `Box`, `==` versus `std::ptr::eq`, and what a reference
//! refuses that a raw pointer allows.
//!
//! Sizes are in words (units of `size_of::<usize>()`); no address is printed.
//!
//!   rustc --edition 2024 pointer_types_evidence.rs -o /tmp/pointer_types_evidence && /tmp/pointer_types_evidence

use std::mem::size_of;
use std::ops::Add;
use std::ptr;
use std::rc::Rc;

fn words<T>() -> usize {
    size_of::<T>() / size_of::<usize>()
}

struct Pair {
    left: String,
    right: String,
}

fn main() {
    println!("1. \"Data location\" is where the pointer sits, not where it points");
    let boxed = Box::new(5);
    let r: &i32 = &*boxed;
    println!("   let r: &i32 = &*boxed;          ptr::eq(r, &*boxed) = {}", ptr::eq(r, &*boxed));
    let names: Vec<&str> = vec!["ferris", "corro"];
    println!(
        "   let names: Vec<&str> = ...;     ptr::eq(&names[0], names.as_ptr()) = {}, capacity {}",
        ptr::eq(&names[0], names.as_ptr()),
        names.capacity()
    );
    let local = 9;
    let on_heap: Box<&i32> = Box::new(&local);
    println!("   let on_heap: Box<&i32> = Box::new(&local);   **on_heap = {}", **on_heap);

    println!();
    println!("2. Rc<T> is writable while it is the only Rc");
    let mut a = Rc::new(1);
    *Rc::get_mut(&mut a).unwrap() += 1;
    println!("   Rc::get_mut, unique:            a = {a}");
    let b = Rc::clone(&a);
    *Rc::make_mut(&mut a) += 10;
    println!(
        "   Rc::make_mut, shared:           a = {a}, b = {b}, Rc::ptr_eq = {}, counts {} and {}",
        Rc::ptr_eq(&a, &b),
        Rc::strong_count(&a),
        Rc::strong_count(&b)
    );

    println!();
    println!("3. *const T and *mut T cast into each other with no unsafe");
    let mut n = 1;
    let q: *mut i32 = &mut n;
    let p: *const i32 = q.cast_const();
    let back: *mut i32 = p as *mut i32;
    unsafe { *back = 2 };
    println!("   q.cast_const() as *mut i32, write 2:   n = {n}, ptr::eq(p, back) = {}", ptr::eq(p, back));

    println!();
    println!("4. Moving out through a Box");
    let boxed_string = Box::new(String::from("moved out"));
    let whole: String = *boxed_string;
    println!("   let whole: String = *boxed_string;             {whole:?}");
    let pair = Box::new(Pair { left: String::from("left"), right: String::from("right") });
    let left = pair.left;
    let right = pair.right;
    println!("   let left = pair.left; let right = pair.right;  {left:?} {right:?}");
    let x: Box<u32> = Box::new(12);
    let y: &u32 = &*x;
    println!("   let y: &u32 = &*x;                             {y}");

    println!();
    println!("5. == on references compares pointees; ptr::eq compares addresses");
    let five = 5;
    let also_five = 5;
    println!("   &five == &also_five             {}", &five == &also_five);
    println!("   ptr::eq(&five, &also_five)      {}", ptr::eq(&five, &also_five));
    println!("   ptr::eq(&five, &five)           {}", ptr::eq(&five, &five));
    println!("   Box::new(5) == Box::new(5)      {}", Box::new(5) == Box::new(5));
    println!("   Rc::new(5) == Rc::new(5)        {}", Rc::new(5) == Rc::new(5));

    println!();
    println!("6. A reference is a pointer with rules");
    let nums = [10, 20, 30];
    let first: &i32 = &nums[0];
    let raw: *const i32 = nums.as_ptr();
    println!("   unsafe {{ *raw.add(1) }}          {}   (the pointer moved)", unsafe { *raw.add(1) });
    println!("   first.add(1), ops::Add in scope {}   (the pointee was added to)", first.add(1));
    println!("   &[i32; 3] {} word, &[i32] {} words", words::<&[i32; 3]>(), words::<&[i32]>());
    println!("   Option<&i32> {} word, Option<*const i32> {} words", words::<Option<&i32>>(), words::<Option<*const i32>>());
}
