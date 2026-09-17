# The drop check

**Level:** 301 · deep dive

**One line:** A `Vec<&String>` may be dropped after the `String` it borrows is already gone, because `Vec`'s destructor promises not to read its elements — a promise std makes with the unstable `#[may_dangle]` and your own `impl Drop` cannot, which is why the same shape with your type is `E0597`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Drop order first: locals are dropped in reverse order of declaration — rustc's own note says "values in a scope are dropped in the opposite order they are defined". So in `let mut v = Vec::new(); let s = String::from("a"); v.push(&s);`, `s` goes first and `v` holds a dangling `&s` while it is being dropped. Why does that compile?
- The same shape with `struct Holder<'a>(&'a String)` and an `impl Drop for Holder<'_>` — `let h; let t = String::from("b"); h = Holder(&t);` — is `E0597`: "`t` dropped here while still borrowed … borrow might be used here, when `h` is dropped and runs the `Drop` code for type `Holder`". Remove the `impl Drop` and it compiles. What exactly did adding a destructor change?
- `#[may_dangle]`: std's source writes `unsafe impl<#[may_dangle] T, …> Drop` for `Vec`, `VecDeque` and `Box`, promising the destructor will drop `T` values without otherwise looking at them. On stable the attribute is `E0658` ("`may_dangle` has unstable semantics and may be removed in the future", issue #34761); the Nomicon's [drop check chapter ↗](https://doc.rust-lang.org/nomicon/dropck.html) shows it behind `#![feature(dropck_eyepatch)]`. Why is the `impl` `unsafe`?
- `PhantomData<T>` for ownership: a type that holds `*const T` and drops `T`s in its destructor must say it owns a `T`, or the drop check does not know to check `T` at all. The claim table on [Phantom types](../../12_Traits/phantom_types/README.md) — which row does a raw-pointer collection need?
- A trait object or a generic whose destructor *does* read the borrowed data: the case the rule exists to refuse, shown as a program that would print freed memory if it compiled.
- Adding a `Drop` impl to a public generic type that holds references: can that break a downstream crate that compiled yesterday?
- `ManuallyDrop`: wrapping the `Holder` above in `ManuallyDrop::new` makes the `E0597` disappear, because nothing will run its destructor at the end of the scope. What has that moved onto you, and when is it the right answer?

## The trap it exists for

A `Drop` impl added for logging to a struct that holds a reference. Code that has compiled for months now fails with `E0597` wherever a holder is declared before the value it borrows, and the error names the struct's destructor rather than anything that looks wrong.

## Where this sits

- [The drop flag](../../18_Ownership/the_drop_flag/README.md) covers whether a location still holds a value at run time; [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) covers when a destructor runs; [Phantom types](../../12_Traits/phantom_types/README.md) covers `PhantomData`'s three spellings. This page covers only the borrow-check rule about what a destructor may see.

## See also

- [The drop flag](../../18_Ownership/the_drop_flag/README.md) — the run-time half of dropping
- [Phantom types](../../12_Traits/phantom_types/README.md) — `PhantomData<T>` as the claim that a struct owns a `T`, which is what the drop check reads
- [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) — the destructor whose existence changes the rule
- [Lifetime annotations](../../18_Ownership/lifetime_annotations/README.md) — the `'a` in `Holder<'a>`
- [`unsafe trait` and `unsafe impl`](../unsafe_traits/README.md) — `unsafe impl Drop` is one more promise signed at the `impl`
- [What `unsafe` turns off](../what_unsafe_turns_off/README.md) — the borrow checker stays on, and this is one of its rules

## If you are coming from another language

- **C++.** Destructors run in reverse order of construction too, and a `std::vector<std::string_view>` whose strings died first is destroyed without complaint — the compiler checks nothing, and a destructor that reads a dangling view is undefined behaviour at run time. Rust's drop check is that same situation turned into a compile-time question: will this destructor look?
- **Python.** No counterpart, and the reason is instructive: a list holding a string keeps the string alive by reference count, so the referent cannot die first. `__del__` never sees a freed object; Rust's borrowed-data model is what makes the question possible.
- **Go.** A garbage collector plus finalizers with no ordering guarantee — `runtime.SetFinalizer` runs at some point after the object becomes unreachable — so there is no destructor order to reason about and nothing like `E0597`.

## Po polsku

**Sprawdzanie przy upuszczaniu** (*drop check*) to reguła borrow checkera o tym, co destruktor może zobaczyć. Zmienne lokalne są niszczone w odwrotnej kolejności deklaracji, więc `Vec<&String>` bywa upuszczany już po zniknięciu `String`a — i to się kompiluje, bo destruktor `Vec` obiecuje (niestabilnym atrybutem `#[may_dangle]`), że elementów nie czyta. Twoja struktura z referencją i własnym `impl Drop` takiej obietnicy złożyć nie może, dlatego identyczny kod daje `E0597`. Pułapka: dopisanie `Drop` „tylko do logowania” potrafi zepsuć kod, który od miesięcy działał.

**Szukaj po polsku:** kolejność niszczenia zmiennych · destruktor w Ruscie · `rust dropck may_dangle` · `rust E0597 when dropped runs the Drop code` · `nomicon drop check`
