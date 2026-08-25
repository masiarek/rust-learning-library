//! Six string types is not six ideas. It is one pattern — owned vs borrowed —
//! across three promises about the bytes: trusted UTF-8, the OS's own text,
//! and text bound for C.
//!
//!   rustc --edition 2024 six_kinds_of_string.rs -o /tmp/six && /tmp/six

use std::ffi::{CString, OsStr, OsString};
use std::path::Path;

fn main() {
    println!("1. The pattern: every pair is String-and-&str again");
    println!("   owned      borrowed   what the bytes promise");
    println!("   String     &str       valid UTF-8, always");
    println!("   OsString   &OsStr     whatever the OS handed you");
    println!("   CString    &CStr      no NUL inside, one NUL at the end");
    println!("   PathBuf    &Path      an OsString that knows about '/'");
    println!("   Vec<u8>    &[u8]      nothing at all — just bytes");

    println!("\n2. OsStr: the honest type for filenames");
    let tidy = OsString::from("results.yaml");
    println!("   to_str() on a UTF-8 name     = {:?}", tidy.to_str());
    // Unix lets a filename be almost any bytes — forge one that is not UTF-8.
    use std::os::unix::ffi::OsStrExt; // unix-only, which is rather the point
    let wild = OsStr::from_bytes(&[b'b', b'v', 0xFF, b'.', b'y']);
    println!("   to_str() on a non-UTF-8 one  = {:?}", wild.to_str());
    println!("   to_string_lossy()            = {:?}   <- data lost, visibly", wild.to_string_lossy());

    println!("\n3. CString: the contract C needs");
    let fine = CString::new("STAR").unwrap();
    println!("   CString::new(\"STAR\")        = {fine:?}");
    let broken = CString::new("STAR\0vote");
    println!("   CString::new(\"STAR\\0vote\")  = Err: {}", broken.unwrap_err());

    println!("\n4. Path: an OsStr that knows the shape of a path");
    let p = Path::new("04_Approval/cases/results.yaml");
    println!("   file_stem() = {:?}", p.file_stem());
    println!("   extension() = {:?}", p.extension());
    println!("   parent()    = {:?}", p.parent());

    println!("\n5. Widening is free; narrowing returns an Option");
    let plain: &str = "turnout.csv";
    let os: OsString = OsString::from(plain); // any &str is a fine OsString
    println!("   &str -> OsString -> to_str() = {:?}", os.to_str());
    println!("   narrowing is where a promise gets CHECKED — which is why the");
    println!("   cheap direction never asks, and the checked one answers with");
    println!("   an Option (OsStr) or a Result naming the fault (CStr)");
}
