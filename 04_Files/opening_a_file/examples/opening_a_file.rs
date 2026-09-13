//! The three doors into a file: `open` reads what is there, `create` empties it
//! on the way in, and `OpenOptions` spells out everything else. Every file here
//! lives in a scratch directory the program makes and removes, so the output is
//! the same on every machine.
//!
//!   rustc --edition 2024 opening_a_file.rs -o /tmp/oaf && /tmp/oaf

use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

/// Where everything below is written. The path is never printed: temp_dir()
/// differs per machine and the process id per run, and this output has to
/// match a recorded answer key on both.
fn scratch_dir() -> io::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("opening_a_file_{}", std::process::id()));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn size(path: &Path) -> io::Result<u64> {
    Ok(fs::metadata(path)?.len())
}

/// `Ok(File)`, or the error's kind -- the one part of an io::Error that reads
/// the same on every operating system.
fn door(r: io::Result<File>) -> String {
    match r {
        Ok(_) => "Ok(File)".to_string(),
        Err(e) => format!("Err({:?})", e.kind()),
    }
}

fn main() -> io::Result<()> {
    let dir = scratch_dir()?;
    let notes = dir.join("notes.txt");

    println!("1. open reads what is there, so on a file that is not there it fails");
    println!("   File::open(notes)            {}", door(File::open(&notes)));

    println!("\n2. create makes the file, and write_all puts bytes in it");
    let mut file = File::create(&notes)?;
    println!("   File::create(notes)          {} bytes on disk", size(&notes)?);
    file.write_all(b"first line\n")?;
    file.write_all(b"second line\n")?;
    drop(file); // the handle closes here -- there is no close() to call
    println!("   after two write_all calls    {} bytes", size(&notes)?);
    let mut text = String::new();
    File::open(&notes)?.read_to_string(&mut text)?;
    println!("   File::open + read_to_string  {text:?}");
    let again = fs::read_to_string(&notes)?;
    println!("   fs::read_to_string(notes)    the same two steps as one call: {}", again == text);

    println!("\n3. The trap: create empties the file at open, before anything is written");
    let before = size(&notes)?;
    let handle = File::create(&notes)?;
    println!("   File::create on the {before}-byte file, nothing written yet: {} bytes", size(&notes)?);
    drop(handle);
    println!("   The old contents are already gone. A crash between this open and the");
    println!("   last write leaves an empty file, and no copy of what was there.");

    println!("\n4. What the two doors are made of, and what the builder adds");
    println!("   File::open   = OpenOptions::new().read(true)");
    println!("   File::create = OpenOptions::new().write(true).create(true).truncate(true)");
    fs::write(&notes, "first line\n")?; // create + write_all as one call
    let mut log = File::options().append(true).open(&notes)?;
    writeln!(log, "second line")?;
    writeln!(log, "third line")?;
    drop(log);
    println!("   append(true), two writeln!   {:?}", fs::read_to_string(&notes)?);
    println!("   create_new(true), file exists   {}", door(File::options().write(true).create_new(true).open(&notes)));
    println!("   create_new(true), fresh name    {}", door(File::options().write(true).create_new(true).open(dir.join("lock"))));
    println!("   write(true), file missing       {}", door(File::options().write(true).open(dir.join("missing.txt"))));
    println!("   append(true), file missing      {}  <- append does not imply create", door(File::options().append(true).open(dir.join("missing.txt"))));
    println!("   no mode at all                  {}", door(File::options().open(&notes)));

    println!("\n5. The error does not carry the path");
    let settings = dir.join("settings.toml");
    if let Err(e) = File::open(&settings) {
        println!("   Display   {e}");
        println!("   kind()    {:?}", e.kind());
        let name = settings.file_name().unwrap().to_string_lossy();
        println!("   put back by hand: {name}: {e}");
    }

    println!("\n6. A BufWriter holds your bytes until it is flushed");
    let report = dir.join("report.txt");
    let mut out = BufWriter::new(File::create(&report)?);
    out.write_all(b"total: 42\n")?;
    println!("   after write_all, before flush  {} bytes on disk", size(&report)?);
    out.flush()?;
    println!("   after flush()                  {} bytes", size(&report)?);
    drop(out);

    fs::remove_dir_all(&dir)?;
    println!("\n7. Scratch directory removed: {}", !dir.exists());
    Ok(())
}
