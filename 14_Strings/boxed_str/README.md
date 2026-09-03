# The third owned form: `Box<str>`, `Rc<str>`, `Arc<str>`

**Level:** 201 → 301 · working knowledge

**One line:** There is an owned string that is not a `String` — drop the capacity word and you get text you cannot grow, a handle the size of a `&str`, and, in the `Rc`/`Arc` case, one buffer that many owners can share.

| | owns the text | can grow | handle | reach for it when |
|---|---|---|---|---|
| `String` | yes | yes | 24 bytes | the text is still being built |
| `Box<str>` | yes | no | 16 bytes | it is finished and you keep a lot of them |
| `Rc<str>` | yes, shared | no | 16 bytes | it is finished **and repeats** |
| `Arc<str>` | yes, shared | no | 16 bytes | the same, across threads |
| `&str` | **no** | no | 16 bytes | someone else will outlive you |

---

## The capacity word is the whole difference

A `String` is three machine words: a pointer, a length, and a **capacity** — the room it bought so the next `push_str` need not reallocate. Text that is finished has no next `push_str`, so the third word is recording a promise nobody will collect.

Drop it and you have `Box<str>`: the same bytes on the heap, the same ownership, the same `Drop` that frees them — two words instead of three.

```rust
let mut s = String::with_capacity(64);
s.push_str("Ada Lovelace");        // len 12, capacity 64
let boxed: Box<str> = s.into_boxed_str();
```

Note where that leaves it against `&str`. Both handles are 16 bytes and both are a pointer plus a length, so the *shape* is identical — but a `Box<str>` **owns**, and frees the text when it drops. The size table above is not saying they are the same thing; it is saying that owning finished text costs no more than pointing at it.

## The round trip, and what it cannot restore

Both conversions are cheap, and only one of them is lossy:

```rust
let boxed: Box<str> = s.into_boxed_str();   // shrinks to fit, then drops the capacity field
let back:  String   = boxed.into_string();  // capacity == len
```

`into_boxed_str` calls [`shrink_to_fit`](../string_methods/string_shrink_to_fit/README.md) first, because a `Box<str>` has nowhere to record spare room. So the 52 bytes of slack in the example above are not carried along and not recoverable — `into_string` hands back a `String` whose capacity is exactly its length, and the next `push_str` reallocates. That is the trade, and it is the right one for text that is done.

**One thing worth measuring rather than assuming:** whether the shrink *moves the bytes*. It is an `realloc`, and an allocator is free to satisfy it in place. Running the loop on this machine, a 12-byte string sitting in a 4096-byte buffer moved; the same string in a 64-byte buffer did not. Neither outcome is promised by the API, so do not build on either — the point is only that `into_boxed_str` is not automatically a copy, and treating it as one has talked people out of a cheap win.

## When the eight bytes matter

Per value, this is a rounding error. The honest answer is arithmetic, not judgement — **count the values**:

| values | `Vec<String>` | `Vec<Box<str>>` | saved |
|---|---|---|---|
| 1,000 | 23 KB | 15 KB | 8,000 bytes |
| 1,000,000 | 23,437 KB | 15,625 KB | 8,000,000 bytes |

At a thousand it is noise and reaching for `Box<str>` is the kind of tidiness that costs a reader more than it saves the machine. At a million — a symbol table, a parsed column, an interner — it is eight megabytes you never allocate and never walk.

Two things the table is **not** saying. The saving is on the **handles**; the text is identical bytes either way, so if your strings are long the ratio collapses. And nothing here is faster: it is the same number of dereferences to reach a byte.

## `Rc<str>` is one hop; `Rc<String>` is two

This is where the shape starts mattering more than the size. `Rc<str>` puts the text **inside** the reference-counted allocation — the count and the bytes are one object:

```rust
let flat: Rc<str> = Rc::from("Ada Lovelace");
```

`Rc<String>` looks like the obvious spelling and is the wrong one. The `Rc` holds a `String`, and that `String` points somewhere else again: **two heap allocations per value, and two dereferences to reach a byte.**

The measurement that shows it is whether the text lives at the same address as the `Rc`'s payload — `true` for `Rc<str>`, `false` for `Rc<String>`, in [section 4 below](#the-verified-output). And note the trap in the sizes, which is the reason people pick wrong: `Rc<String>` has the **smaller handle** — 8 bytes, a thin pointer, against `Rc<str>`'s 16-byte fat one — while costing strictly more everywhere it counts. The handle is the only place `Rc<String>` wins, and the handle is not what you were paying for.

