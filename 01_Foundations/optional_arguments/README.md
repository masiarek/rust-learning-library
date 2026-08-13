# Optional function arguments

**Level:** 201 · working knowledge

**One line:** Rust has no default parameters and no overloading, so "this argument is optional" must be built out of something — and `Option<T>` is only one of five ways, usually not the best.

This is the last of the eight jobs [`std::option`](https://doc.rust-lang.org/core/option/) lists, and the one where reaching for `Option` first does the most damage, because the cost lands on every call site rather than on you.

---

## The constraint

```rust
fn connect(host: &str, port: u16 = 443) { … }   // not Rust. There are no default arguments.
fn connect(host: &str) { … }                     // and you cannot overload:
fn connect(host: &str, port: u16) { … }          // error: duplicate definition
```

Both omissions are deliberate. Default arguments interact badly with traits and inference, and overloading makes "which function did I just call?" a question you cannot answer by reading. So Rust makes you pick a shape, and there are five.

## 1. Two functions

```rust
Vec::new()              HashMap::new()
Vec::with_capacity(n)   HashMap::with_capacity(n)
```

The standard library's own answer when there are exactly two shapes, and the right default for one or two variants. **Two names beat one name plus a `None` nobody can read.** The second name also gets to say *why* it exists, which `None` never does.

## 2. An `Option<T>` parameter

```rust
fn connect(host: &str, port: Option<u16>, timeout: Option<u32>) -> String
```

Now look at what a caller writes:

```rust
connect("a.io", None, None)
connect("a.io", None, Some(5))
```

Every call site must spell out every argument, including the ones it has no opinion about — and `None, None` says nothing about *which* knobs were skipped. Two `None`s are tolerable; five are a puzzle, and reordering the parameters later silently changes the meaning of every existing call.

**When it is genuinely right:** the caller has a runtime value-or-not, not a literal. `connect(host, config.port, None)` is fine, because `config.port` really is an `Option` that came from somewhere. The tell that you have chosen wrong is that almost every call site writes the literal `None` — that argument does not want to be optional, it wants to not be there.

## 3. Take `Option<&T>`, never `&Option<T>`

The sharpest rule on this page, and it costs nothing to follow:

```rust
fn greet(name: Option<&str>)      // ✓ accepts strictly more callers
fn greet(name: &Option<String>)   // ✗ demands a real Option to point at
```

A caller holding a plain `&str` can call the first immediately. To call the second they must *build* an `Option<String>` — allocating a `String` and wrapping it — purely to satisfy the signature. And a caller who does hold an `Option<String>` reaches the first with one call, `.as_deref()`.

`&Option<T>` is almost always a signature written from the callee's point of view. Convert at the boundary with `.as_ref()` / `.as_deref()` and take the flexible one.

## 4. `impl Into<Option<T>>`

```rust
fn retries(times: impl Into<Option<u32>>) -> u32 {
    times.into().unwrap_or(3)
}

retries(5)      // 5
retries(None)   // 3
```

This works because `impl From<T> for Option<T>` exists, so a bare `5` converts to `Some(5)`. It is the closest Rust gets to an optional argument at the call site.

It is also rare, on purpose. It weakens type inference, it does not compose past one or two arguments, and the signature stops telling a reader what to pass. Know it so you recognise it in someone else's API; reach for it seldom.

## 5. An options struct, or a builder

For three or more knobs, give the arguments their names back:

```rust
connect_with("a.io", ConnectOpts { timeout: Some(5), ..Default::default() })
```

`timeout: Some(5)` says at the call site exactly which knob was turned, and adding a fourth option later does not touch a single existing caller. That last property is what makes this the scaling answer.

The builder is the same idea with a fluent face, and is what you want when construction has stages or validation:

```rust
RequestBuilder::new("https://a.io").method("POST").retries(5).build()
```

**Notice there is no `Option` in the builder at all.** Its defaults are ordinary values it already holds. That is the general lesson: `Option` belongs in these designs only when *unset* is meaningfully different from *any particular value* — the same question as [`Option` fields](../option_fields/README.md).

## Choosing

| Situation | Reach for |
|---|---|
| Exactly two shapes | Two functions |
| One knob, and callers really have a runtime `Option` | An `Option<T>` parameter |
| Any borrowed optional argument | `Option<&T>` — never `&Option<T>` |
| Three or more knobs | An options struct with `Default` |
| Many knobs, staged construction, or validation | A builder |

---

## Practice

**One optional argument, four ways.** Give a `banner(title, width)` function an optional width, implemented twice — as two functions, and as an `Option<usize>` parameter. Then write a `footer` that takes an optional note, and choose between `Option<&str>` and `&Option<String>` deliberately.

Write `footer(note: &Option<String>)` first and then try to call it with a plain `"unofficial"`. The signature you cannot call is the lesson: `&Option<T>` demands the caller already be holding an `Option`, while `Option<&T>` accepts everyone.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:optional_arguments_kata -->
*[`optional_arguments_kata.rs`](examples/optional_arguments_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: four ways to make one argument optional.
//!
//!   rustc --edition 2024 optional_arguments_kata.rs -o /tmp/oak && /tmp/oak

/// 1. Two functions. No cleverness, and the common call stays short.
fn banner(title: &str) -> String {
    banner_with_width(title, 24)
}

fn banner_with_width(title: &str, width: usize) -> String {
    format!("{:*^width$}", format!(" {title} "))
}

/// 2. An Option parameter: one function, and every caller says what it means.
fn banner_opt(title: &str, width: Option<usize>) -> String {
    banner_with_width(title, width.unwrap_or(24))
}

/// 3. `Option<&T>`, never `&Option<T>`. This signature accepts a caller who has
///    an owned String, a &str, or nothing — the other one only accepts a caller
///    who already happens to be holding an Option.
fn footer(note: Option<&str>) -> String {
    match note {
        Some(n) => format!("— {n}"),
        None => "—".to_string(),
    }
}

/// 4. `impl Into<Option<T>>`: the call site drops the `Some`. Convenient, and
///    it costs a reader one indirection to work out what may be passed.
fn banner_into(title: &str, width: impl Into<Option<usize>>) -> String {
    banner_with_width(title, width.into().unwrap_or(24))
}

fn main() {
    println!("1. Two functions:");
    println!("   {}", banner("Results"));
    println!("   {}", banner_with_width("Results", 40));

    println!("\n2. An Option parameter — the call site states the absence:");
    println!("   {}", banner_opt("Results", None));
    println!("   {}", banner_opt("Results", Some(40)));

    println!("\n3. Option<&T> takes callers the other shape cannot:");
    let owned = String::from("461 ballots");
    println!("   {}", footer(Some(&owned)));
    println!("   {}", footer(Some("unofficial")));
    println!("   {}", footer(None));
    println!("      A `&Option<String>` parameter would have rejected the middle");
    println!("      line, and forced the caller to build an Option to say nothing.");

    println!("\n4. impl Into<Option<usize>> — no Some at the call site:");
    println!("   {}", banner_into("Results", 40));
    println!("   {}", banner_into("Results", None));
}
```
<!-- /source -->

<!-- output:optional_arguments_kata -->
*Verified output of [`optional_arguments_kata.rs`](examples/optional_arguments_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Two functions:
   ******* Results ********
   *************** Results ****************

2. An Option parameter — the call site states the absence:
   ******* Results ********
   *************** Results ****************

3. Option<&T> takes callers the other shape cannot:
   — 461 ballots
   — unofficial
   —
      A `&Option<String>` parameter would have rejected the middle
      line, and forced the caller to build an Option to say nothing.

4. impl Into<Option<usize>> — no Some at the call site:
   *************** Results ****************
   ******* Results ********
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:optional_arguments -->
*Verified output of [`optional_arguments.rs`](examples/optional_arguments.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: No default arguments — so std writes a second function
  open("a.txt")                     -> a.txt [read]
  open_with_mode("a.txt", "append") -> a.txt [append]
      This is Vec::new / Vec::with_capacity, HashMap::new / with_capacity.
      Two names beat one name plus a None nobody can read.

──── Step 2: The Option parameter, and what it costs the CALLER
  connect("a.io", None, None)          -> a.io:443 (timeout 30s)
  connect("a.io", Some(80), None)      -> a.io:80 (timeout 30s)
  connect("a.io", None, Some(5))       -> a.io:443 (timeout 5s)
      Every call site must spell out every argument, including the ones it
      does not care about — and `None, None` says nothing about WHICH knobs
      were skipped. Two Nones is tolerable; five is a puzzle.

──── Step 3: Option<&T> in argument position, never &Option<T>
  greet_flexible(Some("Ben"))       -> hello, Ben
  greet_flexible(owned.as_deref())  -> hello, Ada
  greet_flexible(None)              -> hello, stranger
  greet_rigid(&owned)               -> hello, Ada
      greet_rigid demands a real Option<String> to point at. A caller holding
      a plain &str has to BUILD one — allocating a String and an Option — to
      call it. Option<&T> takes both, so it is the one to write.

──── Step 4: impl Into<Option<T>>: pass the bare value OR None
  retries(5)     -> 5
  retries(None)  -> 3
      `impl From<T> for Option<T>` exists, so 5 converts to Some(5).
      Cute, and rare on purpose: it weakens inference, it does not compose
      past one or two arguments, and the signature stops being obvious.

──── Step 5: An options struct: the arguments get their names back
  a.io:443 (timeout 30s, 3 retries)
  a.io:443 (timeout 5s, 3 retries)
      `timeout: Some(5)` says at the call site which knob was turned.
      Adding a fourth option later does not touch a single existing caller.

──── Step 6: The builder: for when there are many, and defaults are real values
  POST https://a.io (5 retries)
  GET https://a.io (3 retries)   <- untouched defaults
      Note there is no Option anywhere: the defaults are ordinary values held
      by the builder. Option only appears when 'unset' differs from any value.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/optional_arguments/examples/optional_arguments.rs -o /tmp/oa && /tmp/oa
```

## See also

- [`Option` fields](../option_fields/README.md) — the same *unset vs. any value* question, applied to data
- [Initial values](../initial_values/README.md) — another job where `Option` is the reflex and usually the wrong one
- [`Option::as_deref`](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref) — how a caller reaches an `Option<&T>` parameter
