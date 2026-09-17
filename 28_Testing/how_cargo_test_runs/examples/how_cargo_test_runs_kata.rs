//! Kata solution: the test that passes alone.
//!
//!   rustc --edition 2024 how_cargo_test_runs_kata.rs -o /tmp/hctrk && /tmp/hctrk
//!   rustc --edition 2024 --test how_cargo_test_runs_kata.rs -o /tmp/hctrkt && /tmp/hctrkt
//!
//! `main` replays the two tests on two threads with the interleaving forced by
//! barriers, so the race that only sometimes happens under `cargo test` happens
//! every time here.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Barrier};
use std::thread;

/// Before: the price list is wherever the process happens to be standing.
fn price_before(item: &str) -> Option<u32> {
    lookup(&fs::read_to_string("prices.txt").ok()?, item)
}

/// After: the caller says where the price list is.
fn price_after(root: &Path, item: &str) -> Option<u32> {
    lookup(&fs::read_to_string(root.join("prices.txt")).ok()?, item)
}

fn lookup(text: &str, item: &str) -> Option<u32> {
    text.lines()
        .filter_map(|line| line.split_once('='))
        .find(|(name, _)| *name == item)
        .and_then(|(_, cents)| cents.parse().ok())
}

/// A fixture directory holding one `prices.txt`, removed when dropped.
struct Fixture(PathBuf);

impl Fixture {
    fn new(name: &str, prices: &str) -> Fixture {
        let dir = env::temp_dir().join(format!("hctr-kata-{}-{name}", std::process::id()));
        fs::create_dir_all(&dir).expect("fixture dir");
        fs::write(dir.join("prices.txt"), prices).expect("fixture file");
        Fixture(dir)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The fixed pair: nothing process-wide is touched, so they can run in any
    // order, on any thread, alongside anything.
    #[test]
    fn apples_cost_three_in_the_spring_list() {
        let spring = Fixture::new("spring", "apple=3\npear=4\n");
        assert_eq!(price_after(&spring.0, "apple"), Some(3));
    }

    #[test]
    fn apples_cost_five_in_the_winter_list() {
        let winter = Fixture::new("winter", "apple=5\npear=6\n");
        assert_eq!(price_after(&winter.0, "apple"), Some(5));
    }
}

/// Runs "test A" and "test B" on two threads: A moves into its directory, then
/// B moves into its own, then A reads. `read` is the only thing that varies.
fn forced_race(read: fn(&Path) -> Option<u32>) -> (Option<u32>, Option<u32>) {
    let spring = Arc::new(Fixture::new("race-spring", "apple=3\n"));
    let winter = Arc::new(Fixture::new("race-winter", "apple=5\n"));
    let a_moved = Arc::new(Barrier::new(2));
    let b_moved = Arc::new(Barrier::new(2));
    let original = env::current_dir().expect("cwd");

    let a = {
        let (dir, a_moved, b_moved) = (Arc::clone(&spring), Arc::clone(&a_moved), Arc::clone(&b_moved));
        thread::spawn(move || {
            env::set_current_dir(&dir.0).expect("chdir");
            a_moved.wait();
            b_moved.wait();
            read(&dir.0)
        })
    };
    let b = {
        let (dir, a_moved, b_moved) = (Arc::clone(&winter), Arc::clone(&a_moved), Arc::clone(&b_moved));
        thread::spawn(move || {
            a_moved.wait();
            env::set_current_dir(&dir.0).expect("chdir");
            b_moved.wait();
            read(&dir.0)
        })
    };
    let result = (a.join().expect("a"), b.join().expect("b"));
    env::set_current_dir(original).expect("restore");
    result
}

fn main() {
    println!("1. Each test passes on its own");
    let spring = Fixture::new("alone", "apple=3\n");
    let original = env::current_dir().expect("cwd");
    env::set_current_dir(&spring.0).expect("chdir");
    println!("   test A alone, before the fix: apple = {:?}", price_before("apple"));
    env::set_current_dir(original).expect("restore");
    drop(spring);

    println!();
    println!("2. Together, the second set_current_dir wins for both threads");
    let (a, b) = forced_race(|_| price_before("apple"));
    println!("   test A expected Some(3) and read {a:?}");
    println!("   test B expected Some(5) and read {b:?}");
    println!("   Under cargo test this interleaving happens only sometimes, which is");
    println!("   why the failure looks like a flake rather than a bug.");

    println!();
    println!("3. With the directory passed in, the same interleaving is harmless");
    let (a, b) = forced_race(|root| price_after(root, "apple"));
    println!("   test A expected Some(3) and read {a:?}");
    println!("   test B expected Some(5) and read {b:?}");
    println!("   The threads still call set_current_dir. Nothing reads it any more.");

    println!();
    println!("4. The rule the fix follows");
    println!("   Only main() decides where the files are. It resolves the path");
    println!("   once and hands it down, so a test can hand down a different one.");
}
