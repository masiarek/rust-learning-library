# Phantom types

**Level:** 201 → 301 · working knowledge

**One line:** A type parameter the struct never stores — the compiler treats two taggings of the same data as two unrelated types, and the tag occupies zero bytes.

```rust
use std::marker::PhantomData;

enum Star {}                     // tags. No variants, so no value of one can exist.
enum Approval {}

struct Ballot<Method> {
    scores: Vec<u8>,
    _method: PhantomData<Method>,
}

fn main() {
    let star: Ballot<Star> = Ballot { scores: vec![5, 3, 0], _method: PhantomData };
    let approval: Ballot<Approval> = Ballot { scores: vec![1, 0, 1], _method: PhantomData };
    println!("{} {}", star.scores.len(), approval.scores.len());  // 3 3
}
```

`Ballot<Star>` and `Ballot<Approval>` have the same fields, the same size and the same machine code. They are not the same type, and a function that takes one will not accept the other. Nothing about "which method this ballot belongs to" is stored, looked up, or checked at run time — the distinction lives entirely in the type checker and is gone by the time the program runs.

## The error that forces the field

Declare the parameter without the field and the compiler refuses outright:

```text
error[E0392]: type parameter `Method` is never used
 --> ballot.rs:1:15
  |
1 | struct Ballot<Method> {
  |               ^^^^^^ unused type parameter
  |
  = help: consider removing `Method`, referring to it in a field, or using a marker such as `PhantomData`
  = help: if you intended `Method` to be a const parameter, use `const Method: /* Type */` instead
```

Older books quote this without the word "type" — *"parameter `Breed` is never used"* — and with a concrete `const Breed: usize` in the second suggestion; both lines have been reworded since, so match on the code rather than on the sentence. The three fixes it offers are the whole decision: delete the parameter, store something of that type, or say in one field that you are keeping the parameter deliberately.

There is a second error the first one hides. Once `Method` exists and no field determines it, the compiler cannot infer it at a construction site either:

```text
error[E0282]: type annotations needed for `Ballot<_>`
  --> ballot.rs:11:9
   |
11 |     let b = Ballot { scores: vec![5, 3, 0], _method: PhantomData };
   |         ^                                            ----------- type must be known at this point
   |
help: consider giving `b` an explicit type, where the type for type parameter `T` is specified
   |
11 |     let b: Ballot<Method> = Ballot { scores: vec![5, 3, 0], _method: PhantomData };
   |          ++++++++++++++++
```

Nothing in the data narrows `Method`, so nothing can. Every phantom-typed value has to be annotated or turbofished somewhere — `let star: Ballot<Star> = …`, or a constructor like `Ballot::<Star>::new(…)`. That annotation is not overhead you can design away; it is the tag being chosen, and it is the only place the choice is written down.

## `PhantomData` is a claim, not a silencer

It is tempting to read `PhantomData<T>` as *"ignore this parameter"*. It is the opposite: it tells the compiler how the struct behaves **as if** it held a `T`, and three spellings make three different promises:

| Field | Claims | Consequence |
|---|---|---|
| `PhantomData<T>` | this struct owns a `T` | drop-check treats it as owning one; covariant in `T` |
| `PhantomData<fn() -> T>` | it produces a `T` | covariant, and owns nothing |
| `PhantomData<*const T>` | it merely points at one | invariant, owns nothing, and not `Send`/`Sync` |

All three are zero-sized. The difference is invisible in the layout and shows up in what the borrow checker will let you write — which is why a `Vec<T>` implemented over a raw pointer needs `PhantomData<T>` rather than nothing at all: without it, the compiler does not know the `Vec` owns its elements, and drop order stops being checked. For an ordinary tag like `Star` above, which nobody will ever own, the plain `PhantomData<Method>` is the right and usual choice.

## Concrete specialization

The payoff is an `impl` block for one tag rather than for the parameter:

