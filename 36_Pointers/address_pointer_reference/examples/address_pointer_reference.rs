//! Address, pointer, reference: one number, three types, and what each type adds.
//!
//! No line below prints an address. An address differs on every run, so every
//! line is a comparison, a size, or a value read back.
//!
//!   rustc --edition 2024 address_pointer_reference.rs -o /tmp/apr && /tmp/apr

use std::mem::size_of;
use std::ptr;

fn main() {
    println!("1. One number, three types");
    let x: u32 = 0x2A2A_2A2A; // every byte is 42, so the byte order cannot show
    let r: &u32 = &x; //                a reference
    let p: *const u32 = r; //           a raw pointer: the reference, coerced
    let a: usize = p.addr(); //         an address: only the number
    println!("   r as *const u32 == p           {}", r as *const u32 == p);
    println!("   r as *const u32 as usize == a  {}", r as *const u32 as usize == a);
    println!("   size_of: &u32 {}, *const u32 {}, usize {}",
        size_of::<&u32>(), size_of::<*const u32>(), size_of::<usize>());
    println!("   Same number, same width. The difference is what each type lets you do.");

    println!();
    println!("2. What a pointer adds to an address: the type of what is there");
    println!("   read as u32 through p                 {}", unsafe { *p });
    println!("   read as u8 through p.cast::<u8>()     {}", unsafe { *p.cast::<u8>() });
    println!("   size_of::<*const u8>()          = {}", size_of::<*const u8>());
    println!("   size_of::<*const [u64; 1000]>() = {}", size_of::<*const [u64; 1000]>());
    println!("   Same address, 4 bytes read or 1. The type says how many bytes to");
    println!("   read and what they mean, and it exists only in the compiler: at run");
    println!("   time a pointer to one byte and a pointer to 8,000 are both one word.");

    println!();
    println!("3. What an address alone has lost: provenance");
    let bare = ptr::without_provenance::<u32>(a);
    println!("   bare.addr() == p.addr()        {}", bare.addr() == p.addr());
    println!("   bare == p                      {}", bare == p);
    println!("   Equal as numbers and equal under ==, which compares addresses only.");
    println!("   Reading through `bare` is still undefined behaviour: it was made from");
    println!("   a number, so it carries no permission to touch x. Miri reports it as");
    println!("   a pointer that \"has no provenance\". This program never reads it.");

    println!();
    println!("4. A raw pointer can be made from nothing");
    let made_up = 0x1000 as *const u32; // safe code: nothing is checked
    println!("   0x1000 as *const u32  -> is_null() {}, addr() {:#x}",
        made_up.is_null(), made_up.addr());
    println!("   Making it compiled with no unsafe and no warning. The compiler knows");
    println!("   the type, and nothing about whether 4 bytes of u32 sit at 0x1000.");

    println!();
    println!("5. What a reference adds to a pointer: promises the compiler holds you to");
    println!("   std: a reference is a pointer assumed to be aligned, not null, and");
    println!("   pointing at a valid value of its type. The borrow checker adds that");
    println!("   it outlives every use, and that a &mut to the same place is not live.");
    // SAFETY: p was coerced from `r`, which borrows `x`, and `x` is alive and
    // not mutably borrowed here, so every promise of &u32 holds.
    let back: &u32 = unsafe { &*p };
    println!("   unsafe {{ &*p }} points where r does: {}", ptr::eq(back, r));
    println!("   Turning a pointer back into a reference is the step that needs unsafe:");
    println!("   the compiler cannot check the promises, so you make them.");
}