So: `Rc<str>`, and let the fat pointer be fat.

## Many owners, one buffer

That is the move `Rc<str>` exists for. Clone the handle and the text is not copied — a second owner appears, the count goes up, and the bytes stay where they were:

```rust
let name: Rc<str> = Rc::from("Ada Lovelace");
let table: Vec<Rc<str>> = (0..5).map(|_| Rc::clone(&name)).collect();
// 5 handles, one buffer, strong_count 6
```

The same table as `Vec<String>` is five separate buffers holding the same twelve bytes five times. This is **string interning**, and it is why `Rc<str>` shows up in parsers, symbol tables and any column with repeats: the cost stops scaling with the number of *rows* and starts scaling with the number of *distinct values*. The kata below builds one.

`Arc<str>` is the same thing with an atomic count, for when the table crosses a thread boundary. Same size, same shape; you pay for the atomics only if you need them.

## The trap: `.to_owned()` on an `Rc` clones the pointer

[`ToOwned`](../../12_Traits/to_owned/README.md) names this one and it belongs here too, because this is the page where you acquire the habit that springs it:

```rust
let name: Rc<str> = Rc::from("Ada Lovelace");
let a = name.to_owned();          // Rc<str>  — a new handle, no new text
let view: &str = &name;
let b = view.to_owned();          // String   — an allocation and a memcpy
```

Same method name, two entirely different jobs, and the receiver is the only thing that tells them apart. On the smart pointer it is a cheap new owner; one deref away it is a copy of every byte. Neither is wrong — but if you reached for `.to_owned()` meaning "give me my own copy" and got a shared handle, every later mutation you were planning is a compile error you will read as mysterious.

## If you are coming from another language

**Python.** You have been using `Rc<str>` this whole time. A Python `str` is immutable and reference-counted, so `b = a` copies a pointer and bumps a count — exactly `Rc::clone`. Python has no `String`/`Box<str>` split because its strings never grow in place; every `+=` builds a new object.

| Python | | Rust |
|---|---|---|
| `b = a` | pointer copy, refcount +1 | `Rc::clone(&a)` |
| `sys.intern(s)` | one buffer per distinct value | the `HashMap<&str, Rc<str>>` pool in the kata |
| `sys.getrefcount(s)` | how many owners | `Rc::strong_count(&s)` |
| `s += "x"` | builds a new object | `String::push_str` mutates in place |
| threads | the GIL makes the count safe | `Arc<str>` — you choose to pay for atomics |

The habit that transfers badly: in Python, interning is an *optimization* you can ignore, because sharing is the default and nothing breaks if you skip it. In Rust the sharing is a **type**, so it is a decision you make once, up front, and the compiler holds you to it — `Rc<str>` and `String` are not interchangeable at the point of use.

**ABAP.** `string` is a reference into the string heap, and the kernel shares that buffer between variables until one of them is written — copy-on-write, decided at runtime.

| ABAP | | Rust |
|---|---|---|
| `lv_b = lv_a` on a `string` | shares the buffer, copies on write | `Rc<str>` (shares) or [`Cow`](../../18_Ownership/clone_on_write/README.md) (copies on write) |
| `TYPE c LENGTH 20` | fixed width, no growth | closest is `Box<str>` — finished text |
| a literal in the text pool | one copy per program | `&'static str` |
| `CL_ABAP_STRING_UTILITIES` | length games on fixed fields | slicing, which needs no utility |

What changes: ABAP's sharing is invisible and automatic, so you never decide and never pay attention. Rust makes you write which one you meant, and the reward is that the cost is readable from the type — `Rc<str>` in a struct field says "this repeats, and I know it" in a way no ABAP declaration can.

## When not to reach for any of this

Default to `String`. It is the type every API takes, every tutorial uses, and every reader recognises, and one `String` costs eight bytes more than one `Box<str>` — which is nothing.

The three signals that you have left the default behind honestly:

1. **You are storing a lot of them** and they are finished → `Box<str>`.
2. **They repeat** → `Rc<str>` with a pool, and measure the distinct count first.
3. **They cross a thread** → `Arc<str>`.

Absent one of those, converting is churn: `into_boxed_str()` sprinkled through a codebase makes every signature that takes `&str` fine and every one that wanted a `String` need a conversion, in exchange for a saving nobody can measure.

---

## Practice

**Freeze a candidate column three ways.** Take eight ballot rows naming three distinct candidates, and hold them first as `Vec<String>`, then as `Vec<Box<str>>`, then as `Vec<Rc<str>>` interned through a `HashMap`. For each, report two numbers: the bytes of *handles*, and how many distinct text buffers are actually alive.

