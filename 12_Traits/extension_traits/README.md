# Extension traits

**Level:** 201 · working knowledge

**One line:** A trait you declare, implemented for a type somebody else wrote — the only way to put a new method behind a dot on `str`, on `Vec`, or on every iterator in the language, and the reason a crate ever asks you to write `use itertools::Itertools;`.

```rust
trait Shout {
    fn shout(&self) -> String;
}

impl Shout for str {
    fn shout(&self) -> String {
        let mut s = self.to_uppercase();
        s.push('!');
        s
    }
}

fn main() {
    println!("{}", "hello".shout());   // HELLO!
}
```

`str` is the standard library's. `Shout` is yours. That pairing is the whole pattern, and it is enough to make `.shout()` a real method on every string slice in the program — resolved at compile time, dispatched statically, and costing exactly what a free function would.

## The version that does not compile

The obvious thing to try first is an inherent `impl`, and it is refused:

```rust,compile_fail
impl<T> Vec<T> {
    fn second(&self) -> Option<&T> { self.get(1) }
}
```

```text
error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
 = help: consider defining a trait and implementing it for the type or using a
         newtype wrapper like `struct MyType(ExternalType);` and implement it
```

Worth reading that `help:` line twice, because it is not generic advice — it names both of the language's answers, and this page is about the first one. (The second is the [newtype](../../16_Structs/newtype_score/README.md).) An inherent method is part of a type's own definition, and a type has exactly one crate allowed to define it. A trait impl is not part of the definition; it is a separate claim, and the rules for making one are looser.

## The rule that permits it

They are looser, but not absent. The [orphan rule](../../29_Conversion/from_and_into/README.md) wants the trait **or** the type to be local to your crate:

| The impl | Trait | Type | Verdict |
|---|---|---|---|
| `impl Shout for str` | yours | std's | compiles — this page |
| `impl Display for MyStruct` | std's | yours | compiles — the ordinary case |
| `impl Display for Vec<i32>` | std's | std's | `E0117` |

So the local trait is not decoration and it is not a workaround; it is the entire licence. Coherence is what is being protected: if two crates could each write `impl Display for Vec<i32>`, a program depending on both would have two answers to one question and no way to choose.

## One impl, every iterator

An extension trait becomes interesting when the impl is *blanket* — written once, for every type that satisfies a bound:

```rust
trait Tally: Iterator + Sized {
    fn tally(self) -> usize {
        self.count()
    }
}

impl<I: Iterator> Tally for I {}
```

The impl block is empty because the body lives in the trait as a default. Those five lines put `.tally()` on `vec.into_iter()`, on `"a b c".split(' ')`, on `(0..10).filter(…)` — and on iterators written by crates that did not exist when the line was compiled. Nothing enumerates the types; the bound does the reaching.

## `Sized`, and why it is in that supertrait list

`count(self)` takes the iterator by value, and a value passed by value needs a size. Drop the `Sized` and the compiler stops you inside the trait, before any impl exists:

```text
error[E0277]: the size for values of type `Self` cannot be known at compilation time
help: consider further restricting `Self`
```

`Iterator + Sized` is that restriction. Any extension method taking `self` rather than `&self` needs it — which is most of the interesting ones, because taking `self` is what lets the method *wrap* the receiver and return something new.

## What narrowing the bound buys

The bound is also the difference between a method that belongs somewhere and a method that is everywhere. Write the blanket impl over `Sized` alone and it compiles, and so does this:

```rust
struct Progress<I>(I);

trait ProgressIteratorExt: Sized {          // Sized ONLY — no Iterator bound
    fn progress(self) -> Progress<Self> {
        Progress(self)
    }
}

impl<I> ProgressIteratorExt for I {}

fn main() {
    let _ = 42.progress();                              // an integer
    let _ = String::from("not an iterator").progress();
    let _ = ().progress();                              // the unit type
}
```

All three compile. `.progress()` is now on the unit type. Nothing has gone *wrong* yet — the mistake surfaces later, as a confusing error at the `for` loop, or as autocomplete offering a progress bar on an integer. `impl<I: Iterator> ProgressIteratorExt for I` moves that error back to where the mistake was made. The bound on a blanket impl is the API's audience, written down.

