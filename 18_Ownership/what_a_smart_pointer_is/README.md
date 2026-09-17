# What a smart pointer is

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A smart pointer is a type that owns or manages what it points at and still behaves like a reference — [`Deref` ↗](https://doc.rust-lang.org/std/ops/trait.Deref.html) makes `*p` and method calls reach the target, and `Drop` runs its cleanup — so `Box`, `Rc`, `Arc`, `String` and a `MutexGuard` are all the same idea with different rules about who owns what.

## What it has to cover

- The two traits that make a type a smart pointer, with one hand-written `MyBox<T>` implementing both and printing when each runs
- **The family in one table** — what each owns, whether it can be shared, whether it can cross threads, and what its `Drop` does:

  | Type | Owns | Shared | Across threads | Lesson |
  |---|---|---|---|---|
  | `Box<T>` | one heap value | no | if `T` can | [`Box`](../../26_Collections/the_box/README.md) |
  | `Rc<T>` | a counted heap value | yes, read-only | no | [`Rc`](../reference_counting/README.md) |
  | `Arc<T>` | the same, counted atomically | yes, read-only | yes | [`Arc`](../sharing_across_threads/README.md) |
  | `Cow<'a, B>` | borrowed or owned | — | — | [`Cow`](../clone_on_write/README.md) |
  | `Ref` / `MutexGuard` | a borrow of a cell or lock | no | guard-specific | [Interior mutability](../../09_Advanced/interior_mutability/README.md) |

- `String` and `Vec<T>` counted as smart pointers too, and why The Book does so
- **Deref coercion:** why a `&Box<String>` can be passed where `&str` is wanted, and where the chain of derefs stops
- Choosing: `Box` for one owner, `Rc` for several on one thread, `Arc` for several threads, and `RefCell` or `Mutex` added only when a shared owner must also write

## The trap it exists for

Implementing `Deref` on a newtype to inherit the inner type's methods. It compiles and reads well, then every method name of the target becomes yours to collide with, and a reader cannot tell which type a call reached.

## See also

- [Method resolution](../../12_Traits/method_resolution/README.md) — how a call walks through `Deref` to find a method
- [Coercion](../../29_Conversion/coercion/README.md) — deref coercion as one of the conversions the compiler does for you
- [Drop and RAII](../../12_Traits/drop_and_raii/README.md) — the other trait
- [Stack and heap](../stack_and_heap/README.md) — where the pointer lives and where its target does
- [The Book, chapter 15 ↗](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

## Po polsku

**Inteligentny wskaźnik** (*smart pointer*) to typ, który jest właścicielem wskazywanej wartości albo nią zarządza, a mimo to zachowuje się jak referencja: trait `Deref` sprawia, że `*p` i wywołania metod trafiają do wartości docelowej, a `Drop` wykonuje sprzątanie. `Box`, `Rc`, `Arc`, `String` i strażnik blokady `MutexGuard` to jedna idea z różnymi zasadami własności — jeden właściciel, wielu właścicieli w jednym wątku, wielu właścicieli w wielu wątkach.

**Szukaj po polsku:** inteligentne wskaźniki w Ruscie · `rust smart pointers deref drop` · `rust box rc arc difference`
