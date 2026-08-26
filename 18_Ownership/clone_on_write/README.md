# `Cow`: borrow until somebody writes

**Level:** 201 · working knowledge

**One line:** `Cow<'a, B>` is an ordinary enum with a `Borrowed` arm and an `Owned` arm, so one function can return either without deciding in advance — and `to_mut()` is the moment the copy actually happens, which is the *write* that "clone on write" is named after.

```rust
use std::borrow::Cow;

fn clean(raw: &str) -> Cow<'_, str> {
    if raw.contains('\r') {
        Cow::Owned(raw.replace('\r', ""))   // had to change it, so had to allocate
    } else {
        Cow::Borrowed(raw)                  // nothing to do, so nothing is copied
    }
}

let a = clean("Ada Lovelace");     // Borrowed — the caller's own bytes
let b = clean("Grace Hopper\r");   // Owned    — one new String
```

The signature is the point. `-> String` would be correct and would allocate for every line; `-> &str` cannot be written at all, because the fixed-up line has no home to borrow from. `Cow` is the return type that lets the *data* decide.

## The type, in full

```rust
pub enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

Read the bounds and the whole design falls out. `?Sized` is what admits `str` and `[T]`, the types you only ever meet behind a pointer. [`ToOwned`](../../12_Traits/to_owned/README.md) is what supplies the other arm — `<str as ToOwned>::Owned` is `String`, `<[i32] as ToOwned>::Owned` is `Vec<i32>` — which is why `Cow` needs that trait rather than `Clone`: `Clone` would only ever give back another `&str`.

It lives in `alloc::borrow` and is re-exported as `std::borrow::Cow`; both paths name the same type.

## You do not have to match on it

`Cow<str>` derefs to `&str`, so the reason to name the arms is to *measure* the difference, not to use the value:

```rust
let c = clean("Grace Hopper\r");
c.len();                 // 12
c.starts_with("Grace");  // true
c.to_uppercase();        // "GRACE HOPPER"
```

Reaching for a `match` in ordinary code is usually a sign you wanted `into_owned()`.

## `to_mut()` is the write

This is the part the name refers to, and the part most explanations skip. A `Cow` you only read stays borrowed forever. Ask for a mutable reference and the borrowed arm is promoted — cloned — on the spot:

```rust
let mut c: Cow<'_, str> = Cow::Borrowed("Ada");
c.to_mut().push_str(" Lovelace");   // Borrowed -> Owned, one clone, right here
c.to_mut().push('!');               // already Owned — nothing is cloned this time
```

So the cost is paid once, at the first write, and never again. `into_owned()` is the other exit: it always hands back the owned type, allocating only if the value was still borrowed.

## The tag is free

An enum is as large as its largest variant plus a discriminant, which sounds like `Cow<str>` should cost a word more than `String`. It does not — the discriminant rides in a niche, and there is still one spare:

| type | size |
|---|---|
| `&str` | 16 bytes — pointer + length |
| `String` | 24 bytes — pointer + length + capacity |
| `Cow<str>` | **24 bytes** — the same as `String` |
| `Option<Cow<str>>` | **24 bytes** — a niche is still left over |

So the choice is about ownership and allocations, not about size.

## When not to reach for it

- **When you always end up owning.** If every branch allocates, `Cow` is a lifetime and a second arm bought for nothing. Take a `String`.
- **In a struct field, by reflex.** `struct Row<'a> { name: Cow<'a, str> }` infects the struct with a lifetime, and that lifetime spreads to everything holding one. `Cow<'static, str>` is the version that does not, and it covers the common *"a literal, or something I computed"* case.
- **To avoid a decision.** `Cow` earns its keep when the common path genuinely has nothing to do — cleaning input that is usually already clean, escaping text that usually needs no escaping. When the split is near 50/50 the complexity is rarely repaid.

## If you are coming from another language

**Python.** You already rely on this, invisibly. `str` is immutable, so CPython is free to hand back the *same object* when a method changes nothing — `s.replace('\r', '') is s` is `True` for a string with no `\r`, and `s[:] is s` is `True` as well. That is exactly the borrowed arm, decided at runtime by the data. What changes in Rust is that the decision becomes part of the **type**: Python hides it as an optimization you cannot see or rely on, and `Cow` puts it in the signature where the caller can. The flip side is that Python never makes you say when the copy happens, and `to_mut()` does.

**ABAP.** The kernel does the same trick for deep data objects — assigning one string or internal table to another shares the buffer, and the copy happens on the first write to either side. You have never had to write it because you have also never been able to *see* it: nothing in `lv_a = lv_b.` says whether that statement copied 4 bytes or 400 megabytes. `Cow` is that mechanism with the lid off — `Cow::Borrowed` and `Cow::Owned` are the two states, and `to_mut()` is the write that flips one to the other, all of it in code you can read.

---

## Practice

**Pay only when you have to.** Write `fn ensure_prefix<'a>(s: &'a str, prefix: &str) -> Cow<'a, str>` that returns `s` untouched when it already starts with `prefix`, and a newly built string when it does not. Run four rows through it, two of each kind, and count how many allocated.

Then prove the untouched rows really were not copied — `std::ptr::eq` on the two `as_ptr()` values, not a printed address, which differs every run. Finally, say what `into_owned()` costs on each arm, and write the same function returning `String` to see what it costs on all of them.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:clone_on_write_kata -->
*[`clone_on_write_kata.rs`](examples/clone_on_write_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a function that allocates only when it has something to change.
//!
//!   rustc --edition 2024 clone_on_write_kata.rs -o /tmp/cowk && /tmp/cowk

use std::borrow::Cow;

/// Return `s` unchanged when it already starts with `prefix`, otherwise a new
/// string with the prefix in front. The `'a` is the whole trick: the borrowed
/// arm hands back the caller's own bytes, so it must not outlive them.
fn ensure_prefix<'a>(s: &'a str, prefix: &str) -> Cow<'a, str> {
    if s.starts_with(prefix) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{prefix}{s}"))
    }
}

