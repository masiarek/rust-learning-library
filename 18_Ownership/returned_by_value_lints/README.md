# Lints around returning by value

Beside [Returned by value](../returned_by_value/README.md) · the errors, not the warnings: [Every returned-by-value error, and its fix](../returned_by_value_errors/README.md)

**Level:** 201 · a reference, by lint

**One line:** Eight rustc and clippy lints that touch a value handed back by value: a `.clone()` that returns the reference it was called on, a returned result thrown away, a `Result` or a stack frame too big, a `Box` with no reason to be there. Each has a program that triggers it, what the tool prints, a silent version, and when the lint is right. Then what nothing flags: `Box<dyn Trait>` where `impl Trait` would do, a getter that clones, and a returned array under every threshold that still overflows the stack it runs on.

Every transcript was recorded on rustc 1.98.0 and clippy 0.1.98 by running the command in its title. The lines dropped are the documentation link, the `#[warn(...)]` or "requested on the command line" note, and the closing warning count. Each silent version was run with that lint plus `clippy::all`, `clippy::pedantic` and `rust_2018_idioms`, and printed nothing. Both programs of every pair are compiled on every build by [`check_fences.py`](../../tools/check_fences.py). Clippy itself is not re-run, so a later clippy can reword a warning.

## Turning them on

rustc's warn-by-default lints and clippy's default groups need nothing: `cargo clippy` prints them. The opt-in ones take a flag or a `Cargo.toml` entry:

```sh
cargo clippy -- -W clippy::unnecessary_box_returns -W clippy::return_self_not_must_use -W clippy::large_stack_arrays -W clippy::large_stack_frames -W clippy::large_types_passed_by_value
```

```toml
[lints.clippy]
unnecessary_box_returns = "warn"
return_self_not_must_use = "warn"
large_stack_arrays = "warn"
large_stack_frames = "warn"
large_types_passed_by_value = "warn"
```

`unnecessary_box_returns` and `large_types_passed_by_value` stay quiet about a library's public functions unless `clippy.toml`, next to `Cargo.toml`, says otherwise:

```toml
avoid-breaking-exported-api = false
```

## Find the lint

