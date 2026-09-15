# How to learn `ToOwned`: ten steps, in order

**Level:** 101 → 201 · a learning path

**One line:** `ToOwned` itself adds one idea — a separate owned type — and the confusion lives in five ideas underneath it: unsized types, owned/borrowed pairs, `Clone`, method lookup and blanket impls. Re-reading the trait does not help; learning those five in order does.

## Find your step

Each surprise below is one missing idea, not a mystery about `ToOwned`. Start at the step in your row and work down from there.

| If this surprised you | Start at step |
|---|---|
| `ToOwned` exists at all, when `Clone` already does | 1 |
| `name.clone()` on a `&str` gave back a `&str` | 3 |
| `r.clone()` on a `&String` gave a `String`, not a reference | 4 |
| `a.to_owned()` on a `&Foo` gave a `&Foo`, or `E0308` said *expected `Foo`, found `&Foo`* | 5 |
| `42_i32.to_owned()` compiles, and is `42` | 5 |
| `.to_owned()` on an `Rc` did not copy the string | 7 |
| `.to_owned()` on a `Cow::Borrowed` is still borrowed | 7 |
| `HashMap<String, _>::get` accepts a `&str` | 8 |
| `impl ToOwned for MyType` is `E0119` | 10 |
| you cannot write `Borrow` for your view struct | 10 |

## The steps

Each step names what to learn, a checkpoint, and the page that teaches it. **Predict the checkpoint before you look** — the answers are the verified output at the bottom, from a program CI runs. A wrong prediction means stay on that step: the next one assumes it.

### 1. Some types have no size

`str`, `[T]` and `Path` have no size known at compile time, so you never hold one — only a pointer to one, such as `&str` or `Box<str>`, and that pointer carries the length as a second word.

**Checkpoint.** Is a `&str` the same size as a `&String`?

**Read:** [`str` is unsized](../../14_Strings/str_is_unsized/README.md)

### 2. Owned and borrowed are two different types

`String` and `str`. `Vec<T>` and `[T]`. `PathBuf` and `Path`. The borrowed half is the type **behind** the `&`, not the `&` itself — which is how the owned twin of `str` gets to be a different type, `String`.

**Checkpoint.** For `s: String` and `v: Vec<i32>`, what types are `*s` and `*v`?

**Read:** [`String` vs `&str`](../../14_Strings/string_vs_str/README.md)

### 3. `Clone` hands back `Self` — and every `&T` is `Clone`

`fn clone(&self) -> Self` can only return the type it started from, and `Clone` requires `Sized`, so `str` can never be `Clone`. Separately, a shared reference `&T` is `Copy` for *every* `T`, and therefore `Clone`, whether or not `T` is.

**Checkpoint.** `Ticket` does not implement `Clone`. After `let a = &ticket; let b = a;`, is `a` still usable?

**Read:** [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md), and its section on `&T` being `Copy`

### 4. The dot takes the first receiver that fits

`x.clone()` is a search: methods on `x`'s type first, then on `&x`, then dereference and repeat — and the first match wins. On a `&String`, `String::clone` fits first. On a `&str`, `str` has no `clone`, so the search falls through to the reference's own.

**Checkpoint.** What types are `name.clone()` for `name: &str`, `r.clone()` for `r: &String`, and `Clone::clone(&r)`?

**Read:** [Method resolution](../method_resolution/README.md)

### 5. One blanket impl covers every `Clone` type — references included

`impl<T: Clone> ToOwned for T` gives `to_owned()` to every `Clone` type, doing exactly `clone()`. Steps 3 and 4 meet here: when `Foo` is not `Clone`, `foo_ref.to_owned()` finds nothing on `Foo`, finds the impl for `&Foo` instead, and hands back the pointer.

**Checkpoint.** What is `42_i32.to_owned()`? And for a `Ticket` that is not `Clone`, is `(&ticket).to_owned()` a new ticket or the same address?

**Read:** [Extension traits](../extension_traits/README.md) for one impl reaching every type that fits, then the [`ToOwned`](../to_owned/README.md) section on a reference being `Clone` when its pointee is not

