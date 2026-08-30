//! Kata solution: three drop orders, and the guard released one line early.
//!
//!   rustc --edition 2024 drop_and_raii_kata.rs -o /tmp/drk && /tmp/drk

use std::cell::RefCell;

// A shared log, so the whole run can be printed in order at the end.
thread_local! {
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

fn note(s: String) {
    LOG.with(|l| l.borrow_mut().push(s));
}

fn take_log() -> Vec<String> {
    LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

struct Named(&'static str);

impl Drop for Named {
    fn drop(&mut self) {
        note(format!("drop {}", self.0));
    }
}

struct Outer {
    first: Named,
    second: Named,
}

impl Drop for Outer {
    fn drop(&mut self) {
        note("drop Outer itself".to_string());
    }
}

/// The RAII guard: it holds the lock for exactly as long as it is alive.
struct Guard(&'static str);

impl Guard {
    fn acquire(name: &'static str) -> Self {
        note(format!("acquire {name}"));
        Guard(name)
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        note(format!("release {}", self.0));
    }
}

fn show(title: &str) {
    println!("   {title}");
    for line in take_log() {
        println!("     {line}");
    }
}

fn main() {
    println!("1. Locals: reverse declaration order");
    {
        let _a = Named("a");
        let _b = Named("b");
        let _c = Named("c");
        note("end of block".to_string());
    }
    show("");

    println!();
    println!("2. A struct: itself first, then its fields IN declaration order");
    {
        let o = Outer { first: Named("field first"), second: Named("field second") };
        note(format!("built from {} and {}", o.first.0, o.second.0));
        note("end of block".to_string());
    }
    show("");
    println!("   Opposite rules, and both are forced. A later local may borrow an");
    println!("   earlier one, so locals unwind. A struct's own drop body may still");
    println!("   read its fields, so the struct goes first and the fields follow in");
    println!("   the order they were written.");

    println!();
    println!("3. A Vec drops front to back");
    {
        let _v = vec![Named("v0"), Named("v1"), Named("v2")];
        note("end of block".to_string());
    }
    show("");

    println!();
    println!("4. The guard released one line early");
    {
        let _g = Guard::acquire("whole block");
        note("work with the lock held".to_string());
    }
    show("held to the end of the block:");
    {
        let g = Guard::acquire("dropped early");
        note("work with the lock held".to_string());
        drop(g);
        note("work with the lock NOT held".to_string());
    }
    show("explicitly released, then more work:");
    {
        let _ = Guard::acquire("bound to underscore");
        note("work — but is the lock held?".to_string());
    }
    show("bound to a bare `_`:");
    println!("   The third is the bug. `let _ = Guard::acquire(..)` does not bind");
    println!("   anything, so the guard is a temporary that drops at the END OF");
    println!("   THAT STATEMENT — the lock is released before the work starts. The");
    println!("   fix is one character: `let _g = ...`, which is a binding and lives");
    println!("   to the end of the scope. This is the most common RAII mistake in");
    println!("   Rust, and it compiles without a warning.");

    println!();
    println!("5. Drop still runs when the work does not finish");
    fn early_return(fail: bool) -> &'static str {
        let _g = Guard::acquire("across an early return");
        if fail {
            return "returned early";
        }
        "ran to the end"
    }
    let r = early_return(true);
    show(&format!("early_return(true) -> {r}"));
    println!("   And on a panic too, while the stack unwinds — which is what makes");
    println!("   a guard a guarantee rather than a convention. The exceptions are");
    println!("   std::mem::forget, a reference cycle, and a process that aborts");
    println!("   instead of unwinding.");
}
