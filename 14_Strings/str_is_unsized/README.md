# `str` is unsized

**Level:** 201 · working knowledge

**One line:** The size of a `str` is a property of the value, not of the type — so you never hold a `str`, only ever a pointer to one, and that pointer is where the length lives.

```rust
let s: &str = "hello";
println!("{}", size_of_val(s));   // 5 — the size of THIS value
// let owned: str = *s;           // E0277: `str` has no size known at compile time
```

Everything else on this page falls out of those two lines: the fat pointer, `?Sized`, why `Clone` cannot give you an owned string, and why the same rule governs `[T]` and `dyn Trait`.

## The size belongs to the value

`size_of::<T>()` is a compile-time constant — one number per type, the same for every value of it. But `"hello"` and `"hello, world"` both have type `str`, and they are 5 and 12 bytes. There is no number for the function to return, and it says so rather than picking one:

```text title="Abridged — real rustc output for size_of_str.rs"
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> size_of_str.rs:2:30
  |
2 |     println!("{}", size_of::<str>());
  |                              ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
```

What works instead is [`size_of_val` ↗](https://doc.rust-lang.org/std/mem/fn.size_of_val.html), which takes a *value* rather than a type — and answers 5 for one and 12 for the other. That pair is the whole definition: **sized** means the compiler can name the number from the type alone; **unsized** means it can only be read off a particular value at run time.

## So a `str` can only live behind a pointer

A local variable needs a known stack size, so `let owned: str` cannot exist. Neither can `Vec<str>`, nor `fn make() -> str` — all three are the same E0277. What you get instead is a pointer, and since the length is not in the type, the pointer has to carry it:

| handle | words | what the words are |
|---|---|---|
| [`&str`](../string_slices/README.md) | 2 | pointer + length |
| `&String` | 1 | pointer — the [`String`](../anatomy_of_a_string/README.md) at the far end holds its own len and capacity |
| `String` | 3 | pointer + length + capacity |
| [`Box<str>`](../boxed_str/README.md) | 2 | pointer + length, owned, no capacity |

A two-word pointer is a **fat pointer**. The second word is not bookkeeping about the pointer, it is the missing half of the type — which is why slicing changes it and nothing else: `&s[0..5]` has the same data pointer as `s` and a different length. Section 3 of the run below checks exactly that.

This is also the answer to a question that looks unrelated: a `&str` is *twice* the size of a `&String`, and it is still the right parameter type, because it can name a piece of a string and a `&String` can only name a whole one.

## `Sized` is the bound you never typed

Every type parameter in Rust carries an implicit `T: Sized`. It is the only bound the compiler adds behind your back, and you notice it the first time a perfectly reasonable generic refuses a string literal:

```rust
fn bytes_behind<T>(x: &T) -> usize { size_of_val(x) }

// bytes_behind("hello")   // E0277 — T would be `str`
// bytes_behind(&"hello")  // fine — T is `&str`, which is sized
```

rustc names both the bound and the fix, in its own words:

```text title="Abridged — real rustc output for describe.rs"
  = help: the trait `Sized` is not implemented for `str`
note: required by an implicit `Sized` bound in `describe`
help: consider relaxing the implicit `Sized` restriction
  |
1 | fn describe<T: ?Sized>(x: &T) -> usize { size_of_val(x) }
  |              ++++++++
```

**Relaxing** is the word to keep. `?Sized` is not a capability you are requesting; it is an assumption you are declining to make, and it is the only bound in the language that works that way. Written `T: ?Sized`, the same function accepts `str`, `[i32]` and `dyn Display` — and that is why so many std signatures carry it.

## The family, and the second word

`str` is not a special case. Three unsized types are in everyday use, and the rule they share is the one at the top of the page: always behind a pointer.

| type | a reference to it | the second word |
|---|---|---|
| `str` | `&str` | length in **bytes** |
| `[T]` | `&[T]` | length in **elements** |
| [`dyn Trait`](../../12_Traits/static_vs_dynamic_dispatch/README.md) | `&dyn Trait` | pointer to the **vtable** |

All three are two words wide; what differs is what the second word answers. For a slice it is *how many*, for a trait object it is *which implementation*. And `size_of_val` follows either one — given a `&dyn Display` pointing at an `i32` it reports 4, the size of the value at the far end rather than of the handle.

**Unsizedness is contagious.** A `str` is legal as a struct's *last* field, and the struct is then unsized too:

```rust
struct Record {
    id: u32,
    text: str,   // legal here, E0277 one line higher up
}
```

That declaration compiles. What you cannot do is write a `Record { .. }` literal, or put `text` before `id` — and `&Record` is a fat pointer, two words, exactly like `&str`. This is how `Rc<str>` stores its bytes inside the refcount allocation, and it is worth recognising when you meet it in somebody's code.

## What this forces elsewhere

- **`Clone` requires `Sized`,** so `str` cannot implement it. Calling `.clone()` on a `&str` therefore clones the *reference* and hands back another `&str` — rustc warns (`noop_method_call`), and [`ToOwned`](../../12_Traits/to_owned/README.md) exists precisely to fill the gap: `&str → String`, a different type entirely.
- **The owned forms are two words, not one.** `Box<str>`, `Rc<str>` and `Arc<str>` all carry the length in the handle. That is [`boxed_str`](../boxed_str/README.md)'s subject, including the `Rc<String>` mistake it makes tempting.
- **`Sized` is a marker trait** — no methods, implemented automatically, and meaningful only as a bound. [Marker traits](../../12_Traits/marker_traits/README.md) covers the family.

## If you are coming from another language

**Python.** Every value already lives behind a pointer, so the question never arises — `sys.getsizeof("hello")` and `sys.getsizeof("hello, world")` differ, and nobody is bothered by that, because Python has no per-type size to contradict it. The Rust difference is not that sizes vary; it is that Rust lets you put a value *directly* in a local, a struct field or a `Vec` slot, which is what makes stack layout knowable and heap allocation optional. `str` is the type that opts out of that, and `&str` is what you use instead. Two habits transfer badly: `s = t[0:5]` in Python builds a new string, while `&t[0..5]` in Rust builds a two-word pointer into the old one and copies nothing; and there is no Python counterpart to `?Sized`, because there is no split to relax.

**ABAP.** The distinction is one you already make without a name for it. `DATA lv_a TYPE c LENGTH 10` fixes the length *in the type* — that is Rust's `Sized`. `DATA lv_b TYPE string` gives you a handle, and the runtime owns the buffer behind it — that is roughly `String`. What ABAP never lets you name is the third thing: *the characters themselves, however many there are*, with no handle and no declared length. That is `str`, and the reason you only ever see `&str` is that Rust will not let you name it either — it just makes the refusal visible instead of hiding it in the runtime. The practical consequence is the one worth carrying: passing a `&str` costs two machine words and copies no characters, so the ABAP reflex of "pass it and hope the kernel is clever about the copy" becomes a guarantee you can read in the signature.

## Practice

**Measure both halves of a reference.** Write one function that takes any reference — sized target or not — and reports two numbers: how many machine words the *handle* occupies, and how many bytes the *value* at the far end does. Call it on a `&str`, a `&[i32]`, a `&dyn Display` and a `&i32`, and label each line `fat` or `thin`.

Two things it turns on. `size_of_val` is the half of the pair that accepts an unsized value, and `&T` is itself sized even when `T` is not — so `size_of::<&T>()` is legal *inside* a `T: ?Sized` generic, which is what lets one function answer for all four. Before running it, predict which of the four calls would still compile with the `?Sized` removed.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:str_is_unsized_kata -->
*[`str_is_unsized_kata.rs`](examples/str_is_unsized_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one function that measures BOTH halves of a reference —
//! the handle, and the value at the far end — for sized and unsized targets
//! alike. `&T` is always sized even when `T` is not, so `size_of::<&T>()`
//! is legal inside the generic and is what separates fat from thin.
//!
//! Run:  rustc --edition 2024 str_is_unsized_kata.rs && ./str_is_unsized_kata

use std::fmt::Display;

fn describe<T: ?Sized>(label: &str, value: &T) {
    let words = size_of::<&T>() / size_of::<usize>();
    let shape = if words == 2 { "fat " } else { "thin" };
    let unit = if words == 1 { "word" } else { "words" };
    println!("   {label:<13} {shape} pointer, {words} {unit} — the value is {} bytes",
             size_of_val(value));
}

fn main() {
    println!("Four references, one function:");
    describe("&str", "hello");
    describe("&[i32]", &[1, 2, 3][..]);
    describe("&dyn Display", &7i32 as &dyn Display);
    describe("&i32", &7i32);

    println!();
    println!("Remove the `?Sized` and only the last call still compiles:");
    println!("`i32` is the only one of the four targets whose size is in its type.");
}
```
<!-- /source -->

<!-- output:str_is_unsized_kata -->
*Verified output of [`str_is_unsized_kata.rs`](examples/str_is_unsized_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Four references, one function:
   &str          fat  pointer, 2 words — the value is 5 bytes
   &[i32]        fat  pointer, 2 words — the value is 12 bytes
   &dyn Display  fat  pointer, 2 words — the value is 4 bytes
   &i32          thin pointer, 1 word — the value is 4 bytes

Remove the `?Sized` and only the last call still compiles:
`i32` is the only one of the four targets whose size is in its type.
```
<!-- /output -->

</details>

## The verified output

<!-- source:str_is_unsized -->
*[`str_is_unsized.rs`](examples/str_is_unsized.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! `str` is unsized: the size is a property of the value, not of the type,
//! so you never hold a `str` — only a pointer to one, and that pointer is
//! where the length lives.
//!
//! Run:  rustc --edition 2024 str_is_unsized.rs && ./str_is_unsized

use std::fmt::Display;

// One function, three unsized types. Without `?Sized` the implicit bound on
// `T` is `Sized`, and none of the three calls in section 4 would compile.
fn bytes_behind<T: ?Sized>(x: &T) -> usize {
    size_of_val(x)
}

// Unsizedness is contagious. A `str` is allowed as a struct's LAST field, and
// the struct is then unsized too — so `&Record` is a fat pointer, and there is
// no way to write a `Record { .. }` literal. Move `text` above `id` and even
// the declaration is E0277.
#[allow(dead_code)]
struct Record {
    id: u32,
    text: str,
}

fn main() {
    let word = size_of::<usize>();

    println!("1. The size belongs to the VALUE, not to the type");
    println!("   size_of_val(\"hello\")        = {}", size_of_val("hello"));
    println!("   size_of_val(\"hello, world\") = {}", size_of_val("hello, world"));
    println!("   both values have type `str`, and they are not the same size");
    println!("   size_of::<str>()            = does not compile (E0277)");

    println!();
    println!("2. So a `str` lives behind a pointer, measured in machine words");
    println!("   &str          {} words   pointer + length", size_of::<&str>() / word);
    println!("   &String       {} word    pointer (the String holds its own len)",
             size_of::<&String>() / word);
    println!("   String        {} words   pointer + length + capacity", size_of::<String>() / word);
    println!("   Box<str>      {} words   pointer + length, owned, no capacity",
             size_of::<Box<str>>() / word);

    println!();
    println!("3. The second word IS the length: a subslice shares the first one");
    let s = "hello, world";
    let head = &s[0..5];
    println!("   s.as_ptr() == head.as_ptr()  {}", s.as_ptr() == head.as_ptr());
    println!("   s.len() {}   head.len() {}   same bytes, different length word",
             s.len(), head.len());

    println!();
    println!("4. `?Sized` is what lets one function take all three");
    let nums: &[i32] = &[1, 2, 3];
    let shown: &dyn Display = &7i32;
    println!("   bytes_behind(\"hello\")         = {}", bytes_behind("hello"));
    println!("   bytes_behind(&[1, 2, 3])      = {}   (3 x i32)", bytes_behind(nums));
    println!("   bytes_behind(&7i32 as &dyn D) = {}    (size_of_val follows the vtable)",
             bytes_behind(shown));

    println!();
    println!("5. Every fat pointer is two words — but the second word differs");
    println!("   &str          {} words   the second is a LENGTH", size_of::<&str>() / word);
    println!("   &[i32]        {} words   the second is a LENGTH", size_of::<&[i32]>() / word);
    println!("   &dyn Display  {} words   the second is a VTABLE pointer",
             size_of::<&dyn Display>() / word);
    println!("   &i32          {} word    sized target, nothing to carry",
             size_of::<&i32>() / word);

    println!();
    println!("6. It is contagious: a struct ending in a `str` is unsized too");
    println!("   &Record       {} words   Record {{ id: u32, text: str }}",
             size_of::<&Record>() / word);
    println!("   you cannot write the literal, and you cannot put `text` first");
}
```
<!-- /source -->

<!-- output:str_is_unsized -->
*Verified output of [`str_is_unsized.rs`](examples/str_is_unsized.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The size belongs to the VALUE, not to the type
   size_of_val("hello")        = 5
   size_of_val("hello, world") = 12
   both values have type `str`, and they are not the same size
   size_of::<str>()            = does not compile (E0277)

2. So a `str` lives behind a pointer, measured in machine words
   &str          2 words   pointer + length
   &String       1 word    pointer (the String holds its own len)
   String        3 words   pointer + length + capacity
   Box<str>      2 words   pointer + length, owned, no capacity

3. The second word IS the length: a subslice shares the first one
   s.as_ptr() == head.as_ptr()  true
   s.len() 12   head.len() 5   same bytes, different length word

4. `?Sized` is what lets one function take all three
   bytes_behind("hello")         = 5
   bytes_behind(&[1, 2, 3])      = 12   (3 x i32)
   bytes_behind(&7i32 as &dyn D) = 4    (size_of_val follows the vtable)

5. Every fat pointer is two words — but the second word differs
   &str          2 words   the second is a LENGTH
   &[i32]        2 words   the second is a LENGTH
   &dyn Display  2 words   the second is a VTABLE pointer
   &i32          1 word    sized target, nothing to carry

6. It is contagious: a struct ending in a `str` is unsized too
   &Record       2 words   Record { id: u32, text: str }
   you cannot write the literal, and you cannot put `text` first
```
<!-- /output -->

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [`String` vs `&str`](../string_vs_str/README.md) — the owner-and-view split, of which this page is the mechanism
- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — the three words, one of which `&str` does not have
- [String slices](../string_slices/README.md) — what the two words point at, and the `E0502` that keeps them honest
- [The third owned form](../boxed_str/README.md) — `Box<str>`, `Rc<str>`, `Arc<str>`: owned, and still two words
- [`ToOwned`](../../12_Traits/to_owned/README.md) — the trait `Clone: Sized` made necessary
- [Marker traits](../../12_Traits/marker_traits/README.md) · [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) — `Sized` itself, and the other everyday unsized type
- [Stack and heap](../../18_Ownership/stack_and_heap/README.md) — why a local needs a known size in the first place
- [`Sized` ↗](https://doc.rust-lang.org/std/marker/trait.Sized.html) · [`?Sized` in the Book ↗](https://doc.rust-lang.org/book/ch20-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait) · [Exotically sized types ↗](https://doc.rust-lang.org/nomicon/exotic-sizes.html)

## Po polsku

`str` jest typem o rozmiarze nieznanym w czasie kompilacji (*unsized*, w literaturze też *dynamically sized type*, DST). Sedno mieści się w jednym zdaniu: **rozmiar jest cechą wartości, a nie typu**. `"hello"` i `"hello, world"` mają ten sam typ `str` i zajmują 5 oraz 12 bajtów, więc `size_of::<str>()` nie ma czego zwrócić i w ogóle się nie kompiluje (`E0277`). Działa za to `size_of_val`, które dostaje *wartość* — i odpowiada 5 albo 12, odczytując długość ze wskaźnika.

Stąd wynika reszta. Skoro zmienna lokalna musi mieć znany rozmiar na stosie, `let owned: str` nie może istnieć — podobnie jak `Vec<str>` czy funkcja zwracająca `str`. Zostaje wskaźnik, a że długości nie ma w typie, to wskaźnik musi ją nieść: `&str` to **gruby wskaźnik** (*fat pointer*) — dwa słowa maszynowe, adres i długość. `&String` ma tylko jedno słowo, bo `String` po drugiej stronie sam pamięta swoją długość i pojemność. Drugie słowo grubego wskaźnika nie jest księgowością — to brakująca połowa typu, i dlatego `&s[0..5]` ma **ten sam adres** co `s`, a różni się wyłącznie długością.

Najczęstsze zaskoczenie dotyczy `Sized`. Każdy parametr typu w Ruscie dostaje niewidoczne ograniczenie `T: Sized` — jedyne, które kompilator dopisuje sam. Dlatego `fn f<T>(x: &T)` odmawia przyjęcia literału napisowego: `T` musiałoby być `str`. Zapis `?Sized` nie jest więc *dodatkowym wymaganiem*, tylko **rozluźnieniem** — rezygnacją z założenia, i jest to jedyne ograniczenie w języku, które działa w tę stronę. Sam kompilator używa dokładnie tego słowa: *consider relaxing the implicit `Sized` restriction*.

`str` nie jest wyjątkiem — tak samo zachowują się `[T]` i `dyn Trait`. Wszystkie trzy mają referencje szerokości dwóch słów, ale drugie słowo znaczy co innego: dla wycinka to **liczba elementów**, dla obiektu cechy (*trait object*) to wskaźnik na **tablicę metod** (*vtable*). Warto też wiedzieć, że nieokreśloność rozmiaru jest zaraźliwa: `str` wolno umieścić jako **ostatnie** pole struktury, a wtedy cała struktura staje się unsized — nie da się napisać jej literału, a `&Struktura` też robi się grubym wskaźnikiem. Na tym opiera się `Rc<str>`, który trzyma bajty wewnątrz alokacji z licznikiem referencji.

Na koniec konsekwencja, która wygląda na osobny temat: `Clone` wymaga `Sized`, więc `str` nie może go implementować. `.clone()` na `&str` klonuje **referencję** i oddaje kolejne `&str` (rustc ostrzega, lint `noop_method_call`) — i właśnie po to istnieje `ToOwned`, żeby `&str` mogło stać się `String`, czyli typem innym niż wyjściowy.

**Szukaj po polsku:** typy o nieznanym rozmiarze · gruby wskaźnik · rozmiar wartości a rozmiar typu · obiekt cechy · `rust unsized types` · `rust ?Sized bound` · `rust fat pointer str`
