# Smart pointers

**Level:** 201 · working knowledge

**One line:** A smart pointer is a type that acts like a pointer through `Deref` and does a job through `Drop` — freeing, counting, unlocking, giving a borrow back — and this section is about those two traits, what the jobs cost at run time, and the claims about smart pointers that do not hold.

The Book's [chapter 15 ↗](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) and std's [`Deref` ↗](https://doc.rust-lang.org/std/ops/trait.Deref.html) docs agree on the definition: a smart pointer is usually a struct, and what makes it one is `Deref` (and often `Drop`). The [Pointers](../36_Pointers/README.md) section is the layer underneath — addresses, references and the raw pointers every smart pointer is built on.

## Lessons

| Lesson | Level | What it answers |
|---|---|---|
| [What makes a pointer smart](what_makes_a_pointer_smart/README.md) | 201 | `Deref` and `Drop` in a twenty-line smart pointer; what `String`, `Vec`, `Box`, `Rc`, `Cow` and `Ref` deref to; why owning is usual but not required; why `Weak` has no `Deref` |
| [What a smart pointer costs](what_a_smart_pointer_costs/README.md) | 201 → 301 | Counted by a global allocator: the one-word handle, the 8 and 24 bytes `Box` and `Rc` allocate, the count a clone changes, the `free` that waits for the last `Weak`, and `RefCell`'s flag |
| [Smart pointer claims, run](smart_pointer_claims_checked/README.md) | 201 | "Boiled away", "always owns", "raw pointers such as `Rc`", "without fuss", "single-threaded" — and the §6.2.3 building blocks *Rust in Action* names |

## The smart pointers, and where each is taught

Several of these had lessons before this section existed; they stay where they are, and this table is the map:

| Type | `Deref` to | Its job, done in `Drop` | Lesson |
|---|---|---|---|
| `Box<T>` | `T` | own one value on the heap, free it | [`Box`](../26_Collections/the_box/README.md) |
| `Rc<T>` | `T` | shared ownership, counted; drop the value at zero | [`Rc`: the clone that copies a pointer](../18_Ownership/reference_counting/README.md) |
| `Arc<T>` | `T` | the same, with atomic counts, across threads | [Sharing across threads: `Arc`](../18_Ownership/sharing_across_threads/README.md), [`Send` and `Sync`](../09_Advanced/send_and_sync/README.md) |
| `Weak<T>` | — none | point without keeping alive; `upgrade()` to ask | [What makes a pointer smart](what_makes_a_pointer_smart/README.md#weak-has-no-deref) |
| `String`, `Vec<T>` | `str`, `[T]` | own a growable buffer | [The anatomy of a `String`](../14_Strings/anatomy_of_a_string/README.md), [Collections](../26_Collections/README.md) |
| `Cow<'_, B>` | `B` | borrow until a change needs an owned copy | [`Cow`: borrow until somebody writes](../18_Ownership/clone_on_write/README.md), [What `Cow` explanations get wrong, run](../12_Traits/how_to_learn_to_owned/cow_claims_checked/README.md) |
| `Ref<'_, T>`, `RefMut<'_, T>` | `T` | give a `RefCell` borrow back | [Interior mutability](../09_Advanced/interior_mutability/README.md) *(outline)* |
| `MutexGuard<'_, T>` | `T` | unlock | [Lock poisoning](../09_Advanced/mutex_poisoning/README.md) |
| `Pin<P>` | `P::Target` | promise the pointee will not move | [glossary](../GLOSSARY.md) |

`Cell<T>` and `RefCell<T>` are not pointers — they hold the value inline — but `Rc<RefCell<T>>` is the most common place to meet one, which is why they are here.

[What a smart pointer is](../18_Ownership/what_a_smart_pointer_is/README.md), an outline in Ownership, frames the same family by who owns what, and names the trap of implementing `Deref` on a newtype to inherit its methods.

## The two traits underneath

| Trait | Page |
|---|---|
| `Deref`, `DerefMut` | [What makes a pointer smart](what_makes_a_pointer_smart/README.md), [Method resolution](../12_Traits/method_resolution/README.md), [Coercion](../29_Conversion/coercion/README.md), [Step 3: Owned and borrowed are two different types](../12_Traits/how_to_learn_to_owned/owned_and_borrowed_types/README.md) |
| `Drop` | [`Drop`, and what RAII buys](../12_Traits/drop_and_raii/README.md), [The drop flag](../18_Ownership/the_drop_flag/README.md) |

## Also worth reading

- The Book, [ch. 15 — Smart Pointers ↗](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html): `Box`, `Deref`, `Drop`, `Rc`, `RefCell` and reference cycles, in that order
- [Rust Design Patterns — Collections are smart pointers ↗](https://rust-unofficial.github.io/patterns/idioms/deref.html)
- *Rust in Action*, ch. 6, §6.2.2–6.2.3 — a card per pointer type in Figure 6.4, and the building blocks, [checked here](smart_pointer_claims_checked/README.md#from-rust-in-action-623)
- Video: [*Rc and RefCell Smart Pointers - Rust* ↗](https://youtu.be/KYJ95TxEC18) (danlogs) — `Rc<RefCell<T>>` walked through

## See also

- [Pointers](../36_Pointers/README.md) — addresses, references, wide and raw pointers
- [Ownership](../18_Ownership/README.md) — what a smart pointer's `Drop` is taking part in
- [Traits](../12_Traits/README.md) — where `Deref` and `Drop` sit among the others

## Po polsku

**Inteligentny wskaźnik** (*smart pointer*) to typ, który zachowuje się jak wskaźnik dzięki `Deref`, a przez `Drop` wykonuje jakąś pracę: zwalnia pamięć, zlicza właścicieli, odblokowuje muteks, oddaje pożyczkę `RefCell`. Ten dział opisuje obie cechy, koszt tej pracy w czasie działania programu i twierdzenia o inteligentnych wskaźnikach, które się nie sprawdzają.

Część typów miała już wcześniej własne lekcje (`Box`, `Rc`, `Box<str>`/`Rc<str>`/`Arc<str>`, `Cow`) — tabela „where each is taught” prowadzi do nich zamiast je powtarzać. Warto pamiętać, że `String` i `Vec<T>` też są inteligentnymi wskaźnikami w sensie dokumentacji std, a `Weak<T>` nie ma `Deref` wcale.

**Szukaj po polsku:** inteligentne wskaźniki w Ruście · `Box` `Rc` `Arc` · cecha Deref i Drop · `rust smart pointers explained`
