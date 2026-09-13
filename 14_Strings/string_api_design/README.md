# String parameters worth copying

**Level:** 201 → 301 · working knowledge

**One line:** The signature you write decides what your callers have to allocate — and the four good answers (`&str`, `impl AsRef<str>`, `impl Into<String>`, `Cow<str>`) each buy something different, which is why there is no single right one. This page counts the allocations instead of asserting them.

```rust
fn count_words(s: &str) -> usize {              // only reads the text: borrow it
    s.split_whitespace().count()
}

struct User {
    name: String,
}

impl User {
    fn new(name: impl Into<String>) -> Self {   // keeps the text: let a String move in
        User { name: name.into() }
    }
}
```

Every number below comes from one program that wraps the allocator in a counter — the trick [The global allocator](../../09_Advanced/the_global_allocator/README.md) teaches — and builds each input *before* the count starts, so only the call itself is measured.

## The choices, and who pays

| The function… | Takes or returns | What the caller pays |
|---|---|---|
| only reads the text | `&str` | nothing — from a literal, a `&String` or a slice |
| keeps the text | `impl Into<String>` | one copy from a `&str`; nothing when a `String` is moved in |
| takes text of any kind, or a slice of either kind | `impl AsRef<str>` | nothing per call; one compiled copy of the function per type |
| returns text it may have had to change | `Cow<'_, str>` | nothing when unchanged; one allocation when changed |
| returns new text | `String` | one allocation per call |
| returns new text, over and over | a `&mut String` to write into | one allocation, then reuse |

## Borrow by default

`fn count_words(s: &str)` costs every caller nothing: a literal already is a `&str`, a `&String` coerces to one because `String: Deref<Target = str>` ([`String` vs `&str`](../string_vs_str/README.md) has the mechanism), and a slice is one. Section 1 of the output is three zeros. Most functions that take text should stop here.

## Taking `String` "to keep it simple"

The same function taking `s: String` moves the cost onto every call site that cannot give up its own value. In section 2 a caller holding a literal pays an allocation for `.to_string()`, and a caller that wants to keep its `String` pays one for `.clone()`. Only a caller that was finished with its `String` gets in free — and the function never needed ownership at all, because it reads the text and drops it. The signature asked for something it does not use, and the callers pay for it.

## `impl AsRef<str>`: every kind, one copy per kind

`fn count_words_any<T: AsRef<str>>(s: T)` accepts `&str`, `&String` and `String` alike, for nothing per call — section 3's zeros. The price is paid at compile time instead: the program prints three instantiations, `&str`, `&alloc::string::String` and `alloc::string::String`, which means three compiled copies of the function. For a single parameter that buys almost nothing over plain `&str`: a `&String` coerces anyway, and passing a `String` by value saves the caller one `&` at the cost of giving the `String` away.

The slice is where it earns its keep. `fn total_words<S: AsRef<str>>(items: &[S])` takes a `&[&str]` and a `&[String]` alike, for nothing either way. Written with `items: &[&str]` instead, it refuses `&strings` — a `&Vec<String>` — with `E0308`: deref coercion turns a `&String` into a `&str`, but never a slice of one into a slice of the other.

It does **not** accept `PathBuf`. A path is not promised to be UTF-8, so `PathBuf` implements `AsRef<Path>` and `AsRef<OsStr>`, and asking it for `AsRef<str>` is `E0277`. A function that wants paths and strings alike takes `impl AsRef<Path>`, which a `&str` satisfies too.

## `impl Into<String>` when the function keeps the text

A constructor that stores a name is going to own it, so the only question is who makes the `String`. With `fn new(name: impl Into<String>)` — section 4 — a caller passing `"Ada"` pays the one allocation that has to happen somewhere, and a caller passing a `String` it is done with pays nothing, because the `String` moves in. The `&str` version, `from_borrowed(&name)`, copies even when the caller had a `String` to give. Zero against one, on every construction: that is why `Into<String>` is the idiom for constructors and setters. [`From` and `Into`](../../29_Conversion/from_and_into/README.md) explains the trait. A builder's setters are the same case, since each one keeps its text; the kata below counts what `impl Into<String>` saves there, and what a builder that can build twice costs.

## `Cow<'_, str>` in the return position

