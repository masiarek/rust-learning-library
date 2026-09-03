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

**The second trap, and the one even good answers repeat: "a `str` is immutable."** It is not — it is fixed-**length**. [`String::as_mut_str`](../string_methods/string_as_mut_str/README.md) hands you a `&mut str`, and [`make_ascii_uppercase`](../str_methods/str_make_ascii_uppercase/README.md) rewrites those bytes in place with no allocation; [`split_at_mut`](../str_methods/str_split_at_mut/README.md) will even hand you two `&mut str` into one buffer. What a `str` may never do is change *length*, because UTF-8 is variable-width — swapping an `a` for an `ä` needs a byte that is not there, and a view cannot reallocate what it does not own. So `&str` is read-only because the `&` is, not because `str` is. [Section 6 below](#the-verified-output) mutates one.

**The third trap, the one a diagram teaches best because it can draw an equals sign: "`String` *is* a `Vec<u8>`, `&str` *is* a `&[u8]`."** The layout half is true, and std says so in both places — `String` is declared `struct String { vec: Vec<u8> }`, and the safety comment inside [`as_bytes_mut`](../str_methods/str_as_bytes_mut/README.md) reads *"the cast from `&str` to `&[u8]` is safe since `str` has the same layout as `&[u8]`"* — followed by the clause that settles the argument: **only std can make this guarantee**. For your code they are two types with two different promises: bytes are bytes, a `str` is bytes *that are valid UTF-8*, and every method on `str` is entitled to assume it. So the trip is free one way and checked the other — [`as_bytes`](../str_methods/str_as_bytes/README.md) hands back a view of the same memory for nothing, while [`str::from_utf8`](../str_methods/str_from_utf8/README.md) scans and returns a `Result`. A `&[u8]` will not go where a `&str` is wanted, however identical the two are in memory; that is `E0308`, and it is the type system holding a promise the layout cannot. [Section 7 below](#the-verified-output) makes both trips. [Six kinds of string](../six_kinds_of_string/README.md) is this same idea widened: three promises, each with an owner and a view.

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

**The one exception worth knowing:** if the body always calls `.to_owned()` on the parameter anyway — it stores the text in a struct, sends it to a thread, returns it — then taking `&str` does not save the allocation, it just moves it to the caller and takes away their choice. A caller holding a `String` it no longer needs could have handed the buffer over for free; instead it watches you copy it. Take `String` there, and let the caller with only a literal write `.to_string()` at the call site, where the cost is visible.

## Which one do I write?

| you are writing | reach for | because |
|---|---|---|
| a parameter that reads text | `&str` | literals, `String`s and slices all arrive free |
| a struct field | `String` | the struct outlives the call that built it — a `&str` field drags in [lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) |
| a return of freshly built text | `String` | someone must own the new bytes, and it is not the callee |
| a constant | `&'static str` | the binary already owns the bytes |

The field row is why `String` fills beginner code where `&str` looks tidier — and it is the right call: own by default, borrow when a signature lets you. [What is a record, in memory?](../../16_Structs/representing_a_record/README.md) makes that choice inside a real struct.

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

[Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) is the full story of that error. The string-specific half: duplicating a `&str` duplicates a *view* — sixteen bytes that owe nothing — while duplicating a `String` would duplicate an *obligation*, two owners for one free. Same reason [a struct with a `String` field is never `Copy`](../../16_Structs/copy_vs_clone/README.md).

## If you are coming from another language

**Python.** One `str` type does every job, because the runtime referee — the reference count — decides when text is freed. Rust splits the job in two at compile time instead.

| Python | | Rust |
|---|---|---|
| `s = "hi"` | one type for all text | two types: the owner and the view |
| `t = s` | a second reference; refcount +1 | `String`: a move · `&str`: a copy of the view |
| `s[4:7]` | a **new string**, copied out | `&s[4..7]` — a view, nothing copied |
| immutable | every `str`, always | `&str` is read-only, but `str` is only fixed-**length** — see the trap above |
| `s.encode()` / `b.decode()` | text ⇄ bytes, and only the way back can fail | `as_bytes()` is free · `str::from_utf8` returns a `Result` — same asymmetry, no exception |

The habit that transfers: Python slices cost an allocation each, so you learned not to slice in a loop. Rust's `&str` removes the cost — a slice is a window — and the compiler enforces what the window may not do: outlive the text, or watch it while it changes.

**ABAP.** `string` is dynamic text, and a field symbol is a view into data you did not copy — the same two roles, checked at different moments.

| ABAP | | Rust |
|---|---|---|
| `DATA lv TYPE string` | dynamic, growable text | `String` |
| `lv+4(3)` | offset/length substring access | `&lv[4..7]` |
| `FIELD-SYMBOLS <fs>` assigned into data | a view, no copy | `&str` |
| `TYPE c LENGTH 20` | fixed-width character field | no direct equivalent — closest is `[u8; 20]` |
| `cl_abap_codepage=>convert_to` / `convert_from` | `string` ⇄ `xstring`, and only the way back can fail | `as_bytes()` · `str::from_utf8` — the failure is a `Result`, not a raise |

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

**The pivot, the lifetime, and the reference that cannot leave.** Write the two conversions — `fn to_owned_text(s: &str) -> String` and `fn to_borrowed_text(s: &String) -> &str` — and say which one allocates and why the other cannot. Then give a `User` struct a `username: &str` field, read the `E0106` it earns, add the lifetime the compiler suggests, and write one sentence saying what `<'a>` now forbids.

Rewrite the same struct to own a `String`, instantiate it, and move it into a second variable — then try to read the first. Next, write a function that returns a `&str` into a `String` it created itself, read the `E0515`, and fix it by changing the *return type* rather than the lifetime. Finish with [`Cow`](../../18_Ownership/clone_on_write/README.md): lowercase only when there is something to lowercase, and prove the already-lowercase input never allocated.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:string_ownership_kata -->
*[`string_ownership_kata.rs`](examples/string_ownership_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the five moves — pivot, lifetime, own, dangle, and Cow.
//!
//!   rustc --edition 2024 string_ownership_kata.rs -o /tmp/sok && /tmp/sok

use std::borrow::Cow;

// 1. The conversion pivot ----------------------------------------------------
//
// Two functions, opposite directions, and only one of them allocates.

/// `&str` in, `String` out: the only way out of a borrow is a copy of the bytes.
fn to_owned_text(s: &str) -> String {
    s.to_string()
}

/// `&String` in, `&str` out: no allocation, no copy — a view of bytes that
/// already exist. `s.as_str()`, `&s[..]` and a bare `s` (deref coercion at the
/// call site) are all the same thing; `as_str` is the one that says so.
fn to_borrowed_text(s: &String) -> &str {
    s.as_str()
}

// 2. A struct that borrows needs a lifetime ----------------------------------
//
// Written without one, it does not compile:
//
//   struct User {
//       username: &str,
//   }
//
//   error[E0106]: missing lifetime specifier
//    --> e0106.rs:2:15
//     |
//   2 |     username: &str,
//     |               ^ expected named lifetime parameter
//     |
//   help: consider introducing a named lifetime parameter
//     |
//   1 ~ struct User<'a> {
//   2 ~     username: &'a str,
//     |
//
// The compiler is not asking for decoration. A field holding a reference means
// the struct is only valid while the borrowed text is, and `'a` is where you
// write that down: `UserRef<'a>` may not outlive the string it points into.

struct UserRef<'a> {
    username: &'a str,
    ballots: u32,
}

impl<'a> UserRef<'a> {
    /// The elided lifetime on the return is `&self`'s, not `'a` — either works
    /// here, and `&self`'s is the more conservative of the two.
    fn name(&self) -> &str {
        self.username
    }
}

// 3. The same struct, owning its text ----------------------------------------

#[derive(Debug)]
struct User {
    username: String,
    ballots: u32,
}

// 4. The dangling reference --------------------------------------------------
//
// The function that cannot exist:
//
//   fn label() -> &'static str {
//       let s = String::from("Ada Lovelace");
//       &s
//   }
//
//   error[E0515]: cannot return reference to local variable `s`
//    --> e0515.rs:3:5
//     |
//   3 |     &s
//     |     ^^ returns a reference to data owned by the current function
//
// `s` is dropped at the closing brace, so the reference would point at freed
// memory. The fix is not a longer lifetime — no lifetime can outlive the drop.
// It is to change the return type and hand over the buffer itself.

fn label() -> String {
    let s = String::from("Ada Lovelace");
    s
}

// 5. Cow: borrow until somebody writes ---------------------------------------

/// Lowercase only when there is something to lowercase. The return type is one
/// type with two shapes, so the caller writes one line either way.
fn normalise(s: &str) -> Cow<'_, str> {
    if s.chars().any(char::is_uppercase) {
        Cow::Owned(s.to_lowercase())
    } else {
        Cow::Borrowed(s)
    }
}

fn which(c: &Cow<'_, str>) -> &'static str {
    match c {
        Cow::Borrowed(_) => "Borrowed — no allocation",
        Cow::Owned(_) => "Owned    — allocated",
    }
}

fn main() {
    println!("1. The conversion pivot");
    let literal = "score then automatic runoff";
    let owned = String::from("equal support is allowed");
    println!("   to_owned_text(&str)      -> String  {:?}", to_owned_text(literal));
    println!("   to_borrowed_text(&String)-> &str    {:?}", to_borrowed_text(&owned));
    println!("   one direction copies the bytes, the other only points at them.");

    println!("\n2. A borrowed field needs a lifetime");
    let name = String::from("ada");
    let borrower = UserRef { username: &name, ballots: 3 };
    println!("   UserRef {{ username: {:?}, ballots: {} }}", borrower.name(), borrower.ballots);
    println!("   `borrower` may not outlive `name` — that is all <'a> says.");

    println!("\n3. The owning version moves");
    let u = User { username: String::from("ada"), ballots: 3 };
    let moved = u;
    // println!("{u:?}");   // error[E0382]: borrow of moved value: `u`
    println!("   after `let moved = u;`  {moved:?}");
    println!("   read through the new name: {} cast {} ballots", moved.username, moved.ballots);
    println!("   `u` is gone: one String field is enough to make the whole struct move.");

    println!("\n4. The reference that cannot leave");
    println!("   label() -> String       {:?}   <- E0515 if it returned &str", label());

    println!("\n5. Cow pays only when it has to");
    for s in ["already lowercase", "Mixed Case Here"] {
        let c = normalise(s);
        println!("   {:<19} -> {:<19} {}", s, format!("{c:?}"), which(&c));
    }
    println!("   Both arms are one type, so the caller never branches: {}", normalise("Ada").len());
}
```
<!-- /source -->

<!-- output:string_ownership_kata -->
*Verified output of [`string_ownership_kata.rs`](examples/string_ownership_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The conversion pivot
   to_owned_text(&str)      -> String  "score then automatic runoff"
   to_borrowed_text(&String)-> &str    "equal support is allowed"
   one direction copies the bytes, the other only points at them.

2. A borrowed field needs a lifetime
   UserRef { username: "ada", ballots: 3 }
   `borrower` may not outlive `name` — that is all <'a> says.

3. The owning version moves
   after `let moved = u;`  User { username: "ada", ballots: 3 }
   read through the new name: ada cast 3 ballots
   `u` is gone: one String field is enough to make the whole struct move.

4. The reference that cannot leave
   label() -> String       "Ada Lovelace"   <- E0515 if it returned &str

5. Cow pays only when it has to
   already lowercase   -> "already lowercase" Borrowed — no allocation
   Mixed Case Here     -> "mixed case here"   Owned    — allocated
   Both arms are one type, so the caller never branches: 3
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

6. `str` is fixed-length, NOT immutable
   through a &mut str:  "PER MARTIN-LOF"   <- no allocation, same buffer
   split_at_mut halves: "PER" + " martin-lof"
   only the first half changed: "PER martin-lof"
   what a str cannot do is change LENGTH — one byte out, one byte in.
   `&str` is read-only because the `&` is, not because `str` is.

7. `str` is not `[u8]` — the same shape, a different promise
   size_of::<&str>() = 16, size_of::<&[u8]>() = 16   <- the same two words
   as_bytes():  "héllo" -> [104, 195, 169, 108, 108, 111]
                6 bytes, 5 chars   <- the bytes forget where the chars were
   and back:    str::from_utf8(bytes) = Ok("héllo")
   str::from_utf8([104, 255, 105]) = Err, valid_up_to = 1
   going the other way is checked, so it hands back a Result, never a &str.
   `shout(bytes)` does not compile: E0308, expected `&str`, found `&[u8]`.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/string_vs_str/examples/string_vs_str.rs -o /tmp/svs && /tmp/svs
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — what the three words on the stack actually are
- [Six kinds of string](../six_kinds_of_string/README.md) — the same owner-and-view split, three times over, for bytes that promise different things
- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — `E0382` in full, with a value that announces its own death
- [Borrowing](../../18_Ownership/borrowing/README.md) — the rules every `&str` lives under
- [100 Exercises — String slices ↗](https://rust-exercises.com/100-exercises/04_traits/06_str_slice) — the same distinction drawn rather than described: three memory diagrams, `String` then `&String` then `&str`, with a test to make pass at the end
- [The Rust Book, ch. 4.1 — The `String` Type ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type) · [ch. 8.2 — Storing UTF-8 Encoded Text ↗](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Easy Rust, ch. 14 — Strings ↗](https://dhghomon.github.io/easy_rust/Chapter_14.html) — the gentlest second telling

## Po polsku

Polskie materiały mówią zwykle o „napisach” i tym jednym słowem zacierają całą różnicę, o którą tu chodzi. Precyzyjniej: `String` to łańcuch znaków, który **jest właścicielem** swoich bajtów i to on je na koniec zwalnia, a `&str` to wycinek łańcucha — sam wskaźnik i długość, patrzące na cudze bajty. Widać to w rozmiarach: `&str` zajmuje 16 bajtów (wskaźnik + długość), `String` 24 (wskaźnik + długość + pojemność), a `&String` tylko 8, bo to sam wskaźnik na te trzy słowa. Pytanie brzmi więc nie „który typ jest napisem”, tylko **kto jest winien zwolnienie pamięci** — i dopiero z tego wynika reszta.

Warto od razu unieszkodliwić zdanie, które krąży po polskich (i angielskich) tutorialach: „`str` leży na stosie”. Nieprawda. Na stosie leży sam widok — dwa słowa maszynowe. Bajty literału siedzą w sekcji tylko do odczytu pliku wykonywalnego, bajty w `String`u na stercie, a `str` może równie dobrze wskazywać na tablicę na stosie. `&'static str` też nie mówi o miejscu, tylko o czasie życia: „ten widok jest ważny tak długo, jak działa program”. Dlatego `str` spotyka się wyłącznie za `&` — to tekst o nieznanym rozmiarze, gdziekolwiek ten tekst akurat jest. Drugie zdanie warte unieszkodliwienia brzmi: „`str` jest niezmienny”. Też nieprawda — on ma **stałą długość**, a to co innego. `String::as_mut_str` zwraca `&mut str`, a `make_ascii_uppercase` nadpisuje bajty w miejscu, bez alokacji; `split_at_mut` potrafi wydać dwa `&mut str` do jednego bufora. Czego `str` nie może, to zmienić *długości*: UTF-8 ma zmienną szerokość, więc podmiana `a` na `ä` wymagałaby bajtu, którego nie ma, a widok nie zrealokuje cudzej pamięci. Innymi słowy: `&str` jest tylko do odczytu dlatego, że `&` jest tylko do odczytu — a nie dlatego, że `str` taki jest.

Konwersja działa za darmo tylko w jedną stronę. `String` implementuje `Deref<Target = str>`, więc `&String` sam z siebie zamienia się w `&str` w miejscu wywołania — to jest *deref coercion*, i przy okazji dlatego `owned.to_uppercase()` w ogóle działa: ta metoda należy do `str`. W drugą stronę zawsze płacisz alokacją i kopią, i musisz o to poprosić głośno: `.to_string()` albo `.to_owned()`. Stąd reguła, którą stosuje praktycznie każda baza kodu w Ruscie: **parametr bierze `&str`**, bo na taki argument stać każdego wołającego. Ćwiczenie na tej stronie pokazuje rachunek dokładnie: przy `fn f(s: String)` literał musi zaalokować przez `.to_string()`, `String` musi się sklonować przez `.clone()`, żeby przeżyć wywołanie, albo oddaje się go na zawsze i przy następnym użyciu dostaje `E0382`.

Odwrotna rada dotyczy pól **struktury** — tu domyślnie własność, czyli `String`. Pole `username: &str` natychmiast wywołuje `E0106` i wciąga czasy życia (`struct UserRef<'a>`), a struktura z takim polem nie może przeżyć tekstu, na który patrzy. To dlatego kod początkujących jest pełen `String`ów tam, gdzie `&str` wyglądałby schludniej — i to jest dobra decyzja. Z tego samego korzenia rośnie różnica w przypisaniu: `let b = a;` na `&str` jest kopią (dwa widoki, żaden nikomu nic nie jest winien), a na `String`u **przeniesieniem własności**, bo dwóch właścicieli oznaczałoby dwa zwolnienia tej samej pamięci. I jeszcze jedno, na co warto się przygotować: funkcja zwracająca `&str` na własny lokalny `String` daje `E0515`, a poprawką **nie jest** dłuższy czas życia — żaden nie przetrwa wypuszczenia zasobu. Poprawką jest zmiana typu zwracanego na `String`, albo `Cow<'_, str>`, gdy chcesz płacić tylko wtedy, gdy naprawdę coś zmieniasz.

**Szukaj po polsku:** łańcuch znaków a wycinek · przenoszenie własności napisu · czas życia w polu struktury · `rust String vs &str` · `rust deref coercion &String to &str`