fn arm(c: &Cow<'_, str>) -> &'static str {
    match c {
        Cow::Borrowed(_) => "Borrowed",
        Cow::Owned(_) => "Owned",
    }
}

fn main() {
    let rows = ["BV2261 Ada", "Ben", "BV2261 Cara", "Dan"];

    println!("1. Prefix every row, paying only for the ones that lack it");
    let mut allocated = 0;
    for row in rows {
        let out = ensure_prefix(row, "BV2261 ");
        if matches!(out, Cow::Owned(_)) {
            allocated += 1;
        }
        println!("   {:<14} -> {:<22} {}", format!("{row:?}"), format!("{out:?}"), arm(&out));
    }
    println!("   allocated {} of {} rows", allocated, rows.len());

    println!();
    println!("2. Prove the untouched row was not copied");
    let row = "BV2261 Ada";
    let out = ensure_prefix(row, "BV2261 ");
    println!("   points at the caller's own bytes? {}", std::ptr::eq(row.as_ptr(), out.as_ptr()));

    println!();
    println!("3. What into_owned() costs on each arm");
    let borrowed = ensure_prefix("BV2261 Cara", "BV2261 ");
    let owned = ensure_prefix("Dan", "BV2261 ");
    println!("   {:<8} -> into_owned() allocates now      {:?}", arm(&borrowed), borrowed.into_owned());
    println!("   {:<8} -> into_owned() hands the buffer over {:?}", arm(&owned), owned.into_owned());

    println!();
    println!("4. The same rows through a plain String signature, for comparison");
    fn ensure_prefix_owned(s: &str, prefix: &str) -> String {
        if s.starts_with(prefix) { s.to_owned() } else { format!("{prefix}{s}") }
    }
    let _: Vec<String> = rows.iter().map(|r| ensure_prefix_owned(r, "BV2261 ")).collect();
    println!("   allocated {} of {} rows — correct, and it pays every time", rows.len(), rows.len());
}
```
<!-- /source -->

<!-- output:clone_on_write_kata -->
*Verified output of [`clone_on_write_kata.rs`](examples/clone_on_write_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Prefix every row, paying only for the ones that lack it
   "BV2261 Ada"   -> "BV2261 Ada"           Borrowed
   "Ben"          -> "BV2261 Ben"           Owned
   "BV2261 Cara"  -> "BV2261 Cara"          Borrowed
   "Dan"          -> "BV2261 Dan"           Owned
   allocated 2 of 4 rows

2. Prove the untouched row was not copied
   points at the caller's own bytes? true

3. What into_owned() costs on each arm
   Borrowed -> into_owned() allocates now      "BV2261 Cara"
   Owned    -> into_owned() hands the buffer over "BV2261 Dan"

4. The same rows through a plain String signature, for comparison
   allocated 4 of 4 rows — correct, and it pays every time
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:clone_on_write -->
*Verified output of [`clone_on_write.rs`](examples/clone_on_write.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One return type, two outcomes
   "Ada Lovelace"     -> "Ada Lovelace"   Borrowed - nothing allocated
   "Grace Hopper\r"   -> "Grace Hopper"   Owned    - one allocation
   "Alan Turing"      -> "Alan Turing"    Borrowed - nothing allocated

2. The borrowed arm really is the caller's bytes
   clean(raw) points at raw's own buffer? true
   the owned arm allocated a new one, so it cannot: "Grace Hopper"

3. You do not have to match on it — Cow derefs to &str
   len 12   upper "GRACE HOPPER"   starts_with("Grace") true

4. to_mut() is the write, and the write is where the clone happens
   start          Borrowed - nothing allocated
   after to_mut   Owned    - one allocation   "Ada Lovelace"
   second write   Owned    - one allocation   "Ada Lovelace!"   <- already owned, nothing cloned

5. into_owned() always hands back the owned type
   from Borrowed -> String "Alan"   (allocates here)
   from Owned    -> String "Turing"   (just hands the buffer over)

6. The tag is free: Cow<str> is no bigger than String
   &str                  16 bytes   ptr + len
   String                24 bytes   ptr + len + capacity
   Cow<str>              24 bytes   <- same as String
   Option<Cow<str>>      24 bytes   <- and a spare niche is still left over

7. Not a string thing — any ToOwned pair works
   [3, 1, 2] -> [1, 2, 3]   had to sort, so had to allocate
   Cow<[i32]>            24 bytes

8. What it buys, counted
   clean()              1 of 3 lines allocated
   clean_always_owned() 3 of 3 lines allocated
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 18_Ownership/clone_on_write/examples/clone_on_write.rs -o /tmp/cow && /tmp/cow
```

## See also

- [`ToOwned`](../../12_Traits/to_owned/README.md) — the trait that supplies the owned arm, and the reason `Cow` needs it rather than `Clone`
- [Ownership and moves](../ownership_and_moves/README.md) — the one-owner rule this type bends without breaking
- [Borrowing](../borrowing/README.md) — where the `'a` on the borrowed arm comes from
- [The anatomy of a `String`](../../14_Strings/anatomy_of_a_string/README.md) — pointer, length, capacity: the 24 bytes the table above compares against
- [Six kinds of string](../../14_Strings/six_kinds_of_string/README.md) — `Cow<str>` in its place among the owned/borrowed pairs
- [`Cow` ↗](https://doc.rust-lang.org/std/borrow/enum.Cow.html) · [`ToOwned` ↗](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html)
