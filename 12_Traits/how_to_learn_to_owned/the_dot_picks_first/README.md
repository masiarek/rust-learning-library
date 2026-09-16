# Step 5: The dot takes the first receiver that fits

[How to learn `ToOwned`](../README.md) › **Step 5 of 10** · back: [Step 4 — `Clone` hands back `Self`](../clone_returns_self/README.md) · next: [Step 6 — One blanket impl covers every `Clone` type](../the_blanket_to_owned/README.md)

**Level:** 201 · a step on a learning path

**One line:** `x.method()` is a search — the receiver's own type, then `&` of it, then `&mut` of it, then dereference and repeat — and the first method whose `self` type matches wins. That is why `.clone()` gives a `String` on a `&String` and a `&str` on a `&str`, and why `.to_owned()` on a `&str` needs no dereference at all.

## The candidate list

For a receiver of type `R`, the compiler writes out a list: `R`, then everything `R` dereferences to, one rung at a time. After each type `U` in that list it inserts `&U` and `&mut U`. Then it walks the list, and at each entry looks for a method **whose receiver type is exactly that entry** — `&self` on a `str` impl counts as receiver type `&str`. The first entry with a match ends the search.

For `r: &String` the list is:

`&String` · `&&String` · `&mut &String` · `String` · `&String` · `&mut String` · `str` · `&str` · `&mut str`

## Walk it for `.clone()`

- **`r: &String`.** First entry, `&String`. `String`'s `clone(&self)` has receiver type `&String` — match. Result: a `String`. The reference's own `clone`, receiver type `&&String`, sat one entry later and was never reached.
- **`name: &str`.** First entry, `&str`. A `clone` with receiver type `&str` would have to be `str`'s, and `str` is not `Clone` ([step 4](../clone_returns_self/README.md)). Second entry, `&&str`: the reference's own `clone` — match. Result: a `&str`.
- **`Clone::clone(&r)`.** No search at all. The argument is a `&&String`, which fixes `Self = &String`, so the result is a `&String`. Writing the trait's name is how you choose the impl yourself — [A trait must be in scope](../../trait_in_scope/README.md#three-ways-to-spell-the-same-call) has the three spellings.

## Name the trait, and the `&` you pass decides

`Clone::clone(x)` is `fn clone(&self) -> Self` called as a plain function. Its one argument has to be a `&Self`, and with only the trait named, the compiler reads `Self` straight off the argument's type:

- **`Clone::clone(r)`** — `r` is a `&String`, so `Self = String`, and you get a new `String`: the same function `r.clone()` found.
- **`Clone::clone(&r)`** — `&r` is a `&&String`, so `Self = &String`, and you get the reference, copied.

So naming the trait did not pick the reference's impl. It switched the dot's search off, and then the `&` you wrote picked it.

Name the type as well — `<String as Clone>::clone(…)` — and `Self` is no longer read off the argument; it is fixed. The argument then becomes an ordinary function argument that may be coerced to fit, so `<String as Clone>::clone(&r)` still hands back a `String`, the `&&String` shortened to a `&String` by deref coercion. Coercion only ever removes a `&`, though: `<&String as Clone>::clone(r)` is `E0308`, *expected `&&String`, found `&String`* (rustc 1.98.0).

