# Glossary

Short definitions. Every entry links to the page that explains it properly — a definition that dead-ends hides the lesson that already exists.

**Algebraic data type (ADT)** — The umbrella term for the two ways Rust builds a compound type. A `struct` is a **product** type: it holds a field *and* a field *and* a field, so its possible values multiply. An `enum` is a **sum** type: it is one variant *or* another, so its values add. Every Rust data model is these two composed — `Option` is a sum of two, a struct of three `Option`s is a product of sums. → [What a struct is](16_Structs/what_a_struct_is/README.md)

**`and_then`** — Transform a value with a closure that can *itself* come up empty or fail, flattening the result instead of nesting it. The counterpart to `map` when the closure returns another `Option`/`Result`. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**`anyhow`** — A crate giving applications one catch-all error type with good ergonomics and backtraces. The application-side counterpart to `thiserror`. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**`as_deref`** — Borrow through an owned inner value: `Option<String>` → `Option<&str>`. The usual fix when a method takes `self` but you still need the option afterwards. → [`unwrap_or`](17_Option_and_Result/unwrap_or/README.md)

**Associated function** — A function in an `impl` block that does *not* take `self`, called as `Type::name(..)`. `Ballot::new` is one; so is `String::from`. Rust has no constructor syntax, so `new` is only a convention. A method is the same thing with `self` as the first parameter. → [What a struct is](16_Structs/what_a_struct_is/README.md)

**Basic block** — A straight-line run of instructions with one entry at the top and one exit at the bottom. The unit an optimizer, a disassembler and an obfuscator all work in: LLVM IR names them `bb1`, `bb2`, and a branch is the only way out of one. → [LLVM and its IR](20_Compilers/llvm_and_its_ir/README.md)

**`Box<dyn Error>`** — A type-erased error: any error can convert into it, so unrelated failures can flow through one function. What applications reach for when nothing downstream will `match` on the cause. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**`Clone`** — An explicit duplicate, via `.clone()`. May allocate and may run your own code, and is always visible in the source — which is the point, since an allocation you can see is one you can question. `#[derive(Clone)]` clones each field, and `Copy` requires it. → [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md)

**`const` evaluation** — Your code, executed by the compiler during the build rather than by your program at run time. A `const fn` used as an array length forces it; anything that would panic becomes a build error instead. → [What a compiler does before your program runs](20_Compilers/what_a_compiler_does/README.md)

**Control-flow flattening** — An obfuscation pass that cuts every basic block loose, numbers it, and parks it under one dispatcher loop driven by a state variable. Behaviour is unchanged and the control-flow graph stops describing the program — which also defeats the decompiler that would have read it for you. → [Control-flow flattening](20_Compilers/control_flow_flattening/README.md)

**Control-flow graph (CFG)** — Basic blocks as nodes, branches as edges. The picture every optimizer pass and every disassembler works from, and the thing an obfuscator exists to make meaningless. → [LLVM and its IR](20_Compilers/llvm_and_its_ir/README.md)

**Discriminant** — The number identifying which variant an enum value currently is; `None` is 0 and `Some` is 1, by declaration order. Comparable via `std::mem::discriminant`, but not extractable — and often not even stored. → [`Option` is a one-item collection](17_Option_and_Result/option_as_collection/README.md)

**Enum** — A type that is exactly one of several named variants, each optionally carrying data. `Option` and `Result` are both ordinary enums; nothing about them is built into the language. → [What an enum is](13_Enums/what_an_enum_is/README.md)

**Field init shorthand** — Writing `Ballot { voter }` instead of `Ballot { voter: voter }` when the variable already has the field's name. Purely cosmetic, and worth knowing because it is what most real code looks like. → [What a struct is](16_Structs/what_a_struct_is/README.md)

**`impl` block** — Where a type's functions live. `impl Ballot { … }` is an *inherent* impl (the signatures are yours); `impl Summary for Ballot { … }` is a *trait* impl (the signatures are the trait's). Not nested in the struct and not limited to structs — enums take them identically. A type may have many. → [`impl` blocks](16_Structs/impl_blocks/README.md)

**`is_some_and`** — Ask whether an option is `Some` *and* its value passes a predicate, without unwrapping. Takes `self`, so pair it with `.as_ref()` for non-`Copy` types. → [`Option` is a one-item collection](17_Option_and_Result/option_as_collection/README.md)

**`is_some` / `is_none`** — Ask which variant an option is, as a `bool`, without opening it. Both take `&self`, so the option survives — which is what makes them the natural predicate for `.filter()` over a collection of options. Reach for `is_some_and` instead the moment the next thing you write is `&& x.unwrap() > …`. → [`Option` is a one-item collection](17_Option_and_Result/option_as_collection/README.md)

**`expect`** — Panic with a message you wrote. Preferred over `unwrap` everywhere, because the message records *why* you believed this could not fail — and being unable to write it is the signal to return a `Result` instead. → [`expect`](17_Option_and_Result/expect/README.md)