| Lint | From | On by default? | Fires on |
|---|---|---|---|
| [`noop_method_call`](#noop_method_call) | rustc | yes | `.clone()` on a `&str`, which hands back the reference |
| [`unused_must_use`](#unused_must_use) | rustc | yes | `name.to_uppercase();`, a returned `String` thrown away |
| [`result_large_err`](#result_large_err) | clippy · perf | yes | a `Result` whose `Err` is 128 bytes or more |
| [`unnecessary_box_returns`](#unnecessary_box_returns) | clippy · pedantic | no | `-> Box<Config>` for a sized `Config` |
| [`return_self_not_must_use`](#return_self_not_must_use) | clippy · pedantic | no | a public method returning `Self` with no `#[must_use]` |
| [`large_stack_arrays`](#large_stack_arrays) | clippy · pedantic | no | the `[7; 100_000]` a function builds to return |
| [`large_stack_frames`](#large_stack_frames) | clippy · nursery | no | a function whose frame, return slot included, passes 512,000 bytes |
| [`large_types_passed_by_value`](#large_types_passed_by_value) | clippy · pedantic | no | a large `Copy` parameter, and never the same type returned |

## rustc, on by default

### `noop_method_call`

**rustc** · warn by default · fires on `.clone()` on a `&str`, which hands back the reference

```rust
fn keep(name: &str) -> &str {
    name.clone()
}

fn main() {
    let owner = String::from("ferris");
    println!("{}", keep(&owner)); // ferris
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0"
warning: call to `.clone()` on a reference in this situation does nothing
 --> bad.rs:2:9
  |
2 |     name.clone()
  |         ^^^^^^^^ help: remove this redundant call
  |
  = note: the type `str` does not implement `Clone`, so calling `clone` on `&str` copies the reference, which does not do anything and can be removed
```

**Silent:**

```rust
fn keep(name: &str) -> String {
    name.to_owned()
}

fn main() {
    let owner = String::from("ferris");
    println!("{}", keep(&owner)); // ferris
}
```

**When it is right.** Always right. `clone` returns `Self` by value, `str` cannot be `Clone` because of that, and so the `clone` found is the reference's own, which copies two words. It fires in generic code too: `x.clone()` on an `x: &T` with `T: ?Sized` gets the same warning, noting *the type `T` does not implement `Clone`*. Clippy adds nothing: with `clippy::all`, `pedantic` and `nursery` on and this lint allowed, `clone_on_copy` stays quiet, although `&str` is `Copy`. The same lint on a `let` is on [the `ToOwned` lints page](../../12_Traits/how_to_learn_to_owned/to_owned_lints/README.md#noop_method_call).

### `unused_must_use`

**rustc** · warn by default · fires on `name.to_uppercase();`, a returned `String` thrown away

```rust
fn main() {
    let name = String::from("ferris");
    name.to_uppercase();
    println!("{name}"); // ferris
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0"
warning: unused return value of `str::<impl str>::to_uppercase` that must be used
 --> bad.rs:3:5
  |
3 |     name.to_uppercase();
  |     ^^^^^^^^^^^^^^^^^^^
  |
  = note: this returns the uppercase string as a new String, without modifying the original
help: use `let _ = ...` to ignore the resulting value
  |
3 |     let _ = name.to_uppercase();
  |     +++++++
```

**Silent:**

```rust
fn main() {
    let name = String::from("ferris");
    let name = name.to_uppercase();
    println!("{name}"); // FERRIS
}
```

**When it is right.** Always right. A method that returns its answer by value leaves the original as it was, and the note says so; `name.trim();` on a `&str` gets *this returns the trimmed string as a slice, without modifying the original*. It fires only where the function or its return type is marked `#[must_use]`, so your own methods need the attribute: that is what [`return_self_not_must_use`](#return_self_not_must_use) finds.

## Clippy, on by default

### `result_large_err`

**clippy · perf** · warn by default · fires on a `Result` whose `Err` is 128 bytes or more

```rust
struct ParseError {
    line: usize,
    context: [u8; 256],
}

fn parse_line(text: &str) -> Result<u32, ParseError> {
    text.trim().parse().map_err(|_| ParseError { line: 1, context: [0; 256] })
}

fn main() {
    println!("{:?}", parse_line("42").map_err(|e| e.line)); // Ok(42)
    println!("{:?}", parse_line("x").map_err(|e| (e.line, e.context.len()))); // Err((1, 256))
    println!("{}", size_of::<Result<u32, ParseError>>()); // 272
}
```

```text title="clippy-driver --edition 2024 -W clippy::result_large_err bad.rs — clippy 0.1.98"
warning: the `Err`-variant returned from this function is very large
 --> bad.rs:6:30
  |
6 | fn parse_line(text: &str) -> Result<u32, ParseError> {
  |                              ^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 264 bytes
  |
  = help: try reducing the size of `ParseError`, for example by boxing large elements or replacing it with `Box<ParseError>`
```

**Silent:**

```rust
struct ParseError {
    line: usize,
    context: [u8; 256],
}

fn parse_line(text: &str) -> Result<u32, Box<ParseError>> {
    text.trim().parse().map_err(|_| Box::new(ParseError { line: 1, context: [0; 256] }))
}

fn main() {
    println!("{:?}", parse_line("42").map_err(|e| e.line)); // Ok(42)
    println!("{:?}", parse_line("x").map_err(|e| (e.line, e.context.len()))); // Err((1, 256))
    println!("{}", size_of::<Result<u32, Box<ParseError>>>()); // 16
}
```

**When it is right.** Right when errors are rare and the function is called often. A `Result` comes back by value and is as big as its bigger variant, so the caller reserves 272 bytes for every `Ok(42)`, and boxing the error shrinks that to 16 and moves the cost to the failure, one allocation per error. An `Err` of 128 bytes warned and one of 120 did not; `large-error-threshold` in `clippy.toml` moves the line. Noise when the error path is the common one, or the function runs once.

## Clippy, opt-in

### `unnecessary_box_returns`

**clippy · pedantic** · off by default · fires on `-> Box<Config>` for a sized `Config`

```rust
struct Config {
    port: u16,
    verbose: bool,
}

fn parse_config(text: &str) -> Box<Config> {
    Box::new(Config { port: text.trim().parse().unwrap_or(80), verbose: false })
}

fn main() {
    let config = parse_config("8080");
    println!("{} {}", config.port, config.verbose); // 8080 false
}
```

```text title="clippy-driver --edition 2024 -W clippy::unnecessary_box_returns bad.rs — clippy 0.1.98"
warning: boxed return of the sized type `Config`
 --> bad.rs:6:32
  |
6 | fn parse_config(text: &str) -> Box<Config> {
  |                                ^^^^^^^^^^^ help: try: `Config`
  |
  = help: changing this also requires a change to the return expressions in this function
```

**Silent:**

```rust
struct Config {
    port: u16,
    verbose: bool,
}

fn parse_config(text: &str) -> Config {
    Config { port: text.trim().parse().unwrap_or(80), verbose: false }
}

fn main() {
    let config = parse_config("8080");
    println!("{} {}", config.port, config.verbose); // 8080 false
}
```

**When it is right.** Right for a small sized value: the caller can reserve room for a `Config` as easily as for the `Box`, and the box adds an allocation to fill it. It said nothing about `-> Box<str>` or `-> Box<dyn Display>`, which have no sized form to suggest. Declared `pub` in a `--crate-type lib` build, the same function was silent until `clippy.toml` set `avoid-breaking-exported-api = false`. Noise when the value is large enough that moving it is the cost the box avoids.

### `return_self_not_must_use`

**clippy · pedantic** · off by default · fires on a public method returning `Self` with no `#[must_use]`

```rust
pub struct Request {
    pub url: String,
    pub timeout_secs: u64,
}

impl Request {
    pub fn timeout(self, secs: u64) -> Self {
        Request { timeout_secs: secs, ..self }
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::return_self_not_must_use bad.rs — clippy 0.1.98"
warning: missing `#[must_use]` attribute on a method returning `Self`
 --> bad.rs:7:5
  |
7 | /     pub fn timeout(self, secs: u64) -> Self {
8 | |         Request { timeout_secs: secs, ..self }
9 | |     }
  | |_____^
  |
  = help: consider adding the `#[must_use]` attribute to the method or directly to the `Self` type
```

**Silent:**

```rust
pub struct Request {
    pub url: String,
    pub timeout_secs: u64,
}

impl Request {
    #[must_use]
    pub fn timeout(self, secs: u64) -> Self {
        Request { timeout_secs: secs, ..self }
    }
}
```

**When it is right.** Right for a builder-style method. It changes nothing in place and returns the changed value, so `request.timeout(5);` as a statement moves `request` into the call and drops the answer. With the attribute, rustc's [`unused_must_use`](#unused_must_use) flags that statement: *unused return value of `Request::timeout` that must be used*. It looks at public methods of a library only; the same code as a binary, with private methods, printed nothing.

### `large_stack_arrays`

**clippy · pedantic** · off by default · fires on the `[7; 100_000]` a function builds to return

```rust
fn make_table() -> [u64; 100_000] {
    [7; 100_000]
}

fn main() {
    let table = make_table();
    println!("{}", table[99_999]); // 7
}
```

```text title="clippy-driver --edition 2024 -W clippy::large_stack_arrays bad.rs — clippy 0.1.98"
warning: allocating a local array larger than 16384 bytes
 --> bad.rs:2:5
  |
2 |     [7; 100_000]
  |     ^^^^^^^^^^^^
  |
  = help: consider allocating on the heap with `vec![7; 100_000].into_boxed_slice()`
```

**Silent:**

```rust
fn make_table() -> Vec<u64> {
    vec![7; 100_000]
}

fn main() {
    let table = make_table();
    println!("{}", table[99_999]); // 7
}
```

**When it is right.** It points at the array expression, not at the return type, so it sees a returned array only when the body spells one out. The same 800,000 bytes built with `std::array::from_fn(|_| 7)` passed this lint. Right wherever a thread may have a small stack, which [Where an array lives](../../26_Collections/arrays/where_an_array_lives/README.md#big-arrays-and-the-stack) measures; the local-array version of this entry is on [the arrays lints page](../../26_Collections/arrays/array_lints/README.md#large_stack_arrays).

### `large_stack_frames`

**clippy · nursery** · off by default · fires on a function whose frame, return slot included, passes 512,000 bytes

```rust
fn make_table() -> [u64; 100_000] {
    [7; 100_000]
}

fn main() {
    let table = make_table();
    println!("{}", table[99_999]); // 7
}
```

```text title="clippy-driver --edition 2024 -W clippy::large_stack_frames bad.rs — clippy 0.1.98"
warning: this function may allocate 800000 bytes on the stack
 --> bad.rs:1:4
  |
1 | fn make_table() -> [u64; 100_000] {
  |    ^^^^^^^^^^      -------------- this is the largest part, at 800000 bytes for type `[u64; 100000]`
  |
  = note: 800000 bytes is larger than Clippy's configured `stack-size-threshold` of 512000
  = note: allocating large amounts of stack space can overflow the stack and cause the program to abort

warning: this function may allocate 800121 bytes on the stack
 --> bad.rs:5:4
  |
5 | fn main() {
  |    ^^^^
6 |     let table = make_table();
  |         ----- `table` is the largest part, at 800000 bytes for type `[u64; 100000]`
  |
  = note: 800121 bytes is larger than Clippy's configured `stack-size-threshold` of 512000
```

**Silent:**

```rust
fn make_table() -> Vec<u64> {
    vec![7; 100_000]
}

fn main() {
    let table = make_table();
    println!("{}", table[99_999]); // 7
}
```

**When it is right.** The one lint here that reads a return type as room. The first warning's *largest part* is the `[u64; 100000]` that `make_table` hands back, and the second is `table`, the room `main` reserves for it before the call. Its line is a fixed 512,000 bytes; the stack it guards is chosen at run time: 8 MiB for the main thread here (`ulimit -s` printed 8192), 2 MiB for a spawned thread by [std's current default ↗](https://doc.rust-lang.org/std/thread/index.html#stack-size), and whatever `thread::Builder::stack_size` asks for. It is in *nursery*, so expect false positives.

### `large_types_passed_by_value`

**clippy · pedantic** · off by default · fires on a large `Copy` parameter, and never the same type returned

```rust
#[derive(Clone, Copy)]
struct Frame {
    pixels: [u8; 1024],
}

fn blank() -> Frame {
    Frame { pixels: [0; 1024] }
}

fn brightest(frame: Frame) -> u8 {
    frame.pixels.iter().copied().max().unwrap_or(0)
}

fn main() {
    println!("{}", brightest(blank())); // 0
}
```

```text title="clippy-driver --edition 2024 -W clippy::large_types_passed_by_value bad.rs — clippy 0.1.98"
warning: this argument (1024 byte) is passed by value, but might be more efficient if passed by reference (limit: 256 byte)
  --> bad.rs:10:21
   |
10 | fn brightest(frame: Frame) -> u8 {
   |                     ^^^^^ help: consider passing by reference instead: `&Frame`
   |
```

**Silent:**

```rust
#[derive(Clone, Copy)]
struct Frame {
    pixels: [u8; 1024],
}

fn blank() -> Frame {
    Frame { pixels: [0; 1024] }
}

fn brightest(frame: &Frame) -> u8 {
    frame.pixels.iter().copied().max().unwrap_or(0)
}

fn main() {
    println!("{}", brightest(&blank())); // 0
}
```

**When it is right.** Right for a large `Copy` value that is only read. What the transcript leaves out is the reason it is on this page: `blank` returns the same 1,024 bytes by value, on line 6, and gets no warning. The lint reads parameters only, and only `Copy` ones; without `#[derive(Clone, Copy)]` the `Frame` parameter was silent too.

## What no lint catches

Two ways to return by value that cost more than they need to:

```rust
use std::fmt::Display;

struct User {
    name: String,
}

impl User {
    fn name(&self) -> String {
        self.name.clone()
    }
}

fn label(n: i32) -> Box<dyn Display> {
    Box::new(n)
}

fn main() {
    let user = User { name: String::from("ferris") };
    println!("{} {}", user.name(), label(7)); // ferris 7
}
```

```text title="clippy-driver --edition 2024 -W clippy::all -W clippy::pedantic -W clippy::nursery -W rust_2018_idioms -W unused no_lint.rs; echo exit $? — clippy 0.1.98"
exit 0
```

Nothing but the exit status. `name` allocates a new `String` on every call where `-> &str` would lend the one it has, and `label` puts an `i32` on the heap where `-> impl Display` would return it in the caller's room. `unnecessary_box_returns` was on for this run, as part of `pedantic`, and passed over `Box<dyn Display>` although `label` only ever returns an `i32`.

And one that fits under every threshold on this page:

```rust
fn make_table() -> [u64; 50_000] {
    std::array::from_fn(|i| i as u64)
}

fn main() {
    let worker = std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| make_table()[49_999])
        .unwrap();
    println!("{}", worker.join().unwrap());
}
```

```text title="clippy-driver --edition 2024 -W clippy::all -W clippy::pedantic -W clippy::nursery -W rust_2018_idioms -W unused small_stack.rs; echo exit $? — clippy 0.1.98"
exit 0
```

```text title="rustc --edition 2024 small_stack.rs && ./small_stack; echo exit $? — rustc 1.98.0, macOS, zsh"

thread '<unknown>' (31640211) has overflowed its stack
fatal runtime error: stack overflow, aborting
exit 134
```

`make_table` returns 400,000 bytes by value: under `large_stack_frames`' 512,000, and built without an array expression for `large_stack_arrays` to point at. The thread it runs on has 64 KiB, so the room for the return value is not there, and the process aborts. The number in brackets is the thread's id and changes on every run. No lint knows how big a thread's stack will be; only the code that spawns it does.

Adding `-W clippy::restriction` to either program brings warnings about implicit `return`, missing docs, `println!`, `unwrap` and `as` casts, and none about what the functions return.

## See also

- [Every returned-by-value error, and its fix](../returned_by_value_errors/README.md) — what the compiler refuses, where this page is what it only warns about
- [Returned by value](../returned_by_value/README.md) — the room the caller reserves, which `large_stack_frames` measures
- [Returned by value: reading](../returned_by_value_resources/README.md) — where each of these is explained at length
- [Step 4: `Clone` hands back `Self`](../../12_Traits/how_to_learn_to_owned/clone_returns_self/README.md) — why `noop_method_call` fires on a `&str`
- [The call stack](../the_call_stack/README.md) — the frame a large return value has to fit in
- [Stack and heap](../stack_and_heap/README.md) — what moving a value to the heap with `Box` or `Vec` costs, and saves
- [Returning a trait](../../12_Traits/returning_a_trait/README.md) — `impl Trait` against `Box<dyn Trait>`, the choice no lint makes for you
- [Lints around arrays](../../26_Collections/arrays/array_lints/README.md) — `large_stack_arrays` and `large_types_passed_by_value` on arrays that are not returned
- [Lints around `ToOwned`](../../12_Traits/how_to_learn_to_owned/to_owned_lints/README.md) — the same kind of page for `clone`, `to_owned` and `Cow`
- [Strict clippy](../../05_Tooling/strict_lints/README.md) — turning whole lint groups on for a project, and what that costs

## Po polsku

Osiem lintów rustc i clippy związanych ze zwracaniem przez wartość: dla każdego program, który go wywołuje, dokładny komunikat (rustc 1.98.0, clippy 0.1.98), wersja bez ostrzeżenia i kiedy lint ma rację. `noop_method_call` (`.clone()` na `&str` zwraca samą referencję), `unused_must_use` (wynik zwrócony przez wartość i wyrzucony) i `result_large_err` działają domyślnie; `unnecessary_box_returns`, `return_self_not_must_use`, `large_stack_arrays` i `large_stack_frames` trzeba włączyć. `large_types_passed_by_value` ostrzega o dużym argumencie, ale nigdy o tej samej wartości zwracanej.

Najważniejsza jest sekcja o tym, czego żaden lint nie łapie: `Box<dyn Display>` tam, gdzie wystarczy `impl Display`, getter klonujący `String` zamiast pożyczyć `&str`, oraz tablica 400 000 bajtów zwracana przez wartość, która mieści się pod każdym progiem clippy, a mimo to przepełnia stos wątku o rozmiarze 64 KiB.

**Szukaj po polsku:** `clippy result_large_err` · `clippy unnecessary_box_returns` · `clippy large_stack_frames` · `rust noop_method_call clone &str`
