# `&'static str`

**Level:** 201 · working knowledge

**One line:** On a string literal, `&str` and `&'static str` are the **same type** — the annotation only starts refusing things when the text is not a literal. `'static` is a promise that the text is never dropped, not a promise that it lives in the binary, and a `String` can keep that promise three different ways.

| you write | on a literal | on a borrow of a local |
|---|---|---|
| `let s = "hi";` | inferred `&'static str` | — |
| `let s: &str = …;` | the same type | compiles, lifetime inferred |
| `let s: &'static str = …;` | the same type | `E0597` |

---

## Three spellings, one type

```rust
let s1 = "hi";                  // inferred
let s2: &str = "hi";            // explicit, lifetime elided
let s3: &'static str = "hi";    // explicit, lifetime named
```

All three have type `&'static str`. The elided lifetime in `s2` is not "some other, shorter lifetime" — it is a hole the compiler fills, and a literal only ever fills it with `'static`.

A tempting way to prove it does not prove anything:

```rust
TypeId::of::<&str>() == TypeId::of::<&'static str>()   // true
```

`TypeId::of::<T>` requires `T: 'static`, so `&str` written in that position **already means** `&'static str`. The comparison is one type against itself. It is true, and it is evidence of nothing.

## The annotation only bites off a literal

Here is where the two spellings stop agreeing:

```rust
let owned = String::from("a local String");
let view: &str = &owned;             // fine
let view: &'static str = &owned;     // error
```

```text
error[E0597]: `owned` does not live long enough
 --> e0597b.rs:3:30
  |
2 |     let owned = String::from("a local String");
  |         ----- binding `owned` declared here
3 |     let view: &'static str = &owned;
  |               ------------   ^^^^^^ borrowed value does not live long enough
  |               |
  |               type annotation requires that `owned` is borrowed for `'static`
4 |     println!("{view}");
5 | }
  | - `owned` dropped here while still borrowed
```

So the rule of thumb: **write `&str`.** Reach for `&'static str` when you want the compiler to *reject* anything borrowed from a local — in a return type, a struct field, or an enum variant that must outlive the call that built it.

## `const`, `static`, `let`

```rust
const  GREETING: &str = "hi";   // inlined at each use site — no address of its own
static BANNER:   &str = "hi";   // one object, one address, alive the whole run
let    local          = "hi";   // a local name for the same bytes in the binary
```

The `&'static str` part is identical in all three, and you can leave it elided in all three: the *text* is in the binary either way. What `const` and `static` differ about is the **reference**, not the text — whether there is one object in memory or a fresh copy substituted at every use. For a `&str` that distinction almost never matters; it matters for anything with interior mutability or an address you compare.

## The claim that is not true

Books and cheat sheets repeat this one:

> `String` can never be borrowed as `&'static str`, because the life of a `String` is never as long as the process.

The second half is a good instinct and the first half is false. Three ways a `String` yields a `&'static str`:

```rust
String::from("built at runtime").leak()                 // &'static mut str, stable since 1.72
Box::leak(String::from("boxed").into_boxed_str())       // the older spelling of the same trick
BUILT.get_or_init(|| String::from("…")).as_str()        // a String living inside a static
```

The first two work by **never freeing the buffer**. That is not a loophole, it is the definition being applied honestly: `'static` means *this will never be dropped*, and one way to guarantee that is to promise not to drop it. The cost is a permanent allocation, which is fine for something computed once at startup and a genuine leak if you do it per row.

The third has no leak at all — the `String` really does live for the whole program, because it lives inside a `static`.

## `'static` the bound is not `'static` the reference

This is the confusion the wording above comes from. `'static` appears in two grammatically similar places that mean different things:

| written | means |
|---|---|
| `&'static str` | this **reference** is valid for the rest of the program |
| `T: 'static` | this **type** contains no borrow that could expire |

`String: 'static` is true. So is `i32: 'static`, and `Vec<String>: 'static`. Every owned type satisfies the bound, because there is no borrow inside it to go stale — that is why `std::thread::spawn` and `Box<dyn Error + 'static>` ask for it, and why a `String` sails through. `T: 'static` does **not** mean "lives forever"; it means "*could*, as far as its type is concerned."

## Where you actually write it

Both of these are idiomatic, and both work for the same reason — every value is a literal:

```rust
fn greeting(hour: u8) -> &'static str {
    if hour < 12 { "Good morning!" } else if hour < 18 { "Good afternoon!" } else { "Good evening!" }
}

enum MyError {
    Io,
    Parse(&'static str),
}
```

They stop working the moment one arm needs to *say something about the input* — `format!("expected a digit, found {c}")` is built at runtime, and returning a reference to it is `E0515`. At that point the type is wrong, not the code: return `String`. The kata below walks all three exits.

## If you are coming from another language

**Python.** There is no lifetime to write, because the reference count decides when text is freed — the nearest thing to `&'static str` is a module-level constant or an interned literal, and the nearest thing to `.leak()` is stuffing a value into a module global so it is never collected.

| Python | | Rust |
|---|---|---|
| `"hi"` in a module | interned, alive for the process | `&'static str`, in the binary |
| `sys.intern(s)` | deliberately keep it forever | `String::leak()` |
| `GREETING = "hi"` | a module global | `static GREETING: &str = "hi";` |
| a stale reference | impossible — refcount holds it | `E0597`, at compile time |

What changes: Python's guarantee is *dynamic* — the object stays alive because something still points at it. Rust's is *static* — the compiler proves at build time that nothing can outlive what it points at, and `'static` is the strongest form of that proof. Nothing keeps a Python string alive by accident in Rust; you either point into the binary, or you own it, or you say `.leak()` out loud.

**ABAP.** The text pool is a genuinely close analogue: a text symbol lives in the program's own storage for the life of the session, exactly like a literal in the binary.

| ABAP | | Rust |
|---|---|---|
| `TEXT-001` / a literal | in the program's text pool | `&'static str` — in the binary |
| `CONSTANTS c_x TYPE string VALUE 'hi'` | fixed at compile time | `const X: &str = "hi";` |
| `CLASS-DATA` on a global class | one object per session | `static X: …` |
| `DATA lv TYPE string` in a method | freed when the method ends | `String`, dropped at end of scope |

What changes: ABAP has no way to say "this reference must outlive the method", so a field symbol pointing at a freed work area is a runtime dump. `&'static str` is that sentence, written in the type, and checked before the program runs.

---

## Practice

**Return a label three ways.** Write `fn label(n: u32) -> &'static str` that returns a label for row `n`, first by building it with `format!` — read the `E0515` you get, and note it is a *different* error from the `E0597` above.

Then make it compile three ways: by matching `n` to a closed set of literals, by leaking the built `String`, and by changing the return type to `String`. For each one, say what it costs and when you would ship it — including which of the three leaks if it is called in a loop.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:static_str_kata -->
*[`static_str_kata.rs`](examples/static_str_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three ways to return a label, and what each one costs.
//!
//!   rustc --edition 2024 static_str_kata.rs -o /tmp/stk && /tmp/stk

// The naive version does not compile:
//
//   fn label_naive(n: u32) -> &'static str {
//       let built = format!("row {n}");
//       &built
//   }
//
//   error[E0515]: cannot return reference to local variable `built`
//    --> src/main.rs:3:5
//     |
//   3 |     &built
//     |     ^^^^^^ returns a reference to data owned by the current function

/// Fix 1 — every answer is a literal, so nothing is built at runtime.
/// Works only because the set of labels is closed.
fn label_fixed(n: u32) -> &'static str {
    match n {
        0 => "header",
        1 => "first row",
        2 => "second row",
        _ => "later row",
    }
}

/// Fix 2 — build it, then promise never to free it.
/// Honest &'static str, and a permanent leak of one allocation per call.
fn label_leaked(n: u32) -> &'static str {
    format!("row {n}").leak()
}

/// Fix 3 — hand the caller the buffer and let them drop it.
/// Not &'static str at all, and almost always the right answer.
fn label_owned(n: u32) -> String {
    format!("row {n}")
}