A function that usually hands back its input unchanged, and occasionally has to fix it, can return `Cow<'_, str>`: `Borrowed` when nothing changed, `Owned` when something did. In section 5, `tidy` returns `Borrowed` at no cost for text with no tabs, and `Owned` for one allocation when it had a tab to replace. std's own example is [`String::from_utf8_lossy`](../string_methods/string_from_utf8_lossy/README.md), which borrows when the bytes were already valid. As a *parameter* type `Cow` rarely helps — the caller has to build one — so this page keeps it on the return side; [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md) covers the type itself.

## Returning text

Section 6 compares the two shapes that work. `fn label(n) -> String` allocates once per call: three calls, three allocations. `fn label_into(n, out: &mut String)` writes into a buffer the caller lends it — one allocation the first time, then the same buffer again, because clearing a `String` keeps its capacity. In a loop that builds many lines, the second shape is the one to reach for.

The third shape, returning a `&str` into a `String` made inside the function, never compiles. With no input to borrow from, rustc stops at `E0106` and suggests `'static`; write that, and it becomes `E0515` — the `String` is dropped at the closing brace, and the reference may not outlive it. [`&'static str`](../static_str/README.md) has a kata on returning a label that meets `E0515`.

## The rule of thumb

- **A parameter** takes `&str` — unless the function keeps the text, and then `impl Into<String>`.
- **A return value** is a `String` if it is new text, a `&str` if it is part of an input (the lifetime says which input), and a `Cow<'_, str>` if it is usually the input and sometimes new.
- **A struct field** is a `String`. A `&str` field is possible, and it makes the whole struct a borrow — see [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md).
- **In a loop**, lend the function a `&mut String` rather than taking a new `String` back every time.

## If you are coming from another language

- **C++** — `&str` is `std::string_view`, and `impl Into<String>` is the sink-parameter idiom: take `std::string` by value and `std::move` it into the member. Rust makes the move the default and the copy explicit, so the idiom cannot quietly turn into a copy the way a forgotten `std::move` does.
- **Python** — every `str` argument is a reference to an immutable object, so there is no signature to choose and no cost to see. Rust puts the cost in the signature, which is the only reason this page exists.
- **ABAP** — a method's `IMPORTING` parameter is passed by reference unless you write `VALUE( )`, which copies, and one passed by reference cannot be changed in the method ([`METHODS` parameters ↗](https://help.sap.com/doc/abapdocu_758_index_htm/7.58/en-US/abapmethods_parameters.htm)). `&str` is that read-only by-reference default; taking a `String` by value is `VALUE( )` on every call, except that Rust moves the caller's `String` in rather than copying it, so a caller that wants to keep its own has to clone.

## Practice

**A builder that moves, not copies.** Write an `EmailBuilder` whose `to`, `subject` and `body` setters take `impl Into<String>` and hand the builder back, so the calls chain, and a `build(self)` that turns it into an `Email`. With the counter from this page, count a chain of literals and a chain of `String`s the caller is finished with. Then give the same builder `&str` setters and pass it those `String`s again. Last, add a `build_again(&self)` that leaves the builder usable, and count what each call of it costs — once, and ten times.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:email_builder_kata -->
*[`email_builder_kata.rs`](examples/email_builder_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a builder whose setters take `impl Into<String>` — and the
//! price of a `build` that borrows the builder so it can run twice.
//!
//!   rustc --edition 2024 email_builder_kata.rs -o /tmp/ebk && /tmp/ebk

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

/// Counts allocations and reallocations, as on the lesson page.
struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

fn counted<T>(f: impl FnOnce() -> T) -> (T, usize) {
    let before = ALLOCS.load(Relaxed);
    let out = f();
    (out, ALLOCS.load(Relaxed) - before)
}

struct Email {
    to: String,
    subject: String,
    body: String,
}

/// Starts as three empty `String`s, and an empty `String` owns no buffer.
#[derive(Default)]
struct EmailBuilder {
    to: String,
    subject: String,
    body: String,
}

impl EmailBuilder {
    fn new() -> Self {
        EmailBuilder::default()
    }

    // Each setter keeps its text, so each asks for anything that can become a
    // `String` — and hands the builder back, so the calls chain.
    fn to(mut self, to: impl Into<String>) -> Self {
        self.to = to.into();
        self
    }

    fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = subject.into();
        self
    }

    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    /// Consumes the builder, so its three `String`s move into the `Email`.
    fn build(self) -> Email {
        Email { to: self.to, subject: self.subject, body: self.body }
    }

    /// Borrows the builder so it can build again — and so must copy every field.
    fn build_again(&self) -> Email {
        Email { to: self.to.clone(), subject: self.subject.clone(), body: self.body.clone() }
    }
}