### 6. `ToOwned` is `Clone` with a separate owned type

Now the trait is small. `type Owned` is what lets `str`'s answer be `String`. For every `Clone` type it is the blanket impl from step 5; the impls std writes by hand are all on the unsized types from step 1.

**Checkpoint.** What does each of `"hi"`, `[1_i32, 2][..]` and `Path::new("notes.txt")` turn into with `.to_owned()`?

**Read:** [`ToOwned`](../to_owned/README.md), its first three sections

### 7. The traps are steps 4 and 5 together

`Rc<String>` is `Clone`, so `.to_owned()` on it is `Rc::clone` — a pointer and a count. `Cow` is `Clone`, so `.to_owned()` on it is another `Cow` in the same variant. Neither is a bug in `ToOwned`: both are lookup finding the outer type first.

**Checkpoint.** After `shared.to_owned()` on an `Rc<String>`, what is the strong count? After `.to_owned()` on a `Cow::Borrowed`, which variant do you hold?

**Read:** the [`ToOwned`](../to_owned/README.md) section on the trap the blanket impl sets, and [`Rc`: the clone that copies a pointer](../../18_Ownership/reference_counting/README.md)

### 8. `Borrow` is the way back

`type Owned: Borrow<Self>` promises that an owned value can lend out its borrowed form — `String: Borrow<str>`. That is what lets `HashMap<String, _>::get` take a `&str`, and what lets a `Cow` hand you a `&str` from either arm.

**Checkpoint.** Does `seats.get("Ada")` compile on a `HashMap<String, u32>`, and what does it return?

**Read:** [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md)

### 9. `clone_into` refills instead of allocating

The provided method writes into an owned value you already have. It saves something only when that value already has room: into a fresh `String::new()` it allocates exactly as `to_owned` would.

**Checkpoint.** `buf` was made by `String::with_capacity(64)`. Does `"reuse me".clone_into(&mut buf)` change its capacity?

**Read:** [`clone_into`](../clone_into/README.md)

### 10. Implementing it: on the referent, or not at all

A type that is `Clone` cannot have an impl of its own (`E0119`). A struct holding references cannot be lent out of an owned struct, because `borrow` must return a `&`. The trait fits an unsized wrapper around one buffer; for everything else, write an inherent method.

**Checkpoint.** `NameRef<'a>` derives `Clone` and has an inherent `fn to_owned(&self) -> Name`. What do `view.to_owned()` and `ToOwned::to_owned(&view)` return?

**Read:** [Implementing `ToOwned` for your own type](../implementing_to_owned/README.md)

## Four tests for anything else you read

Most explanations of `ToOwned` get the trait right and one step underneath it wrong. The tell for each:

- **"On a reference, `clone()` just copies the pointer."** True only when the pointee is not `Clone` — step 4. On a `&String` it allocates a new `String`.
- **"To implement `ToOwned` for your struct, write `type Owned = Self`."** It compiles only while the struct is not `Clone`, is `Clone` under another name, and becomes `E0119` the day someone adds `#[derive(Clone)]` — step 10.
- **"`clone_into` is another way to get owned data."** It is the way to *refill* owned data; into an empty `String` it saves nothing — step 9.
- **"Prefer `to_owned()` to `to_string()`, because it is faster."** Not since Rust 1.9; the [`ToOwned`](../to_owned/README.md) page has the measurements. The argument that survives is that `to_owned` names what changes — the owner.

## If you are coming from another language

- **C++** — steps 1 and 2 are `std::string` and `std::string_view`, except that Rust also names the thing the view points at. Step 3 is a copy constructor you have to call by name. Steps 4 and 5 are where C++ intuition misleads: copying `*p` and copying `p` are spelled differently there, while in Rust `r.clone()` can mean either, and lookup decides.
- **Python** — none of this exists, because every name is already a shared reference and `copy.copy` is the one duplicate. Step 3 is the nearest thing to new: in Rust, being copyable is a trait a type may decline to implement.

## Check your predictions

<details markdown="1">
<summary><strong>The checkpoint answers</strong></summary>

