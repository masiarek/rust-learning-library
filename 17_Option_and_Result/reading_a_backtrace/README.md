# Reading a backtrace

**Level:** 201 · working knowledge

**One line:** A panic names the line that failed; the **backtrace** names the caller that was wrong — and it is off until you ask for it, it prints innermost frame first, and `--release` deletes the middle of the chain.

```bash
RUST_BACKTRACE=1 cargo run
```

That is the whole fix for the commonest bad half-hour in Rust: a panic in a helper, and no idea which of its callers handed it the bad value. The rest of this page is what the output means.

## The message names the helper, not the mistake

One price table with a hole in it, one helper, two callers:

```rust
const PRICES: &[(&str, u32)] = &[("BOLT-1", 40), ("NUT-2", 15), ("SHIP-STD", 599)];

fn unit_price(sku: &str) -> u32 {
    PRICES.iter().find(|(s, _)| *s == sku).unwrap().1
}

fn shipping_total() -> u32 { unit_price("SHIP-STD") }                    // fine
fn cart_total() -> u32 { unit_price("BOLT-1") + unit_price("WIDGET-9") } // the bug
```

`unit_price` is correct. `cart_total` asks it for a SKU that was never in the table. And the panic points at `unit_price`:

```text title="Abridged — real output, the thread id after 'main' dropped because it changes every run"
thread 'main' panicked at reading_a_backtrace.rs:28:44:
called `Option::unwrap()` on a `None` value
```

Line 28 is the `unwrap`. It is the same line whichever caller was wrong, so the message cannot tell you, and with thirty callers instead of two it is not even a shortlist. [`#[track_caller]`](../what_a_panic_costs/README.md) already did what it could — it walked the location out of `core/src/option.rs` and into your file — but it walks up exactly one level, to the frame that called `unwrap`, and stops.

## Switch it on

| Setting | What you get |
|---|---|
| unset | the message, plus a `note:` telling you the variable exists |
| `RUST_BACKTRACE=1` | the frames, symbol names, `at file:line` for anything built with debug info |
| `RUST_BACKTRACE=full` | the same list with addresses and every std frame the short form hid |

`0` is explicitly off, which is the value to set when something else in your environment has turned it on.

## How to read the list

Real output, with the `at /rustc/<hash>/…` line under each std frame removed:

```text title="Abridged — real output, the toolchain-path lines dropped"
stack backtrace:
   0: __rustc::rust_begin_unwind          ┐
   1: core::panicking::panic_fmt          │ the panic machinery
   2: core::panicking::panic              │ (starts at the top, always)
   3: core::option::unwrap_failed         ┘
   4: reading_a_backtrace::unit_price     ┐ yours
   5: reading_a_backtrace::cart_total     │  ← the caller that was wrong
   6: reading_a_backtrace::main           ┘
   7: <fn() as core::ops::function::FnOnce<()>>::call_once   ┐ the runtime
```

**Frame 0 is innermost.** The list reads inward-out: `main` called `cart_total` called `unit_price`, and you are seeing that backwards. So the answer to *who did this* is the frame **below** the panic site, not above it.

Three bands, and only the middle one is yours. The top band is always some arrangement of `rust_begin_unwind` and `panic_fmt` — worth recognising once so you can skip it forever, since it describes the panic and not the bug. The bottom band is the runtime that called `main`. **Your first frame is the first line naming your crate**, and on the main thread your last is `main` — a spawned thread ends somewhere else entirely, which is the exercise at the bottom of this page.

## What varies, and what does not

Three things in that output move, which matters the moment you paste one into an issue or try to record one as a test:

- **The thread id.** The real first line is `thread 'main' (12771775) panicked at …`, and that number is different on every single run.
- **The toolchain hash.** `at /rustc/88d9e12ae178…/library/std/src/panicking.rs:679:5` names the commit rustc was built from and the line inside it — both change with every release.
- **Which std frames appear at all**, and how many, between platforms.

What does not move is the middle band: your own function names, in your own order. That is why the example behind this page prints only those, and why a bug report is more useful with the whole thing pasted in than with your summary of it.

Missing `at file:line` on your own frames is not a bug either — a bare `rustc file.rs` emits no debug info, so names resolve from the symbol table and line numbers have nowhere to come from. Cargo's `dev` profile has `debug = true`, so under `cargo run` they are there.

## `--release` deletes the middle of the chain

The same program, the same panic, built with `-O`:

```text title="Abridged — real output from the same file, std frames 0–3 identical"
   4: reading_a_backtrace::main
```

`unit_price` and `cart_total` are gone. Neither was deleted from your program — both were **inlined into `main`**, and an inlined function has no frame to appear in. The panic location is still exactly right (`reading_a_backtrace.rs:28:44` — it is baked in at compile time), so a release backtrace tells you *where* with full precision and can tell you nothing at all about *how you got there*.