fn main() {
    println!("Fix 1 — match to literals, -> &'static str");
    for n in [0, 1, 2, 7] {
        println!("   label_fixed({n}) = {:?}", label_fixed(n));
    }
    println!("   cost: nothing. limit: cannot name the row number.");

    println!("\nFix 2 — leak, -> &'static str");
    for n in [0, 1, 2, 7] {
        println!("   label_leaked({n}) = {:?}", label_leaked(n));
    }
    println!("   cost: 4 allocations that are never freed. Called in a loop, that is");
    println!("   an unbounded leak — fine for a value computed once at startup, not");
    println!("   for one computed per row.");

    println!("\nFix 3 — return String");
    for n in [0, 1, 2, 7] {
        let owned = label_owned(n);
        println!("   label_owned({n}) = {:?}  ({} bytes, dropped at end of scope)", owned, owned.len());
    }
    println!("   cost: one allocation per call, freed. This is the one to ship.");

    println!("\nWhich to reach for:");
    println!("   closed set of answers        -> &'static str, all arms literals");
    println!("   computed once, lives forever -> .leak(), and say so in a comment");
    println!("   computed per call            -> String");
    println!("   The mistake is reading &'static str as \"a fast string\". It is a");
    println!("   promise about lifetime, and the only free way to keep it is a literal.");
}
```
<!-- /source -->

<!-- output:static_str_kata -->
*Verified output of [`static_str_kata.rs`](examples/static_str_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Fix 1 — match to literals, -> &'static str
   label_fixed(0) = "header"
   label_fixed(1) = "first row"
   label_fixed(2) = "second row"
   label_fixed(7) = "later row"
   cost: nothing. limit: cannot name the row number.

Fix 2 — leak, -> &'static str
   label_leaked(0) = "row 0"
   label_leaked(1) = "row 1"
   label_leaked(2) = "row 2"
   label_leaked(7) = "row 7"
   cost: 4 allocations that are never freed. Called in a loop, that is
   an unbounded leak — fine for a value computed once at startup, not
   for one computed per row.

Fix 3 — return String
   label_owned(0) = "row 0"  (5 bytes, dropped at end of scope)
   label_owned(1) = "row 1"  (5 bytes, dropped at end of scope)
   label_owned(2) = "row 2"  (5 bytes, dropped at end of scope)
   label_owned(7) = "row 7"  (5 bytes, dropped at end of scope)
   cost: one allocation per call, freed. This is the one to ship.

Which to reach for:
   closed set of answers        -> &'static str, all arms literals
   computed once, lives forever -> .leak(), and say so in a comment
   computed per call            -> String
   The mistake is reading &'static str as "a fast string". It is a
   promise about lifetime, and the only free way to keep it is a literal.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:static_str -->
*Verified output of [`static_str.rs`](examples/static_str.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three spellings, one type
   let s1 = "hi";                  "hi"
   let s2: &str = "hi";            "hi"
   let s3: &'static str = "hi";    "hi"
   TypeId::of::<&str>() == TypeId::of::<&'static str>() -> true
   ...and that comparison is vacuous: TypeId::of<T> requires T: 'static,
   so `&str` written there ALREADY means `&'static str`. One type, twice.

2. The annotation only bites when the text is not a literal
   let view: &str = &owned;          "a local String"   <- compiles
   let view: &'static str = &owned;  E0597: `owned` does not live long enough
   On a literal the two annotations agree. On a borrow they do not.

3. const, static, let
   const  GREETING: &str = "hi";   "hi"   inlined at each use, no address
   static BANNER:   &str = "hi";   "hi"   one address, lives the whole run
   let    s1             = "hi";   "hi"   a local name for the same bytes
   All three point into the binary. `const` and `static` differ in whether
   there is one object or a copy per use — not in how long the text lives.

4. "A String can never be borrowed as &'static str" — three ways it can
   String::leak()          "built at runtime from 7"
   Box::leak(into_boxed)   "boxed then leaked"
   a String in a static    "stored in a static"
   The first two never free the buffer — that is the price, and it is
   deliberate: 'static means "never dropped", not "in the binary".

5. 'static the BOUND is not 'static the reference
      String satisfies T: 'static
      &'static str satisfies T: 'static
      i32 satisfies T: 'static
   T: 'static means "contains no borrow that could expire" — every owned
   type qualifies, String included. It does NOT mean "lives forever".

