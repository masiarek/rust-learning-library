//! Claims from Rust in Action, chapter 6 ("Memory"), each run on this toolchain.
//!
//! The book's listings are not reproduced here. Each section tests one claim
//! with a program of its own; the page says which listing or sentence it checks.
//!
//!   rustc --edition 2024 rust_in_action_chapter_6.rs -o /tmp/ria6 && /tmp/ria6

use std::ffi::{CStr, c_char};
use std::fmt::Debug;
use std::mem::{align_of, size_of, size_of_val};
use std::ptr;

static NAME: [u8; 10] = *b"carrytowel"; //     bytes with no terminator
static C_NAME: [u8; 11] = *b"thanksfish\0"; // the same kind of bytes, NUL-terminated
static HIGH: [u8; 4] = [104, 200, 105, 0]; //  one byte above 127

fn main() {
    println!("1. \"Option<T> occupies 0 bytes\" (null pointer optimization)");
    for (ty, bytes) in [
        ("&u8", size_of::<&u8>()),
        ("Option<&u8>", size_of::<Option<&u8>>()),
        ("Box<u8>", size_of::<Box<u8>>()),
        ("Option<Box<u8>>", size_of::<Option<Box<u8>>>()),
        ("*const u8", size_of::<*const u8>()),
        ("Option<*const u8>", size_of::<Option<*const u8>>()),
        ("u64", size_of::<u64>()),
        ("Option<u64>", size_of::<Option<u64>>()),
    ] {
        println!("   {ty:<18} {bytes:>2} bytes");
    }
    println!("   The Option costs 0 EXTRA bytes where the inner type has a niche;");
    println!("   it never occupies 0 bytes, and without a niche it costs a word.");

    println!();
    println!("2. \"References are aligned to multiples of usize\"");
    let pair = [0u8; 2];
    let first = &pair[0] as *const u8 as usize;
    let second = &pair[1] as *const u8 as usize;
    println!("   align_of::<u8>() = {}, align_of::<usize>() = {}", align_of::<u8>(), align_of::<usize>());
    println!("   two neighbouring &u8, one byte apart; one is not a multiple of 8: {}",
        first % 8 != 0 || second % 8 != 0);
    println!("   A reference is aligned to align_of::<T>(), which for u8 is 1.");

    println!();
    println!("3. \"A pointer and an integer\" for dynamically sized types");
    let small = 7u8;
    let text = String::from("seven");
    let a: &dyn Debug = &small;
    let b: &dyn Debug = &text;
    let slice: &[u8] = &NAME[..4];
    println!("   &[u8] second word is a length:     slice.len() = {}", slice.len());
    println!("   &dyn Debug second word is a vtable: size_of_val = {} and {}", size_of_val(a), size_of_val(b));
    println!("   The integer is right for slices and str; a trait object carries a");
    println!("   pointer to a table of size, alignment, drop and methods instead.");

    println!();
    println!("4. \"When a compiler creates a pointer to an i32, it can verify 4 bytes\"");
    let invented = 0x1000 as *const i32; // safe code, no warning
    println!("   0x1000 as *const i32 compiled; size_of::<i32>() = {}", size_of::<i32>());
    println!("   is_null() = {} — and nothing checked that an i32 lives there.", invented.is_null());
    println!("   The size is known from the type. Whether the bytes exist is only");
    println!("   guaranteed for a reference, never for a raw pointer.");

    println!();
    println!("5. Listing 6.2's sizes: a reference to an array, and a box of a slice");
    let by_ref: &[u8; 10] = &NAME;
    let boxed: Box<[u8]> = Box::new(C_NAME);
    println!("   usize {}   &[u8; 10] {}   Box<[u8]> {}   [u8; 10] {}   [u8; 11] {}",
        size_of::<usize>(), size_of_val(&by_ref), size_of_val(&boxed), size_of_val(&NAME), size_of_val(&C_NAME));
    println!("   boxed holds a copy, not the static: same bytes {}, same address {}",
        *boxed == C_NAME, ptr::eq(boxed.as_ptr(), C_NAME.as_ptr()));
    println!("   &[u8; 10] is thin (the 10 is in the type); Box<[u8]> is wide (the 11");
    println!("   moved into the pointer). Neither is a String's three words.");

    println!();
    println!("6. Listing 6.3's two strings, without from_raw_parts");
    let as_str: &str = std::str::from_utf8(&NAME).unwrap();
    let as_cstr: &CStr = CStr::from_bytes_with_nul(&C_NAME).unwrap();
    println!("   str::from_utf8(&NAME)               {as_str:?}   borrowed, nothing to free");
    println!("   CStr::from_bytes_with_nul(&C_NAME)  {as_cstr:?}   checks the NUL is last");
    println!("   equal to the literal c\"thanksfish\": {}", as_cstr == c"thanksfish");
    println!("   The book builds a String over the static with from_raw_parts, and");
    println!("   that String then frees memory no allocator gave it. See the page.");

    println!();
    println!("7. \"The conversion to c_char works because we stay under 128\"");
    // SAFETY: HIGH is a static, NUL-terminated at index 3, alive for the program.
    let read = unsafe { CStr::from_ptr(HIGH.as_ptr().cast::<c_char>()) };
    println!("   bytes read through a *const c_char: {:?}", read.to_bytes());
    println!("   to_string_lossy():                  {:?}", read.to_string_lossy());
    println!("   A pointer cast converts no byte. 200 came through as 200; only the");
    println!("   UTF-8 decoding in to_string_lossy cared, and replaced it with U+FFFD.");

    println!();
    println!("8. Printing a pointer to a str with {{:p}}, as memscan-3 does");
    let local_str = "a";
    let shown = format!("{:p}", local_str as *const str);
    println!("   the text starts with \"Pointer {{ addr: \" {}", shown.starts_with("Pointer { addr: "));
    println!("   and ends with \"metadata: 1 }}\"        {}", shown.ends_with("metadata: 1 }"));
    println!("   A *const str is wide, and {{:p}} on rustc 1.98.0 prints both words,");
    println!("   where the book's run printed a bare address.");
}
