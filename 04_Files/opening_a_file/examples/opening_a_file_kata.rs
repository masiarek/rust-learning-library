//! Kata solution: a log that is never truncated, and a lock that is claimed
//! exactly once -- `append` with `create`, and `create_new` matched on
//! `AlreadyExists` rather than on "an error happened".
//!
//!   rustc --edition 2024 opening_a_file_kata.rs -o /tmp/oafk && /tmp/oafk

use std::fs::{self, File};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};

fn scratch_dir() -> io::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("opening_a_file_kata_{}", std::process::id()));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Append one line, creating the file on the first call. Never truncates.
fn record(path: &Path, line: &str) -> io::Result<()> {
    let mut log = File::options().append(true).create(true).open(path)?;
    writeln!(log, "{line}")
}

/// Claim a lock file: Ok(true) if this call created it, Ok(false) if somebody
/// already holds it, Err for anything else -- a missing directory, no permission.
fn claim(path: &Path) -> io::Result<bool> {
    match File::options().write(true).create_new(true).open(path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(false),
        Err(e) => Err(e),
    }
}

fn show(r: io::Result<bool>) -> String {
    match r {
        Ok(b) => format!("Ok({b})"),
        Err(e) => format!("Err({:?})", e.kind()),
    }
}

fn main() -> io::Result<()> {
    let dir = scratch_dir()?;
    let log = dir.join("events.log");

    println!("1. The mistake worth making first: append without create");
    match File::options().append(true).open(&log) {
        Ok(_) => println!("   opened"),
        Err(e) => println!("   Err({:?}) -- append means \"at the end\", not \"make it\"", e.kind()),
    }

    println!("\n2. record: three calls, three lines, nothing lost");
    for line in ["started", "step 1 done", "finished"] {
        record(&log, line)?;
    }
    let text = fs::read_to_string(&log)?;
    for line in text.lines() {
        println!("   {line}");
    }
    println!("   {} lines", text.lines().count());

    println!("\n3. The same three calls through File::create instead");
    let wrong = dir.join("events_wrong.log");
    for line in ["started", "step 1 done", "finished"] {
        writeln!(File::create(&wrong)?, "{line}")?;
    }
    let text = fs::read_to_string(&wrong)?;
    println!("   {text:?} -- {} line: each open emptied the file first", text.lines().count());

    println!("\n4. claim: created once, refused once, created again after release");
    let lock = dir.join("build.lock");
    println!("   first claim    {}", show(claim(&lock)));
    println!("   second claim   {}", show(claim(&lock)));
    fs::remove_file(&lock)?;
    println!("   after remove   {}", show(claim(&lock)));
    println!("   A missing parent directory is a different answer, an Err rather than a false:");
    println!("   no such dir    {}", show(claim(&dir.join("no_such_dir").join("build.lock"))));
    println!("   Matching on the kind keeps \"somebody else has it\" apart from \"the lock");
    println!("   cannot be taken at all\". A bare .is_err() folds the two together.");

    fs::remove_dir_all(&dir)?;
    Ok(())
}