## The trap, which produces no warning at all

An inherent method wins over a trait method. Always, silently, with no lint:

```rust
trait SliceExt<T> {
    fn len(&self) -> usize;          // collides with the inherent [T]::len
    fn second(&self) -> Option<&T>;  // collides with nothing
}
```

```text
v.len()             = 3      <- the inherent method. Yours was never called.
SliceExt::len(&v)   = 999    <- yours, reachable only by naming the trait
v.second()          = Some(20)
```

Both methods are declared in the same trait, in the same file, on the same value. One of them is unreachable through a dot and there is no diagnostic — not an error, not a warning, not a note. This is the failure mode to actually fear, because it is also a *future* event: the day the standard library or the crate you are extending adds an inherent method with your name, your method stops being called and your program keeps compiling. Name extension methods so a collision would be a surprise, and prefer a verb the underlying type would not plausibly claim.

## Three more ways it goes wrong, all of them loud

- **The method vanishes** unless the trait is imported — `E0599`, with the help line *"items from traits can only be used if the trait is in scope"*. This is not special to extension traits, but it is where beginners meet it, and it is why a crate's docs open with a `use` line. Full treatment: [A trait must be in scope](../trait_in_scope/README.md).
- **Two extension traits, one method name**, both in scope — `E0034: multiple applicable items in scope`, with both candidates named. The fix is the long spelling, `A::shout(&x)`. The standard library is *stuck* on exactly this collision: `Iterator::intersperse` collided with `Itertools::intersperse` in every crate that imported the latter, stabilisation was reverted after crater found the breakage widespread, and it is **still unstable on 1.98** — `E0658`, [tracking issue #79524 ↗](https://github.com/rust-lang/rust/issues/79524), five years on. What happens in the meantime is the part worth knowing, because it is not `E0034`: rustc carries a dedicated lint, `unstable-name-collisions`, which resolves the call to the **stable** method — the crate's, not std's — and warns. That lint is in the *future-incompatible* group, which makes the warning a scheduled breakage rather than a matter of taste.
- **A blanket impl forecloses a special case** — once `impl<I: Iterator> Ext for I` exists, `impl Ext for vec::IntoIter<u8>` is `E0119: conflicting implementations`. Rust has no specialization on stable, so the blanket impl is a decision you cannot partially revisit later.

## Where you have already used one

Almost every crate that adds methods to types it does not own:

| Crate | Trait to import | Adds |
|---|---|---|
| [`itertools` ↗](https://docs.rs/itertools/latest/itertools/trait.Itertools.html) | `Itertools` | `.chunks()`, `.unique()`, `.sorted()` on every iterator |
| [`rand` ↗](https://docs.rs/rand/latest/rand/trait.Rng.html) | `Rng` | `.gen_range()` on every random source |
| [`futures` ↗](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html) | `StreamExt` | `.next()` on every `Stream` — see [Iterator vs Stream](../../24_Iterators/iterator_vs_stream/README.md) |
| [`anyhow` ↗](https://docs.rs/anyhow/latest/anyhow/trait.Context.html) | `Context` | `.context("…")` on every `Result` |

The `Ext` suffix is a convention, not a rule, and it exists because the trait's name is a thing you have to type in a `use` line — so it wants to say *"I am here to be imported, not to be implemented."*

The standard library uses the pattern on itself: `Iterator` has 70-odd provided methods and a single required one, so implementing `next` gets you the rest. `ToString` is a blanket impl over `Display`, which is why writing `Display` makes `.to_string()` appear without you asking.

## When a free function is the better answer

The dot is a real benefit — it reads left to right and it chains — but it costs a `use` line at every call site, a name in a shared namespace, and the collision risk above. A plain `fn shout(s: &str) -> String` costs none of those. Reach for the extension trait when the method *chains* (each call returning something the next call takes), when it belongs conceptually to the receiver, or when you are extending a trait rather than a type. Reach for the function when it is one call in one place.

## If you are coming from another language

- **Python.** The instinct is monkey-patching, and on a class you own it works: `Mine.shout = lambda self: …`. On a builtin it does not — `str.shout = …` raises `TypeError: cannot set 'shout' attribute of immutable type 'str'`, which is Python's `E0116`, thrown at run time instead of compile time. Rust is the *more* permissive language here, which is not the usual direction: the local trait is a sanctioned, checked way to do the thing Python refuses outright. And where Python's patch is global and invisible — any module can be the one that patched `list`, and nothing at the call site says so — Rust's is inert until imported, so the `use` line at the top of the file is the disclosure.
- **ABAP.** There is no extension-method mechanism at all: you cannot add a method to `CL_ABAP_TSTMP` or to a DDIC type, and the idiom is a helper class of static methods — `ZCL_STRING_UTIL=>SHOUT( iv_text )`. That is exactly the free function this page's last section recommends, so the ABAP habit is already the conservative Rust answer; what Rust adds is the option of putting it behind a dot when chaining earns it. The nearest ABAP relative is the interface implemented by a wrapper class, which is the [newtype](../../16_Structs/newtype_score/README.md), not this.
- **C#, Kotlin, Swift** have this as a language feature (`static class … this string s`, `fun String.shout()`, `extension String`). The difference worth knowing: those resolve by *namespace or file* import, while Rust resolves by **trait** — the unit you import is the trait, one type can carry several, and the compiler will tell you when two are ambiguous rather than picking one.

## The verified output

Every line below is produced by [`extension_traits.rs`](examples/extension_traits.rs) and recorded by `tools/run_examples.py` — nothing here is hand-typed.

<!-- output:extension_traits -->
*Verified output of [`extension_traits.rs`](examples/extension_traits.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A local trait, implemented for `str`:
   HELLO!
   OWNED TOO!

2. One blanket impl reaches every iterator:
   vec into_iter  -> 3
   str split      -> 4
   filtered range -> 4

3. `.progress()` on a plain range, then on a `map`:
    [   ]
    [*  ]
    [** ]
    [***]
    [   ]
    [*  ]
    [** ]
    [***]

4. The inherent method wins, and nothing warns:
   v.len()             = 3
   SliceExt::len(&v)   = 999
   v.second()          = Some(20)
```
<!-- /output -->

Note the bar draws four frames for three items. `next()` prints *before* it delegates, so the final call — the one that returns `None` and ends the loop — draws the full bar. That is the talk's behaviour too, and it is the reason the bar reaches 100% instead of stopping one step short.

## Practice

**Give every slice a `.middle()`, then find out why your `.first()` never runs.**

Write one extension trait with two methods: `middle`, returning the element at the halfway index, and `first`, returning `999` regardless of the slice. Implement it for `[T]`, call both through a dot, and then reach the one the dot cannot see. The point is not the arithmetic — it is the two-line difference in what happens to a name that collides with an inherent method and a name that does not.

<!-- output:extension_traits_kata -->
*Verified output of [`extension_traits_kata.rs`](examples/extension_traits_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
middle — no collision, so the dot finds it:
  [10, 20, 30]    .middle() = Some(20)
  [10, 20, 30, 40].middle() = Some(30)
  []              .middle() = None

first — collides, so the dot never gets here:
  odd.first()           = Some(10)
  SliceExt::first(&odd) = 999

The give-away is the TYPE, not a warning:
  by_dot   is core::option::Option<&i32>
  by_trait is usize

  Two calls, one name, one value — and no diagnostic anywhere.
```
<!-- /output -->

## See also

- [A trait must be in scope](../trait_in_scope/README.md) — the `use` line an extension trait cannot work without, and the three ways to spell the call
- [Method resolution](../method_resolution/README.md) — the search the dot performs, and the order that makes an inherent method win
- [No method named…](../no_method_named/README.md) — `E0599` in full: four causes, of which a missing `use` is one
- [`From` and `Into`](../../29_Conversion/from_and_into/README.md) — the orphan rule stated properly, and a blanket impl you already rely on
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the other answer `E0116` suggests, for when you want to *restrict* a foreign type rather than extend it
- [Phantom types](../phantom_types/README.md) — where the progress bar goes next: a second type parameter, so `with_delims()` exists only once there is a bar
- [Traits: links and videos](../resources/README.md) — Will Crichton's Strange Loop talk, which live-codes the `.progress()` example this page borrows

## Po polsku

Cecha rozszerzająca (*extension trait*) to jedyny sposób, żeby dopisać metodę do typu, którego się nie napisało — do `str`, do `Vec`, do każdego iteratora — i postawić ją za kropką. Reguła jest jednozdaniowa: deklarujesz **własną** cechę i implementujesz ją dla **cudzego** typu. Reguła sieroty (*orphan rule*) wymaga, żeby lokalna była cecha **albo** typ, więc `impl Shout for str` przechodzi, a `impl Display for Vec<i32>` to `E0117` — obie strony są obce. Nie jest to obejście ani sztuczka: ta lokalna cecha **jest** pozwoleniem, a chroniona wartość nazywa się spójność (*coherence*) — gdyby dwie skrzynie mogły napisać ten sam `impl` inaczej, program zależny od obu miałby dwie odpowiedzi na jedno pytanie.

Zacznij od komunikatu, który dostaniesz, próbując zrobić to wprost. `impl<T> Vec<T> { … }` daje `E0116`, a w podpowiedzi kompilator sam wymienia obie legalne drogi: zdefiniuj cechę, albo opakuj typ w nowy typ (*newtype*). Warto ten `help:` przeczytać uważnie, bo to nie jest ogólnikowa rada — to spis treści dwóch osobnych stron tej biblioteki. Druga rzecz, którą trzeba zobaczyć wcześnie, to `impl<I: Iterator> Tally for I {}` — implementacja *hurtowa* (*blanket impl*), napisana raz i obejmująca każdy typ spełniający ograniczenie, także te, które powstaną później. Ciało metody mieszka wtedy w samej cesze jako implementacja domyślna, więc blok `impl` bywa pusty. A `Sized` w liście nadcech nie jest ozdobą: metoda biorąca `self` przez wartość musi znać rozmiar, inaczej `E0277` zatrzyma cię jeszcze przed pierwszą implementacją.

Najważniejsze zdanie na tej stronie dotyczy jednak pułapki, która **nie daje żadnego ostrzeżenia**. Metoda własna typu (*inherent method*) zawsze wygrywa z metodą z cechy. Jeśli nazwiesz swoją metodę `len`, to `v.len()` wywoła `[T]::len` ze standardowej biblioteki, twoja nie wykona się nigdy, a kompilator nie powie ani słowa — ani błędu, ani ostrzeżenia, ani noty. Do własnej wersji dojdziesz tylko długim zapisem, `SliceExt::len(&v)`. I to jest zdarzenie **przyszłe**, nie tylko dzisiejsze: w dniu, w którym `std` albo rozszerzana skrzynia doda metodę własną o twojej nazwie, twoja przestanie być wywoływana, a program nadal będzie się kompilował. Stąd praktyczna rada: nazywaj metody rozszerzające czasownikiem, którego rozszerzany typ raczej nie zechce dla siebie.

Dla czytelnika znającego Pythona jest tu niespodzianka kierunkowa. Odruch to małpie łatanie (*monkey patching*) — i na własnej klasie działa, ale na typie wbudowanym nie: `str.shout = …` rzuca `TypeError: cannot set 'shout' attribute of immutable type 'str'`, czyli dokładnie `E0116`, tyle że w czasie działania. To Rust jest tu językiem **bardziej** pozwalającym, co zdarza się rzadko. Różnica jest też w widoczności: łatka w Pythonie jest globalna i anonimowa — dowolny moduł mógł podmienić metodę na `list` i w miejscu wywołania nie widać tego wcale — a w Ruscie metoda z cechy nie istnieje, dopóki cecha nie zostanie zaimportowana, więc linijka `use` na górze pliku pełni rolę jawnej deklaracji. W ABAP-ie odpowiednika nie ma w ogóle; robi się klasę pomocniczą ze statycznymi metodami (`ZCL_STRING_UTIL=>SHOUT( )`), co jest po prostu zwykłą funkcją — i bywa lepszą odpowiedzią także w Ruscie, kiedy wywołanie jest jedno i nic się nie łańcuchuje.

**Szukaj po polsku:** cecha rozszerzająca · reguła sieroty · implementacja hurtowa · małpie łatanie · `rust extension trait` · `rust E0116 inherent impl` · `rust E0117 orphan rule` · `rust blanket impl`