```rust
impl<Method> Ballot<Method> {          // written once, for every method
    fn total(&self) -> u32 { self.scores.iter().map(|&s| u32::from(s)).sum() }
}

impl Ballot<Star> {                     // exists ONLY on Ballot<Star>
    fn max_score(&self) -> u8 { 5 }
}

impl Ballot<Approval> {
    fn max_score(&self) -> u8 { 1 }
}
```

Note what is *not* here: no `Method` parameter on the specialized blocks, no `match` on a stored enum, and no field holding the number 5. The value returned by `max_score` is part of the compiled program, selected when the type was chosen. Add a third method later and the compiler names every place that has to grow.

The same tag on a function signature turns a runtime check into a type error:

```rust
fn star_runoff_pair(ballot: &Ballot<Star>) -> (u8, u8) { /* … */ }
```

```text
error[E0308]: mismatched types
    |
    |     let _ = star_runoff_pair(&approval);
    |             ---------------- ^^^^^^^^^ expected `&Ballot<Star>`, found `&Ballot<Approval>`
```

An Approval ballot cannot reach the STAR runoff, and no line of the runoff has to say so.

## Make the tag uninhabited

The usual advice is an empty struct — `struct Star;` — and it works. An `enum Star {}` works better: with no variants there is no expression anywhere that produces a `Star`, so nobody can accidentally construct one, pass it, store it, or wonder what it means. A tag exists to be *named*, and an uninhabited type is one that can only be named.

Both are zero-sized, so this costs nothing either way. The one case for the unit struct is when you also want the tag as a value — something to store, pass, or put in a list. That is a different design, and the one to reach for when the choice is made at run time rather than at compile time.

## Typestate: the tag records where the value is

Tag a value with *what has happened to it* rather than what kind it is, and each transition consumes the old value and hands back a differently-tagged one:

```rust
struct Blank;
struct Marked;

struct Paper<State> {
    scores: Vec<u8>,
    _state: PhantomData<State>,
}

impl Paper<Blank> {
    fn issue() -> Self { /* … */ }
    fn mark(self, scores: Vec<u8>) -> Paper<Marked> { /* … */ }
}

impl Paper<Marked> {
    fn cast(self) -> Vec<u8> { self.scores }
}
```

`Paper<Blank>` has no `cast` — an unmarked paper cannot reach the ballot box, and the refusal is `E0599`, "no method named `cast` found for struct `Paper<Blank>`". `mark` takes `self` rather than `&mut self`, so the blank paper is moved-from and reusing it is `E0382`. The illegal orderings are not checked; they are unwriteable.

[The right to vote is a value](../../09_Advanced/one_person_one_vote/README.md) builds the same thing out of *distinct* types — `Voter`, `Eligible`, `Receipt` — with no generics at all. The choice between the two is whether the states share behaviour: distinct types duplicate every common method, and `impl<State> Paper<State>` writes it once. That page's closing warning applies here unchanged, and is the honest limit of both: move semantics govern one value, not how many values a constructor hands out.

## What it costs

Phantom types buy compile-time separation by spending run-time flexibility, and the bill comes in three parts:

- **No heterogeneous collection.** `Ballot<Star>` and `Ballot<Approval>` are different types, so `vec![star, approval]` does not compile. If an election file can hold either, the tag has to become an enum or a `dyn` trait object at the boundary where the data arrives — which is exactly the [dispatch trade](../static_vs_dynamic_dispatch/README.md), arriving one layer earlier.
- **Nothing to ask at run time.** There is no `ballot.method()`. Recovering the tag as a value means a trait with an associated constant (`trait Method { const NAME: &'static str; }`) and a bound, at which point the phantom parameter is carrying a payload after all.
- **Annotation everywhere.** Every construction names its tag, per `E0282` above. In a codebase with two tags this is a feature; with eight it is noise, and an enum field starts to read better.

The rule of thumb: reach for a phantom type when mixing the two kinds is a *bug*, not a case to handle. Metres and feet, validated and raw, signed and unsigned, this election's ballots and that one's.

## If you are coming from another language

