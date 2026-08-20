# Glossary

Short definitions. Every entry links to the page that explains it properly — a definition that dead-ends hides the lesson that already exists.

**`and_then`** — Transform a value with a closure that can *itself* come up empty or fail, flattening the result instead of nesting it. The counterpart to `map` when the closure returns another `Option`/`Result`. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`anyhow`** — A crate giving applications one catch-all error type with good ergonomics and backtraces. The application-side counterpart to `thiserror`. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`as_deref`** — Borrow through an owned inner value: `Option<String>` → `Option<&str>`. The usual fix when a method takes `self` but you still need the option afterwards. → [`unwrap_or`](01_Foundations/unwrap_or/README.md)

**`Box<dyn Error>`** — A type-erased error: any error can convert into it, so unrelated failures can flow through one function. What applications reach for when nothing downstream will `match` on the cause. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**Discriminant** — The number identifying which variant an enum value currently is; `None` is 0 and `Some` is 1, by declaration order. Comparable via `std::mem::discriminant`, but not extractable — and often not even stored. → [`Option` is a one-item collection](01_Foundations/option_as_collection/README.md)

**Enum** — A type that is exactly one of several named variants, each optionally carrying data. `Option` and `Result` are both ordinary enums; nothing about them is built into the language.

**`is_some_and`** — Ask whether an option is `Some` *and* its value passes a predicate, without unwrapping. Takes `self`, so pair it with `.as_ref()` for non-`Copy` types. → [`Option` is a one-item collection](01_Foundations/option_as_collection/README.md)

**`expect`** — Panic with a message you wrote. Preferred over `unwrap` everywhere, because the message records *why* you believed this could not fail — and being unable to write it is the signal to return a `Result` instead. → [`expect`](01_Foundations/expect/README.md)

