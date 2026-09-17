//! Kata solution: the fixture helper that returned a path to nowhere.
//!
//!   rustc --edition 2024 temp_dirs_in_tests_kata.rs -o /tmp/tditk && /tmp/tditk
//!   rustc --edition 2024 --test temp_dirs_in_tests_kata.rs -o /tmp/tditkt && /tmp/tditkt

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);

/// The same stand-in for `tempfile::TempDir` as the lesson's example.
struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new() -> io::Result<TempDir> {
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        let path = env::temp_dir().join(format!("temp_dirs_in_tests_kata-{}-{n}", std::process::id()));
        fs::create_dir(&path)?;
        Ok(TempDir { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// The code under test: how many lines of a log file mention an error.
fn count_errors(log: &Path) -> io::Result<usize> {
    Ok(fs::read_to_string(log)?.lines().filter(|l| l.contains("ERROR")).count())
}

/// Broken: `dir` is dropped at the closing brace, and takes the file with it.
/// It compiles, because a `PathBuf` borrows nothing from the directory it names.
fn log_fixture_broken(contents: &str) -> PathBuf {
    let dir = TempDir::new().expect("dir");
    let log = dir.path().join("app.log");
    fs::write(&log, contents).expect("write");
    log
}

/// Fixed: the caller receives the owner, so the directory lives as long as they keep it.
struct LogFixture {
    _dir: TempDir, // never read; held only so that dropping the fixture removes it
    log: PathBuf,
}

fn log_fixture(contents: &str) -> LogFixture {
    let dir = TempDir::new().expect("dir");
    let log = dir.path().join("app.log");
    fs::write(&log, contents).expect("write");
    LogFixture { _dir: dir, log }
}

const LOG: &str = "INFO start\nERROR disk full\nINFO retry\nERROR disk full\n";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_two_errors() {
        let fixture = log_fixture(LOG);
        assert_eq!(count_errors(&fixture.log).unwrap(), 2);
    }

    #[test]
    fn an_empty_log_has_none() {
        let fixture = log_fixture("");
        assert_eq!(count_errors(&fixture.log).unwrap(), 0);
    }
}

fn main() {
    println!("1. The broken helper");
    let log = log_fixture_broken(LOG);
    println!("   log_fixture_broken(..) returned a path; the file exists: {}", log.exists());
    match count_errors(&log) {
        Ok(n) => println!("   count_errors = {n}"),
        Err(e) => println!("   count_errors = Err({:?})", e.kind()),
    }
    println!("   NotFound, from code that wrote the file one line earlier. It is not");
    println!("   a permissions problem: the directory was dropped with the helper's");
    println!("   stack frame, before the test got its path.");

    println!();
    println!("2. The fixed helper returns the owner along with the path");
    let fixture = log_fixture(LOG);
    println!("   the file exists: {}", fixture.log.exists());
    println!("   count_errors = {:?}", count_errors(&fixture.log).map_err(|e| e.kind()));
    let log = fixture.log.clone();
    drop(fixture);
    println!("   after the fixture is dropped, the file exists: {}", log.exists());

    println!();
    println!("3. Why the compiler did not stop the broken one");
    println!("   A borrow would have been caught: returning dir.path() is a &Path");
    println!("   into a local, and that is error E0515. to_owned() or join() makes");
    println!("   an independent PathBuf, which is only a name. Nothing ties a name");
    println!("   to the value whose Drop deletes the thing it names.");
}