- **Python** — `Generic[T]` with a `TypeVar` you never store is the same shape and does not do the same job: annotations are erased at run time, so `Ballot[Star]` and `Ballot[Approval]` are one class and `isinstance` cannot tell them apart. What you get is a mypy or pyright diagnostic, if the checker is run, on the files it covers. Two consequences worth naming. Nothing stops a caller at run time, so a Python library that cares still validates on the way in — the check the type was supposed to replace stays in the code. And the erased tag cannot select an implementation: there is no Python spelling of `impl Ballot<Star>`, so `max_score` becomes a dictionary lookup, a subclass, or a `match` on a stored field — all of which put the method name back into the data. The nearest honest Python equivalent to this whole page is separate classes with a shared base, and it costs you the shared code that `impl<Method>` writes once.
- **ABAP** — there is no counterpart, and it is worth being precise about why, because the thing that *looks* like one is not. Generic programming in ABAP is either `TYPE any` plus RTTI (a run-time question) or an interface reference (a compile-time one, but the type is then the interface, not the concrete type). Neither can express "the same class, tagged two ways, distinguishable at syntax-check time". The tempting near-miss is the dictionary: declare `ZDE_BALLOT_STAR` and `ZDE_BALLOT_APPROVAL` as two data elements and it reads exactly like a tag — but two data elements over the same technical type are assignment-compatible, and the syntax check says nothing when you mix them. A domain's fixed values and check table constrain the *value*, at run time, on the database; they do not make two fields different types. So the honest mapping is that the ABAP answer here is two classes, or one class with a `method_type` attribute and the discipline to check it — which is precisely the boolean-beside-the-data arrangement a phantom type exists to replace.
- **C++** — this is tag dispatch, and `Ballot<Star>` is what a `template <typename Method> class Ballot` with an unused parameter already does, no marker field required. C++ has no `E0392`, because it has no drop-check or variance to inform: the parameter can simply go unused. `PhantomData` is the price Rust pays for having those two things, and the reason it is a field rather than an attribute.

## The verified output

<!-- output:phantom_types -->
*Verified output of [`phantom_types.rs`](examples/phantom_types.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One struct, two types
   Ballot<Star>     total 8   max 5
   Ballot<Approval> total 2   max 1
   `star_runoff_pair(&approval)` is E0308: expected `Ballot<Star>`,
   found `Ballot<Approval>`. The two share every byte and no type.

2. The specialization is concrete, not generic
   impl Ballot<Star> { .. }     method_name = STAR
   impl Ballot<Approval> { .. } method_name = Approval
   Neither name is stored anywhere: `Ballot` has one field, a Vec.
   runoff pair of the STAR ballot = (5, 3)

3. The tag costs nothing
   size_of::<Vec<u8>>()             = 24
   size_of::<Ballot<Star>>()        = 24
   size_of::<Ballot<Approval>>()    = 24
   size_of::<PhantomData<Star>>()   = 0

4. What the phantom field CLAIMS — all three are zero-sized
   PhantomData<Vec<u8>>          = 0   owns a Vec<u8>
   PhantomData<fn() -> Vec<u8>>  = 0   merely produces one
   PhantomData<*const Vec<u8>>   = 0   only points at one
   Same size, three different promises about variance and drop.

5. Typestate: the tag moves with the value
   issue() -> mark() -> cast() = [5, 2, 0]
   `blank.mark(..)` took `self`, so `blank` is moved-from: using it
   again is E0382. And `Paper<Blank>` has no `cast` at all — an
   unmarked paper cannot reach the ballot box, by construction.
```
<!-- /output -->

## See also

- [Marker traits](../marker_traits/README.md) — the other half of `std::marker`: a trait with no methods rather than a parameter with no data
- [What a type annotation does](../../15_First_Programs/what_an_annotation_does/README.md) — the annotation and the turbofish `E0282` sends you to write
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the same instinct without generics: one type, one door, no tag
- [The right to vote is a value](../../09_Advanced/one_person_one_vote/README.md) — typestate built from distinct types and move semantics, and the hole neither approach closes
- [Static vs dynamic dispatch](../static_vs_dynamic_dispatch/README.md) — where to go when the tag has to be a run-time choice after all
