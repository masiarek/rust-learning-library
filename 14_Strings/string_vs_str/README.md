# `String` vs `&str`

**Level:** 101 → 201 · working knowledge

**One line:** `String` owns text and can grow it; `&str` is a borrowed view of text that lives anywhere — the same owner-and-view split as `Vec<T>` and `&[T]`, and the reason a function that only reads text should take `&str`.

| | `String` | `&str` |
|---|---|---|
| owns its bytes | yes — and frees them | no — it is looking at someone else's |
| where it sits | three words on the stack, bytes on the heap | two words (pointer + length), pointing anywhere |
| can grow | yes, with `mut` | never |
| `let b = a;` | a **move** — `a` is dead | a **copy** — both views alive |
| you meet it as | struct fields, text you build | parameters, literals, slices |

---

## Three ways to have text

```rust
let literal = "The Big Lebowski";   // &'static str — the bytes are in the binary
let owned = String::from(literal);  // a fresh heap buffer, yours to grow
let view = &owned[4..7];            // &str — a window into `owned`, nothing copied
```

All three print the same way and compare the same way. What differs is who owns the bytes: the *program* (for as long as it runs), *you* (until `owned` drops), and *nobody* — a view owns nothing and may not outlive what it looks at.

**The trap in most tutorials: "a `str` is stack-allocated."** It is not. The text of a literal is baked into your executable's read-only data — not the stack, not the heap — and `&'static str` says exactly that: a view that lives as long as the program does. What sits on the stack is only the view itself, a pointer and a length. The *data* of a `str` can live anywhere: in the binary (a literal), on the heap (inside a `String`), even in a stack array. That is why you only ever meet `str` behind a `&`: it is text-of-unknown-size, wherever text happens to be.

## What each one costs

The machine's view, from the [verified output](#the-verified-output) below:

```rust
size_of::<&str>()    // 16 — a pointer and a length
size_of::<String>()  // 24 — a pointer, a length, and a capacity
size_of::<&String>() //  8 — just a pointer, to the three words above
```

Two words versus three: a `String`'s extra word is `capacity`, the room it bought for growing — [The anatomy of a `String`](../anatomy_of_a_string/README.md) opens that up.

## One parameter serves every caller

```rust
fn shout(s: &str) -> String {
    s.to_uppercase()
}

shout(literal);   // a &str already
shout(&owned);    // &String, coerced to &str — free
shout(view);      // a &str already
```

The middle call is the important one. `String` implements `Deref<Target = str>`, so a `&String` **coerces** to `&str` at the call site: no conversion, no copy, no method to remember. It also means a `String` *inherits* `str`'s methods — `owned.to_uppercase()` works because the method is on `str` and the coercion is automatic.

The reverse direction is never free: `&str → String` allocates a buffer and copies the bytes, and you ask for it out loud with `.to_string()` (or `.to_owned()`).

So the API rule almost every Rust codebase follows: **parameters take `&str`, because every caller can afford one.** A `fn f(s: String)` forces each call site to allocate, clone, or surrender its value — the kata below makes you pay each of those once.

## Which one do I write?

| you are writing | reach for | because |
|---|---|---|
| a parameter that reads text | `&str` | literals, `String`s and slices all arrive free |
| a struct field | `String` | the struct outlives the call that built it — a `&str` field drags in [lifetimes](../../01_Foundations/how_to_learn_lifetimes/README.md) |
| a return of freshly built text | `String` | someone must own the new bytes, and it is not the callee |
| a constant | `&'static str` | the binary already owns the bytes |

The field row is why `String` fills beginner code where `&str` looks tidier — and it is the right call: own by default, borrow when a signature lets you. [What is a ballot, in memory?](../../01_Foundations/representing_a_ballot/README.md) makes that choice inside a real struct.

## `String` moves, `&str` copies

The same two lines, twice — one compiles, one does not:

```rust
let s1 = "hello";
let s2 = s1;
println!("{s1} {s2}");   // hello hello — a &str is Copy: two views, no owner

let big1 = String::from("hello");
let big2 = big1;         // the buffer changed owners
// println!("{big1}");   // error[E0382]: borrow of moved value: `big1`
```

[Ownership and moves](../../01_Foundations/ownership_and_moves/README.md) is the full story of that error. The string-specific half: duplicating a `&str` duplicates a *view* — sixteen bytes that owe nothing — while duplicating a `String` would duplicate an *obligation*, two owners for one free. Same reason [a struct with a `String` field is never `Copy`](../../01_Foundations/copy_vs_clone/README.md).

## If you are coming from another language

**Python.** One `str` type does every job, because the runtime referee — the reference count — decides when text is freed. Rust splits the job in two at compile time instead.

| Python | | Rust |
|---|---|---|
| `s = "hi"` | one type for all text | two types: the owner and the view |
| `t = s` | a second reference; refcount +1 | `String`: a move · `&str`: a copy of the view |
| `s[4:7]` | a **new string**, copied out | `&s[4..7]` — a view, nothing copied |
| immutable | every `str`, always | the view is read-only; the owner may be `mut` |

The habit that transfers: Python slices cost an allocation each, so you learned not to slice in a loop. Rust's `&str` removes the cost — a slice is a window — and the compiler enforces what the window may not do: outlive the text, or watch it while it changes.

**ABAP.** `string` is dynamic text, and a field symbol is a view into data you did not copy — the same two roles, checked at different moments.