**`Infallible`** — An enum with no variants, used as the `E` of a `Result` that cannot fail (`String`'s `FromStr`, `u64::try_from(u32)`). Because `Err` cannot be built, the compiler drops the tag and the `Result` costs what the value costs. It is the stable stand-in for `Result<T, !>`, and `match e {}` — zero arms over zero variants — is how you open one without an `unwrap`. → [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md), [The never type `!`](15_First_Programs/the_never_type/README.md)

**Never type (`!`)** — The type with **zero** values, carried by an expression that does not finish: `panic!()`, `loop {}`, `return`, `break`, `continue`, `process::exit`. Because no value of it can exist, one coerces into a slot of any type — which is why a diverging `match` arm type-checks beside a `u32` while a `println!` arm (type `()`) does not. Stable only after an `->`; `Result<(), !>` is still `E0658`. → [The never type `!`](15_First_Programs/the_never_type/README.md)

**`From`** — The conversion trait. `?` calls it implicitly to turn one error type into the function's own, which is what makes custom error enums and `Box<dyn Error>` ergonomic. → [`From` and `Into`](29_Conversion/from_and_into/README.md), [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**`if let`** — A `match` with only the arm you care about. Sugar for the two-arm version, at the price of the compiler no longer checking that you covered every variant. → [`if let`](17_Option_and_Result/if_let/README.md)

**Exhaustiveness** — The compiler's insistence that a `match` account for every variant, so adding one breaks the builds that now have a hole. `if let`, `while let`, and `matches!` all opt out of it for one expression. → [`if let`](17_Option_and_Result/if_let/README.md)

**Let chain** — Several `let` bindings and conditions joined with `&&` in one `if let` head, each binding visible to the next. Stable since Rust 1.88 and only in edition 2024. → [`if let`](17_Option_and_Result/if_let/README.md)

**Linker** — The separate program that turns object files into one executable, matching each symbol a file *needs* against a definition somewhere else. Not part of rustc, which is why its errors have no error code, no span, and platform-specific wording. → [The linker](20_Compilers/the_linker/README.md)

**LLVM** — Four things under one name: the umbrella project (Clang, LLD, LLDB), the library rustc links, the IR, and the optimizer-plus-generator pipeline. Rust's compiler is a front end on top of it; everything below the IR is shared with C++ and Swift. → [LLVM and its IR](20_Compilers/llvm_and_its_ir/README.md)

**LLVM IR** — A typed, machine-independent assembly language: rustc's output and LLVM's input. The waist of the hourglass — one front end per language above it, one back end per chip below. → [LLVM and its IR](20_Compilers/llvm_and_its_ir/README.md)

**Lock poisoning** — A `Mutex`/`RwLock` remembering that a thread panicked while holding its exclusive guard, so every later `lock()` returns `Err`. Not an error the lock hit: a warning that the invariant behind it may be half-restored. → [Lock poisoning](09_Advanced/mutex_poisoning/README.md)

**Method** — An associated function whose first parameter is `self`, `&self` or `&mut self`, so it can be called with a dot. `b.total()` is sugar for `Ballot::total(&b)`. The receiver you choose decides what the caller keeps: `&self` and `&mut self` hand it back, `self` consumes it. → [`impl` blocks](16_Structs/impl_blocks/README.md)

**Name mangling** — Encoding a function's module path and generic arguments into its symbol, so two functions named `new` produce different symbols. `_RNvCs...` is Rust's v0 scheme; `#[unsafe(no_mangle)]` opts out and is how a Rust function becomes callable from C. → [The linker](20_Compilers/the_linker/README.md)

**Object file** — Machine code plus two lists: the symbols this file defines, and the ones it still needs. Not a program until a linker has filled in the second list. → [The linker](20_Compilers/the_linker/README.md)

**Opaque predicate** — A condition that always takes the same branch for a reason no local analysis can see — `n * (n - 1) % 2 == 0` is always true because one of two consecutive integers is even. Obfuscation uses them to guard code that never runs but cannot be pruned. → [Control-flow flattening](20_Compilers/control_flow_flattening/README.md)

**`PoisonError`** — What a poisoned `lock()` returns. It carries the guard, so nothing is lost — `into_inner()` hands you the data anyway, which makes `.unwrap()` a decision rather than the only option. → [Lock poisoning](09_Advanced/mutex_poisoning/README.md)

**`io::Result<T>`** — Not a different type: a one-line alias for `Result<T, std::io::Error>`. The pattern behind `fmt::Result` and `thread::Result` too. → [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md)

**`let … else`** — Bind a pattern or leave the current scope; the `else` block must diverge. The idiomatic guard clause — it keeps the happy path unindented. → [`if let`](17_Option_and_Result/if_let/README.md)

**`matches!`** — Ask whether a value fits a pattern and get back a `bool`, optionally with a guard. What to write instead of an `if let` whose body only sets a flag. → [`if let`](17_Option_and_Result/if_let/README.md)

**Receiver** — The `self` parameter of a method, and the design decision on every one you write. `&self` reads, `&mut self` changes in place (and forces a `mut` binding at the call site — `E0596`), `self` consumes so the caller cannot use the value again — which is the guarantee, not the obstacle. → [`impl` blocks](16_Structs/impl_blocks/README.md)

**Scrutinee** — The expression a `match` or `if let` is examining. Worth knowing because edition 2024 changed when a temporary built there is dropped. → [`if let`](17_Option_and_Result/if_let/README.md)

**`Self`** — Capitalised, it is the *type* the current `impl` block is for; lowercase `self` is the *value*. `fn new(..) -> Self` returns the type, and keeps working if the type is renamed. → [`impl` blocks](16_Structs/impl_blocks/README.md)

**Shadowing** — Declaring a variable whose name is already in use. The new one hides the old for the rest of the scope and may have a different type, which is what makes `let x = x.unwrap_or(0)` possible; it is not mutation, and the old variable returns when the scope ends. It also does not *drop* anything — the shadowed value stays alive to the end of the scope, nameless. → [Shadowing and `unwrap`](17_Option_and_Result/shadowing_and_unwrap/README.md), [A shadow does not drop](18_Ownership/shadowing_does_not_drop/README.md), [When to shadow](18_Ownership/when_to_shadow/README.md) for whether to reach for it here — the test is whether the new binding is the same concept in a new form, [Nothing checks a shadow](18_Ownership/nothing_checks_a_shadow/README.md) for how little the compiler will do about it if you get it wrong, [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md) for the mechanical difference from `mut` and how to prove it

**Place** — Where a value lives, as distinct from the *name* you reach it by. `mut` gives one name and one place and permits writes into it; a shadow declares a **second** place and moves the name onto it, leaving the first untouched and still borrowable. Conflating the two is the single source of every "is shadowing just `mut`?" confusion, and the reason the popular comparison table gets its memory row backwards. → [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md)

**Scope** — The region of source a name is valid in: from its `let` to the closing brace of the block that declared it. It is what ends a **shadow** — the outer name returns at the brace — while a write through `mut` goes into a place declared elsewhere and so outlives the block it happened in. That asymmetry is the row the usual shadowing-vs-`mut` table omits, and the mechanism behind the accumulator that never accumulates. A separate question from *when the value dies* and from *when a borrow ends* — three answers at three different moments. → [Scope is about names, not values](18_Ownership/scope_is_about_names/README.md), [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md#the-row-the-table-is-missing), [When to shadow](18_Ownership/when_to_shadow/README.md), [A shadow does not drop](18_Ownership/shadowing_does_not_drop/README.md)

**Non-lexical lifetimes (NLL)** — The borrow checker measuring a borrow from the `&` to its **last use**, not to the end of the block. Since it arrived with the 2018 edition, `let r = &v; println!("{r}"); v.push(x);` compiles — so most pre-2018 advice about "adding a block to end the borrow early" is now unnecessary work. The block is still right when several lines share the borrow, and still required when the reference is returned or stored. → [Scope is about names, not values](18_Ownership/scope_is_about_names/README.md), [Borrowing](18_Ownership/borrowing/README.md)

**`let_underscore_lock`** — A `rustc` lint, **deny by default**, that refuses `let _ = mutex.lock().unwrap();` outright: the guard binds to nothing and is dropped on the spot, leaving an unlocked critical section that reads exactly like a locked one. It is a special case for std's locks, not for the pattern — a hand-rolled RAII guard released the same way draws no diagnostic at all. → [Scope is about names, not values](18_Ownership/scope_is_about_names/README.md), [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md)

**Item scope** — `fn`, `struct`, `const` and `use` are in scope for the **entire** enclosing block, including above the line that declares them, which is why a nested `fn` can be called before it is written and a `let` cannot. A binding's scope starts *after* its own initializer — the mechanism that makes `let x = x + 1;` read the previous `x` instead of itself. → [Scope is about names, not values](18_Ownership/scope_is_about_names/README.md)

**Binding mutability** — `mut` is a property of the **binding**, never of the value: values are not mutable or immutable, handles to them are. `let s = …; let mut s = s;` moves one `String` into a mutable binding and mutates it, and `let s = s;` freezes it again — one value, three bindings, two answers. So "`let mut` means mutable data" is the introductory framing to unlearn first. → [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md)

**`E0506`** — *cannot assign to `x` because it is borrowed.* The write half of the borrow rule, and the cleanest proof that a shadow is not an assignment: the same four lines compile with `let` and are refused with `mut`. Its neighbour `E0505` covers a *move* out of a borrowed value rather than a write into it. → [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md), [Borrowing](18_Ownership/borrowing/README.md)

**Struct** — A type that names a group of values, each field with its own type. Three flavors: named-field, tuple struct, and unit struct. It holds *no* behaviour — methods live in a separate `impl` block, shared behaviour in traits — which is why Rust has no classes and no inheritance. → [What a struct is](16_Structs/what_a_struct_is/README.md)

**Struct update syntax** — `Config { retries: Some(3), ..Default::default() }` — fill the fields you name, take the rest from another value. It **moves** the fields you did not name, one at a time, so a non-`Copy` field leaves the original *partially* dead — the compiler names the field, not the value. → [Struct update syntax](16_Structs/struct_update/README.md)

**Tuple struct** — A struct whose fields are numbered rather than named: `struct Precinct(u32);`, reached as `.0`. Really a named-field struct whose names are digits (`Precinct { 0: 7 }` compiles). A private field makes its *constructor* private too, which is what the newtype pattern relies on. → [What a struct is](16_Structs/what_a_struct_is/README.md)

**Unit struct** — A struct with no fields at all: `struct Sealed;`, and the type has exactly one value. Since it holds no data, behaviour is the only thing it can hold, which is the point. Not the same as `struct Sealed {}`, which must be built with braces. → [What a struct is](16_Structs/what_a_struct_is/README.md)

**Value namespace** — Rust resolves types and values in separate namespaces, and functions live in the *value* one — so `let score = score();` shadows the function `score` and makes it uncallable for the rest of the scope. `error[E0618]` says so explicitly, and it is the only shadowing mistake the compiler names out loud. → [When to shadow](18_Ownership/when_to_shadow/README.md)

**`clippy::shadow_unrelated`** — An allow-by-default lint flagging a shadow whose new value is not derived from the old one — the one case where a reused name means two different things. Its siblings `shadow_same` and `shadow_reuse` ban the freeze and parse-and-narrow idioms respectively, so they are rarely worth turning on — with the sting that `shadow_reuse` is the only one of the three that catches a shadowed accumulator. → [When to shadow](18_Ownership/when_to_shadow/README.md), and all three run against one file in [Nothing checks a shadow](18_Ownership/nothing_checks_a_shadow/README.md)

**`restriction` (clippy lint group)** — Lints that forbid something legal and idiomatic, for codebases that have decided against it. Allow-by-default and *meant* to stay that way, unlike `correctness` or `suspicious` — so finding a lint here is clippy saying "this is a style commitment, not a bug filter." All three shadow lints live in it. → [Nothing checks a shadow](18_Ownership/nothing_checks_a_shadow/README.md)

**Pattern binding** — The name a pattern introduces, as `x` in `Some(x)`. A fresh name rather than a shadow — and for a non-`Copy` type it *moves* the value out of what you matched on, unless you borrow. → [Shadowing and `unwrap`](17_Option_and_Result/shadowing_and_unwrap/README.md)

**Or-pattern** — Alternatives joined by `|` inside a single pattern, so one arm accepts several shapes: `8 | 12 | 18`. Not the bitwise `|`, which is what the same characters mean in an *expression*; every alternative must bind the same names at the same types, and rustc checks each alternative separately for reachability. → [One arm, many values](17_Option_and_Result/one_arm_many_values/README.md)

**Range pattern** — A span as a pattern: `0..=7` inclusive, `0..7` exclusive. Composes with `|` (`9..=11 | 13..=17`), and two ranges left exactly one value apart are reported by the default-on `non_contiguous_range_endpoints` lint. → [One arm, many values](17_Option_and_Result/one_arm_many_values/README.md)

**`unreachable_patterns`** — The warn-by-default lint for a `match` arm no value can reach, because an earlier arm already covers it. Fires per *alternative* rather than per arm, so a too-wide range above an or-pattern names the one alternative it swallowed. → [One arm, many values](17_Option_and_Result/one_arm_many_values/README.md)

**`Cow<'a, B>`** — Clone-on-write: an enum with a `Borrowed(&'a B)` arm and an `Owned(<B as ToOwned>::Owned)` arm, so a function can return the caller's own bytes when it changed nothing and a fresh buffer when it did. `to_mut()` is the write that promotes one to the other. Costs no more than the owned type — `Cow<str>` is 24 bytes, the same as `String`. → [`Cow`: borrow until somebody writes](18_Ownership/clone_on_write/README.md)

**`Copy`** — The trait marking a type that is duplicated instead of moved on assignment (`i32`, `bool`, `char`, `&T`, and `Option`s of them). A `String` cannot be `Copy`, because two owners of one allocation would mean two frees — so the test is not size but whether duplicating the bytes would duplicate an *obligation*. → [Ownership and moves](18_Ownership/ownership_and_moves/README.md), and why `if let Some(n) = opt` leaves `opt` usable for an `Option<i32>` but not an `Option<String>` → [Shadowing and `unwrap`](17_Option_and_Result/shadowing_and_unwrap/README.md), and how it differs from `Clone` → [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md)

**`while let`** — Loop for as long as the pattern keeps matching; the `None` is the exit condition. Nothing checks that the body moves toward it, so the scrutinee has to consume. → [`while let`](17_Option_and_Result/while_let/README.md)

**`map`** — Transform the value inside a wrapper, leaving the wrapper alone. Nests rather than flattens if the closure returns another wrapper — that is when you want `and_then`. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**`#[must_use]`** — An attribute making the compiler warn when a returned value is discarded. Both `Option` and `Result` carry it, which is why an ignored error is a warning rather than a silent bug. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**`Ok(())`** — Success carrying no value, because `()` has exactly one value and zero size. What every `Display` impl and every `fn main() -> Result<(), E>` ends with. → [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md)

**Non-lexical lifetimes** — A borrow lives until its **last use**, not to the end of its block. Why the same two statements compile in one order and are `E0502` in the other. → [Borrowing](18_Ownership/borrowing/README.md)

**Shared reference (`&T`)** — Access without ownership, held by any number of readers at once. "Shared", not "immutable": a `Cell` or `Mutex` still mutates through one. → [Borrowing](18_Ownership/borrowing/README.md)

**Exclusive reference (`&mut T`)** — The only reference to a value while it lives, and it excludes readers too — including the owner. → [Borrowing](18_Ownership/borrowing/README.md)

**Niche** — A bit pattern a type can never legally hold (null for a `Box`, any byte but 0/1 for a `bool`). `None` takes the niche when one exists, which is why the wrapper is often free. → [`Option` is a one-item collection](17_Option_and_Result/option_as_collection/README.md)

**Exhaustiveness** — A `match` must cover every variant, or it does not compile. The value is not the check itself but what it does later: add a variant and every match that ignores it becomes a build error, so the list of places to revisit is computed rather than remembered. A `_` arm opts out permanently — it is a promise that every *future* variant belongs in that bucket. → [Six kinds of zero](17_Option_and_Result/six_kinds_of_zero/README.md)

**Sum type (tagged union)** — A type that is exactly one of several alternatives, each free to carry different data. `Option` is one with two variants and no special powers; when a problem has six cases, writing your own is the idiomatic move, not a departure. → [Six kinds of zero](17_Option_and_Result/six_kinds_of_zero/README.md)

**Null-pointer optimization** — The niche rule applied to pointers: `Option<Box<T>>` is the same size as `Box<T>`, because null was never a legal `Box`. Null safety at zero runtime cost. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**Panic** — The unrecoverable failure: the thread gives up, its stack unwinds, and no caller gets to decide. `unwrap` chooses one on your behalf; an uncaught one leaves the process with exit code **101**, not 1. → [What a panic costs](17_Option_and_Result/what_a_panic_costs/README.md)

**Unwinding** — Walking back up the stack after a panic, running every destructor on the way. It restores *resources* — locks released, files closed — and nothing about the half-finished work. `panic = "abort"` skips it, and no destructor runs at all. → [What a panic costs](17_Option_and_Result/what_a_panic_costs/README.md)

**`catch_unwind`** — Run a closure and get an `Err` back instead of dying if it panics. For FFI boundaries and test harnesses, not for control flow: it cannot catch an abort, and it says nothing about whether your data is still coherent. → [What a panic costs](17_Option_and_Result/what_a_panic_costs/README.md)

**`#[track_caller]`** — An attribute that makes a panic report the *caller's* line rather than the line inside the callee. It is why `unwrap`'s panic names your code instead of `core/src/option.rs`. → [What a panic costs](17_Option_and_Result/what_a_panic_costs/README.md)

**Partial function** — A function undefined over part of its input range (`first()` on an empty list, `sqrt` of a negative). Returning `Option<T>` makes it **total**: "no answer" becomes one of the answers. → [Partial functions](17_Option_and_Result/partial_functions/README.md)

**`checked_*`** — The standard library's total versions of arithmetic that is partial (`checked_div`, `checked_add`, `checked_sub`, `checked_pow`). Same operation, `None` instead of a panic or a wrap. → [Partial functions](17_Option_and_Result/partial_functions/README.md)

**Total function** — One with an answer for every input. The goal `Option` serves: widening the return type is what converts a partial function into a total one. → [Partial functions](17_Option_and_Result/partial_functions/README.md)

**`Option<T>`** — Either `Some(T)` or `None`. Models a value that might not be there, when "why not?" has exactly one possible answer. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**`ok`** — Turn a `Result` into an `Option`, discarding the error. What a hand-written `match Ok => Some, Err(_) => None` is re-implementing — and a downgrade whenever the caller could have used the reason. → [Returning `None` on error](17_Option_and_Result/none_on_error/README.md)

**Ownership** — The rule that every value has exactly one owner, and is dropped when that owner goes out of scope. What makes "freed exactly once" true by construction rather than by discipline. → [Ownership and moves](18_Ownership/ownership_and_moves/README.md)

**Move** — Transferring ownership. The bytes do not travel; what changes is who owes the free, and therefore when it happens. The source variable becomes unusable by name. → [Ownership and moves](18_Ownership/ownership_and_moves/README.md)

**`Drop`** — The code that runs when a value's owner goes out of scope. Implementing it is the easiest way to *watch* ownership, since the value announces its own death. It cannot return anything, cannot fail, and is not guaranteed to run — leaking is safe. → [`Drop`, and what RAII buys](12_Traits/drop_and_raii/README.md), [Ownership and moves](18_Ownership/ownership_and_moves/README.md)

**Drop order** — Within a scope, *locals* drop in **reverse declaration order** — but a struct's **fields** drop in **declaration order**, so two values that died in the right sequence as locals flip the moment you move them into one struct, with no diagnostic. Six ordinary things move a drop off the scope end entirely: a move, `drop(x)`, a temporary (end of statement), `let _ =` (immediately), being a field of something else, and an **assignment**, which frees whatever the location was holding before storing the new value. The consequence people miss: a shadowed value is declared *before* the shadow, so it dies *after* it — which is why shadowing a lock guard leaves the first lock held. → [Scope is about names, not values](18_Ownership/scope_is_about_names/README.md), [Assignment drops the old value](18_Ownership/assignment_is_a_drop/README.md), [A shadow does not drop](18_Ownership/shadowing_does_not_drop/README.md), [When to shadow](18_Ownership/when_to_shadow/README.md)

**Drop flag** — A hidden boolean `rustc` puts in a *stack frame* — never in the value — when a location may or may not still hold something at the closing brace, so the drop can be decided at run time. It is what makes a conditional move legal, why a partly-moved struct still drops its remaining fields, and why a struct with its own `Drop` cannot be split at all (`E0509`): there would be no whole value to hand `drop(&mut self)`. Before 1.0 the flag lived inside the value, which is why `size_of` is now the evidence that it does not. → [The drop flag](18_Ownership/the_drop_flag/README.md)

**Dangling reference** — A reference that outlives the value it points at. Rust makes it unwriteable (`E0505` when a borrowed value would be freed, `E0106` when a function tries to return one, `E0515` when the body tries to return one to a local); C and C++ compile the same shape silently. → [A shadow does not drop](18_Ownership/shadowing_does_not_drop/README.md), and the stack version — a pointer into a frame that has been reissued — in [A stack slot is reused](18_Ownership/a_stack_slot_is_reused/README.md)

**Stack frame** — The region a single call is given for its parameters and locals, reserved on entry and released whole on return. Arguments are *moved into* it, so a parameter is a local of that call; a returned value moves *out* of it into a slot the caller provided. Whether it exists at all is a property of one build — a small value can stay in a register, and the optimizer can inline four calls into one frame — while the lifetimes the borrow checker enforces are a property of the program. → [The call stack](18_Ownership/the_call_stack/README.md)

**Stack overflow** — Running out of the fixed region a thread was given, almost always by recursing deeper than the data bounds. Rust detects it with a guard page and **aborts**: no unwinding, no `Drop`, and `catch_unwind` does not see it, because there is no stack left to run a destructor on. The size is chosen when the thread is spawned (`thread::Builder::stack_size`, or `RUST_MIN_STACK`) and never grows. → [Recursion and the size of the stack](18_Ownership/recursion_and_the_stack/README.md)

**Undefined behaviour** — A program the language standard declines to define at all, so no output is the "right" one. The reason C's use-after-free can print nothing on one run and the correct answer on the next, and the reason such a program can never have a recorded answer key. → [C and C++](31_C_and_Cpp/README.md) · [A shadow does not drop](18_Ownership/shadowing_does_not_drop/README.md)

**Partial move** — Moving one field out of a struct, leaving the other fields readable but the struct as a whole unusable. Ownership is tracked per field, not per variable. → [Ownership and moves](18_Ownership/ownership_and_moves/README.md)

**Prelude** — The set of names in scope in every Rust file without an import. `Option`, `Result`, and their variants live there, which is why you write `Some(x)` and not `Option::Some(x)`.

**`?`** — Unwrap the happy value, or return the sad one from the current function — converting the error via `From` on the way out. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**`Result<T, E>`** — Either `Ok(T)` or `Err(E)`. Models an operation that might fail, when the caller could reasonably ask *why* it failed. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**Type alias** — A second name for an existing type (`type Result<T> = std::result::Result<T, Error>;`). It creates no new type and no conversion — the compiler expands it before checking anything — so it cannot carry its own trait impls. → [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md)

**`thiserror`** — A crate that derives the `Display`/`Error`/`From` boilerplate for a custom error enum. The library-side counterpart to `anyhow`. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**Sentinel value** — A legal value borrowed to mean "no value" (`0`, `-1`, `""`, `0.0`). What `Option` and `Result` exist to replace, and what a guard silently reintroduces when its branch returns a number instead of an error. → [Zero wins is not zero games](17_Option_and_Result/wrong_guard/README.md)

**`take`** — Swap `None` into an `Option` and hand back what was there. The standard way to move a non-`Copy` value out of a `&mut` field, which the borrow checker otherwise refuses. → [`Option` is a one-item collection](17_Option_and_Result/option_as_collection/README.md)

**`transpose`** — Flip `Option<Result<T, E>>` into `Result<Option<T>, E>` and back. → [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md)

**`unwrap_or`** — Replace `None`/`Err` with a default you supply. The default is an ordinary argument, so it is evaluated on every call, needed or not; and once applied, nothing downstream can tell it from a real value. → [`unwrap_or`](17_Option_and_Result/unwrap_or/README.md)

**`FnOnce`** — The loosest of the three closure traits, and the one *every* closure implements: callable at most once, and therefore allowed to consume what it captured. Every fallback closure is `FnOnce`, which is why `unwrap_or_else(move || owned)` can hand out an owned value. → [The three closure traits](23_Closures/three_closure_traits/README.md)

**`or_else`** — Try another source and stay inside the wrapper: `Option` → `Option`. The one to reach for when there is a second and third place to look; its neighbour `unwrap_or_else` ends the chain with a plain value instead. → [`unwrap_or_else`](17_Option_and_Result/unwrap_or_else/README.md)

**`unwrap_or_else`** — Replace `None`/`Err` with a value a closure produces, computed only if it is needed. On a `Result` the closure is handed the error, which makes it the only fallback that can salvage a row *and* record why. → [`unwrap_or_else`](17_Option_and_Result/unwrap_or_else/README.md)

**`Default`** — The trait supplying a type's zero value. Derived, it takes every field's own default; written by hand, it states the domain's answer; left unimplemented, it stops `unwrap_or_default()` from compiling, which is often the right outcome. → [`unwrap_or_default`](17_Option_and_Result/unwrap_or_default/README.md)

**`unwrap_or_default`** — Replace `None`/`Err` with `T::default()`. The shortest fallback and the only one whose value is decided somewhere other than the call site — on a `Result`, without even naming the error. → [`unwrap_or_default`](17_Option_and_Result/unwrap_or_default/README.md)

**`mem::take`** — Swap a value out of a `&mut` by leaving `Default::default()` behind. The same trait as `unwrap_or_default`, used for the opposite half of the job. → [`unwrap_or_default`](17_Option_and_Result/unwrap_or_default/README.md)

**`map_or`** — Transform the value, or fall back — in one call, with the fallback written first and run last. `map_or_else` is the lazy pair, and on a `Result` its *error* closure comes first. → [`map_or` and `map_or_else`](17_Option_and_Result/map_or/README.md)

**`is_none_or`** — Ask whether an option is absent *or* its value passes a predicate (Rust 1.82). The name for what `map_or(true, pred)` was doing the long way, as `is_some_and` is for `map_or(false, pred)`. → [`map_or` and `map_or_else`](17_Option_and_Result/map_or/README.md)

**Edition** — The three-yearly opt-in that lets Rust change syntax without breaking old code; a crate names one and they interoperate freely. Worth knowing because `rustc` on its own defaults to **2015**, so a modern file needs `--edition 2024` passed by hand. The keyword list is one of the things an edition sets — `async` from 2018, `gen` from 2024 — so `r#` exists to keep the older crate callable. → [Running a scratch program](15_First_Programs/rustc_without_cargo/README.md), [Raw identifiers `r#`](15_First_Programs/raw_identifiers/README.md)

**Raw identifier (`r#`)** — `r#type`, `r#match`, `r#async`: the escape that lets a keyword be an ordinary name, suggested by rustc itself the first time a name collides. It escapes the *parser*, not the name — `r#ordinary` and `ordinary` are one item — and `crate`, `self`, `Self` and `super` refuse it, being path roots rather than keywords. Lifetimes take it as `'r#fn`. Not the raw *string* `r#"…"#`, which is an unrelated feature sharing a letter. → [Raw identifiers `r#`](15_First_Programs/raw_identifiers/README.md)

**Binary target** — A compilation unit with a `main`, which Cargo turns into one executable. `src/main.rs`, every `src/bin/*.rs`, and every `[[bin]]` entry in the manifest is one; the auto-discovered ones do not replace the listed ones. → [Running a scratch program](15_First_Programs/rustc_without_cargo/README.md)

**`rustc --test`** — Builds the test harness as the entry point instead of your `main`, which is what `cargo test` runs underneath. It works on a loose file, so a single-file example can still have real `#[test]` functions. → [Running a scratch program](15_First_Programs/rustc_without_cargo/README.md)

**Cranelift** — An alternative code-generation backend to LLVM, tuned to emit machine code *quickly* rather than to emit quick machine code. Nightly-only and right for `[profile.dev]` alone: on a release profile it hands you a slower binary. → [Compile times](05_Tooling/compile_times/README.md)

**Monomorphization** — Compiling one generic function separately for each concrete type it is used with. It is what makes generics cost nothing at runtime, and it is why a generics-heavy crate spends most of its build in codegen. → [Compile times](05_Tooling/compile_times/README.md)

**`cargo build --timings`** — Writes an HTML chart of how long each crate in the build took and which ones blocked others. The thing to run *before* changing any build setting, because which optimization helps is a property of the project. → [Compile times](05_Tooling/compile_times/README.md)

**`line-tables-only`** — A `[profile.dev] debug` setting keeping just enough DWARF to resolve a backtrace to a file and line, dropping the type and variable information a debugger needs. Cheap to produce and much cheaper to link. → [Compile times](05_Tooling/compile_times/README.md)

**Parallel front end** — rustc's nightly `-Z threads=N`, which multi-threads parsing, type checking and borrow checking. The *back* end has been parallel for years; this is the half that used to leave your other cores idle. → [Compile times](05_Tooling/compile_times/README.md)

**`rustfmt`** — The formatter that ships with the toolchain and applies the community style guide. It works on a **whole file** and has no fragment mode, which is the source of every surprise around it: an IDE asked to reformat a selection cannot use it and silently falls back to its own formatter. → [Formatting](05_Tooling/formatting/README.md)

**`cargo fmt -- --check`** — The CI form: writes nothing, prints the diff it would have applied, and exits non-zero. What turns a formatting *preference* into a fact about the repository — without it, whoever last opened a file decides how it looks. → [Formatting](05_Tooling/formatting/README.md)

**`#[rustfmt::skip]`** — An attribute exempting one item from formatting, for the rare block whose hand-alignment carries meaning: a matrix, a table of constants. Deliberately per-item — the global alternative is a settings argument with no end. → [Formatting](05_Tooling/formatting/README.md)

**Toolchain shim** — The small stand-in `rustup` puts on your `PATH` under the names `rustc`, `cargo` and friends. It resolves which real toolchain this invocation wants and execs it, which is why every version pin only works for callers that go *through* it — an absolute path to the real binary silently ignores all of them. → [rustup](05_Tooling/rustup/README.md)

**Channel** — Which stream of Rust a toolchain follows: `stable` (a release every six weeks), `beta` (the next stable, early), or `nightly` (built from master, where unstable features are permitted). A dated form, `nightly-2026-08-11`, freezes one night — the only form that identifies a compiler. → [rustup](05_Tooling/rustup/README.md), [Nightly by default](05_Tooling/nightly/README.md)

**`rust-toolchain.toml`** — A file naming the toolchain a project needs, which rustup installs and uses automatically. Fourth of rustup's five precedence rungs and the only one that travels with the code. `channel = "stable"` pins nothing, because stable moves; write the version number. → [Pinning the toolchain](05_Tooling/pinning_the_toolchain/README.md)

**Caret requirement** — What a bare version string in `Cargo.toml` means: `rayon = "1.12.0"` accepts anything from 1.12.0 up to, but not including, 2.0.0. A range, not a version — `Cargo.lock` records which member of the range you actually built. → [Adding a dependency](05_Tooling/cargo_dependencies/README.md)

**MSRV** — Minimum Supported Rust Version, declared as `rust-version` in a crate's manifest and printed by `cargo info`. A dependency's MSRV becomes your project's floor, which is what makes it worth reading before `cargo add` rather than after. → [Adding a dependency](05_Tooling/cargo_dependencies/README.md)

**Lint priority** — The `priority = -1` on a `[lints.clippy]` group entry, making the whole group apply before the individual lints beneath it so those can still override a member. Without it Cargo rejects the manifest rather than resolving the conflict. → [Strict clippy lints](05_Tooling/strict_lints/README.md)

**Nix** — A package manager treating a build as a pure function of its inputs, so identical inputs give identical outputs on any machine and incompatible versions coexist without conflict. The engine under `devenv`, and the actual cost of adopting it. → [devenv](05_Tooling/devenv/README.md)

**devenv** — Cachix's front end to Nix: one `devenv.nix` declares a project's compiler, CLI tools, system libraries, environment variables and running services, reproduced identically wherever it is entered. Worth its price when your project needs *services*; overkill when a pinned compiler would have done. → [devenv](05_Tooling/devenv/README.md)

**Workspace inheritance** — Declaring something once in a workspace's root manifest and having members pick it up: `[workspace.lints]` (taken wholesale with `[lints] workspace = true`, which `cargo new` writes for you) and `[workspace.dependencies]` (opted into per crate, so a package still declares what it uses). The alternative is copying config into every project and watching it drift. → [A tree of practice projects](05_Tooling/practice_workspace/README.md)

**bacon** — A background code checker: it watches the files and re-runs `cargo check`, clippy or the tests into a pane you leave open, with `c` and `t` to switch. No config and no project changes, which makes it the cheapest tool in the toolchain. → [bacon](05_Tooling/bacon/README.md)

**`black_box`** — `std::hint::black_box`, which hides a value from the optimiser so it cannot notice your benchmark's answer is a constant and delete the work. Without it you time an empty loop and conclude the code is infinitely fast. → [cargo-nextest](05_Tooling/nextest/README.md), and used throughout [Compile times](05_Tooling/compile_times/README.md)

**Process-per-test** — nextest's model: each test runs in its own process rather than as a thread in a shared one. A test that *aborts* becomes one reported failure instead of killing the run, and tests cannot leak globals into each other. The cost is that doctests are not supported. → [cargo-nextest](05_Tooling/nextest/README.md)

**Typestate** — Encoding what stage a value has reached into its *type*, so that operations valid only at one stage do not exist at the others. An unauthenticated request and an authenticated one become different types rather than one type with a boolean. → [The right to post is a value](09_Advanced/one_account_one_review/README.md)

**Consuming method** — A method taking `self` by value rather than `&self`, so calling it moves the receiver and the caller cannot use it again. Turns "at most once" from a rule you enforce into one the borrow checker enforces. → [The right to post is a value](09_Advanced/one_account_one_review/README.md)

**Extractor** — A web-framework type built from the incoming request before the handler runs, which fails the request instead of returning if it cannot be built. A handler's argument list is therefore its access-control policy; `rocket` calls the same idea a *request guard*. → [The right to post is a value](09_Advanced/one_account_one_review/README.md)

**Scaled integer (fixed-point)** — Carrying an exact fractional value as an integer count of some fixed unit `1/l`, chosen before the computation starts, so no division ever happens during it. Exact wherever every denominator in play divides `l` — which has to be checked, not assumed. → [Scale the denominator away](09_Advanced/scaled_integers/README.md)

**`i128`** — A 128-bit signed integer, an ordinary primitive with no crate and no allocation behind it: 16 bytes, `Copy`, two registers. Exact under `+ − ×` up to a ceiling of 39 digits, no more exact under `÷` than an `i64`, and the widest Rust has — there is no `i256` to escape into. → [What `i128` is exact about](09_Advanced/i128_exactness/README.md)

**Overflow checks** — The debug-build panic on integer overflow, absent from release builds, where the same expression wraps instead. The reason arithmetic whose range you have not proved should say which it wants: `checked_*`, `saturating_*`, `wrapping_*` or `overflowing_*`. → [Scale the denominator away](09_Advanced/scaled_integers/README.md)

**`black_box`** — A hint that stops the optimizer reasoning about a value, so a benchmark measures the code rather than LLVM's ability to delete it. Without it a loop over constants can compile to nothing and time at zero. → [Scale the denominator away](09_Advanced/scaled_integers/README.md)

**Closure (under an operation)** — Whether applying an operation to two values of a type always yields a value *of that type*. Integers are closed under `+ − ×` and not under `÷`, which is why a wider integer buys range but never makes division exact. → [What `i128` is exact about](09_Advanced/i128_exactness/README.md)

**`__divti3`** — The compiler-rt routine an `i128` division compiles to. Neither x86-64 nor aarch64 has a 128-bit divide instruction, so `/` and `%` on `i128` are a function call rather than an instruction — the one operation where widening is not close to free. → [What `i128` is exact about](09_Advanced/i128_exactness/README.md)

**Cross-multiplication** — Comparing `a/b` against `c/d` as `a*d` against `c*b`, so the ranking is exact because no division happens. The usual fix when integer division has collapsed distinct values onto one; it trades truncation for a product that needs headroom. → [What `i128` is exact about](09_Advanced/i128_exactness/README.md)

**Arbitrary precision** — A number that grows to fit its value rather than overflowing, as Python's `int` does. Not the same property as exactness: `i128` is exact and bounded, Python's `int` is exact and unbounded, and the cost of the second is that operations get slower as the value gets wider. → [What `i128` is exact about](09_Advanced/i128_exactness/README.md)

**`num_rational::Ratio`** — Rust's rational type, a numerator and denominator reduced by `gcd` after each operation. `Ratio<i128>` is the closest thing to Python's `fractions.Fraction`, with the difference that matters: it has a ceiling, and the `gcd` is what buys the range rather than overhead on top of it. → [What `i128` is exact about](09_Advanced/i128_exactness/README.md)

**Arbitrary precision** — Integers that grow to fit their value instead of wrapping or failing at a fixed width. Python's `int` is one and its `Fraction` inherits it for free; Rust's standard library has none, so exactness beyond `i128` means a crate (`num-bigint`, `num-rational`) and an allocation per value. → [When the denominators compound](09_Advanced/compounding_weights/README.md)

**lcm addition** — Adding two fractions over `lcm(b, d)` rather than `b*d`. Identical answer, far smaller intermediate — which decides whether a fixed-width rational survives a long computation, because what overflows is usually the product being reduced away, not the result. → [When the denominators compound](09_Advanced/compounding_weights/README.md)

**NaN** — The floating-point value for "no numeric value can be determined" (`0.0/0.0`, `sqrt(-1)`). It is not equal to itself and is neither less than nor greater than anything, which is the single reason `f64` cannot implement `Eq` or `Ord`. → [What a float actually stores](19_Numbers/what_a_float_stores/README.md)

**Total order vs partial order** — `Ord` promises that any two values compare as exactly one of `<`, `==`, `>`; `PartialOrd` admits that some pairs have no answer and returns `Option<Ordering>`. Floats get only the partial one, so `.sort()`, `sort_by_key` and `HashMap` keys are closed to them by the compiler rather than by convention. → [What a float actually stores](19_Numbers/what_a_float_stores/README.md)

**`total_cmp`** — `f64`'s escape hatch: IEEE 754's totalOrder as an `Ordering`, so `sort_by(f64::total_cmp)` never panics. Worth reading twice before use — it gives NaN a defined seat in the ranking rather than excluding it. → [What a float actually stores](19_Numbers/what_a_float_stores/README.md)

**`f64::EPSILON`** — The gap between 1.0 and the next representable float (about 2.2e-16). Not a general-purpose comparison tolerance: it is far too small for large magnitudes and needlessly generous for tiny ones, so pick a tolerance from the problem instead. → [What a float actually stores](19_Numbers/what_a_float_stores/README.md)

**Reassociation** — Regrouping a chain of `+` or `*` — `(a+b)+(c+d)` where you wrote `((a+b)+c)+d`. Valid on real numbers and not on floats, where each grouping rounds differently, so the compiler is forbidden from doing it to your arithmetic. It is also what unlocks vectorizing a sum, which is why there is an opt-in. → [Letting the compiler reorder a float sum](19_Numbers/letting_the_compiler_reorder/README.md)

**Algebraic float methods** — `algebraic_add`, `_sub`, `_mul`, `_div`, `_rem` on `f32`/`f64`, stable since Rust 1.98: the same arithmetic, marked as safe to reassociate. Permission rather than instruction — an unoptimized build reorders nothing — and the return is always a real float, never undefined behavior. What you give up is knowing which of the legal answers you got. → [Letting the compiler reorder a float sum](19_Numbers/letting_the_compiler_reorder/README.md)

**`-ffast-math`** — The C/C++ flag the algebraic methods are compared to, and the comparison is a contrast: it applies to a whole translation unit rather than one operation, and it bundles a *claim* that NaN and infinity never occur, so a program that meets one gets undefined behavior. Rust's version makes no claim about your data. → [Letting the compiler reorder a float sum](19_Numbers/letting_the_compiler_reorder/README.md)

**Byte** — Eight bits, and the smallest thing in memory with an address of its own; `u8` in Rust, and the unit `size_of` reports in. Bytes of other widths existed historically and Rust cannot express them — there is no `CHAR_BIT`. → [Meet the byte](19_Numbers/meet_the_byte/README.md)

**Byte literal** — `b'F'` is a `u8` (70) and `b"Sw"` is a `&[u8; 2]`, distinct from `'F'` (a `char`, four bytes) and `"Sw"` (a UTF-8 `&str`). The separate syntax exists because, unlike C, Rust's `char` is not the byte. → [Meet the byte](19_Numbers/meet_the_byte/README.md)

**Char boundary** — A byte offset in a `&str` that starts a character rather than landing inside one. Slicing to a non-boundary compiles and panics, which is why `is_char_boundary` exists and why indexing a string by an integer does not compile at all. → [Meet the byte](19_Numbers/meet_the_byte/README.md)

**Endianness** — The order the bytes of a multi-byte number are stored in: big-endian puts the most significant first, little-endian the least. A single byte has none. Name it explicitly at any boundary with `to_be_bytes` / `to_le_bytes` rather than letting `to_ne_bytes` bake in this CPU's preference. → [Meet the byte](19_Numbers/meet_the_byte/README.md)

**Fat pointer** — A reference carrying a second word beside the address: `&str` and `&[T]` add a length (16 bytes on a 64-bit target), `&dyn Trait` adds a vtable pointer. It is why `size_of::<&str>()` is not 8. → [Arrays and slices](26_Collections/arrays_and_slices/README.md), [Meet the byte](19_Numbers/meet_the_byte/README.md)

**Shift masking** — With overflow checks off, `a << b` uses `b` modulo the type's bit width, so `1u8 << 8` is `1u8 << 0` — the same expression that panics in a debug build silently returns a wrong answer in release. `checked_shl` is the honest form whenever the shift amount is not a visible literal. → [Meet the byte](19_Numbers/meet_the_byte/README.md)

**Hexadecimal** — Base 16, and the spelling bit patterns are written in because 16 is 2⁴: one digit is exactly four bits, so a byte is exactly two digits and the boundary between bytes never falls inside a character. Base 10 has no such correspondence; base 8 has one at the wrong granularity, since 3 does not divide 8. → [Why hexadecimal](19_Numbers/why_hexadecimal/README.md)

**Nibble** — Four bits, half a byte, and exactly one hex digit. A byte's two hex digits are literally its two nibbles: `b >> 4` is the left one, `b & 0x0F` the right. → [Why hexadecimal](19_Numbers/why_hexadecimal/README.md)

**Radix** — The base a numeral is written in. It is the *second argument* to `from_str_radix`, not something a prefix in the string can convey — which is why `u8::from_str_radix("0xff", 16)` is an error rather than a courtesy. → [Why hexadecimal](19_Numbers/why_hexadecimal/README.md)

**Numeric literal prefix (`0x` / `0b` / `0o`)** — Rust's four literal spellings — `0xBE`, `0b1011_1110`, `0o276`, `190` — all producing the same value, with `_` permitted anywhere in any of them for grouping. The prefix belongs to the *source*; it is not part of the number and not accepted by the parser. → [Why hexadecimal](19_Numbers/why_hexadecimal/README.md)

**`from_str_radix`** — The integer parser that takes a base. The target type bounds it, so `u8::from_str_radix("100", 16)` is `Err(PosOverflow)` — 0x100 does not fit a `u8` and it refuses rather than truncating. Its asymmetry with `{:#x}`, which prints a prefix it will not read back, is the reason format and parse are not inverses without `strip_prefix`. → [Why hexadecimal](19_Numbers/why_hexadecimal/README.md)

**Two's complement** — How a signed integer stores a negative value, and therefore what its hex spelling shows: `format!("{:x}", -1i8)` is `"ff"` and `-1i32` is `"ffffffff"`. No minus sign appears and the width of the type shows through, because hex spells the bits rather than the quantity. → [Why hexadecimal](19_Numbers/why_hexadecimal/README.md)

**Interval arithmetic** — Carrying each value as the bracket `[lo, hi]` it is guaranteed to lie in rather than as a single rounded number, so the result arrives with its own error bound attached. It answers an ordering question — *are these two ranges disjoint?* — not a value question, which is why it suits a count whose only real output is who won. → [Did the rounding decide it?](09_Advanced/interval_arithmetic/README.md)

**Sound vs complete** — A method is **sound** when everything it asserts is true, and **complete** when it asserts everything true. Interval arithmetic is the first without the second: it never names a wrong winner, and it sometimes declines to name a right one. One-sided error is what makes a conservative method safe to build on — the cost of being wrong in the only direction it can be wrong is extra work, never a wrong answer. → [Did the rounding decide it?](09_Advanced/interval_arithmetic/README.md)

**Soundness bug** — The other sense of *sound*, and the one Rust's guarantee is stated in: a type checker is **sound** when no program it accepts can reach undefined behaviour, so a soundness bug is one counterexample — a program with no `unsafe` anywhere in it that `rustc` accepted and should have rejected. A property of the compiler, not of your code, and the only kind of compiler bug whose symptom is a clean build. Thirty were reported between 2022 and 2025; eleven reach memory. → [When the type checker is wrong](20_Compilers/when_the_type_checker_is_wrong/README.md)

**`Display`** — The printing trait for the person *using* the program, reached by `{}`. It cannot be derived, and that is deliberate: nothing about a type says whether a human wants `Ada scored 5/2/0` or a row in a table. Writing it is also what makes `.to_string()` exist, via a blanket impl. → [Debug and Display](15_First_Programs/debug_vs_display/README.md)

**`Debug`** — The printing trait for the person *writing* the program, reached by `{:?}`. Derivable because the answer is structural — the type's name, its fields, their names — which is the same reason it reaches the field you did not think of as output. Its format is explicitly not stable, so nothing should parse it. → [Debug and Display](15_First_Programs/debug_vs_display/README.md)

**Alternate flag (`#`)** — The `#` in `{:#?}` and `{:#x}`: one bit on the `Formatter` that an impl may read. The derived `Debug` uses it to pretty-print one field per line, and `f.debug_struct()` honours it for free in a hand-written impl. → [Debug and Display](15_First_Programs/debug_vs_display/README.md)

**`ToString`** — The trait behind `.to_string()`, which you never implement: `impl<T: Display> ToString for T` gives it to every type that has a `Display`. The dividend Display pays and Debug does not — the `Debug` string is reachable only through `format!("{:?}")`. → [Debug and Display](15_First_Programs/debug_vs_display/README.md)

**Doc comment** — `///` or `//!`, and not a comment: the compiler parses it into a `#[doc = "..."]` attribute on an item, so it must have an item to attach to. `//!` is *inner* (it documents what it is inside, hence the top of a file); `///` is *outer* (it documents the item below it). → [Comments that compile](15_First_Programs/comments_that_compile/README.md)

**Doctest** — A fenced code block inside a doc comment, which `cargo test` compiles and runs like any other test. Each is its own crate linked against yours, so it sees the public API only — an integration test that is also the documentation. The reason Rust's examples cannot quietly rot into ones that no longer compile. → [The example that is a test](28_Testing/doc_tests/README.md), [Comments that compile](15_First_Programs/comments_that_compile/README.md)

**`unused_doc_comments`** — The warn-by-default lint for a `///` that attached to a statement or expression rather than an item. Worth knowing by name because it is the quiet failure: the doc comment parsed, the build succeeded, and nothing will ever read what you wrote. → [Comments that compile](15_First_Programs/comments_that_compile/README.md)

**Lint** — A named check built into `rustc` itself rather than a separate tool, each with a level you can set independently. The `= note:` line under a warning gives you the name, so `#[allow(unused_variables)]` never has to be guessed at. If you can `#[allow]` it, it was a lint; the borrow checker and type errors are not. → [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md)

**Lint level** — One of `allow` (silent), `warn` (printed, still compiles, exits 0), `deny` (an error, the build fails) or `forbid` (deny, plus no later `allow` of that lint). Set on an item, a block, or the crate with `#![…]` — or from outside with `RUSTFLAGS="-D warnings"`, which is how CI makes warnings fail without editing any source. → [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md)

**`unused_variables`** — The warn-by-default lint almost everybody meets first, and a question rather than a complaint: its suggested `_name` fix is conditional on *"if this is intentional"*, and when it is not, the warning has found a bug the underscore would hide. → [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md)

**Wildcard pattern (`_`)** — Not a variable name but a pattern that binds nothing, so it takes ownership of nothing. Two consequences, and only one of them is the famous one. `let _ = make_guard();` drops a **temporary** at the semicolon rather than at the end of the scope — which for a `MutexGuard`, file lock, span or transaction handle releases it before the code it was protecting runs. But `let _ = existing;` on a value that already has an owner moves nothing and changes no drop timing: `existing` is still usable on the next line. → [The wildcard `_`](30_Pattern_Matching/the_wildcard/README.md), [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md)

**Underscore-prefixed binding (`_name`)** — An ordinary binding whose leading underscore exempts it from `unused_variables` and changes nothing else: it still binds, and the value still lives to the end of the scope. The form to reach for whenever a value's lifetime is the point, and the one thing a bare `_` is not. → [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md)

**Bit flag** — A named constant with exactly one bit set, combined into an integer with `|` and tested with `& flag != 0`. It is the degenerate bit field: one bit wide, its two values named set and unset. A flag whose value is `0` cannot be tested this way at all, because `x & 0` is `0` for every `x`. → [Bit flags](19_Numbers/bit_flags/README.md)

**Bit field** — A run of adjacent bits inside a larger integer holding one value. Pack it mask-then-shift, unpack it shift-then-mask. The topmost field is the one that forgives a missing mask, which is why the habit of omitting it survives long enough to reach a field that does not. → [Bit flags](19_Numbers/bit_flags/README.md)

**Bit mask** — An integer whose set bits mark the positions you want, so `x & mask` keeps those and zeroes the rest. Testing membership of a multi-bit mask is `x & m == m` (all of it), not `x & m != 0` (any of it) — the two agree only for a single bit. → [Bit flags](19_Numbers/bit_flags/README.md)

**Block expression** — A `{ }` block used for its *value* rather than only for its scope: `let quorum = { let half = voters / 2; half + 1 };`. Working names stay inside, one value comes out, and the binding it lands in needs no `mut`. C has no such thing (GCC's `({ … })` is a non-standard extension), which is why the idiom looks strange coming from there. → [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md)

**Tail expression** — The last line of a block written *without* a semicolon; it is what the block evaluates to. A function body is a block, so a tail expression is how a function returns without `return` — and a semicolon on it makes the block worth `()` instead, which is the first-week `E0308`. → [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md)

**Unit type (`()`)** — The type with exactly one value, also written `()`. What a block is worth when its last line is a statement, what a function with no `-> T` returns, and what an `if` without `else` evaluates to. Reading "found `()`" in an error as "found nothing" is close enough: it means *no useful value was produced here*. → [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md)

**`E0308`** — *mismatched types.* The most common error in Rust, and its most common **cause** is a semicolon: the `^^^` points at a function's declared return type while the `help:` four lines down names the semicolon that threw the value away. Changing the signature silences it and breaks the function. → [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md)

**`E0317`** — *`if` may be missing an `else` clause.* An `if` used as a value with no `else`, carrying the note that says why in one line: "`if` expressions without `else` evaluate to `()`". → [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md)

**Inline format argument (`{n}`)** — Naming a variable directly inside a format string, stabilized in Rust 1.58. It captures an **identifier** and nothing else — resolved by the macro at compile time, by ordinary name lookup — so `{n + 1}`, `{v.len()}` and `{self.voter}` are all compile errors, and the format string itself must be a literal. Not a Python f-string, which takes a full expression. → [The braces take a name](15_First_Programs/braces_take_a_name/README.md)

**Format spec** — Everything after the `:` in `{value:>width$.prec$}` — fill, alignment, width, precision, and which trait to print through. A separate small language from the capture before the colon; a trailing `$` is what marks a width or precision as a *name* rather than a literal number. Which trait `{}` and `{:?}` reach for is a different question. → [The braces take a name](15_First_Programs/braces_take_a_name/README.md), [Debug and Display](15_First_Programs/debug_vs_display/README.md)

**Type annotation** — The `: Type` on a `let`, a parameter or a field. Not a comment but an *input*: the compiler solves the expression against it. On a string literal it decides nothing — `let s = "a";` and `let s: &str = "a";` are the same program — and on four other shapes it decides what the program is. → [What a type annotation does](15_First_Programs/what_an_annotation_does/README.md)

**Integer fallback** — What an unsuffixed integer literal becomes when nothing else in the function decides: `i32`, and `f64` for a float. Not what `1` *means*, only what Rust settles on last, after every annotation, parameter type and later use has had its say. → [What a type annotation does](15_First_Programs/what_an_annotation_does/README.md)

**Turbofish (`::<T>`)** — The `::<i32>` in `"42".parse::<i32>()` — the same information a type annotation carries, written at the call instead of on the binding. The form to reach for when the value is not being bound to a name. → [What a type annotation does](15_First_Programs/what_an_annotation_does/README.md)

**`E0284`** — *type annotations needed.* Raised when an expression's type is chosen by its target and there is no target — `let x = "42".parse().unwrap();` being the one everybody meets. The `help:` line offers the fix as a hole to fill: `let x: /* Type */ = …`. → [What a type annotation does](15_First_Programs/what_an_annotation_does/README.md)

**`String`** — The owned, growable text type: three words on the stack (pointer, length, capacity), UTF-8 bytes on the heap. A `Vec<u8>` that promises valid UTF-8, with the same `new` / `with_capacity` / `reserve` vocabulary. Own it in fields, build it for returns — and take `&str` in parameters. → [`String` vs `&str`](14_Strings/string_vs_str/README.md), [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md)

**String slice (`&str`)** — A borrowed view of UTF-8 text living anywhere — the binary, a `String`'s heap buffer, a stack array: one pointer plus one length, owning nothing. `Copy`, read-only, and the type every text-reading parameter should take, since literals, `String`s and slices all arrive as one for free. → [`String` vs `&str`](14_Strings/string_vs_str/README.md)

**String literal** — `"…"` in source: a `&'static str` whose bytes are baked into the executable's read-only data — not the stack, not the heap — alive for the whole run. "Stack-allocated string" in a tutorial is this fact, misplaced. → [`String` vs `&str`](14_Strings/string_vs_str/README.md)

**String concatenation (`+`)** — `impl Add<&str> for String` is the only impl there is: the left operand must **own** a buffer and is consumed, the right is only borrowed. So `"a" + "b"` is `E0369`, `a + b` on two `String`s is `E0308`, and `a + &b` compiles because the answer *is* `a`'s buffer grown — which is why a `+` chain allocates nothing after its first piece. `format!` borrows everything and never asks the question. → [Concatenating strings](14_Strings/concatenating_strings/README.md)

**`E0369`** — *cannot add `X` to `Y`.* No operator impl exists for that pair of types, and on text it always means the same thing: the left operand was a `&str`, a view with no buffer to grow. The note spells it out — "string concatenation requires an owned `String` on the left" — and `E0368` is the same complaint about `+=`. → [Concatenating strings](14_Strings/concatenating_strings/README.md)

**Deref coercion** — The compiler's automatic `&String` → `&str` (and `&Vec<T>` → `&[T]`, `&PathBuf` → `&Path`) at call sites, via `Deref`. It is also why a `String` *inherits* `str`'s methods — `owned.to_uppercase()` finds the method through the coercion. The reverse direction is never free: `.to_string()` allocates. → [`String` vs `&str`](14_Strings/string_vs_str/README.md), [Coercion](29_Conversion/coercion/README.md)

**Coercion** — The one conversion nobody writes: the compiler adjusts an expression's type where the wanted type is already known — a function argument, a `let` with an annotation, a return value, a struct field. The list is closed (deref, `&mut T` → `&T`, array → slice, concrete → `dyn Trait`, `fn` item → `fn` pointer, `!` → anything) and contains **no numeric conversion at all**, which is why `u8 + u16` does not compile. It also never reaches inside another type: `Option<&String>` is not `Option<&str>`. → [Coercion: the conversion you never write](29_Conversion/coercion/README.md)

**Capacity** — The room a growable buffer has bought, as distinct from `len`, the part in use. Growth doubles it, `with_capacity` pre-pays it, `shrink_to_fit` returns it — and it is bookkeeping, not content: equality and hashing never see it. → [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md)

**`char`** — One Unicode scalar value, four bytes wide as a value — decoded, so it can be compared, classified and ranged over. Inside a `String` the same character is 1–4 UTF-8 bytes. `'a'` is a `char`; `"a"` is a `&str` holding one. → [Meet the `char`](14_Strings/meet_the_char/README.md)

**UTF-8** — The encoding every `String` and `&str` promises: ASCII costs one byte, `é` two, an emoji four. The promise is checked where bytes enter (`from_utf8`) so no method inside ever re-checks — and it is why `.len()` counts bytes and `s[0]` does not compile. → [Meet the `char`](14_Strings/meet_the_char/README.md)

**Grapheme cluster** — What a *reader* calls one character: `e` plus a combining accent is two `char`s, one grapheme. The third answer to "how long is this string", and the one std cannot count — that is the `unicode-segmentation` crate's job. → [Meet the `char`](14_Strings/meet_the_char/README.md)

**`OsString` / `OsStr`** — Owned and borrowed text exactly as the operating system hands it over — filenames, env vars, arguments — with no UTF-8 promise, because the OS makes none. Narrowing to `&str` is `to_str()` returning an `Option`, and the `None` is a real answer. → [Six kinds of string](14_Strings/six_kinds_of_string/README.md)

**`CString` / `CStr`** — Owned and borrowed text under C's contract: no NUL byte inside, one NUL at the end. `CString::new` refuses an interior NUL with an error naming the byte — the string C would have silently truncated. → [Six kinds of string](14_Strings/six_kinds_of_string/README.md)

**Union** — A type declared exactly like a struct whose fields share one piece of storage rather than sitting side by side, so its size is its *largest* field rather than the sum. Writing a field is safe; reading one is `unsafe`, because nothing in a union records which field is live. Unrelated to the unit struct despite the name. → [What a union is](09_Advanced/what_a_union_is/README.md)

**Tagged union** — A union plus a discriminant saying which field is live. In C you build one by hand out of a `struct`, an `enum` and a `union`, and remember to check the tag; in Rust it is spelled `enum` and the `match` checks the tag for you. The Reference defines a `repr(C)` enum with fields as literally this. → [What a union is](09_Advanced/what_a_union_is/README.md)

**`E0277`** — "the trait bound was not satisfied", and for structs it is four unrelated problems wearing one number: no `Display` for `{}`, no `Debug` for `{:?}`, a `str` field that has no *size*, and a derived `Eq` with no `PartialEq` under it. The code identifies the shape of the complaint, never the fix — the `note:` line does. → [When a struct refuses](16_Structs/when_a_struct_refuses/README.md)

**Alternate flag** — What `{:#?}` sets and `{:?}` does not, readable inside an impl as `f.alternate()`. A derived `Debug` and anything built with `f.debug_struct()` honour it; a hand-written `write!` chain silently ignores it, so both forms print identically — and `dbg!`, which is hard-wired to `{:#?}`, quietly gets the flat one. → [What `dbg!` does](15_First_Programs/what_dbg_does/README.md)

**Phantom type** — A type parameter that appears in a struct's declaration and in none of its data, so two taggings of identical bytes become unrelated types. The tag is checked by the compiler and gone by run time, which is what makes mixing metres with feet, or an Approval ballot with a STAR count, unwriteable rather than merely wrong. → [Phantom types](12_Traits/phantom_types/README.md)

**`PhantomData<T>`** — The zero-sized field that carries a phantom parameter. Not a way to silence the compiler but a claim to it: `PhantomData<T>` says the struct owns a `T`, `PhantomData<fn() -> T>` that it merely produces one, `PhantomData<*const T>` that it only points at one — same size, different variance and drop behaviour. → [Phantom types](12_Traits/phantom_types/README.md)

**`E0392`** — "type parameter is never used": a generic parameter declared and then referred to by nothing. Its three suggested fixes are the whole decision — delete it, store something of that type, or keep it deliberately with a `PhantomData` field. → [Phantom types](12_Traits/phantom_types/README.md)

**Generic (`<T>`)** — A type the caller fills in. One definition serves every type, and the compiler settles which at compile time by stamping out a copy per type used — so `Container<u8>` is one byte, `Container<u8>` and `Container<String>` are unrelated types, and neither pays anything at run time. → [What a generic is](22_Generics/what_a_generic_is/README.md)

**Trait bound** — The `: Trait` on a generic parameter, and the complete list of what the body may do with it. An unbounded `T` can be stored, moved and dropped and nothing else. Rust checks the generic body once against its bounds rather than at each instantiation, so the error lands on the definition rather than on somebody's call site — and the bound belongs on the `impl` block that spends it, never on the struct. → [Where the bound goes](22_Generics/where_the_bound_goes/README.md)

**`where` clause** — The bound moved below the signature. Identical to the inline form for a bare parameter, and the *only* spelling available when the subject is not one: `where Vec<T>: Debug` and `where T::Item: Clone` cannot be written inside the angle brackets at all. → [Where the bound goes](22_Generics/where_the_bound_goes/README.md)

**Argument-position `impl Trait`** — `fn f(v: impl Display)`, the parameter that is never named. Two of them are two *independent* types, where `<T: Display>` used twice is one type twice — and with no `T` to name, the caller cannot turbofish it. → [Where the bound goes](22_Generics/where_the_bound_goes/README.md)

**Closure** — A function that also carries the variables it mentioned from the scope around it. The compiler writes an anonymous struct with one field per capture, so a closure's size is exactly what it captured — zero bytes if that is nothing — and no two closures share a type, however identical their text. → [What a closure is](23_Closures/what_a_closure_is/README.md)

**`Fn` / `FnMut`** — The two tighter closure traits, sitting above `FnOnce` on a supertrait ladder: `Fn` takes `&self` so it is repeatable and may not mutate its captures, `FnMut` takes `&mut self` so it is repeatable and may. What a closure gets is decided by what its body does with the captures — read, mutate, or move out — and never by the `move` keyword. → [The three closure traits](23_Closures/three_closure_traits/README.md)

**`move` (on a closure)** — Captures by value rather than by reference. It answers a lifetime question — a closure that outlives the scope it was written in cannot hold a borrow of it — and decides nothing about which `Fn` trait the closure implements. On a `Copy` type it copies, silently, which is the one `move` bug that compiles and does nothing. → [The `move` keyword](23_Closures/the_move_keyword/README.md)

**Adapter / consumer** — The two halves of an iterator chain. An **adapter** (`map`, `filter`, `take`, `zip`) returns another iterator and computes nothing; a **consumer** (`collect`, `sum`, `find`, `for_each`) returns something else and is what actually runs the chain — for as long as it needs an answer, which is why `find` may call your closure once where `collect` calls it for every item. → [Iterators are lazy](24_Iterators/iterators_are_lazy/README.md)

**`IntoIterator`** — The trait a `for` loop actually requires. `Vec` implements it three times — for `Vec<T>`, `&Vec<T>` and `&mut Vec<T>` — which is what makes `for x in v`, `for x in &v` and `for x in &mut v` three different loops with three different item types, and what keeps the collection re-iterable instead of turning it into a one-shot iterator. → [`iter`, `iter_mut`, `into_iter`](24_Iterators/iter_iter_mut_into_iter/README.md)

**`size_hint`** — An iterator's optional estimate of how many items are left. The default is `(0, None)` and `collect` believes it, so the `Vec` grows by doubling; writing three honest lines of `size_hint` turns nine items collected into one allocation of exactly nine instead of a walk up to sixteen. → [Implementing `Iterator`](24_Iterators/implementing_iterator/README.md)

**Lifetime annotation** — The `<'a>` on a function or struct. It names a relationship between lifetimes that already exist so the compiler can check it; it grants nothing, and a correctly annotated signature still fails when the arrangement is unsafe. Where one name covers two references, the region chosen is the one where *both* are valid — the shorter. → [Lifetime annotations](18_Ownership/lifetime_annotations/README.md)

**Lifetime elision** — The three rules that let most signatures omit `'a`: every elided input lifetime gets its own; one input lifetime fills every elided output; and `&self`, when present, fills them instead. They are why `&` in a signature is usually free, and `E0106` fires exactly where they run out. → [Lifetime annotations](18_Ownership/lifetime_annotations/README.md)

**`E0106`** — "missing lifetime specifier": an output borrows and the signature does not say from which input. The `help:` line is the whole diagnosis — *"the signature does not say whether it is borrowed from `a` or `b`"* — and it is missing from the **signature**, not the body, which the compiler refuses to consult because a signature is a contract with every caller. → [Lifetime annotations](18_Ownership/lifetime_annotations/README.md)

**`E0716`** — "temporary value dropped while borrowed": a temporary freed at the end of its statement, held by a borrow used after it. Often a signature problem rather than a call-site one — tying two parameters to one `'a` when the answer borrows from only one of them demands that the other live just as long. → [Lifetime annotations](18_Ownership/lifetime_annotations/README.md)

**Tuple** — A struct whose fields are numbered instead of named, written the way its value is: `(i32, i32)`. Length and element types are part of the type, so a pattern cannot match the wrong shape; comparison is field by field, left to right, which makes a tuple a free sort key. Readable for about two fields — `row.2` in a function two files away is a comment nobody wrote. → [Tuples](26_Collections/tuples/README.md)

**Array (`[T; N]`)** — A fixed-length block of one type, laid out inline with no header, and a **separate type for every length** — which is why it almost never appears in a signature. `Copy` when `T` is. → [Arrays and slices](26_Collections/arrays_and_slices/README.md)

**Slice (`&[T]`)** — A borrowed view of a run of elements: a pointer and a length, so one function serves an array of any length, a `Vec`, or part of either. The length moved out of the *type* and into the *value*, which is the whole trick. Take `&[T]` in a signature, never `&Vec<T>`. → [Arrays and slices](26_Collections/arrays_and_slices/README.md)

**Amortised growth** — `Vec` doubles its capacity when it fills, so *n* pushes cost O(*n*) in total rather than O(*n*²), at the price of copying everything already stored each time it grows. `with_capacity` and `collect` over a known-length iterator skip the copying entirely. The exact sequence is std's choice, not a language guarantee. → [`Vec`](26_Collections/the_vec/README.md)

**`entry` API** — `*map.entry(k).or_insert(0) += 1`: one hash lookup that inserts a default if the key is absent and hands back a `&mut` to the value either way. The counting loop written any other way costs two or three lookups, and written with `insert` is silently wrong. → [`HashMap`](26_Collections/the_hashmap/README.md)

**Hash randomisation** — std's `HashMap` seeds its hasher per process, so two runs iterate in different orders. A defence against hash-flooding, and the reason anything printed or compared must be sorted first — or come from a [`BTreeMap`](26_Collections/sorted_collections/README.md), which is ordered by key by construction. → [`HashMap`](26_Collections/the_hashmap/README.md)

**Total order (`Ord`) against partial (`PartialOrd`)** — A `BTreeMap` key must be totally ordered: every two values compare, and the comparison is consistent. `f64` is not, because `NaN` compares `false` against everything including itself — so a float-keyed `BTreeMap` is refused, and the refusal lands on the first `insert` rather than the declaration, since `new` and `len` need no `Ord`. → [`BTreeMap` and `BTreeSet`](26_Collections/sorted_collections/README.md)

**Module** — A namespace *and* a privacy boundary. Items are private by default, and "private" means to this module **and its descendants** — so a child sees up through `super::`, a parent cannot see in, and everything sharing a module can reach the private fields of everything else in it. When an invariant escapes, the module is the access list to audit, not the callers. → [Modules and visibility](27_Modules/modules_and_visibility/README.md)

**`pub(crate)` / `pub(super)`** — Visibility narrower than `pub`: anywhere in this crate, or the parent module and its descendants. `pub` itself is relative — it publishes an item to the same audience the module already had, so a `pub fn` in a private module is public to nobody. → [Modules and visibility](27_Modules/modules_and_visibility/README.md)

**`use`** — A shortcut, not an import: it binds a name in this module and loads, compiles and links nothing. `as` renames (the fix for a collision), braces bring several names from one path, and `*` is the glob whose ambiguity error lands at the *use site*, months later. → [Bringing names in with `use`](27_Modules/the_use_declaration/README.md)

**Crate root** — `src/main.rs` for a binary, `src/lib.rs` for a library. That file *is* the crate's top module, so `crate::` starts there. A package with both is two crates: a library, and a binary that uses it by name. → [One module per file](27_Modules/one_module_per_file/README.md)

**Attribute** — Metadata the compiler acts on, in five families: `derive`, the lint levels, `cfg`, the test markers, and the codegen/API set (`inline`, `repr`, `must_use`, `non_exhaustive`). `#[…]` applies to the item below it and `#![…]` to the item it is inside; a doc comment is one too, since `///` is `#[doc = "…"]`. → [What an attribute is](27_Modules/what_an_attribute_is/README.md)

**`forbid` vs `deny`** — Both make a lint an error; only `forbid` refuses to be relaxed by an inner `allow`, which is itself then an error. So `#![forbid(unsafe_code)]` is a promise worth making and `#![forbid(warnings)]` is a trap. → [What an attribute is](27_Modules/what_an_attribute_is/README.md)

**Const promotion** — Taking a reference to a `const` gives the value an anonymous `static` to live in, because a substituted value has no address of its own. Two `&MAX` may therefore compare equal — an observation about the build, not a guarantee. A `static`'s single address *is* a guarantee. → [`const` and `static`](27_Modules/const_and_static/README.md)

**`const fn`** — A function callable in a const context. May branch, loop, index and do arithmetic; may not allocate, call a non-const function, or read a `static`. Marking one is a promise about what it does not do and part of your public API — un-`const`-ing it later is a breaking change. → [`const` and `static`](27_Modules/const_and_static/README.md)

**Unit test / integration test** — A unit test lives in a `#[cfg(test)] mod tests` inside `src/` and can reach private items; an integration test is a file directly in `tests/`, compiled as a **separate crate**, so it sees only the public API. The choice is decided by one question — can the behaviour be observed from outside? → [Where a test goes](28_Testing/where_a_test_goes/README.md)

**`should_panic`** — Marks a test that passes by panicking. Without `expected = "…"` it passes on *any* panic, including one introduced by the refactor the test existed to catch. → [Where a test goes](28_Testing/where_a_test_goes/README.md)

**`From` / `Into`** — The infallible conversion pair. Implement `From`; `Into` arrives free through a blanket impl, and the reflexive `From<T> for T` is why `impl Into<T>` in an argument also accepts a `T`. `?` calls `From::from` on the error, which is what makes an error enum with one impl per variant the standard shape. → [`From` and `Into`](29_Conversion/from_and_into/README.md)

**`TryFrom` / `TryInto`** — The same pair plus a `type Error`, for a conversion allowed to refuse. `u8::try_from(300i32)` reports what `300i32 as u8` swallows. For text, implement `FromStr` instead and get `.parse()`. → [`TryFrom` and `TryInto`](29_Conversion/tryfrom_and_tryinto/README.md)

**Orphan rule** — You may implement a trait for a type when the trait or the type (or a type parameter of the impl) is local to your crate. `impl From<Vec<u8>> for String` is `E0117` because neither is; the way round it is a newtype. Coherence is the reason: two crates could otherwise write the same impl differently. → [`From` and `Into`](29_Conversion/from_and_into/README.md)

**`as` cast** — The built-in conversion that always succeeds, and therefore loses data four ways without saying so: narrowing keeps the low bits, signedness reinterprets them, float→int truncates then saturates (`NaN` → 0), and int→float rounds. Reach for `From` when it cannot fail and `TryFrom` when it can. → [Casting with `as`](29_Conversion/casting_with_as/README.md)

**`checked_` / `wrapping_` / `saturating_`** — The three named arithmetic behaviours, each replacing the default — which panics in debug and wraps in release, the one case where the two profiles disagree about the answer. Only `checked_` hands the decision back, as an `Option`. → [Casting with `as`](29_Conversion/casting_with_as/README.md)

**RAII** — *Resource acquisition is initialisation*: a value acquires the resource when it is created and releases it in `Drop`, so there is nothing for the caller to remember and no `finally` to write. It survives an early `return` and a panic. In Rust it is not a pattern but the way every value already works — `String`, `File` and `MutexGuard` are all this. → [`Drop`, and what RAII buys](12_Traits/drop_and_raii/README.md)

**Operator overloading** — Every operator is a trait: `a + b` is `Add::add(a, b)`, `v[i]` is `*v.index(i)`. `type Output` means none of the three types has to match, each operator is one trait for one *pair* of types (so `3 * p` needs a second impl after `p * 3`), and the test for whether to implement one is whether every reader would guess the same answer. → [Operators are traits](12_Traits/operators_are_traits/README.md)

**Scoped thread** — `thread::scope` guarantees every thread it started has finished before it returns, so the closures may **borrow** local data — no `Arc`, no clone, and the borrow checker still helping. The right default for a fan-out over data you already have; `spawn` needs `'static` precisely because it offers no such guarantee. → [Spawning a thread](09_Advanced/spawning_a_thread/README.md)

**`mpsc` channel** — Multi-producer, single-consumer: `Sender` is `Clone`, `Receiver` is not, and `send` **moves** the value, so at every moment it has exactly one owner and nothing needs a lock. The receiver's loop ends when the *last* `Sender` drops — which is why a `Sender` you kept hangs a program that has finished its work, with no error message at all. (Not the toolchain sense of *channel*, which is `stable`/`beta`/`nightly`.) → [Channels](09_Advanced/channels/README.md)

**Backpressure** — What a bounded channel gives you: `sync_channel(n)` blocks the producer once *n* values are queued, so it runs at the consumer's speed. An unbounded channel with a fast producer and a slow consumer is a memory leak with good manners. → [Channels](09_Advanced/channels/README.md)


**`fold`** — Carry an accumulator through a sequence and hand it back. The consumer the named ones are built from: std's `Sum for i32` is literally `iter.fold(0, |a, b| a + b)`, and `count` is a fold that adds one per item. Reach for it when the answer is not the same type as the items — a `String`, a `HashMap`, or two answers in one tuple. → [`fold` and `reduce`](24_Iterators/fold_and_reduce/README.md)

**`reduce`** — `fold` with the first item as the starting value, so the accumulator must be the item type and the answer is an `Option`: an empty sequence has no first item, and `reduce` will not invent one. That is the right shape for a maximum and the wrong one for a sum, where `0` really is the answer. → [`fold` and `reduce`](24_Iterators/fold_and_reduce/README.md)

**`FromIterator`** — The trait `collect` calls. It is why `collect` has no behaviour of its own: the **target type** decides whether you get a `Vec`, a deduplicated set, a map where a later key overwrites an earlier one, or one `Result` covering every row. Implement it once and your own type becomes collectable. → [`collect` and `FromIterator`](24_Iterators/collect_and_fromiterator/README.md)

**`collect::<Result<Vec<_>, _>>()`** — The flip that turns an iterator **of** `Result`s into one `Result` **of** a collection, stopping at the first `Err`. The standard way to parse a file of rows and put a `?` at the end — and it keeps only the first failure, so a validation report wants `partition` instead. → [`collect` and `FromIterator`](24_Iterators/collect_and_fromiterator/README.md)

**`DoubleEndedIterator`** — The trait behind `rev`, `rfind` and `rfold`: one extra method, `next_back`. Its obligation is the part no compiler checks — both ends consume from the same sequence and must meet in the middle, never handing out an element twice. → [`DoubleEndedIterator` and `ExactSizeIterator`](24_Iterators/double_ended_and_exact_size/README.md)

**`ExactSizeIterator`** — An empty impl block that is entirely a promise: `len()` is provided, and its body reads `size_hint()` and trusts it. A wrong `size_hint` therefore yields a wrong `len()` rather than a panic. `filter` cannot have it (it cannot know how many pass) and neither can `str::chars`. → [`DoubleEndedIterator` and `ExactSizeIterator`](24_Iterators/double_ended_and_exact_size/README.md)

**`Stream`** — An iterator whose `next` may answer *"not yet"*: `poll_next` returns `Poll<Option<Item>>` instead of `Option<Item>`. Not in std — it lives in `futures`, and the eventual std name, `AsyncIterator`, is still unstable. There is no `for x in stream`; the async form is `while let Some(x) = stream.next().await`. → [`Iterator` versus `Stream`](24_Iterators/iterator_vs_stream/README.md)