/// The same builder with `&str` setters, for comparison.
#[derive(Default)]
struct BorrowingBuilder {
    to: String,
    subject: String,
    body: String,
}

impl BorrowingBuilder {
    fn to(mut self, to: &str) -> Self {
        self.to = to.to_owned();
        self
    }

    fn subject(mut self, subject: &str) -> Self {
        self.subject = subject.to_owned();
        self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = body.to_owned();
        self
    }

    fn build(self) -> Email {
        Email { to: self.to, subject: self.subject, body: self.body }
    }
}

fn main() {
    println!("1. Literals: each field is copied once, because it has to be");
    let (_, n) = counted(EmailBuilder::new);
    println!("   EmailBuilder::new()        {n} allocations   (an empty String owns no buffer)");
    let (email, n) = counted(|| {
        EmailBuilder::new().to("ada@example.com").subject("Hello").body("See you at ten.").build()
    });
    println!("   three literal setters      {n} allocations");
    println!("   to {:?}, subject {:?}, body {:?}", email.to, email.subject, email.body);

    println!();
    println!("2. Strings the caller is finished with");
    let (to, subject, body) = (String::from("ben@example.com"), String::from("Minutes"), String::from("Attached."));
    let (_, n) = counted(move || EmailBuilder::new().to(to).subject(subject).body(body).build());
    println!("   three owned setters        {n} allocations   (each String moves in)");

    println!();
    println!("3. The same Strings, through &str setters");
    let (to, subject, body) = (String::from("ben@example.com"), String::from("Minutes"), String::from("Attached."));
    let (_, n) = counted(|| BorrowingBuilder::default().to(&to).subject(&subject).body(&body).build());
    println!("   three &str setters         {n} allocations   (Strings to give, copied anyway)");

    println!();
    println!("4. build(self) against build_again(&self)");
    let builder = EmailBuilder::new()
        .to(String::from("cara@example.com"))
        .subject(String::from("Reminder"))
        .body(String::from("Tomorrow at nine."));
    let (_, once) = counted(|| builder.build_again());
    let (_, ten) = counted(|| {
        for _ in 0..10 {
            std::hint::black_box(builder.build_again());
        }
    });
    let (_, consumed) = counted(move || builder.build());
    println!("   build_again(&self), once   {once} allocations   (every field cloned)");
    println!("   build_again(&self), ×10    {ten} allocations");
    println!("   build(self)                {consumed} allocations   (the fields move out)");
    println!("   A builder that can build twice pays one copy per field per build.");
    println!("   Consume it, unless the caller really does build more than once.");
}
```
<!-- /source -->

<!-- output:email_builder_kata -->
*Verified output of [`email_builder_kata.rs`](examples/email_builder_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Literals: each field is copied once, because it has to be
   EmailBuilder::new()        0 allocations   (an empty String owns no buffer)
   three literal setters      3 allocations
   to "ada@example.com", subject "Hello", body "See you at ten."

2. Strings the caller is finished with
   three owned setters        0 allocations   (each String moves in)

3. The same Strings, through &str setters
   three &str setters         3 allocations   (Strings to give, copied anyway)

4. build(self) against build_again(&self)
   build_again(&self), once   3 allocations   (every field cloned)
   build_again(&self), ×10    30 allocations
   build(self)                0 allocations   (the fields move out)
   A builder that can build twice pays one copy per field per build.
   Consume it, unless the caller really does build more than once.
```
<!-- /output -->

</details>

## The verified output

