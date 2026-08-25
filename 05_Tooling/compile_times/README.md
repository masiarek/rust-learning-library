# Compile times: where a debug build's seconds go, and the three knobs that reach them

**Level:** 201 · working knowledge

**One line:** A build is four phases, not one — parse, type-check, generate code, link — and every optimization worth knowing works by shrinking exactly one of them, which is why the honest question is never *"how do I make Rust faster?"* but *"which phase am I paying for?"*

Rust's compile times are the language's most-complained-about property, and most of the advice you will find is a list of flags with no model attached. A list of flags is unusable, because the same flag that halves one project's build does nothing measurable to another's. The model is small enough to hold in your head, and once you have it the flags sort themselves.

This page follows the three optimizations in [Let's Get Rusty's *How to decrease your Rust compile times by 50%* ↗](https://youtu.be/vFp4IbC2aZ0) — reduced debug info, parallel type checking, and the Cranelift backend — and adds the arithmetic that says what each one can and cannot buy *you*.

---

## First: measure, because the answer is per-project

```sh
cargo build --timings
```

That writes `target/cargo-timings/cargo-timing.html` — a chart of every crate in the build, how long each took, and which ones blocked others. Open it before changing anything. Two projects with identical line counts can have completely different shapes:

| If the profile is dominated by… | …the project probably has |
|---|---|
| **type & borrow check** | heavy generics, deep trait bounds, big macro expansions (`serde`, `diesel`) |
| **codegen** | a lot of monomorphization — one generic function compiled once per concrete type |
| **link** | many dependencies, or full debug info feeding an enormous object graph to the linker |

A knob that reaches a phase you barely spend time in is a knob that does nothing. That sounds obvious written down; it is the single most common reason someone applies all the advice and sees no change.

## 1. Reduce debug information — stable, and the first thing to try

`[profile.dev]` defaults to `debug = true`, meaning full DWARF: every type, every local variable, every lexical scope. It is expensive to *produce*, and then it is expensive a second time, because those fattened object files are what the linker has to chew through.

```toml
# Cargo.toml
[profile.dev]
debug = "line-tables-only"
```

You keep file-and-line resolution, so a backtrace still names the line that panicked. What you give up is a debugger's ability to inspect locals and step reliably — so if you do not live in `lldb` or `gdb`, you have given up nothing you will notice. `debug = 0` goes further and drops line numbers from backtraces too, which is usually a step too far.

Be precise about what this does *not* affect: the `panicked at src/main.rs:5:9` in a panic message comes from [`#[track_caller]`](https://doc.rust-lang.org/std/panic/struct.Location.html) and is compiled into the call itself, not read out of debug info. That line stays exactly as good at any `debug` setting. It is the **backtrace** — the frames underneath — that gets vaguer.

No nightly, no risk, works today. Start here.

## 2. Parallel type checking — nightly, still experimental

rustc's *back* end has been multi-threaded for years: codegen units are farmed out across cores. The *front* end — parsing, name resolution, type checking, borrow checking — was single-threaded, so on a 10-core machine nine cores idled through the phase that dominates a generics-heavy crate.

```sh
RUSTFLAGS="-Z threads=8" cargo +nightly build
```

The parallel front end is real and it works, but it is not finished: diagnostics can come out in a different order run to run, and it still has open ICEs. Fine on your own machine where you can drop the flag the moment something looks strange; not something to put in CI, where a nondeterministic compiler is a debugging nightmare wearing a build failure's clothes.

## 3. Cranelift instead of LLVM — nightly, and debug builds only

LLVM is a code generator tuned to emit *fast* code. [Cranelift ↗](https://cranelift.dev/) is one tuned to emit code *fast*. For a debug build — where you do not care how the binary performs, you care how soon you can run the test — that is precisely the right trade.

```sh
rustup +nightly component add rustc-codegen-cranelift-preview
```

```toml
# .cargo/config.toml
[unstable]
codegen-backend = true

[profile.dev]
codegen-backend = "cranelift"
```

Note the scoping. Putting Cranelift on your **release** profile hands you a slower binary and defeats the entire point, so it belongs under `[profile.dev]` and nowhere else. It also does not support every target or every scrap of inline assembly, so some dependency somewhere may refuse to build — which is a reason to keep it in a config you can comment out, not a reason to skip it.

## Stacking all three

The three knobs reach three different phases, which is exactly why they stack instead of overlapping. Here is that arithmetic on one measured profile — five numbers written down as data, so you can substitute your own:

<!-- output:compile_times -->
*Verified output of [`compile_times.rs`](examples/compile_times.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Clean debug build, as measured:              90.0 s

after adding                       total    saved  needs
-------------------------------------------------------------
debug = "line-tables-only"        73.3 s    18.6%  stable
-Z threads=8                      64.5 s    28.3%  nightly
codegen-backend = cranelift       47.4 s    47.3%  nightly

phase                        before    after   removed
-------------------------------------------------------------
parse & macro expansion       4.0 s    4.0 s     0.0 s
type & borrow check          22.0 s   13.2 s     8.8 s
codegen (LLVM)               38.0 s   20.9 s    17.1 s
debug info                   14.0 s    2.1 s    11.9 s
link                         12.0 s    7.2 s     4.8 s

90.0 s -> 47.4 s, a 47% cut.
The floor: 4.0 s no knob reached, now 8% of the build.
The next thing worth attacking is codegen (LLVM) at 20.9 s.
```
<!-- /output -->

Read the last two lines rather than the headline. **The floor** is Amdahl's law doing its usual work: the phase nothing reached did not get smaller, so it went from 4% of the build to 8% of it, and it now bounds how much further any of this can go. **The next thing worth attacking** is the question the table exists to answer, and it is a different phase for every project.

## Why this page has no number of its own

Every other page in this library ends in output a program actually printed, and this one does not — deliberately. A build time is [exactly the input the answer keys cannot hold](../../CONTRIBUTING.md): it depends on your cores, your RAM, your disk, your toolchain version and what else your laptop was doing, so a recorded one would be a number nobody could reproduce, sitting on a page whose whole promise is that its numbers are reproducible. The same goes for a backtrace, which prints addresses.

So the program above measures nothing. The profile is *data on the page*, in plain sight where you can disagree with it, and the program does the arithmetic — which is the part that transfers. That distinction is worth internalizing beyond this page: **"our build got 47% faster" is a fact about one machine; "debug info and link time were 29% of our build" is a fact about the project**, and only the second one tells anybody what to do next.

If you want the first kind of number for your own project, measure it the same way twice:

```sh
cargo clean && time cargo build          # clean build
touch src/main.rs && time cargo build    # incremental rebuild
```

Both matter, and they respond differently — link time is nearly all of a small incremental rebuild, so the debug-info knob usually shows up much larger there than on a clean build.

## The knob that is not in the video

Swap the **linker**. `ld64` on macOS and GNU `ld` on Linux are both slow next to [`lld` ↗](https://lld.llvm.org/) or [`mold` ↗](https://github.com/rui314/mold), and since link time is most of an incremental rebuild, this is often the single biggest win on the edit-compile-test loop specifically. Recent Rust has started defaulting to `lld` on x86-64 Linux; macOS has not.

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

It also sets up the trap in the practice below.

## If you are coming from another language

- **Python** — there is no compile step to optimize, so the closest analogue is import time and C-extension builds. What transfers is the discipline: profile before you tune. What is new is that in Rust the cost is paid by *you*, at every edit, rather than by the user at every run — which is what makes twenty seconds feel so much more expensive than it sounds.
- **ABAP** — activation is the counterpart, and it is likewise something you wait on constantly rather than occasionally. The difference is that the SAP system decides what to regenerate and you cannot renegotiate it; Cargo's profiles are yours, so a debug build can be configured to be genuinely cheap.
- **C++** — this will all be familiar, including the phases and the linker being the bottleneck. Two things genuinely differ: Rust has no header-inclusion problem to fix (so no precompiled headers, no `#include` hygiene work), and monomorphization is more aggressive than C++ templates in practice, which moves proportionally more of the build into codegen.

## Practice

**Credit a fourth knob honestly.** Add the linker swap above to the ladder — assume it leaves 45% of link time — and run it in two positions: once as the *first* change applied, once as the *last*. Report what it appears to save in each, and what the final build time is in each.

Before you look: predict both. The interesting outcome is not which order is faster.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:compile_times_kata -->
*[`compile_times_kata.rs`](examples/compile_times_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata: add a fourth knob — a faster linker — and credit it honestly.
//!
//! Same profile as the lesson, same three knobs, plus `linker = "lld"`, which
//! leaves 45% of link time. The question is what the linker swap is *worth*, and
//! the answer is that the question is badly posed: run the ladder with the swap
//! first and again with it last, and the same change earns a different number of
//! seconds while the final total does not move at all.

#[derive(Clone, Copy)]
struct Phase {
    name: &'static str,
    seconds: f64,
}

struct Tweak {
    name: &'static str,
    effect: &'static [(&'static str, f64)],
}

impl Tweak {
    fn survival(&self, phase: &str) -> f64 {
        self.effect
            .iter()
            .find(|(name, _)| *name == phase)
            .map(|&(_, fraction)| fraction)
            .unwrap_or(1.0)
    }
}

const BASELINE: &[Phase] = &[
    Phase { name: "parse & macro expansion", seconds: 4.0 },
    Phase { name: "type & borrow check", seconds: 22.0 },
    Phase { name: "codegen (LLVM)", seconds: 38.0 },
    Phase { name: "debug info", seconds: 14.0 },
    Phase { name: "link", seconds: 12.0 },
];

const DEBUGINFO: Tweak = Tweak {
    name: "debug = \"line-tables-only\"",
    effect: &[("debug info", 0.15), ("link", 0.60)],
};
const THREADS: Tweak = Tweak {
    name: "-Z threads=8",
    effect: &[("type & borrow check", 0.60)],
};
const CRANELIFT: Tweak = Tweak {
    name: "codegen-backend = cranelift",
    effect: &[("codegen (LLVM)", 0.55)],
};
const LLD: Tweak = Tweak {
    name: "linker = \"lld\"",
    effect: &[("link", 0.45)],
};

/// Walk one ordering, printing what each rung takes off the running total.
fn ladder(label: &str, order: &[&Tweak]) -> f64 {
    println!("{label}");
    let mut current: Vec<f64> = BASELINE.iter().map(|p| p.seconds).collect();
    for tweak in order {
        let before = current.iter().sum::<f64>();
        current = current
            .iter()
            .zip(BASELINE)
            .map(|(seconds, phase)| seconds * tweak.survival(phase.name))
            .collect();
        let after = current.iter().sum::<f64>();
        println!("  {:<31} -{:>5.2} s   ({after:>5.1} s left)", tweak.name, before - after);
    }
    current.iter().sum()
}

fn main() {
    let first = ladder("Linker swapped FIRST:", &[&LLD, &DEBUGINFO, &THREADS, &CRANELIFT]);
    println!();
    let last = ladder("Linker swapped LAST:", &[&DEBUGINFO, &THREADS, &CRANELIFT, &LLD]);

    println!("\nSame four changes, same final build: {first:.2} s and {last:.2} s.");
    println!(
        "But the linker swap is credited with 6.60 s in one order and 3.96 s in the other, \
         so \"lld saved us N seconds\" is a fact about the ORDER, not about lld."
    );
    let baseline: f64 = BASELINE.iter().map(|p| p.seconds).sum();
    println!("Overall: {baseline:.1} s -> {last:.1} s, a {:.0}% cut.", 100.0 * (baseline - last) / baseline);
}
```
<!-- /source -->

<!-- output:compile_times_kata -->
*Verified output of [`compile_times_kata.rs`](examples/compile_times_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Linker swapped FIRST:
  linker = "lld"                  - 6.60 s   ( 83.4 s left)
  debug = "line-tables-only"      -14.06 s   ( 69.3 s left)
  -Z threads=8                    - 8.80 s   ( 60.5 s left)
  codegen-backend = cranelift     -17.10 s   ( 43.4 s left)

Linker swapped LAST:
  debug = "line-tables-only"      -16.70 s   ( 73.3 s left)
  -Z threads=8                    - 8.80 s   ( 64.5 s left)
  codegen-backend = cranelift     -17.10 s   ( 47.4 s left)
  linker = "lld"                  - 3.96 s   ( 43.4 s left)

Same four changes, same final build: 43.44 s and 43.44 s.
But the linker swap is credited with 6.60 s in one order and 3.96 s in the other, so "lld saved us N seconds" is a fact about the ORDER, not about lld.
Overall: 90.0 s -> 43.4 s, a 52% cut.
```
<!-- /output -->

The final total is identical, because scaling factors multiply and multiplication commutes. What moves is the **credit**: the linker swap earns 6.60 s applied first and 3.96 s applied last, and the debug-info change earns 16.70 s or 14.06 s depending on whether the linker got there first — because both of them are eating the same phase.

That is the practical lesson hiding under the arithmetic. A build optimization's saving is not a property of the optimization; it is a property of the optimization *and everything already applied*. So "mold saved us seven seconds" is not portable advice, benchmarking two changes separately and adding the results overstates them, and the only honest way to report a stack is the way the table above does — cumulatively, in a stated order.

</details>

## Sources

- [Let's Get Rusty — *How to decrease your Rust compile times by 50%* ↗](https://youtu.be/vFp4IbC2aZ0) — the three knobs, in five minutes
- [The Cargo book on `profile.debug` ↗](https://doc.rust-lang.org/cargo/reference/profiles.html#debug) — every accepted value, and what each keeps
- [corrode.dev — *Tips for faster Rust compile times* ↗](https://corrode.dev/blog/tips-for-faster-rust-compile-times/) — the long version, and the best-organized list of the rest
- [The rustc dev guide on the parallel front end ↗](https://rustc-dev-guide.rust-lang.org/parallel-rustc.html) — what `-Z threads` actually parallelizes, and what is still serial
