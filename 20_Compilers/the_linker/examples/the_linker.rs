//! Two names this crate never defines, and the program still runs.
//!
//! `abs` and `strlen` live in the C library on this machine. `rustc` compiles
//! the calls with the names left blank and records that it needs them; the
//! linker matches each one to a definition in libc and fills the blanks in.

unsafe extern "C" {
    fn abs(input: i32) -> i32;
    fn strlen(s: *const core::ffi::c_char) -> usize;
}

fn main() {
    // `unsafe` because nothing in Rust checked these signatures against the
    // C library's — a wrong one compiles, links, and misbehaves at run time.
    let a = unsafe { abs(-42) };
    let n = unsafe { strlen(c"linker".as_ptr()) };

    println!("abs(-42)         = {a}");
    println!("strlen(\"linker\") = {n}");

    // Declaring a name that no object file defines is not a compile error —
    // rustc has nothing to check it against. It is a *link* error, and it
    // arrives after every other stage has already succeeded:
    //
    //     unsafe extern "C" { fn definitely_not_a_symbol(x: i32) -> i32; }
    //
    //     = note: Undefined symbols for architecture x86_64:
    //               "_definitely_not_a_symbol", referenced from: ...
    //             ld: symbol(s) not found
    //
    // Wording is the linker's, not Rust's, so it differs by platform: GNU ld
    // on Linux says `undefined reference to `definitely_not_a_symbol'`.
}