**`Infallible`** — An enum with no variants, used as the `E` of a `Result` that cannot fail (`String`'s `FromStr`, `u64::try_from(u32)`). Because `Err` cannot be built, the compiler drops the tag and the `Result` costs what the value costs. → [The `Result` you are reading is probably an alias](01_Foundations/result_aliases/README.md)

**`From`** — The conversion trait. `?` calls it implicitly to turn one error type into the function's own, which is what makes custom error enums and `Box<dyn Error>` ergonomic. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`if let`** — A `match` with only the arm you care about. Sugar for the two-arm version, at the price of the compiler no longer checking that you covered every variant. → [`if let`](01_Foundations/if_let/README.md)

**Exhaustiveness** — The compiler's insistence that a `match` account for every variant, so adding one breaks the builds that now have a hole. `if let`, `while let`, and `matches!` all opt out of it for one expression. → [`if let`](01_Foundations/if_let/README.md)

**Let chain** — Several `let` bindings and conditions joined with `&&` in one `if let` head, each binding visible to the next. Stable since Rust 1.88 and only in edition 2024. → [`if let`](01_Foundations/if_let/README.md)

**Lock poisoning** — A `Mutex`/`RwLock` remembering that a thread panicked while holding its exclusive guard, so every later `lock()` returns `Err`. Not an error the lock hit: a warning that the invariant behind it may be half-restored. → [Lock poisoning](09_Advanced/mutex_poisoning/README.md)

**`PoisonError`** — What a poisoned `lock()` returns. It carries the guard, so nothing is lost — `into_inner()` hands you the data anyway, which makes `.unwrap()` a decision rather than the only option. → [Lock poisoning](09_Advanced/mutex_poisoning/README.md)

**`io::Result<T>`** — Not a different type: a one-line alias for `Result<T, std::io::Error>`. The pattern behind `fmt::Result` and `thread::Result` too. → [The `Result` you are reading is probably an alias](01_Foundations/result_aliases/README.md)

**`let … else`** — Bind a pattern or leave the current scope; the `else` block must diverge. The idiomatic guard clause — it keeps the happy path unindented. → [`if let`](01_Foundations/if_let/README.md)

**`matches!`** — Ask whether a value fits a pattern and get back a `bool`, optionally with a guard. What to write instead of an `if let` whose body only sets a flag. → [`if let`](01_Foundations/if_let/README.md)

**Scrutinee** — The expression a `match` or `if let` is examining. Worth knowing because edition 2024 changed when a temporary built there is dropped. → [`if let`](01_Foundations/if_let/README.md)

**Shadowing** — Declaring a variable whose name is already in use. The new one hides the old for the rest of the scope and may have a different type, which is what makes `let x = x.unwrap_or(0)` possible; it is not mutation, and the old variable returns when the scope ends. → [Shadowing and `unwrap`](01_Foundations/shadowing_and_unwrap/README.md)

**Pattern binding** — The name a pattern introduces, as `x` in `Some(x)`. A fresh name rather than a shadow — and for a non-`Copy` type it *moves* the value out of what you matched on, unless you borrow. → [Shadowing and `unwrap`](01_Foundations/shadowing_and_unwrap/README.md)

**`Copy`** — The trait marking a type that is duplicated instead of moved on assignment (`i32`, `bool`, `char`, `&T`, and `Option`s of them). A `String` cannot be `Copy`, because two owners of one allocation would mean two frees — so the test is not size but whether duplicating the bytes would duplicate an *obligation*. → [Ownership and moves](01_Foundations/ownership_and_moves/README.md), and why `if let Some(n) = opt` leaves `opt` usable for an `Option<i32>` but not an `Option<String>` → [Shadowing and `unwrap`](01_Foundations/shadowing_and_unwrap/README.md)

**`while let`** — Loop for as long as the pattern keeps matching; the `None` is the exit condition. Nothing checks that the body moves toward it, so the scrutinee has to consume. → [`while let`](01_Foundations/while_let/README.md)

**`map`** — Transform the value inside a wrapper, leaving the wrapper alone. Nests rather than flattens if the closure returns another wrapper — that is when you want `and_then`. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`#[must_use]`** — An attribute making the compiler warn when a returned value is discarded. Both `Option` and `Result` carry it, which is why an ignored error is a warning rather than a silent bug. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`Ok(())`** — Success carrying no value, because `()` has exactly one value and zero size. What every `Display` impl and every `fn main() -> Result<(), E>` ends with. → [The `Result` you are reading is probably an alias](01_Foundations/result_aliases/README.md)

**Non-lexical lifetimes** — A borrow lives until its **last use**, not to the end of its block. Why the same two statements compile in one order and are `E0502` in the other. → [Borrowing](01_Foundations/borrowing/README.md)

**Shared reference (`&T`)** — Access without ownership, held by any number of readers at once. "Shared", not "immutable": a `Cell` or `Mutex` still mutates through one. → [Borrowing](01_Foundations/borrowing/README.md)

**Exclusive reference (`&mut T`)** — The only reference to a value while it lives, and it excludes readers too — including the owner. → [Borrowing](01_Foundations/borrowing/README.md)

**Niche** — A bit pattern a type can never legally hold (null for a `Box`, any byte but 0/1 for a `bool`). `None` takes the niche when one exists, which is why the wrapper is often free. → [`Option` is a one-item collection](01_Foundations/option_as_collection/README.md)

**Exhaustiveness** — A `match` must cover every variant, or it does not compile. The value is not the check itself but what it does later: add a variant and every match that ignores it becomes a build error, so the list of places to revisit is computed rather than remembered. A `_` arm opts out permanently — it is a promise that every *future* variant belongs in that bucket. → [Six kinds of zero](01_Foundations/six_kinds_of_zero/README.md)

**Sum type (tagged union)** — A type that is exactly one of several alternatives, each free to carry different data. `Option` is one with two variants and no special powers; when a problem has six cases, writing your own is the idiomatic move, not a departure. → [Six kinds of zero](01_Foundations/six_kinds_of_zero/README.md)

**Null-pointer optimization** — The niche rule applied to pointers: `Option<Box<T>>` is the same size as `Box<T>`, because null was never a legal `Box`. Null safety at zero runtime cost. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**Panic** — The unrecoverable failure: the thread gives up, its stack unwinds, and no caller gets to decide. `unwrap` chooses one on your behalf; an uncaught one leaves the process with exit code **101**, not 1. → [What a panic costs](01_Foundations/what_a_panic_costs/README.md)

**Unwinding** — Walking back up the stack after a panic, running every destructor on the way. It restores *resources* — locks released, files closed — and nothing about the half-finished work. `panic = "abort"` skips it, and no destructor runs at all. → [What a panic costs](01_Foundations/what_a_panic_costs/README.md)

**`catch_unwind`** — Run a closure and get an `Err` back instead of dying if it panics. For FFI boundaries and test harnesses, not for control flow: it cannot catch an abort, and it says nothing about whether your data is still coherent. → [What a panic costs](01_Foundations/what_a_panic_costs/README.md)

**`#[track_caller]`** — An attribute that makes a panic report the *caller's* line rather than the line inside the callee. It is why `unwrap`'s panic names your code instead of `core/src/option.rs`. → [What a panic costs](01_Foundations/what_a_panic_costs/README.md)

**Partial function** — A function undefined over part of its input range (`first()` on an empty list, `sqrt` of a negative). Returning `Option<T>` makes it **total**: "no answer" becomes one of the answers. → [Partial functions](01_Foundations/partial_functions/README.md)

**`checked_*`** — The standard library's total versions of arithmetic that is partial (`checked_div`, `checked_add`, `checked_sub`, `checked_pow`). Same operation, `None` instead of a panic or a wrap. → [Partial functions](01_Foundations/partial_functions/README.md)

**Total function** — One with an answer for every input. The goal `Option` serves: widening the return type is what converts a partial function into a total one. → [Partial functions](01_Foundations/partial_functions/README.md)

**`Option<T>`** — Either `Some(T)` or `None`. Models a value that might not be there, when "why not?" has exactly one possible answer. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`ok`** — Turn a `Result` into an `Option`, discarding the error. What a hand-written `match Ok => Some, Err(_) => None` is re-implementing — and a downgrade whenever the caller could have used the reason. → [Returning `None` on error](01_Foundations/none_on_error/README.md)

**Ownership** — The rule that every value has exactly one owner, and is dropped when that owner goes out of scope. What makes "freed exactly once" true by construction rather than by discipline. → [Ownership and moves](01_Foundations/ownership_and_moves/README.md)

**Move** — Transferring ownership. The bytes do not travel; what changes is who owes the free, and therefore when it happens. The source variable becomes unusable by name. → [Ownership and moves](01_Foundations/ownership_and_moves/README.md)

**`Drop`** — The code that runs when a value's owner goes out of scope. Implementing it is the easiest way to *watch* ownership, since the value announces its own death. → [Ownership and moves](01_Foundations/ownership_and_moves/README.md)

**Partial move** — Moving one field out of a struct, leaving the other fields readable but the struct as a whole unusable. Ownership is tracked per field, not per variable. → [Ownership and moves](01_Foundations/ownership_and_moves/README.md)

**Prelude** — The set of names in scope in every Rust file without an import. `Option`, `Result`, and their variants live there, which is why you write `Some(x)` and not `Option::Some(x)`.

**`?`** — Unwrap the happy value, or return the sad one from the current function — converting the error via `From` on the way out. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`Result<T, E>`** — Either `Ok(T)` or `Err(E)`. Models an operation that might fail, when the caller could reasonably ask *why* it failed. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**Type alias** — A second name for an existing type (`type Result<T> = std::result::Result<T, Error>;`). It creates no new type and no conversion — the compiler expands it before checking anything — so it cannot carry its own trait impls. → [The `Result` you are reading is probably an alias](01_Foundations/result_aliases/README.md)

**`thiserror`** — A crate that derives the `Display`/`Error`/`From` boilerplate for a custom error enum. The library-side counterpart to `anyhow`. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**Sentinel value** — A legal value borrowed to mean "no value" (`0`, `-1`, `""`, `0.0`). What `Option` and `Result` exist to replace, and what a guard silently reintroduces when its branch returns a number instead of an error. → [Zero wins is not zero games](01_Foundations/wrong_guard/README.md)

**`take`** — Swap `None` into an `Option` and hand back what was there. The standard way to move a non-`Copy` value out of a `&mut` field, which the borrow checker otherwise refuses. → [`Option` is a one-item collection](01_Foundations/option_as_collection/README.md)

**`transpose`** — Flip `Option<Result<T, E>>` into `Result<Option<T>, E>` and back. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`unwrap_or`** — Replace `None`/`Err` with a default you supply. The default is an ordinary argument, so it is evaluated on every call, needed or not; and once applied, nothing downstream can tell it from a real value. → [`unwrap_or`](01_Foundations/unwrap_or/README.md)

**`FnOnce`** — The loosest of the three closure traits: callable at most once, and therefore allowed to consume what it captured. Every fallback closure is `FnOnce`, which is why `unwrap_or_else(move || owned)` can hand out an owned value. → [`unwrap_or_else`](01_Foundations/unwrap_or_else/README.md)

**`or_else`** — Try another source and stay inside the wrapper: `Option` → `Option`. The one to reach for when there is a second and third place to look; its neighbour `unwrap_or_else` ends the chain with a plain value instead. → [`unwrap_or_else`](01_Foundations/unwrap_or_else/README.md)

**`unwrap_or_else`** — Replace `None`/`Err` with a value a closure produces, computed only if it is needed. On a `Result` the closure is handed the error, which makes it the only fallback that can salvage a row *and* record why. → [`unwrap_or_else`](01_Foundations/unwrap_or_else/README.md)

**`Default`** — The trait supplying a type's zero value. Derived, it takes every field's own default; written by hand, it states the domain's answer; left unimplemented, it stops `unwrap_or_default()` from compiling, which is often the right outcome. → [`unwrap_or_default`](01_Foundations/unwrap_or_default/README.md)

**`unwrap_or_default`** — Replace `None`/`Err` with `T::default()`. The shortest fallback and the only one whose value is decided somewhere other than the call site — on a `Result`, without even naming the error. → [`unwrap_or_default`](01_Foundations/unwrap_or_default/README.md)

**`mem::take`** — Swap a value out of a `&mut` by leaving `Default::default()` behind. The same trait as `unwrap_or_default`, used for the opposite half of the job. → [`unwrap_or_default`](01_Foundations/unwrap_or_default/README.md)

**`map_or`** — Transform the value, or fall back — in one call, with the fallback written first and run last. `map_or_else` is the lazy pair, and on a `Result` its *error* closure comes first. → [`map_or` and `map_or_else`](01_Foundations/map_or/README.md)

**`is_none_or`** — Ask whether an option is absent *or* its value passes a predicate (Rust 1.82). The name for what `map_or(true, pred)` was doing the long way, as `is_some_and` is for `map_or(false, pred)`. → [`map_or` and `map_or_else`](01_Foundations/map_or/README.md)

**Edition** — The three-yearly opt-in that lets Rust change syntax without breaking old code; a crate names one and they interoperate freely. Worth knowing because `rustc` on its own defaults to **2015**, so a modern file needs `--edition 2024` passed by hand. → [Running a scratch program](01_Foundations/rustc_without_cargo/README.md)

**Binary target** — A compilation unit with a `main`, which Cargo turns into one executable. `src/main.rs`, every `src/bin/*.rs`, and every `[[bin]]` entry in the manifest is one; the auto-discovered ones do not replace the listed ones. → [Running a scratch program](01_Foundations/rustc_without_cargo/README.md)

**`rustc --test`** — Builds the test harness as the entry point instead of your `main`, which is what `cargo test` runs underneath. It works on a loose file, so a single-file example can still have real `#[test]` functions. → [Running a scratch program](01_Foundations/rustc_without_cargo/README.md)

**Cranelift** — An alternative code-generation backend to LLVM, tuned to emit machine code *quickly* rather than to emit quick machine code. Nightly-only and right for `[profile.dev]` alone: on a release profile it hands you a slower binary. → [Compile times](05_Tooling/compile_times/README.md)

**Monomorphization** — Compiling one generic function separately for each concrete type it is used with. It is what makes generics cost nothing at runtime, and it is why a generics-heavy crate spends most of its build in codegen. → [Compile times](05_Tooling/compile_times/README.md)

**`cargo build --timings`** — Writes an HTML chart of how long each crate in the build took and which ones blocked others. The thing to run *before* changing any build setting, because which optimization helps is a property of the project. → [Compile times](05_Tooling/compile_times/README.md)

**`line-tables-only`** — A `[profile.dev] debug` setting keeping just enough DWARF to resolve a backtrace to a file and line, dropping the type and variable information a debugger needs. Cheap to produce and much cheaper to link. → [Compile times](05_Tooling/compile_times/README.md)

**Parallel front end** — rustc's nightly `-Z threads=N`, which multi-threads parsing, type checking and borrow checking. The *back* end has been parallel for years; this is the half that used to leave your other cores idle. → [Compile times](05_Tooling/compile_times/README.md)