Two ways out when the panic only happens in release. Add `debug = true` to the `[profile.release]` section: the optimiser still inlines, but the debug info records *what* it inlined, and the frames come back — `-O -g` on this file prints all three names again. Or reproduce it in a debug build first, which is cheaper when you can.

## Capturing one without a panic

[`std::backtrace::Backtrace` ↗](https://doc.rust-lang.org/std/backtrace/struct.Backtrace.html) is the same machinery as a value, so a stack is something you can take at a point of your choosing, store in an error type, and print later.

```rust
use std::backtrace::Backtrace;

let bt = Backtrace::capture();        // obeys RUST_BACKTRACE, like the panic hook
let bt = Backtrace::force_capture();  // ignores it — always walks, always pays
```

`capture()` is the one to reach for by default, because it is the one a user can switch off. Its [`status()` ↗](https://doc.rust-lang.org/std/backtrace/enum.BacktraceStatus.html) is an enum rather than a bool, and the third variant is the one that saves an afternoon: `Disabled` means nobody asked, `Captured` means you have frames, and `Unsupported` means this target cannot walk a stack at all — no amount of setting the variable will change it.

## If you are coming from another language

**Python.** A traceback is the same list with three of its conventions the other way round. It is **on by default** — an uncaught exception prints it, with no environment variable to remember — and it prints **outermost first**, under the header *"most recent call last"*, so the line you want is at the **bottom** where Rust puts it near the top. It also carries `file:line` and the **source line itself** for every frame, which Rust never does. What transfers exactly is `traceback.format_stack()`, which is `Backtrace::force_capture()` under a different name: a stack taken deliberately, away from any exception. And one Python habit does not survive the trip — `except Exception:` around a call, looking at `e.__traceback__` to see where it came from, has no Rust equivalent you should be reaching for, because a panic is not an exception and [`catch_unwind`](../what_a_panic_costs/README.md) is not `except`. In Rust the caller's context travels in a `Result`, and the backtrace is for the bug you did not plan for.

**ABAP.** The nearest thing is the **short dump** in `ST22`, and the mapping is close enough to be useful: the *Call stack* / *Aktive Aufrufe* section is the backtrace, listed the same way Rust lists it — the failing routine at the top, its caller below. Two differences worth holding on to. The dump is **always** produced, with no switch anywhere, and stays readable in the system afterwards for as long as housekeeping is set to keep it — where Rust discards the stack unless the variable was set *before the process started*. There is no ABAP equivalent of losing the evidence because nobody asked in advance. And the dump carries the variable values at each level, which a Rust backtrace does not — it is a list of names, nothing more, so anything you want to know about the data has to be in the panic message or in a value you captured yourself. `Backtrace::force_capture()` stored in an error struct is the closest thing to writing your own dump at a point you choose.

## Practice

**Find the wall.** The page above says the backtrace answers *who called this?*. Find the boundary where it stops answering.

1. Take the same `unit_price` / `cart_total` pair and call it from a function `checkout_request` that hands the work to `thread::spawn` and `join`s it. Run with `RUST_BACKTRACE=1`. Which of the four names appear in the backtrace, and why exactly those?
2. Predict, before running: is `main` in it?
3. Now make the failure report name the submitter anyway. You may not move the panic and you may not change which thread the work runs on.
4. What is the general rule, and which other boundaries does it apply to?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:reading_a_backtrace_kata -->
*[`reading_a_backtrace_kata.rs`](examples/reading_a_backtrace_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a backtrace describes ONE stack, and a thread is a wall.
//!
//! `RUST_BACKTRACE=1` answers "who called this?" perfectly — right up to the
//! point where the work crossed a thread boundary. Past that the trail stops,
//! because the code that handed the job over is not on the stack that failed.
//! The fix is to capture the caller's stack *at the handover* and carry it.
//!
//!   rustc --edition 2024 reading_a_backtrace_kata.rs -o /tmp/rabk && /tmp/rabk

use std::backtrace::Backtrace;
use std::panic;
use std::process::{Command, Stdio};
use std::thread;

const PRICES: &[(&str, u32)] = &[("BOLT-1", 40), ("NUT-2", 15), ("SHIP-STD", 599)];
const OURS: &str = "reading_a_backtrace_kata::";

fn unit_price(sku: &str) -> u32 {
    PRICES.iter().find(|(s, _)| *s == sku).unwrap().1
}

fn cart_total() -> u32 {
    unit_price("BOLT-1") + unit_price("WIDGET-9")
}

/// The submitter. It hands the work to another thread and waits — so it is on
/// the *calling* stack, never on the one that fails.
fn checkout_request() -> Result<u32, Submitted> {
    submit(cart_total)
}

// ── part 1: watch the trail stop at the thread boundary ─────────────────────

fn run_child(arg: &str) -> String {
    let me = std::env::current_exe().expect("the running binary has a path");
    let out = Command::new(me)
        .arg(arg)
        .env("RUST_BACKTRACE", "1")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect("the child runs");
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// Our own functions in a dump — closures dropped, so the list is the same on
/// every platform.
fn our_frames(dump: &str) -> Vec<&str> {
    dump.lines()
        .filter_map(|l| l.split_once(OURS))
        .map(|(_, n)| n.trim())
        .filter(|n| !n.contains("::"))
        .collect()
}

// ── part 2: carry the submitter's stack across by hand ──────────────────────

/// A failure plus the stack of whoever *asked* for the work, captured at the
/// moment of asking rather than at the moment of failing.
struct Submitted {
    what: String,
    submitted_from: Backtrace,
}

fn submit<T: Send + 'static>(job: fn() -> T) -> Result<T, Submitted> {
    // Captured HERE, on the calling thread, while its stack is still standing.
    let submitted_from = Backtrace::force_capture();
    thread::spawn(job).join().map_err(|payload| Submitted {
        // A panic payload is `&'static str` for a fixed message and `String`
        // for a formatted one. `unwrap` produces the first; ask for both.
        what: payload
            .downcast_ref::<&str>()
            .map(|s| (*s).to_owned())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "(unknown payload type)".to_owned()),
        submitted_from,
    })
}

fn main() {
    if std::env::args().any(|a| a == "--in-a-thread") {
        // The panic hook still prints this thread's backtrace to stderr.
        let _ = checkout_request();
        return;
    }

    println!("── Part 1: the same panic, on a thread of its own");
    let dump = run_child("--in-a-thread");
    let frames = our_frames(&dump);
    println!("our frames in the worker's backtrace: {frames:?}");
    println!("  unit_price       (where it failed)  present: {}", frames.contains(&"unit_price"));
    println!("  cart_total       (its caller)       present: {}", frames.contains(&"cart_total"));
    println!("  checkout_request (the submitter)    present: {}", frames.contains(&"checkout_request"));
    println!("  main                                present: {}", frames.contains(&"main"));
    println!("The last two ran on the other stack, so the trail simply ends.");

    println!("\n── Part 2: capture the submitter's stack at the handover");
    // The worker still panics, and the default hook would print its message and
    // its backtrace to stderr. Silence it: this half of the exercise is about
    // the stack we captured ourselves, on the near side of the boundary.
    panic::set_hook(Box::new(|_| {}));
    match checkout_request() {
        Ok(total) => println!("no panic, total = {total}"),
        Err(e) => {
            println!("the worker failed with: {}", e.what);
            let here = e.submitted_from.to_string();
            let names = our_frames(&here);
            println!("the captured stack names:");
            println!("  checkout_request  present: {}", names.contains(&"checkout_request"));
            println!("  main              present: {}", names.contains(&"main"));
            println!("Those are the two the worker's own backtrace could not see.");
        }
    }

    println!("\n── The rule");
    println!("Every boundary that moves work to another stack — a thread, a");
    println!("channel, an executor — ends the backtrace. Capture on the near");
    println!("side and carry it, or the far side reports a chain with no origin.");
}
```
<!-- /source -->

<!-- output:reading_a_backtrace_kata -->
*Verified output of [`reading_a_backtrace_kata.rs`](examples/reading_a_backtrace_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
── Part 1: the same panic, on a thread of its own
our frames in the worker's backtrace: ["unit_price", "cart_total"]
  unit_price       (where it failed)  present: true
  cart_total       (its caller)       present: true
  checkout_request (the submitter)    present: false
  main                                present: false
The last two ran on the other stack, so the trail simply ends.

── Part 2: capture the submitter's stack at the handover
the worker failed with: called `Option::unwrap()` on a `None` value
the captured stack names:
  checkout_request  present: true
  main              present: true
Those are the two the worker's own backtrace could not see.

── The rule
Every boundary that moves work to another stack — a thread, a
channel, an executor — ends the backtrace. Capture on the near
side and carry it, or the far side reports a chain with no origin.
```
<!-- /output -->

</details>

## The verified output

The panic transcripts on this page were produced by hand from the same file, because rustc records the path it was handed and a panic prints a fresh thread id every run — neither survives being an answer key. The program below measures the parts that do, by re-running itself as a child process with `RUST_BACKTRACE` set and unset explicitly, so what it prints does not depend on what is set in your shell.

<!-- output:reading_a_backtrace -->
*Verified output of [`reading_a_backtrace.rs`](examples/reading_a_backtrace.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The working caller returns 599 cents.

──── Step 1: the message you get by default
panicked at   reading_a_backtrace.rs:28:44
message       called `Option::unwrap()` on a `None` value
our frames    0
but it tells you how to ask: true

──── Step 2: the same panic with RUST_BACKTRACE=1
panicked at   reading_a_backtrace.rs:28:44   <- unchanged
message       called `Option::unwrap()` on a `None` value   <- unchanged
our frames, in the order the backtrace printed them:
   0: unit_price
   1: cart_total
   2: main

──── Step 3: which of the two callers was wrong
the panic named      unit_price   (both callers reach it)
the backtrace named  cart_total
is shipping_total in the chain?  false
innermost frame first, so the caller is the line BELOW the panic site.

──── Step 4: capturing one without a panic, and without the variable
Backtrace::capture()        RUST_BACKTRACE unset -> Disabled
Backtrace::capture()        RUST_BACKTRACE=1     -> Captured
Backtrace::force_capture()  either way           -> Captured
and it saw this very function: true
```
<!-- /output -->

## See also

- [What a panic costs](../what_a_panic_costs/README.md) — what unwinding does to your program while this page reads what it printed; its step 1 is where the `#[track_caller]` claim above is verified
- [`expect`: writing down the proof](../expect/README.md) — the message you wish the backtrace had not been needed for
- [`unwrap` is a TODO you forgot to remove](../../02_Errors/unwrap_is_a_todo/README.md) — where the `unwrap` in `unit_price` came from
- [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md) — the other way to find out where a value came from, and the other thing that writes to stderr
- [Debugging Rust](../../32_Debugging/README.md) — the map this page sits on
- [`std::backtrace` ↗](https://doc.rust-lang.org/std/backtrace/index.html) — the module, including the exact `RUST_BACKTRACE` precedence rules
- [`std::panic::set_hook` ↗](https://doc.rust-lang.org/std/panic/fn.set_hook.html) — how to replace what gets printed, which is where a custom reporter starts

## Po polsku

Najważniejsze zdanie jest praktyczne: **ślad stosu (*backtrace*) w Ruscie jest domyślnie wyłączony**, a komunikat paniki wskazuje linię, która się wywróciła, nie tę, która zawiniła. Gdy `unwrap` siedzi w pomocniczej funkcji wołanej z trzydziestu miejsc, komunikat za każdym razem pokaże tę samą linię — i dopiero `RUST_BACKTRACE=1` powie, który z trzydziestu wywołujących podał złą wartość. Wartość `full` dokłada adresy i ramki biblioteki standardowej, `0` wyłącza wszystko z powrotem.

Kolejność czytania bywa myląca dla osób przychodzących z Pythona. Rust drukuje **od środka na zewnątrz**: ramka 0 to miejsce paniki, a wywołujący jest **niżej** — w Pythonie jest dokładnie odwrotnie (*most recent call last*), więc odruch „patrzę na koniec listy" prowadzi tu do `main` zamiast do błędu. Lista dzieli się na trzy pasma: na górze maszyneria paniki (`rust_begin_unwind`, `panic_fmt`), pośrodku **twoje** funkcje, na dole środowisko uruchomieniowe, które zawołało `main`. Warto rozpoznać pierwsze pasmo raz i już zawsze je przeskakiwać — opisuje panikę, nie błąd.

Dwie pułapki, obie kosztujące pół dnia. Pierwsza: w buildzie z `--release` **znikają ramki funkcji wstawionych w miejscu wywołania** (*inlining*) — w przykładzie na tej stronie z trzech nazw zostaje sama `main`, choć miejsce paniki nadal jest wskazane co do kolumny. Lekarstwem jest `debug = true` w sekcji `[profile.release]` albo odtworzenie błędu w buildzie deweloperskim. Druga: **ślad stosu opisuje jeden stos**, więc na granicy wątku (a tak samo kanału czy egzekutora zadań asynchronicznych) trop się urywa — kod, który zlecił robotę, nie stoi na stosie, który się wywrócił. Rozwiązanie jest w ćwiczeniu na tej stronie: `Backtrace::force_capture()` po stronie zlecającego, przeniesiony razem z błędem.

I jedna różnica warta zapamiętania wobec ABAP-a: krótki zrzut (*short dump*) w `ST22` powstaje **zawsze** i leży w systemie tygodniami, podczas gdy Rust wyrzuca stos, jeśli nikt nie poprosił o niego **przed** uruchomieniem procesu. Zrzut w ABAP-ie niesie też wartości zmiennych na każdym poziomie — ślad stosu w Ruscie to wyłącznie lista nazw, więc wszystko, co chcesz wiedzieć o danych, musi trafić do komunikatu paniki albo zostać uchwycone samodzielnie.

**Szukaj po polsku:** ślad stosu Rust · `RUST_BACKTRACE=1` · panika a wyjątek · `std::backtrace` · wstawianie funkcji a ślad stosu
