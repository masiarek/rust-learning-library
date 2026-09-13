//! A file is bytes; a String is a promise about bytes. On the way out the
//! promise is dropped for free, on the way in it is checked at the door, and
//! with include_str! the check happens before the program exists.
//!
//!   rustc --edition 2024 a_file_is_bytes.rs -o /tmp/afib && /tmp/afib

use std::fmt::Write as _; // write! into a String
use std::fs::{self, File};
use std::io::{self, Read, Write}; // write! into a File
use std::path::PathBuf;

/// Every file below lives here; the path is never printed, because temp_dir()
/// and the process id differ per machine and per run.
fn scratch_dir() -> io::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("a_file_is_bytes_{}", std::process::id()));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn main() -> io::Result<()> {
    let dir = scratch_dir()?;
    let word = "Łódź";

    println!("1. On the way out, three spellings of the same seven bytes");
    println!("   {word:?} is {} chars and {} bytes: {:02X?}", word.chars().count(), word.len(), word.as_bytes());
    let (a, b, c) = (dir.join("a.txt"), dir.join("b.txt"), dir.join("c.txt"));
    File::create(&a)?.write_all(b"\xC5\x81\xC3\xB3d\xC5\xBA")?; // a byte string: the bytes, spelled out
    File::create(&b)?.write_all(word.as_bytes())?; // a &str, widened to &[u8]
    fs::write(&c, word)?; // fs::write takes AsRef<[u8]>, and str is one
    println!("   write_all(b\"..\")       {:02X?}", fs::read(&a)?);
    println!("   write_all(as_bytes())  {:02X?}", fs::read(&b)?);
    println!("   fs::write(path, word)  {:02X?}", fs::read(&c)?);
    println!("   write_all(word) with no as_bytes() is E0308: the file wants &[u8], and");
    println!("   a &str is not one until you say so. The transcript is on the page.");

    println!("\n2. write! into a String and write! into a File are two traits");
    let mut s = String::new();
    write!(s, "{word} = {} bytes", word.len()).unwrap(); // fmt::Write: cannot fail
    let d = dir.join("d.txt");
    write!(File::create(&d)?, "{word} = {} bytes", word.len())?; // io::Write: can
    println!("   String  {s:?}");
    println!("   File    {:?}", fs::read_to_string(&d)?);
    println!("   Same macro, same text. fmt::Write returns a fmt::Error a String never");
    println!("   produces, so it is .unwrap(); io::Write returns an io::Error, so it is ?.");

    println!("\n3. On the way in, the promise is checked at the door");
    let latin2 = dir.join("lodz_latin2.txt");
    fs::write(&latin2, [0xA3u8, 0xF3, 0x64, 0xBC])?; // the same word, as ISO-8859-2 wrote it
    match fs::read_to_string(&latin2) {
        Ok(text) => println!("   read_to_string   Ok({text:?})"),
        Err(e) => println!("   read_to_string   Err({:?}): {e}", e.kind()),
    }
    let bytes = fs::read(&latin2)?;
    println!("   fs::read         Ok({bytes:02X?})  -- no promise, so nothing to check");
    match String::from_utf8(bytes.clone()) {
        Ok(text) => println!("   from_utf8        Ok({text:?})"),
        Err(e) => println!("   from_utf8        Err: {e}"),
    }
    println!("   from_utf8_lossy  {:?}", String::from_utf8_lossy(&bytes));
    println!("   Three letters replaced and one kept: d is 0x64 in every code page there is.");
    let utf8 = dir.join("lodz_utf8.txt");
    fs::write(&utf8, word)?;
    println!("   the same word saved as UTF-8 reads back: {:?}", fs::read_to_string(&utf8)?);

    println!("\n4. A failed check leaves the String you passed in untouched");
    let mut kept = String::from("kept:");
    let outcome = File::open(&latin2)?.read_to_string(&mut kept);
    println!("   read_to_string into \"kept:\" from the Latin-2 file  {:?}, buffer {kept:?}", outcome.map_err(|e| e.kind()));
    let mut grown = String::from("kept:");
    File::open(&utf8)?.read_to_string(&mut grown)?;
    println!("   and from the UTF-8 file it appends:                {grown:?}");

    println!("\n5. include_str! moves the check to compile time");
    let me: &'static str = include_str!("a_file_is_bytes.rs");
    let raw: &'static [u8] = include_bytes!("a_file_is_bytes.rs");
    println!("   this program's own source, embedded; its first line:");
    println!("   {:?}", me.lines().next().unwrap());
    println!("   include_bytes! of the same file has the same length: {}", raw.len() == me.len());
    println!("   No file is opened at run time. Had the file been Latin-2, this program");
    println!("   would not have compiled -- that transcript is on the page too.");

    fs::remove_dir_all(&dir)?;
    println!("\n6. Scratch directory removed: {}", !dir.exists());
    Ok(())
}
