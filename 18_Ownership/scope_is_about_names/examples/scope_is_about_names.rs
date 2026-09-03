//! Scope is about names, not values.
//!
//! One word gets asked three questions: when can I still write this name,
//! when is the value freed, and when does a borrow stop mattering. The three
//! answers arrive at three different moments, and this program prints all
//! three so the order is observed rather than asserted.
//!
//!   rustc --edition 2024 scope_is_about_names.rs -o /tmp/scope && /tmp/scope

use std::sync::Mutex;

struct Tracked(&'static str);

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("    drop: {}", self.0);
    }
}

/// Two fields, declared in this order, to be watched at the brace.
struct Order {
    _marked: Tracked,
    _counted: Tracked,
}

fn banner(title: &str) {
    println!("\n──── {title}");
}

fn main() {
    banner("1. A name ends at the brace; its value need not");

    let carried = {
        let _scratch = Tracked("scratch");
        let leaving = Tracked("carried out");
        println!("  inside:  two names, two values");
        leaving //                          the VALUE leaves; the name does not
    };
    println!("  after:   the block is over, and that value is alive as `carried`");
    println!("  after:   `leaving` and `_scratch` are not names here —");
    println!("           error[E0425]: cannot find value `leaving` in this scope");
    println!("  So the block ended two names and one value. They are not one event.");
    drop(carried);

    banner("2. A borrow ends at its last use, not at the brace");

    let mut orders = vec!["alpha"];
    let first = &orders[0]; //             the borrow starts here
    println!("  first   = {first}   <- and ends here, at its last use");
    orders.push("beta"); //                 legal: nothing is borrowed any more
    println!("  orders = {orders:?}   <- mutated afterwards, same block, no error");
    println!("  Move that first println! BELOW the push and it stops compiling:");
    println!("      error[E0502]: cannot borrow `orders` as mutable because it is");
    println!("                    also borrowed as immutable");
    println!("        |   let first = &orders[0];");
    println!("        |                ------- immutable borrow occurs here");
    println!("        |   orders.push(\"beta\");");
    println!("        |   ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here");
    println!("        |   println!(\"{{first}}\");");
    println!("        |              ----- immutable borrow later used here");
    println!("  No brace moved. What moved was the last use — which is the whole");
    println!("  of the borrow's life, and it is shorter than the name's.");

    banner("3. When each value actually dies");

    {
        let _first = Tracked("local declared first");
        let _second = Tracked("local declared second");
        println!("  two locals, and the brace is next:");
    }
    println!("  ^ reverse declaration order — the last one declared dies first");

    {
        let _order = Order {
            _marked: Tracked("field declared first"),
            _counted: Tracked("field declared second"),
        };
        println!("  one struct, two fields, and the brace is next:");
    }
    println!("  ^ DECLARATION order. The reverse rule is about locals, not fields");

    println!("  temporary: {}", Tracked("an unnamed temporary").0);
    println!("  ^ dropped at the end of the statement that built it, not the block");

    {
        let _ = Tracked("let _      = …");
        println!("  `let _`      binds nothing, so the value died on that line");
        let _kept = Tracked("let _kept = …");
        println!("  `let _kept`  binds normally; it is still alive, until:");
    }

    let early = Tracked("drop(early)");
    drop(early); //                         a function that takes ownership
    println!("  drop(x) is not a keyword and not magic: it takes x and returns ()");

    banner("4. Held, or not: `try_lock` answers what style cannot");

    let counter = Mutex::new(0_u32);
    {
        // Written without the allow, this is a hard ERROR, not a warning:
        //   error: non-binding let on a synchronization lock
        //   note: `#[deny(let_underscore_lock)]` (part of `#[deny(let_underscore)]`)
        //         on by default
        #[allow(let_underscore_lock)]
        let _ = counter.lock().unwrap();
        println!(
            "  after `let _      = counter.lock()`:  try_lock succeeds = {}",
            counter.try_lock().is_ok()
        );
    }
    {
        let _guard = counter.lock().unwrap();
        println!(
            "  after `let _guard = counter.lock()`:  try_lock succeeds = {}",
            counter.try_lock().is_ok()
        );
    }
    println!("  Same thread, same mutex, one identifier apart, and `try_lock` can");
    println!("  tell them apart. The first is an unlocked critical section that");
    println!("  reads exactly like a locked one — which is why rustc DENIES it");
    println!("  by default, and why the line above needs an #[allow] to exist.");

    println!("  But the lint knows std's locks, not the pattern:");
    {
        let _ = Tracked("a hand-rolled guard, released early");
        println!("  no error, no warning — the value is already gone");
    }

    banner("5. Some names are in scope before they are written");

    println!("  limit() = {}   <- called above its own definition", limit());

    fn limit() -> u32 {
        7
    } //                                    an ITEM: in scope for the whole block

    println!("  A `let` cannot do that. Read one above its own line and you get");
    println!("      error[E0425]: cannot find value `limit` in this scope");
    println!("  Items are in scope for the entire block; a binding's scope starts");
    println!("  at its `let` and runs to the brace. Same word, two different rules.");
}