That long form is **fully qualified syntax**, the name The Book uses. Older texts call all of these calls *UFCS*, universal function call syntax, after [RFC 132 ↗](https://rust-lang.github.io/rfcs/0132-ufcs.html). [A trait must be in scope](../../trait_in_scope/README.md#three-ways-to-spell-the-same-call) has the three spellings side by side, and [the case where only the long one works](../../trait_in_scope/README.md#the-case-that-forces-the-long-spelling).

## Walk it for `.to_owned()` — and see where a deref happens

- **`t: &str`.** First entry, `&str`. `str`'s `to_owned(&self)` has receiver type `&str` — match, a `String`. **No dereference happened.** And if one had been needed, the search would have reached `&&str` first, where `&str`'s own `to_owned` (from the blanket impl, [step 6](../the_blanket_to_owned/README.md)) was waiting, and handed back a `&str`. Getting a `String` is the proof that the first entry matched. **Nor was a reference added**: there was no autoref either. The `&str` went in exactly as it was — `s.to_owned()` is `<str as ToOwned>::to_owned(s)`, with `s` itself as the argument, which [step 1's output](../clone_vs_to_owned/README.md#checkpoint) runs both ways.
- **`m: &mut str`.** Nothing fits `&mut str`, `&&mut str` or `&mut &mut str` — a `&mut str` is not `Clone`. Dereference to `str`: nothing by value; then `&str`: `str`'s `to_owned` — match, a `String`. **This** is a call where autoderef does the work.
- **`tt: &&str`.** First entry, `&&str`: `&str`'s own `to_owned` — match, a `&str`. The search stops two rungs before it could reach `str`.

## Where `.clone()` goes further — and where it never does

- **`r2: &&String`.** First entry, `&&String`. `&String` is `Clone`, and its `clone` takes exactly a `&&String` — match, a `&String`. However many `&`s you stack, `.clone()` peels one and stops, because a shared reference is always `Clone`. rustc warns about it, `suspicious_double_ref_op`; `Clone::clone(*r2)` is the spelling that reaches the `String`.
- **`mu: &mut String`.** A `&mut` is **not** `Clone` — std says so with a negative impl, `impl !Clone for &mut T` — so nothing fits `&mut String`, `&&mut String` or `&mut &mut String`. Dereference to `String`: nothing by value; then `&String`: `String::clone` — match, a new `String`. On a `&mut`, `.clone()` does reach the value, one deref and one autoref later.
- **`r2.capacity()`.** Nothing named `capacity` takes a `&&String`, a `&&&String` or a `&mut &&String`. Dereference once to `&String`: `String::capacity(&self)` takes exactly that — match. This is what autoderef is *for*: a method the outer type does not have. `.clone()` rarely gets there on a shared reference, because the reference always has a `clone` of its own.

## Checkpoint

**Predict before you open the answer.** What types are `name.clone()` for `name: &str`, `r.clone()` for `r: &String`, and `Clone::clone(&r)`? Then `Clone::clone(r)` without the `&`, the two fully qualified calls, and the three `to_owned` calls from the sections above.

<details markdown="1">
<summary><strong>The answer</strong></summary>

<!-- output:the_dot_picks_first -->
*Verified output of [`the_dot_picks_first.rs`](examples/the_dot_picks_first.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Checkpoint. What type does each clone return?
   name.clone()      name: &str    -> &str
   r.clone()         r: &String    -> alloc::string::String
   Clone::clone(&r)                -> &alloc::string::String

Name only the trait, and the argument's type decides Self
   Clone::clone(r)     r: &String    Self = String  -> alloc::string::String
   Clone::clone(&r)    &r: &&String  Self = &String -> &alloc::string::String
Name the type too, and the argument is coerced to fit it
   <String as Clone>::clone(&r)      Self = String  -> alloc::string::String
   <&String as Clone>::clone(&r)     Self = &String -> &alloc::string::String

The same search, for to_owned
   t.to_owned()      t: &str       -> alloc::string::String
   m.to_owned()      m: &mut str   -> alloc::string::String
   tt.to_owned()     tt: &&str     -> &str

Where the search goes past the first entry, and where it does not
   r2.clone()        r2: &&String    -> &alloc::string::String   the inner reference, copied
   Clone::clone(*r2)                 -> alloc::string::String
   mu.clone()        mu: &mut String -> alloc::string::String   &mut is not Clone: deref, then &
   r2.capacity()     r2: &&String    -> usize           capacity takes &String: one deref
```
<!-- /output -->

</details>

## What this step sets up

- [Step 6](../the_blanket_to_owned/README.md): the blanket impl puts a `to_owned` on every `&T`, which is one more candidate for this search to find first.
- [Step 7](../to_owned_traps/README.md): on an `Rc<String>` or a `Cow`, the outer type is the first entry, and it is `Clone`.
- [Step 10](../implementing_it_or_not/README.md): at the same entry an inherent method beats a trait method, which is how a `to_owned` of your own wins.

## Go deeper

**Read:** [Method resolution](../../method_resolution/README.md) — [the search](../../method_resolution/README.md#the-search), [it crosses as many pointers as it needs](../../method_resolution/README.md#it-crosses-as-many-pointers-as-it-needs) and [the trap: an inherent method shadows the target's](../../method_resolution/README.md#the-trap-an-inherent-method-shadows-the-targets).

**Around it in the library:**

- [Coercion — method calls are a *different* rule that looks the same](../../../29_Conversion/coercion/README.md#method-calls-are-a-different-rule-that-looks-the-same)
- [A trait must be in scope](../../trait_in_scope/README.md), the `use` that puts a trait's methods in the search at all
- ["No method named …"](../../no_method_named/README.md), what `E0599` means when the search finds nothing
- [Reborrowing](../../../18_Ownership/reborrowing/README.md), what the `&mut U` entries do to a `&mut` receiver
- [`String` vs `&str`](../../../14_Strings/string_vs_str/README.md), where `s.len()` reaching `str::len` first shows up

**Docs:** [Method call expressions ↗](https://doc.rust-lang.org/reference/expressions/method-call-expr.html), in the Reference: the candidate list, in order · [The dot operator ↗](https://doc.rust-lang.org/nomicon/dot-operator.html), in the Rustonomicon · [`noop_method_call` ↗](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#noop-method-call)

**Words:** *method* and *receiver* in the [glossary](../../../GLOSSARY.md).

**Kata:** none of its own — [step 6's](../the_blanket_to_owned/README.md) exercises the same search.

## Po polsku

`x.metoda()` to wyszukiwanie. Kompilator bierze typ odbiorcy, potem kolejne typy po dereferencji, a po każdym typie `U` wstawia `&U` i `&mut U`. Idzie po tej liście i zatrzymuje się na pierwszej metodzie, której typ odbiorcy (`self`) pasuje **dokładnie**.

Dla `t: &str` pierwszy kandydat to `&str`, a `to_owned(&self)` z implementacji dla `str` ma odbiorcę `&str` — pasuje od razu, wynik to `String`, **bez żadnej dereferencji**. Gdyby dereferencja była potrzebna, wyszukiwanie trafiłoby wcześniej na `&&str` i zwróciło `&str`. Autodereferencja naprawdę pracuje np. dla `m: &mut str`. Wywołanie z nazwą cechy pomija wyszukiwanie w ogóle — i wtedy o `Self` decyduje typ argumentu: `Clone::clone(r)` daje `String`, a `Clone::clone(&r)` daje `&String`, bo to `&` wybrało implementację dla referencji. Pełna składnia `<String as Clone>::clone(&r)` ustala `Self` z góry, a argument zostaje dopasowany przez *deref coercion*.

**Szukaj po polsku:** wyszukiwanie metod w Ruscie · `rust method resolution autoref autoderef` · `rust clone on &str returns &str`

---

[How to learn `ToOwned`](../README.md) › **Step 5 of 10** · back: [Step 4](../clone_returns_self/README.md) · next: [Step 6 — One blanket impl covers every `Clone` type](../the_blanket_to_owned/README.md)