Predict all six numbers before you run it. Then say which conversion copied text and which did not — and finish by calling `.to_owned()` on one of the `Rc<str>` rows and on a `&str` borrowed from it, and work out which one allocated.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:boxed_str_kata -->
*[`boxed_str_kata.rs`](examples/boxed_str_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: freeze a candidate column three ways, and count what changed.
//!
//!   rustc --edition 2024 boxed_str_kata.rs -o /tmp/bsk && /tmp/bsk

use std::collections::HashMap;
use std::mem::size_of;
use std::rc::Rc;

/// Eight ballot rows naming three candidates — the shape that makes interning pay.
const COLUMN: [&str; 8] = [
    "Ada Lovelace",
    "Grace Hopper",
    "Ada Lovelace",
    "Barbara Liskov",
    "Ada Lovelace",
    "Grace Hopper",
    "Barbara Liskov",
    "Ada Lovelace",
];

/// How many distinct text buffers are alive behind these handles?
fn distinct_buffers(ptrs: impl Iterator<Item = *const u8>) -> usize {
    ptrs.map(|p| p as usize).collect::<std::collections::HashSet<_>>().len()
}

fn main() {
    println!("The column: {} rows, {} distinct names",
        COLUMN.len(),
        COLUMN.iter().collect::<std::collections::HashSet<_>>().len());

    println!("\n1. Vec<String> — grow-able, and nobody is going to grow it");
    let as_strings: Vec<String> = COLUMN.iter().map(|s| s.to_string()).collect();
    let handles = as_strings.len() * size_of::<String>();
    println!("   handles      {:>3} x {:>2} = {:>3} bytes", as_strings.len(), size_of::<String>(), handles);
    println!("   text buffers {:>3}", distinct_buffers(as_strings.iter().map(|s| s.as_ptr())));

    println!("\n2. Vec<Box<str>> — same text, one word less per handle");
    let as_boxed: Vec<Box<str>> = as_strings.into_iter().map(|s| s.into_boxed_str()).collect();
    let boxed_handles = as_boxed.len() * size_of::<Box<str>>();
    println!("   handles      {:>3} x {:>2} = {:>3} bytes   ({} saved)",
        as_boxed.len(), size_of::<Box<str>>(), boxed_handles, handles - boxed_handles);
    println!("   text buffers {:>3}   <- unchanged: freezing a handle copies no text",
        distinct_buffers(as_boxed.iter().map(|b| b.as_ptr())));

    println!("\n3. Vec<Rc<str>> — intern, and the repeats stop paying");
    let mut pool: HashMap<&str, Rc<str>> = HashMap::new();
    let as_rc: Vec<Rc<str>> = COLUMN
        .iter()
        .map(|&name| Rc::clone(pool.entry(name).or_insert_with(|| Rc::from(name))))
        .collect();
    println!("   handles      {:>3} x {:>2} = {:>3} bytes",
        as_rc.len(), size_of::<Rc<str>>(), as_rc.len() * size_of::<Rc<str>>());
    println!("   text buffers {:>3}   <- three names, three buffers, eight rows",
        distinct_buffers(as_rc.iter().map(|r| r.as_ptr())));
    let ada = &pool["Ada Lovelace"];
    println!("   \"Ada Lovelace\" appears 4 times; strong_count = {}", Rc::strong_count(ada));
    println!("   (4 rows + 1 held by the pool)");

    println!("\n4. The trap — same method name, two different jobs");
    let row: &Rc<str> = &as_rc[0];
    let cheap: Rc<str> = row.to_owned();           // a new handle
    let view: &str = row;
    let real: String = view.to_owned();            // an allocation and a memcpy
    println!("   Rc<str>.to_owned()  same buffer? {}   <- clones the POINTER", cheap.as_ptr() == row.as_ptr());
    println!("   (&str).to_owned()   same buffer? {}  <- clones the TEXT", real.as_ptr() == row.as_ptr());

    println!("\nWhat to reach for:");
    println!("   still being built            -> String");
    println!("   finished, stored in bulk     -> Box<str>   (8 bytes/value, no copy to make)");
    println!("   finished, and it REPEATS     -> Rc<str>    (one buffer per distinct value)");
    println!("   finished, and crosses threads-> Arc<str>");
    println!("   The first choice is about growth; the rest are about how many you keep.");
}
```
<!-- /source -->

<!-- output:boxed_str_kata -->
*Verified output of [`boxed_str_kata.rs`](examples/boxed_str_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The column: 8 rows, 3 distinct names

1. Vec<String> — grow-able, and nobody is going to grow it
   handles        8 x 24 = 192 bytes
   text buffers   8

2. Vec<Box<str>> — same text, one word less per handle
   handles        8 x 16 = 128 bytes   (64 saved)
   text buffers   8   <- unchanged: freezing a handle copies no text

3. Vec<Rc<str>> — intern, and the repeats stop paying
   handles        8 x 16 = 128 bytes
   text buffers   3   <- three names, three buffers, eight rows
   "Ada Lovelace" appears 4 times; strong_count = 5
   (4 rows + 1 held by the pool)

4. The trap — same method name, two different jobs
   Rc<str>.to_owned()  same buffer? true   <- clones the POINTER
   (&str).to_owned()   same buffer? false  <- clones the TEXT

What to reach for:
   still being built            -> String
   finished, stored in bulk     -> Box<str>   (8 bytes/value, no copy to make)
   finished, and it REPEATS     -> Rc<str>    (one buffer per distinct value)
   finished, and crosses threads-> Arc<str>
   The first choice is about growth; the rest are about how many you keep.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:boxed_str -->
*Verified output of [`boxed_str.rs`](examples/boxed_str.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Four owned strings, four handle sizes
   String       24 bytes  (pointer + length + capacity)
   Box<str>     16 bytes  (pointer + length)
   Rc<str>      16 bytes  (pointer + length)
   Arc<str>     16 bytes  (pointer + length)
   &str         16 bytes  (pointer + length) — borrowed, owns nothing
   Drop the capacity word and the handle is a &str's size — but it OWNS.

2. The round trip, and what the capacity word was holding
   String::with_capacity(64) + push_str  len 12 cap 64
   .into_boxed_str()                     len 12  (no capacity field at all)
   .into_string()                        len 12 cap 12
   52 bytes of slack are gone: into_boxed_str shrinks first, so the
   trip back cannot restore a capacity nobody recorded.

3. When the 8 bytes matter
        1000 values:  Vec<String>       23 KB   Vec<Box<str>>       15 KB   saved 7 KB
     1000000 values:  Vec<String>    23437 KB   Vec<Box<str>>    15625 KB   saved 7812 KB
   Count the values, not the bytes. At a thousand it is noise; at a
   million it is eight million bytes you did not have to touch. And it
   is a saving on the HANDLES only — the text is the same either way.

4. Rc<str> is one hop; Rc<String> is two
   Rc<str>     handle 16 bytes   text stored inside the Rc? true
   Rc<String>  handle  8 bytes   text stored inside the Rc? false
   Rc<String> has the smaller handle and costs MORE: the Rc holds a
   String, and that String points somewhere else again — two heap
   allocations per value, and two dereferences to reach a byte.

5. Many owners, one buffer
   5 clones of one Rc<str>: all pointing at the same buffer? true
   strong_count = 6  (the 5 clones plus the original)
   text allocated once: 12 bytes
   the same table as Vec<String>: 5 separate buffers, 60 bytes of text

6. The trap: .to_owned() on an Rc clones the POINTER
   name.to_owned() points at the same buffer? true
   strong_count is now 7 — a new owner, no new text
   (&*name).to_owned() points at the same buffer? false
   Same method name, two different jobs. On the smart pointer it is a
   cheap new handle; one deref away it is an allocation and a memcpy.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/boxed_str/examples/boxed_str.rs -o /tmp/bs && /tmp/bs
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — the three words this page removes one of
- [`String` vs `&str`](../string_vs_str/README.md) — the owner-and-view split this page adds a third column to
- [`String::into_boxed_str`](../string_methods/string_into_boxed_str/README.md) · [`String::shrink_to_fit`](../string_methods/string_shrink_to_fit/README.md) — the two methods, one page each
- [`ToOwned`](../../12_Traits/to_owned/README.md) — where the `.to_owned()` trap is named in full
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md) — the other answer to "do I have to copy this?"
- [`Rc<T>` ↗](https://doc.rust-lang.org/std/rc/struct.Rc.html) · [`Arc<T>` ↗](https://doc.rust-lang.org/std/sync/struct.Arc.html) · [`Box<str>` from `String` ↗](https://doc.rust-lang.org/std/string/struct.String.html#method.into_boxed_str)

## Po polsku

Sama myśl jest krótka: `String` trzyma trzy słowa maszynowe (wskaźnik, długość, **pojemność**), a tekst, którego i tak nie zamierzasz rozbudowywać, nie potrzebuje zapasowego miejsca. Wyrzuć trzecie słowo i masz `Box<str>` — te same bajty na stercie, ta sama własność i to samo zwalnianie pamięci, ale uchwyt mniejszy o 8 bajtów, za to bez `push_str`. Warto zauważyć, gdzie to stawia `Box<str>` względem `&str`: oba uchwyty mają 16 bajtów i oba są parą (wskaźnik, długość), tylko że `Box<str>` **jest właścicielem**. Posiadanie skończonego tekstu nie kosztuje więc więcej niż wskazywanie na cudzy.

Konwersje w obie strony są tanie, ale jedna gubi informację. `into_boxed_str` najpierw wywołuje `shrink_to_fit`, bo `Box<str>` nie ma gdzie zapisać zapasu — więc droga powrotna przez `into_string` oddaje `String`, którego pojemność równa się długości, i pierwszy `push_str` znowu alokuje. Osobna sprawa, warta zmierzenia zamiast zgadywania: czy skracanie **przenosi bajty**. To zwykły `realloc`, a alokator ma prawo załatwić go w miejscu — na tej maszynie 12-bajtowy tekst w buforze na 4096 bajtów został przeniesiony, a ten sam tekst w buforze na 64 bajty już nie. Żadne z tych zachowań nie jest obiecane przez API, więc nie należy na nich polegać; wniosek jest tylko taki, że `into_boxed_str` nie jest automatycznie kopiowaniem.

Kiedy te 8 bajtów ma znaczenie? Odpowiedź jest arytmetyczna, nie uznaniowa: **licz wartości, nie bajty**. Przy tysiącu wartości oszczędność to 8000 bajtów, czyli szum, a sięganie po `Box<str>` kosztuje czytelnika więcej, niż daje maszynie. Przy milionie — tablica symboli, sparsowana kolumna, interner — to osiem milionów bajtów, których nigdy nie zaalokowałeś. Dwie rzeczy, których ta tabela **nie** mówi: oszczędność dotyczy **uchwytów**, a nie tekstu (przy długich napisach proporcja znika), i nic tu nie działa szybciej — liczba wyłuskań do bajtu jest ta sama.

Najciekawsza pułapka dotyczy jednak kształtu, a nie rozmiaru. `Rc<str>` trzyma tekst **wewnątrz** alokacji ze zliczaniem referencji: licznik i bajty to jeden obiekt, jeden skok do bajtu. `Rc<String>` wygląda na oczywisty zapis i jest tym złym — `Rc` trzyma `String`, a ten `String` wskazuje gdzie indziej, czyli **dwie alokacje na wartość i dwa wyłuskania**. I właśnie dlatego ludzie wybierają źle: `Rc<String>` ma *mniejszy* uchwyt (8 bajtów, cienki wskaźnik) niż `Rc<str>` (16 bajtów, wskaźnik gruby), a przegrywa wszędzie indziej. Uchwyt to jedyne miejsce, w którym `Rc<String>` wygrywa — i nie o uchwyt tu chodziło.

Po co więc `Rc<str>`? Po to, żeby wielu właścicieli dzieliło **jeden bufor**: klonowanie uchwytu nie kopiuje tekstu, tylko podnosi licznik. To jest **internowanie napisów**, i dlatego `Rc<str>` pojawia się w parserach, tablicach symboli i każdej kolumnie z powtórzeniami — koszt przestaje rosnąć z liczbą *wierszy*, a zaczyna z liczbą *różnych wartości*. `Arc<str>` to to samo z licznikiem atomowym, kiedy tablica przekracza granicę wątku. Stąd też pułapka warta zapamiętania: `.to_owned()` na inteligentnym wskaźniku klonuje **wskaźnik**, a nie tekst pod nim — jeden `deref` dalej, na `&str`, ta sama nazwa metody oznacza już alokację i kopię wszystkich bajtów.

Na koniec rada domyślna: pisz `String`. To typ, który przyjmuje każde API i rozpoznaje każdy czytelnik, a jedna wartość kosztuje 8 bajtów więcej, czyli nic. Od domyślnego wyboru odchodź tylko przy jednym z trzech sygnałów: trzymasz ich **dużo** i są skończone (`Box<str>`), **powtarzają się** (`Rc<str>` z pulą — ale najpierw zmierz liczbę różnych wartości), albo **przekraczają wątek** (`Arc<str>`).

**Szukaj po polsku:** inteligentny wskaźnik · zliczanie referencji · internowanie napisów · pojemność a długość · `rust Box<str> vs String` · `rust Rc<str> Arc<str>`
