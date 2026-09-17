//! A test that writes to a fixed path cannot run beside another one that does,
//! and the fix is a directory per test that is removed when its owner is dropped.
//!
//!   rustc --edition 2024 temp_dirs_in_tests.rs -o /tmp/tdit && /tmp/tdit
//!
//! `TempDir` below is a stand-in with the shape of `tempfile::TempDir`, so this
//! program needs no crate. No path is ever printed: they differ per machine.

use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::panic;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

static NEXT: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);

/// A fresh directory, removed with everything in it when dropped.
///
/// The name is predictable (process id and a counter). That is fine here and
/// is what `std::env::temp_dir`'s docs warn against in real code, because the
/// temporary directory may be shared with other users. Use the `tempfile` crate.
struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new() -> io::Result<TempDir> {
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        let path = env::temp_dir().join(format!("temp_dirs_in_tests-{}-{n}", std::process::id()));
        fs::create_dir(&path)?; // refuses a name that already exists
        LIVE.fetch_add(1, Ordering::SeqCst);
        Ok(TempDir { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
        LIVE.fetch_sub(1, Ordering::SeqCst);
    }
}

/// The code under test takes the path. Only `main` would decide the real one.
fn default_editor(config: &Path) -> io::Result<Option<String>> {
    default_editor_from(BufReader::new(fs::File::open(config)?))
}

/// The same logic with the file taken out: anything that is `BufRead` will do.
fn default_editor_from(reader: impl BufRead) -> io::Result<Option<String>> {
    for line in reader.lines() {
        if let Some(value) = line?.strip_prefix("editor = ") {
            return Ok(Some(value.to_string()));
        }
    }
    Ok(None)
}

/// Several files that find each other by relative path, all under one root.
fn workspace_members(root: &Path) -> io::Result<Vec<String>> {
    let list = fs::read_to_string(root.join("workspace.txt"))?;
    list.lines()
        .map(|member| fs::read_to_string(root.join(member).join("name.txt")))
        .map(|name| name.map(|n| n.trim().to_string()))
        .collect()
}

/// Two "tests" on two threads. Each writes its config to the directory it is
/// given, then reads it back. Two barriers force one order every run: A writes,
/// then B writes, then both read.
fn two_tests(dir_a: PathBuf, dir_b: PathBuf) -> (Option<String>, Option<String>) {
    let a_wrote = Arc::new(Barrier::new(2));
    let b_wrote = Arc::new(Barrier::new(2));
    let a = {
        let (a_wrote, b_wrote) = (Arc::clone(&a_wrote), Arc::clone(&b_wrote));
        thread::spawn(move || {
            let config = dir_a.join("config.txt");
            fs::write(&config, "editor = vim\n").expect("write");
            a_wrote.wait();
            b_wrote.wait();
            default_editor(&config).expect("read")
        })
    };
    let b = thread::spawn(move || {
        a_wrote.wait();
        let config = dir_b.join("config.txt");
        fs::write(&config, "editor = hx\n").expect("write");
        b_wrote.wait();
        default_editor(&config).expect("read")
    });
    (a.join().expect("a"), b.join().expect("b"))
}

fn exists_word(p: &Path) -> &'static str {
    if p.exists() { "exists" } else { "is gone" }
}

fn main() -> io::Result<()> {
    println!("1. Two tests, one fixed path");
    let shared = TempDir::new()?; // stands in for a hard-coded "/tmp"
    let (a, b) = two_tests(shared.path().to_owned(), shared.path().to_owned());
    println!("   test A wrote vim and read {a:?}");
    println!("   test B wrote hx  and read {b:?}");
    println!("   The second write replaced the first before A read it back.");
    drop(shared);

    println!();
    println!("2. A directory per test");
    let (dir_a, dir_b) = (TempDir::new()?, TempDir::new()?);
    let (a, b) = two_tests(dir_a.path().to_owned(), dir_b.path().to_owned());
    println!("   test A wrote vim and read {a:?}");
    println!("   test B wrote hx  and read {b:?}");
    println!("   Same threads, same interleaving. Nothing is shared, so nothing races.");
    drop((dir_a, dir_b));

    println!();
    println!("3. The directory lives exactly as long as its owner");
    let dir = TempDir::new()?;
    let path = dir.path().to_owned();
    fs::write(path.join("config.txt"), "editor = vim\n")?;
    println!("   while `dir` is alive, the directory {}", exists_word(&path));
    drop(dir);
    println!("   after drop(dir), it {} (and the file in it with it)", exists_word(&path));

    println!();
    println!("4. The trap: the handle was a temporary");
    let path = TempDir::new()?.path().to_owned();
    println!("   let path = TempDir::new()?.path().to_owned();");
    println!("   the directory {} before the next line runs", exists_word(&path));
    let err = fs::write(path.join("config.txt"), "editor = vim\n").unwrap_err();
    println!("   writing a file into it: Err({:?})", err.kind());
    let _ = TempDir::new()?;
    println!("   let _ = TempDir::new()?;     directories alive now: {}", LIVE.load(Ordering::SeqCst));
    let _dir = TempDir::new()?;
    println!("   let _dir = TempDir::new()?;  directories alive now: {}", LIVE.load(Ordering::SeqCst));
    drop(_dir);

    println!();
    println!("5. A failing test still removes its directory");
    let mut seen = None;
    let quiet = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let dir = TempDir::new().expect("dir");
        seen = Some(dir.path().to_owned());
        assert_eq!(default_editor(&dir.path().join("config.txt")).ok(), Some(None));
    }));
    panic::set_hook(quiet);
    let seen = seen.expect("the test got as far as creating it");
    println!("   the test panicked: {}; its directory {}", result.is_err(), exists_word(&seen));
    println!("   Unwinding runs Drop. panic = \"abort\" and process::exit do not.");

    println!();
    println!("6. Several files: paths relative to a root, never to the working directory");
    let root = TempDir::new()?;
    fs::write(root.path().join("workspace.txt"), "api\ncli\n")?;
    for (member, name) in [("api", "shop-api"), ("cli", "shop-cli")] {
        fs::create_dir(root.path().join(member))?;
        fs::write(root.path().join(member).join("name.txt"), format!("{name}\n"))?;
    }
    println!("   workspace_members(root) = {:?}", workspace_members(root.path())?);
    println!("   The layout is built inside the root, so relative paths between the");
    println!("   files still hold, and set_current_dir is never called.");
    drop(root);

    println!();
    println!("7. Or no file at all");
    let text: &[u8] = b"theme = dark\neditor = hx\n";
    println!("   default_editor_from(bytes)       = {:?}", default_editor_from(text)?);
    println!("   default_editor_from(empty bytes) = {:?}", default_editor_from(&b""[..])?);
    println!("   Opening the file, and the error when it is missing, still need one");
    println!("   test that uses a real path.");

    println!();
    println!("   directories still alive at the end: {}", LIVE.load(Ordering::SeqCst));
    Ok(())
}
