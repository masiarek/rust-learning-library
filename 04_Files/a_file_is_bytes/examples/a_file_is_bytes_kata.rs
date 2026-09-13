//! Kata solution: read a file that may be ISO-8859-2 -- InvalidData as the
//! signal, an eighteen-letter table as the decoder, and the two bytes that
//! pass as UTF-8 by accident.
//!
//!   rustc --edition 2024 a_file_is_bytes_kata.rs -o /tmp/afibk && /tmp/afibk

use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

fn scratch_dir() -> io::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("a_file_is_bytes_kata_{}", std::process::id()));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// The nine Polish letters ISO-8859-2 places above 0x7F, in both cases.
const LATIN2_POLISH: [(u8, char); 18] = [
    (0xA1, 'Ą'), (0xB1, 'ą'), (0xC6, 'Ć'), (0xE6, 'ć'), (0xCA, 'Ę'), (0xEA, 'ę'),
    (0xA3, 'Ł'), (0xB3, 'ł'), (0xD1, 'Ń'), (0xF1, 'ń'), (0xD3, 'Ó'), (0xF3, 'ó'),
    (0xA6, 'Ś'), (0xB6, 'ś'), (0xAC, 'Ź'), (0xBC, 'ź'), (0xAF, 'Ż'), (0xBF, 'ż'),
];

/// ASCII maps to itself, the eighteen letters go through the table, and any
/// other high byte becomes U+FFFD -- the same honesty as from_utf8_lossy, one
/// byte at a time, because a single-byte code page has no sequences to lose.
fn latin2_to_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| match LATIN2_POLISH.iter().find(|(code, _)| *code == b) {
            _ if b < 0x80 => b as char,
            Some(&(_, ch)) => ch,
            None => '\u{FFFD}',
        })
        .collect()
}

/// UTF-8 first; on InvalidData, and only then, decode as Latin-2. Every other
/// error -- the file is missing, the disk failed -- goes back to the caller.
fn read_text(path: &Path) -> io::Result<(String, &'static str)> {
    match fs::read_to_string(path) {
        Ok(text) => Ok((text, "utf-8")),
        Err(e) if e.kind() == ErrorKind::InvalidData => Ok((latin2_to_string(&fs::read(path)?), "latin-2")),
        Err(e) => Err(e),
    }
}

fn replaced(s: &str) -> usize {
    s.chars().filter(|&c| c == '\u{FFFD}').count()
}

fn main() -> io::Result<()> {
    let dir = scratch_dir()?;
    let word = "Łódź";

    println!("1. Two files, one word, two encodings");
    let utf8 = dir.join("utf8.txt");
    let latin2 = dir.join("latin2.txt");
    fs::write(&utf8, word)?;
    fs::write(&latin2, [0xA3u8, 0xF3, 0x64, 0xBC])?;
    for (name, path) in [("utf8.txt", &utf8), ("latin2.txt", &latin2)] {
        let (text, how) = read_text(path)?;
        println!("   {name:<11} {} bytes -> {text:?} via {how}", fs::metadata(path)?.len());
    }
    println!("   from_utf8_lossy on the Latin-2 bytes would have replaced {} of 4;",
        replaced(&String::from_utf8_lossy(&fs::read(&latin2)?)));
    println!("   the table replaced {}, because it knows what the bytes meant.", replaced(&read_text(&latin2)?.0));

    println!("\n2. A high byte the table does not know is still reported, not invented");
    let odd = dir.join("odd.txt");
    fs::write(&odd, [0x4C, 0xF3, 0x64, 0xFF])?; // 0xFF is a dot-above in Latin-2; not in our table
    let (text, how) = read_text(&odd)?;
    println!("   {text:?} via {how}, {} byte replaced", replaced(&text));

    println!("\n3. The mistake worth making first: matching on is_err() instead of the kind");
    let missing = dir.join("missing.txt");
    println!("   read_text(missing)  {}", match read_text(&missing) {
        Ok((text, how)) => format!("Ok({text:?} via {how})"),
        Err(e) => format!("Err({:?})", e.kind()),
    });
    println!("   With is_err() the missing file would have gone down the Latin-2 branch,");
    println!("   fs::read would have failed a second time, and the caller would see an");
    println!("   error about decoding a file that was never there.");

    println!("\n4. The signal is a heuristic, and here is the file that fools it");
    let accident = dir.join("accident.txt");
    fs::write(&accident, [0xC3u8, 0xB3])?; // Latin-2 for the two letters below
    let (text, how) = read_text(&accident)?;
    println!("   C3 B3 read as {text:?} via {how}");
    println!("   In Latin-2 that is Ă then ł -- the table, which knows only Polish letters,");
    println!("   gives {:?} -- and in UTF-8 it is one letter. The UTF-8 reading wins", latin2_to_string(&[0xC3, 0xB3]));
    println!("   because it is tried first and it succeeds.");
    println!("   Nothing in the bytes says which was meant. A file format that names its");
    println!("   encoding -- or a caller that does -- is the fix; guessing is not.");

    fs::remove_dir_all(&dir)?;
    Ok(())
}
