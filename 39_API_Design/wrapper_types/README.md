# Wrapper types

**Level:** 301 · deep dive

**One line:** A newtype is a new type over an old representation, and how it hands out the inner value — `Deref`, `AsRef`, `Borrow` or a plain method — decides how much of the inner type's API becomes yours to support forever.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Two jobs for a newtype in a public API: a static distinction the compiler checks (C-NEWTYPE), and a representation you can change later because callers never saw it (C-NEWTYPE-HIDE) — [A score is not a number](../../16_Structs/newtype_score/README.md) has the first
- `Deref`, judged by two sources that word it differently: the std docs say implement it when the type behaves transparently like the target, the deref is cheap, and coercion will not surprise anyone; the API Guidelines say "only smart pointers implement `Deref`" (C-DEREF). Which rule explains `String: Deref<Target = str>`?
- Deref polymorphism: using `Deref` to imitate inheritance so one struct borrows another's methods — the anti-pattern in the unofficial Rust Design Patterns book — and what happens when both types define a method with the same name
- `AsRef<T>`: a cheap reference conversion for generic parameters such as `impl AsRef<Path>`, with no promise about equality — [String parameters worth copying](../../14_Strings/string_api_design/README.md)
- `Borrow<T>`: the stronger promise that `Eq`, `Ord` and `Hash` give the same answers on the wrapper and the borrowed form, which is why `HashMap::get` asks for it — [`Borrow`](../../12_Traits/borrow_trait/README.md)
- Getting the value back out: `into_inner(self)` for wrappers such as `BufReader` and `AtomicBool` (C-CONV), and `#[repr(transparent)]` when the wrapper's layout has to equal the inner type's
- What a newtype loses: every trait impl the inner type had — `Debug`, `Display`, `Add` — until you derive or forward it, and the orphan rule as the reason a newtype is how you implement a foreign trait for a foreign type
- Smart pointers do not add inherent methods (C-SMART-PTR): `Box::leak(b)` is an associated function, not `b.leak()`, because a method on `Box<T>` could shadow one on `T`

## The trap it exists for

`impl Deref for Prices { type Target = Vec<u32>; … }` to avoid writing `len` and `iter`. It works, and it also publishes every `&self` method `Vec` has — `capacity`, `as_ptr`, `binary_search` — as part of your type. Switching the storage to a `VecDeque` later is a breaking change for callers you never knew used them.

## Where this sits

[A score is not a number](../../16_Structs/newtype_score/README.md) teaches the newtype and its validating constructor; [`Borrow`](../../12_Traits/borrow_trait/README.md) and [String parameters worth copying](../../14_Strings/string_api_design/README.md) cover `Borrow` and `AsRef` in depth; [Coercion](../../29_Conversion/coercion/README.md) covers deref coercion itself. This page is only the choice of access route for a public wrapper.

## See also

- [A score is not a number: the newtype](../../16_Structs/newtype_score/README.md) — one door into the type, checked once
- [`Borrow`: look up an owned key with a borrowed one](../../12_Traits/borrow_trait/README.md) — the promise `AsRef` does not make
- [String parameters worth copying](../../14_Strings/string_api_design/README.md) — `impl AsRef<str>` counted in allocations
- [Coercion: the conversion you never write](../../29_Conversion/coercion/README.md) — where `Deref` acts without being called
- [`Box`](../../26_Collections/the_box/README.md) — the smart pointer `Deref` was designed for
- [SemVer hazards](../semver_hazards/README.md) — what a published `Deref` target commits you to
- [`Deref`: when to implement it ↗](https://doc.rust-lang.org/std/ops/trait.Deref.html#when-to-implement-deref-or-derefmut) — std's three conditions
- [Deref polymorphism ↗](https://rust-unofficial.github.io/patterns/anti_patterns/deref.html) — the anti-pattern, in the Rust Design Patterns book

## If you are coming from another language

- **Go.** Embedding a struct promotes its methods to the outer type, by design, and the language spec says which method wins when both levels define one. Deref polymorphism imitates embedding with a trait that was designed for pointers, and the answer to the same question comes from method resolution rather than from anything written for this use.
- **Java.** A class wrapping a `List` writes a delegating method for each operation it exposes. `Deref` is a shortcut that exposes every `&self` method at once, which Java has no counterpart for — and inheritance, the thing Deref polymorphism imitates, is what Rust left out.
- **C++.** `operator*` and `operator->` are overloadable, and smart pointers are their conventional users. Rust's `Deref` is the same hook, with auto-deref at method calls added on top, which is what makes it tempting outside smart pointers.
- **Python.** A wrapper whose `__getattr__` forwards to the wrapped object is the dynamic form of Deref polymorphism, and has the same problem: the wrapper's API is whatever the inner object's API happens to be today.

## Po polsku

Nowy typ (*newtype*) to nowa nazwa nad starą reprezentacją, a sposób wydawania wartości wewnętrznej — `Deref`, `AsRef`, `Borrow` albo zwykła metoda — decyduje, ile API typu wewnętrznego staje się twoim zobowiązaniem. `Deref` na `Vec<u32>` oszczędza pisania `len` i `iter`, ale publikuje wszystkie metody `&self` wektora; późniejsza zmiana przechowywania na `VecDeque` psuje kod użytkowników. Używanie `Deref` do udawania dziedziczenia (*Deref polymorphism*) to znany antywzorzec.

**Szukaj po polsku:** typ opakowujący · wzorzec newtype · `rust deref polymorphism anti-pattern` · `rust asref vs borrow` · `rust newtype deref`
