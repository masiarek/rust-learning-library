# Implementing your trait for references

**Level:** 301 · deep dive

**One line:** If `fn show<D: Describe>(d: D)` accepts a `Price` but not a `&Price` or a `Box<Price>`, every caller holding a reference has to move or clone — and three forwarding impls, `impl<T: Describe + ?Sized> Describe for &T` and its two siblings, fix that for every caller at once.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The refusal on 1.98.0: `show(&p)` is `E0277`, *the trait bound `&Price: Describe` is not satisfied*, and the compiler's help — *consider removing the leading `&`-reference* — moves the value the caller wanted to keep. `show(Box::new(p))` gets *consider dereferencing here*
- The three forwarding impls, each calling `(**self).describe()`, and why each needs `?Sized` so `&dyn Describe` and `Box<dyn Describe>` qualify too
- What std already does, so callers expect it of you: `Read` for `&mut R` and `Box<R>` (a function taking `R: Read` by value still accepts `&mut file`), `Iterator` for `&mut I` (so `first_two(&mut it)` leaves `it` usable), and `Fn` for `&F`
- Which impls a trait's receivers allow: with only `&self` methods, can it forward through `&T`, `&mut T`, `Box<T>`, `Rc<T>` and `Arc<T>`? With a `&mut self` method, which of those drop out? With a `self` method, any?
- Why method-call tests hide the gap: `p.describe()` and `(&p).describe()` both compile through auto-referencing and auto-deref, so only a generic call reveals the missing impl
- Adding the impls in a later release: does a blanket impl for `&T` collide with anything a downstream crate may already have written? See [SemVer hazards](../semver_hazards/README.md)
- *Rust for Rustaceans* ch. 3 → "Unsurprising" → "Ergonomic Trait Implementations" as the source

## The trap it exists for

Testing a trait only through method calls. Every test compiles because the dot operator borrows and dereferences as needed, the crate ships, and the first user with a `&Price` in hand and a generic function to call gets `E0277` — with a compiler suggestion that moves their value away.

## Where this sits

[Blanket impls](../../22_Generics/blanket_impls/README.md) covers blanket impls in general and the coherence rules that limit them; [Method resolution](../../12_Traits/method_resolution/README.md) explains why the dot hides the gap; [`Read` and `Write`](../../12_Traits/read_and_write/README.md) is where std's forwarding impls make I/O testable. This page is only the three forwarding impls a trait author owes callers.

## See also

- [Blanket impls](../../22_Generics/blanket_impls/README.md) — one impl for every type that meets a bound, and `E0119`
- [Method resolution](../../12_Traits/method_resolution/README.md) — auto-ref and auto-deref at the dot
- [`Read` and `Write`](../../12_Traits/read_and_write/README.md) — `&[u8]` as a reader, `&mut R` as a reader too
- [Step 6: one blanket impl covers every `Clone` type](../../12_Traits/how_to_learn_to_owned/the_blanket_to_owned/README.md) — a std blanket impl read line by line
- [Marker traits](../../12_Traits/marker_traits/README.md) — `Sized`, the bound `?Sized` relaxes
- [Generic or concrete parameters](../generic_or_concrete_parameters/README.md) — the generic function these impls exist for
- [Rust API Guidelines — C-RW-VALUE ↗](https://rust-lang.github.io/api-guidelines/interoperability.html#c-rw-value) — why a reader is taken by value, which only works because of `impl Read for &mut R`

## If you are coming from another language

- **C++.** A function taking `const Describable&` binds to any lvalue of a matching type, with no extra declarations. In Rust `&Price` is a distinct type from `Price`, and a trait bound sees the difference until an impl bridges it.
- **Go.** The method set of `*T` includes the methods declared on `T`, so a pointer satisfies every interface its value does, automatically. Rust has no such rule for trait bounds; the forwarding impl is the rule, written by hand once per trait.
- **Java.** Every object variable is already a reference, so the question never comes up — which is also why Java code can never say "this function takes ownership".

## Po polsku

Jeśli funkcja generyczna `fn show<D: Describe>(d: D)` przyjmuje `Price`, ale nie `&Price` ani `Box<Price>`, każdy, kto trzyma referencję, musi przenieść albo sklonować wartość — a podpowiedź kompilatora („usuń `&`”) prowadzi właśnie do przeniesienia. Rozwiązaniem są trzy implementacje przekazujące (*forwarding impls*) dla `&T`, `&mut T` i `Box<T>` z `?Sized`, tak jak robi to biblioteka standardowa dla `Read` i `Iterator`. Testy wywołujące tylko metody tego nie wykryją, bo operator kropki sam dokłada i zdejmuje referencje.

**Szukaj po polsku:** implementacja cechy dla referencji · implementacja ogólna (*blanket impl*) · `rust impl trait for &T` · `rust ergonomic trait implementations` · `rust forwarding impl box dyn`