<!-- output:string_api_design -->
*Verified output of [`string_api_design.rs`](examples/string_api_design.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The default: take &str
   count_words("one two three")   0 allocations
   count_words(&owned)            0 allocations   (&String coerced to &str)
   count_words(&owned[..12])      0 allocations

2. The anti-pattern: take String by value
   a caller with a literal        1 allocation    (.to_string() first)
   a caller keeping its String    1 allocation    (.clone() first)
   a caller finished with its own 0 allocations   (moved in)
   Only the last caller got it free, and the function never needed to own
   the text: it reads it and drops it.

3. impl AsRef<str>: every kind of text, one copy of the function per kind
   &str 0, &String 0, String 0 allocations
   the three instantiations: &str, &alloc::string::String, alloc::string::String
   Three types in, three compiled copies of count_words_any out. The cost is
   code size, paid once per type — never per call.
   total_words(&strs)      3 words, 0 allocations   (a &[&str])
   total_words(&strings)   3 words, 0 allocations   (a &[String])
   The slice is where it earns its keep. A &[&str] parameter would refuse
   &strings with E0308: a &String coerces to &str, but a slice of one never
   coerces to a slice of the other. PathBuf is refused outright (E0277): a
   path is not promised to be UTF-8, so it is AsRef<Path> and AsRef<OsStr>,
   never AsRef<str>.

4. impl Into<String>: when the function is going to keep the text
   User::new("Ada")             1 allocation    (the &str is copied once, as it must be)
   User::new(name)              0 allocations   (the caller's String moves in)
   User::from_borrowed(&name)   1 allocation    (a String existed, and was copied anyway)

5. Cow<str> as the return type: borrow unless something changed
   tidy("no tabs here")   -> Borrowed 0 allocation(s)
   tidy("one\ttab")       -> Owned    1 allocation(s)

6. Returning text: a new String, or a buffer the caller lends you
   label(n), three calls                 3 allocations   (one String each)
   label_into(n, &mut buf), three calls  1 allocation    (one buffer, reused: "row 3")
   The third shape, a &str into a String made inside the function, never
   compiles. With no input to borrow from it is E0106 first; write 'static
   as rustc suggests and it becomes E0515, because the String dies at the
   closing brace and the reference may not outlive it.
```
<!-- /output -->

## See also

- [`String` vs `&str`](../string_vs_str/README.md) — the two types, and why deref coercion makes `&str` free to accept
- [Making a `String`](../making_a_string/README.md) — the spellings of `&str` → `String`, `.into()` among them
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md)
- [`From` and `Into`](../../29_Conversion/from_and_into/README.md) — the trait behind `impl Into<String>`
- [`ToOwned`](../../12_Traits/to_owned/README.md)
- [`&'static str`](../static_str/README.md) — the `E0515` a borrowed return meets
- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — the counter behind every number on this page
- [When `String` is too slow](../when_string_is_too_slow/README.md) — what comes after the right signature
- [STRINGS.md](../../STRINGS.md) — where this page sits on the route

## Po polsku

To **sygnatura decyduje, kto płaci** — a ta strona nie zgaduje, tylko liczy alokacje. Przyjmij `&str`, a wywołujący nie zapłaci nic: literał już jest wycinkiem łańcucha, a `&String` przechodzi dzięki automatycznej dereferencji (*deref coercion*). Przyjmij `String` przez wartość „dla prostoty”, a każdy, kto ma tylko wycinek albo chce zachować swój `String`, musi zrobić kopię. `impl AsRef<str>` przyjmuje `&str`, `String` i `&String`, płacąc monomorfizacją — osobną kopią funkcji dla każdego typu — ale **nie** `PathBuf`, bo ścieżka nie musi być poprawnym UTF-8. Naprawdę przydaje się dopiero przy wycinku (*slice*): `&[S]` z `S: AsRef<str>` przyjmuje zarówno `&[&str]`, jak i `&[String]`. `impl Into<String>` ma sens, gdy funkcja i tak zatrzymuje tekst: istniejący `String` wchodzi przez przeniesienie, za darmo. Settery budowniczego (*builder*) to ten sam przypadek, a `build(self)` przenosi pola, zamiast je kopiować. `Cow<'_, str>` zarabia na siebie w **typie zwracanym** — pożycza, dopóki nic się nie zmieniło, i alokuje dopiero przy zapisie.

**Szukaj po polsku:** parametr `&str` czy `String` · projektowanie API tekstowego · `rust AsRef<str> vs Into<String>` · `rust Cow<str> return type`
