//! How `cargo test` runs your tests: a binary per target, a process per binary,
//! and everything a process owns shared by the tests inside it.
//!
//!   rustc --edition 2024 how_cargo_test_runs.rs -o /tmp/hctr && /tmp/hctr
//!
//! The program starts copies of itself to stand in for a second test binary,
//! so every comparison across the process boundary is a real one.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// A per-process counter, like a `static` connection pool or a `OnceLock` cache.
static CONNECTIONS_OPENED: AtomicUsize = AtomicUsize::new(0);

fn open_connection() {
    CONNECTIONS_OPENED.fetch_add(1, Ordering::SeqCst);
}

const MODE_VAR: &str = "HOW_CARGO_TEST_RUNS_MODE";

/// The second "test binary": the same executable, started as a new process.
fn child(role: &str, arg: Option<String>) {
    match role {
        "static" => {
            open_connection();
            println!("{}", CONNECTIONS_OPENED.load(Ordering::SeqCst));
        }
        "chdir" => {
            env::set_current_dir(arg.expect("a directory")).expect("chdir");
            println!("moved");
        }
        "env" => println!("{}", env::var(MODE_VAR).unwrap_or_else(|_| "unset".into())),
        "panic" => panic!("a failing test"),
        _ => unreachable!(),
    }
}

fn run_child(role: &str, arg: Option<&str>, envs: &[(&str, &str)]) -> (String, Option<i32>) {
    let mut cmd = Command::new(env::current_exe().expect("own path"));
    cmd.arg(role).stderr(Stdio::null());
    if let Some(a) = arg {
        cmd.arg(a);
    }
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("child runs");
    (String::from_utf8_lossy(&out.stdout).trim().to_string(), out.status.code())
}

fn main() {
    let mut args = env::args().skip(1);
    if let Some(role) = args.next() {
        return child(&role, args.next());
    }

    println!("1. One test binary per target");
    println!("   src/lib.rs and its modules      1 binary");
    println!("   src/main.rs, each src/bin/*.rs  1 binary each");
    println!("   each file directly in tests/    1 binary each");
    println!("   tests/<dir>/main.rs             1 binary");
    println!("   tests/<dir>/mod.rs, no main.rs  none: a helper module");
    println!("   doc tests, edition 2024         1 merged binary, a process per test");
    println!("   cargo runs these binaries one after another. Inside each one,");
    println!("   the harness runs the tests as threads of a single process.");

    println!();
    println!("2. Threads of one process share its working directory");
    let original = env::current_dir().expect("cwd");
    let fixture: PathBuf = env::temp_dir().join(format!("how_cargo_test_runs-{}", std::process::id()));
    fs::create_dir_all(&fixture).expect("fixture dir");
    let fixture = fixture.canonicalize().expect("canonical");
    {
        let target = fixture.clone();
        // "Test A" moves into its fixture directory...
        thread::spawn(move || env::set_current_dir(target).expect("chdir"))
            .join()
            .expect("thread");
    }
    // ...and "test B", on another thread, is now somewhere it never asked to be.
    let moved = env::current_dir().expect("cwd").canonicalize().expect("canonical") == fixture;
    println!("   one thread called set_current_dir; the main thread moved too: {moved}");
    env::set_current_dir(&original).expect("restore");
    println!("   A relative path in any other test now resolves against that");
    println!("   directory, and whether it does depends on which thread got there first.");

    println!();
    println!("3. A second process has its own");
    let (said, _) = run_child("chdir", Some(fixture.to_str().expect("utf-8 path")), &[]);
    let unchanged = env::current_dir().expect("cwd") == original;
    println!("   the child said {said:?}; this process's directory is unchanged: {unchanged}");
    fs::remove_dir_all(&fixture).expect("cleanup");

    println!();
    println!("4. A static is shared by one binary's tests, and by nobody else");
    let handles: Vec<_> = (0..2).map(|_| thread::spawn(open_connection)).collect();
    for h in handles {
        h.join().expect("thread");
    }
    println!("   two threads each opened one: this process counts {}", CONNECTIONS_OPENED.load(Ordering::SeqCst));
    let (count, _) = run_child("static", None, &[]);
    println!("   a new process opened one:    it counts {count}");
    println!("   A static Mutex serialises the tests in tests/api.rs against each");
    println!("   other and does nothing at all about tests/cli.rs.");

    println!();
    println!("5. The environment is copied into a child, never shared back");
    let (seen, _) = run_child("env", None, &[(MODE_VAR, "test")]);
    println!("   Command::env gave the child {MODE_VAR}={seen}");
    println!("   this process still has it: {}", env::var(MODE_VAR).is_ok());
    println!("   Inside one binary there is no safe way to vary it per test:");
    println!("   std::env::set_var is `unsafe` in edition 2024, and its docs say the");
    println!("   only sound option in a multi-threaded program (outside Windows) is");
    println!("   not to call it. A test binary is always multi-threaded.");

    println!();
    println!("6. A failed test is a panic, and a failed test binary is exit status 101");
    let (_, code) = run_child("panic", None, &[]);
    println!("   a process whose main thread panicked exited with {code:?}");
    println!("   libtest exits with the same 101 when any test failed, and cargo");
    println!("   passes that status on. It also stops at the first failing binary:");
    println!("   the targets after it are not run unless you pass --no-fail-fast.");
}