| ABAP | | Rust |
|---|---|---|
| `DATA lv TYPE string` | dynamic, growable text | `String` |
| `lv+4(3)` | offset/length substring access | `&lv[4..7]` |
| `FIELD-SYMBOLS <fs>` assigned into data | a view, no copy | `&str` |
| `TYPE c LENGTH 20` | fixed-width character field | no direct equivalent — closest is `[u8; 20]` |

What changes: an unassigned or stale field symbol is a runtime dump (`GETWA_NOT_ASSIGNED`), found in the debugger, in production, on a Friday. A `&str` that could outlive its `String` is a compile error — the same bug class, moved to the moment you can still fix it cheaply.

---

## Practice

**One `&str` parameter, three callers — then flip it.** Write `first_word(s: &str) -> &str` returning the text before the first space. Call it with a string literal, with a `String`, and with a slice of that `String`; confirm the `String` is still usable afterwards.

Then change the parameter to `String` and make all three call sites compile again. Catalogue what each one now has to do — and which call site kills the variable it was built from.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:string_vs_str_kata -->
*[`string_vs_str_kata.rs`](examples/string_vs_str_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the signature decides who pays.
//!
//!   rustc --edition 2024 string_vs_str_kata.rs -o /tmp/svsk && /tmp/svsk

/// Borrow in, borrow out: the answer is a window into the caller's own text.
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

/// The same job with an owning signature — every caller must now surrender a String.
fn first_word_owned(s: String) -> String {
    s.split_whitespace().next().unwrap_or("").to_string()
}

fn main() {
    println!("Round 1 — fn first_word(s: &str) -> &str");
    let literal = "score then automatic runoff";
    let owned = String::from("equal preference is allowed");
    println!("   from a literal        {:?}", first_word(literal));
    println!("   from a String         {:?}   <- &owned, coerced for free", first_word(&owned));
    println!("   from a slice of one   {:?}", first_word(&owned[6..]));
    println!("   and `owned` is still usable afterwards: {} bytes", owned.len());

    println!("\nRound 2 — fn first_word_owned(s: String) -> String");
    println!("   from a literal        {:?}   <- had to allocate with .to_string()", first_word_owned(literal.to_string()));
    println!("   from a String         {:?}   <- had to .clone() to keep `owned`", first_word_owned(owned.clone()));
    println!("   from a slice of one   {:?}   <- allocate again", first_word_owned(owned[6..].to_string()));
    println!("   or hand it over:      {:?}   <- moved; `owned` is gone (E0382 next use)", first_word_owned(owned));

    println!("\nThe catalogue:");
    println!("   &str parameter:   every caller pays nothing — a literal, a String,");
    println!("                     and a slice all coerce or borrow for free.");
    println!("   String parameter: every caller pays — an allocation, a clone, or");
    println!("                     the value itself. Take &str unless you must own it.");
}
```
<!-- /source -->

<!-- output:string_vs_str_kata -->
*Verified output of [`string_vs_str_kata.rs`](examples/string_vs_str_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Round 1 — fn first_word(s: &str) -> &str
   from a literal        "score"
   from a String         "equal"   <- &owned, coerced for free
   from a slice of one   "preference"
   and `owned` is still usable afterwards: 27 bytes

Round 2 — fn first_word_owned(s: String) -> String
   from a literal        "score"   <- had to allocate with .to_string()
   from a String         "equal"   <- had to .clone() to keep `owned`
   from a slice of one   "preference"   <- allocate again
   or hand it over:      "equal"   <- moved; `owned` is gone (E0382 next use)

The catalogue:
   &str parameter:   every caller pays nothing — a literal, a String,
                     and a slice all coerce or borrow for free.
   String parameter: every caller pays — an allocation, a clone, or
                     the value itself. Take &str unless you must own it.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:string_vs_str -->
*Verified output of [`string_vs_str.rs`](examples/string_vs_str.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three ways to have text
   literal  &'static str  "The Big Lebowski"
   owned    String        "The Big Lebowski"
   view     &str          "Big"   <- bytes 4..7 of `owned`, borrowed

2. What each one costs on the stack
   size_of::<&str>()    = 16 bytes  (pointer + length)
   size_of::<String>()  = 24 bytes  (pointer + length + capacity)
   size_of::<&String>() = 8 bytes  (just a pointer)

3. One &str parameter serves every caller
   shout(literal) = "THE BIG LEBOWSKI"
   shout(&owned)  = "THE BIG LEBOWSKI"   <- &String coerced to &str, free
   shout(view)    = "BIG"

4. The owner can grow; the view never can
   grows = "counted"   <- push_str needs a `mut String`
   a &str has no push_str — the text may not even be yours to change

5. String moves; &str copies
   &str:   s1 = "hello", s2 = "hello"   <- both alive
   String: big2 = "hello", and `big1` is now unusable — E0382
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/string_vs_str/examples/string_vs_str.rs -o /tmp/svs && /tmp/svs
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — what the three words on the stack actually are
- [Ownership and moves](../../01_Foundations/ownership_and_moves/README.md) — `E0382` in full, with a value that announces its own death
- [Borrowing](../../01_Foundations/borrowing/README.md) — the rules every `&str` lives under
- [The Rust Book, ch. 4.1 — The `String` Type ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type) · [ch. 8.2 — Storing UTF-8 Encoded Text ↗](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Easy Rust, ch. 14 — Strings ↗](https://dhghomon.github.io/easy_rust/Chapter_14.html) — the gentlest second telling