<!-- output:how_to_learn_to_owned -->
*Verified output of [`how_to_learn_to_owned.rs`](examples/how_to_learn_to_owned.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Step 1. Is &str the same size as &String?
   size_of::<&str>() == size_of::<&String>():     false
   size_of::<&str>() == 2 * size_of::<&String>(): true   (address + length)

Step 2. For s: String and v: Vec<i32>, what types are *s and *v?
   *s : str
   *v : [i32]

Step 3. Ticket is not Clone. After `let b = a;` is `a` still usable?
   yes: a.seat = 12, b.seat = 12   (&Ticket is Copy)

Step 4. What type does each clone return?
   name.clone()      name: &str    -> &str
   r.clone()         r: &String    -> alloc::string::String
   Clone::clone(&r)                -> &alloc::string::String

Step 5. What is 42_i32.to_owned()? Is (&ticket).to_owned() a new ticket?
   42_i32.to_owned() -> i32 42
   a.to_owned() is the same address as a: true   (it is a &Ticket)

Step 6. What does each .to_owned() turn into?
   "hi".to_owned()                   -> alloc::string::String
   [1_i32, 2][..].to_owned()         -> alloc::vec::Vec<i32>
   Path::new("notes.txt").to_owned() -> std::path::PathBuf

Step 7. Strong count after Rc::to_owned? Variant after Cow::to_owned?
   Rc::strong_count = 2, same allocation: true
   Cow::Borrowed(..).to_owned() -> Cow::Borrowed

Step 8. Does seats.get("Ada") compile on a HashMap<String, u32>?
   yes, because String: Borrow<str> -> Some(3)

Step 9. Does clone_into change a roomy buffer's capacity?
   capacity unchanged: true, buf = "reuse me"

Step 10. NameRef derives Clone and has an inherent to_owned. What comes back?
   view.to_owned()          -> Name    { first: "Ada" }   the inherent method
   ToOwned::to_owned(&view) -> NameRef { first: "Ada" }   the blanket impl
```
<!-- /output -->

</details>

## See also

- [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) — the same kind of page, for the other wall
- [`ToOwned`](../to_owned/README.md) — the trait page steps 5 to 7 keep returning to
- [Implementing `ToOwned` for your own type](../implementing_to_owned/README.md) — step 10 in full

## Po polsku

Jeśli `ToOwned` wciąż się „nie klei”, to prawie na pewno nie z powodu samej cechy (*trait*). Ona wnosi jedną nową informację — typ powiązany `Owned`, czyli osobny typ dla wersji posiadanej. Zamieszanie siedzi w pięciu pojęciach pod spodem: typy bez znanego rozmiaru (`str`, `[T]`, `Path`), para własność–pożyczka jako **dwa różne typy** (`String` i `str`), `Clone` zwracające `Self`, wyszukiwanie metody po kropce i implementacja zbiorcza (*blanket impl*). Dlatego ponowne czytanie dokumentacji `ToOwned` nie pomaga, a przejście tych kroków po kolei — tak.

Najważniejsze są kroki 3–5 razem. Każda referencja współdzielona `&T` jest `Copy`, więc i `Clone`, więc — przez `impl<T: Clone> ToOwned for T` — ma `to_owned()`. Kiedy `Foo` nie jest `Clone`, `(&foo).to_owned()` nie znajduje niczego na `Foo` i spada na implementację dla `&Foo`: dostajesz **kopię wskaźnika**, a z adnotacją typu — `E0308` bez słowa o `Clone`. Na `&String` jest odwrotnie: `String::clone` pasuje pierwsze, więc dostajesz nowy `String`. Te same znaki, dwa wyniki, a rozstrzyga kolejność wyszukiwania.

Każdy krok ma punkt kontrolny: najpierw przewidź wynik, potem porównaj go ze zweryfikowanym wydrukiem na dole strony. Błędna przepowiednia znaczy „zostań na tym kroku” — następny zakłada, że poprzedni już siedzi.

**Szukaj po polsku:** typy bez znanego rozmiaru · implementacja zbiorcza · `rust to_owned vs clone` · `rust autoref method resolution` · `rust clone on reference returns reference`
