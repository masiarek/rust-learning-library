//! Calling C from Rust: the call is free, the data is not.
//!
//! Nothing here needs a build script, a bindings crate, or a C compiler.
//! `std` already links the platform C library, so a declaration is enough.

use std::ffi::{CStr, CString, c_char, c_int};

// Edition 2024: the block itself is `unsafe`, because writing out a foreign
// signature IS the promise — nothing checks it against the real function.
unsafe extern "C" {
    // `safe` marks the ones with no precondition to get wrong: abs takes an
    // integer, cannot be handed a bad pointer, and needs no unsafe block.
    safe fn abs(n: c_int) -> c_int;
    // strlen takes a raw pointer and trusts you that a 0 is coming. Never `safe`.
    fn strlen(s: *const c_char) -> usize;
}

// The other direction. In edition 2024 `no_mangle` is an unsafe attribute:
// you are claiming this exact symbol name is yours to take.
#[unsafe(no_mangle)]
pub extern "C" fn rust_double(x: c_int) -> c_int {
    x * 2
}

fn main() {
    println!("1. The call itself");
    // No marshalling layer, no JNI, no ctypes, no runtime to start first —
    // an ordinary call to a function that happens to live in libc.
    println!("   abs(-7) from C = {}   no unsafe block: declared `safe`", abs(-7));
    println!("   (-7i32).abs()  = {}   same answer, no boundary crossed", (-7i32).abs());

    println!();
    println!("2. The data is the part that costs");
    let name = "Ferris";
    let c_name = CString::new(name).expect("no interior NUL");
    println!(
        "   &str    {:?}  {} bytes, length carried beside the pointer",
        name,
        name.len()
    );
    println!(
        "   CString {:?}  {} bytes, length is wherever the 0 turns up",
        c_name,
        c_name.as_bytes_with_nul().len()
    );
    // CString::new allocated a fresh buffer, copied six bytes into it and
    // appended a seventh. That is the overhead the slogan leaves out.

    println!();
    println!("3. So C gets asked a question Rust already knew the answer to");
    let from_c = unsafe { strlen(c_name.as_ptr()) };
    println!("   strlen(..) = {from_c}   walks the bytes looking for the 0");
    println!("   name.len() = {}   reads a field", name.len());

    println!();
    println!("4. And the conversion can fail");
    match CString::new("has\0a nul") {
        Ok(_) => unreachable!("a string with an interior NUL cannot become a CString"),
        Err(e) => println!(
            "   CString::new refused: interior NUL at byte {}",
            e.nul_position()
        ),
    }

    println!();
    println!("5. Coming back the other way");
    // A *const c_char is a promise that a 0 exists somewhere ahead and that
    // the memory outlives the borrow. Rust can check neither, so: unsafe.
    let borrowed: &CStr = unsafe { CStr::from_ptr(c_name.as_ptr()) };
    println!("   CStr::from_ptr(..) = {:?}", borrowed.to_str().unwrap());
    println!("   ..borrowed from c_name, which must outlive it");

    println!();
    println!("6. And out again");
    println!("   rust_double(21) = {}   exported to C as `rust_double`", rust_double(21));
}