6. Where you actually write it
   in a return type:  greeting(9) = "Good morning!"
                      greeting(20) = "Good evening!"
   in an enum:        Parse("expected a digit")  ->  "expected a digit"
   in an enum:        Io  ->  "no detail carried"
   Both work because every arm is a literal. Return a String instead the
   moment one arm needs to say which digit it expected.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/static_str/examples/static_str.rs -o /tmp/st && /tmp/st
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [`String` vs `&str`](../string_vs_str/README.md) — where the literal-lives-in-the-binary story starts
- [String slices](../string_slices/README.md) — the view itself, and the one way it panics
- [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) — `'static` is one lifetime; this is the rest of them
- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) — what "dropped here while still borrowed" is measuring
- [`str` (primitive) ↗](https://doc.rust-lang.org/std/primitive.str.html) · [`String::leak` ↗](https://doc.rust-lang.org/std/string/struct.String.html#method.leak) · [Rust by Example — `'static` ↗](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html)

## Po polsku

Polska nazwa — **statyczny czas życia** — podsuwa dokładnie to nieporozumienie, które ta strona rozbraja. Słowo „statyczny” jest w polskim żargonie programistycznym obciążone przez C i Javę, gdzie `static` mówi o *miejscu*: zmienna globalna, pole klasy, coś, co leży na stałe w pamięci. W Ruscie `'static` nie mówi o miejscu, tylko o **czasie**, i obiecuje jedną rzecz: ta wartość nigdy nie zostanie wypuszczona (*dropped*). Stąd pierwszy wniosek strony, który po polsku brzmi zaskakująco: na literale `&str` i `&'static str` to **ten sam typ**. Pominięty czas życia w `let s: &str = "hi";` nie jest „jakimś krótszym” czasem — jest luką, którą wypełnia kompilator, a dla literału wypełnia ją zawsze przez `'static`.

Adnotacja zaczyna cokolwiek zmieniać dopiero wtedy, gdy tekst nie jest literałem. `let view: &'static str = &owned;` na pożyczce ze zmiennej lokalnej daje `E0597`, i warto przeczytać zdanie, które kompilator dopisuje pod spodem: *type annotation requires that `owned` is borrowed for `'static`*. Winna jest **adnotacja**, która zażądała za dużo, a nie samo pożyczanie. Praktycznie: pisz `&str`, a po `&'static str` sięgaj wtedy, kiedy właśnie chcesz, żeby kompilator odrzucił wszystko pożyczone od zmiennej lokalnej — w typie zwracanym, w polu struktury, w wariancie wyliczenia.

Największy zysk z tej strony to jednak rozróżnienie dwóch `'static`, bo z ich pomylenia bierze się zdanie powtarzane w książkach i na ściągach: „`String` nigdy nie może być pożyczony jako `&'static str`”. `&'static str` mówi o **referencji** — ta referencja jest ważna do końca programu. `T: 'static` to **ograniczenie typu** — ten typ nie zawiera w sobie żadnego pożyczenia, które mogłoby wygasnąć. Dlatego `String: 'static` jest prawdą, podobnie jak `i32: 'static` czy `Vec<String>: 'static`. Po polsku brzmi to absurdalnie, dopóki czyta się `'static` jako „żyje wiecznie”; przestaje, gdy przeczyta się je jako „**mógłby**, bo nic pożyczonego w środku nie ma”. I stąd tamto zdanie z książek jest po prostu nieprawdziwe: `String::leak()`, `Box::leak(…into_boxed_str())` oraz `String` mieszkający wewnątrz `static` dają `&'static str` na trzy różne sposoby — przy czym dwa pierwsze płacą alokacją, której nikt już nigdy nie zwolni.

Na koniec dwie rzeczy, które łatwo przy okazji pomylić. `const` i `static` różnią się **referencją**, a nie tekstem: tekst literału i tak leży w binarce, a różnica dotyczy tego, czy istnieje jeden obiekt pod jednym adresem, czy kopia wstawiana w każdym miejscu użycia — dla `&str` prawie bez znaczenia, istotne dopiero przy porównywaniu adresów i przy mutowalności wewnętrznej. Oraz: `E0597` i `E0515` to dwa różne błędy i warto je nazywać osobno — pierwszy mówi „ta lokalna zmienna nie żyje dość długo jak na to, czego zażądałeś adnotacją”, drugi „zwracasz referencję do czegoś, co należy do tej funkcji”. Ten drugi spotkasz w chwili, gdy któreś ramię `match` zechce powiedzieć coś o danych wejściowych i sięgniesz po `format!`; wtedy zły jest typ, nie kod — zwróć `String`.

**Szukaj po polsku:** statyczny czas życia · czas życia referencji · ograniczenie typu w Ruscie · `rust 'static lifetime vs bound` · `rust E0597 does not live long enough` · `rust String leak static str`
