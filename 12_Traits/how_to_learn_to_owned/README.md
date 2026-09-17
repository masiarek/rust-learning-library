# How to learn `ToOwned`: ten steps, in order

**Level:** 101 → 201 · a learning path

**One line:** `ToOwned` itself adds one idea — a separate owned type — and the confusion lives in five ideas underneath it: unsized types, owned/borrowed pairs, `Clone`, method lookup and blanket impls. Re-reading the trait does not help; learning those five in order does, one page per step.

Each step is its own page: the idea, a checkpoint to predict, the verified answer from a program CI runs, and links out to the lesson that teaches it in full and to the official documentation. **Predict each checkpoint before you open its answer.** A wrong prediction means stay on that step: the next one assumes it.

**Start at [step 1](clone_vs_to_owned/README.md)** — or, if something specific surprised you, find its row below and start there. Step 1 states the question the path answers; steps 2 to 6 build what it takes to answer it; 7 to 9 apply it; 10 is optional, because most code never implements `ToOwned` at all.

**Before step 1:** the path assumes that a `String` owning a heap buffer, and a `&str` borrowing a view of one, is physical to you rather than abstract. If it is not yet, start in the library's [Ownership](../../18_Ownership/README.md) section — [Stack and heap](../../18_Ownership/stack_and_heap/README.md) and [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) are the two pages this path leans on — or with The Book's [ch. 4.1, *What Is Ownership?* ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html). Three of its sections are this path's layer zero: [*The Stack and the Heap* ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap); [*Memory and Allocation* ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation), where a `String` is a pointer, a length and a capacity with its bytes on the heap, and a literal's text is hardcoded into the executable; and [*Variables and Data Interacting with Clone* ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone), which [step 4](clone_returns_self/README.md) picks up.

## Find your step

Each surprise below is one missing idea, not a mystery about `ToOwned`. Start at the step in your row and work down from there.

| If this surprised you | Start at step |
|---|---|
| `ToOwned` exists at all, when `Clone` already does | [1](clone_vs_to_owned/README.md#two-signatures) |
| `size_of::<&str>()` is not the size of a pointer | [2](types_with_no_size/README.md) |
| `name.clone()` on a `&str` gave back a `&str` | [4](clone_returns_self/README.md) |
| `r.clone()` on a `&String` gave a `String`, not a reference | [5](the_dot_picks_first/README.md#walk-it-for-clone) |
| `.clone()` on a `&&String` gave a `&String`, but on a `&mut String` a `String` | [5](the_dot_picks_first/README.md#where-clone-goes-further-and-where-it-never-does) |
| someone said `.to_owned()` on a `&str` works "by autoderef" | [5](the_dot_picks_first/README.md) |
| `Clone::clone(&r)` gave a `&String`, but `Clone::clone(r)` and `r.clone()` gave a `String` | [5](the_dot_picks_first/README.md#name-the-trait-and-the-you-pass-decides) |
| `a.to_owned()` on a `&Foo` gave a `&Foo`, or `E0308` said *expected `Foo`, found `&Foo`* | [6](the_blanket_to_owned/README.md) |
| `42_i32.to_owned()` compiles, and is `42` | [6](the_blanket_to_owned/README.md) |
| `"hi".to_owned()` is a `String`, but `ToOwned::to_owned(&s)` on a `&str` is a `&str` | [6](the_blanket_to_owned/README.md#one-str-two-impls) |
| `HashMap<String, _>::get` accepts a `&str` | [7](borrow_the_way_back/README.md) |
| a `Cow` diagram showed `to_mut()` and `push` as one step | [7](borrow_the_way_back/README.md) |
| `.to_owned()` on an `Rc` did not copy the string | [8](to_owned_traps/README.md#rc-the-outer-type-is-the-first-candidate) |
| `.to_owned()` on a `Cow::Borrowed` is still borrowed | [8](to_owned_traps/README.md#cow-to_owned-is-not-into_owned) |
| an article said `Cow` has `into_borrowed`, or that `to_mut()` gives a `&mut str` | [the `Cow` claims, run](cow_claims_checked/README.md) |
| `clone_into` into a `String::new()` saved nothing, but into an empty `String::with_capacity(64)` it did | [9](clone_into_refills/README.md) |
| `impl ToOwned for MyType` is `E0119` | [10](implementing_it_or_not/README.md) |
| you cannot write `Borrow` for your view struct | [10](implementing_it_or_not/README.md) |
| you are not sure whether to write `clone`, `to_owned`, `to_string` or `into` | [after the steps](clone_to_owned_or_from/README.md) |

## The steps

| Step | Page | What clicks | Checkpoint |
|---|---|---|---|
| 1 | [`ToOwned` is `Clone` with a separate owned type](clone_vs_to_owned/README.md) | `type Owned`, and the impl sitting on `str`, not on `&str` | what `"hi"`, `[1, 2][..]` and a `Path` turn into |
| 2 | [Some types have no size](types_with_no_size/README.md) | why you only ever hold a pointer to a `str`, and why it is two words | is `&str` the size of `&String`? |
| 3 | [Owned and borrowed are two different types](owned_and_borrowed_types/README.md) | the borrowed half is the type behind the `&`, named by `Deref` | the types of `*s` and `*v` |
| 4 | [`Clone` hands back `Self` — and every `&T` is `Clone`](clone_returns_self/README.md) | why `str` cannot be `Clone`, and why every reference is | is `a` usable after `let b = a;`? |
| 5 | [The dot takes the first receiver that fits](the_dot_picks_first/README.md) | the candidate list, and where a deref does — and does not — happen | three `clone`s and three `to_owned`s |
| 6 | [One blanket impl covers every `Clone` type](the_blanket_to_owned/README.md) | `impl<T: Clone> ToOwned for T`, what it does on a `&Foo`, and why a `&str` has two impls | `42_i32.to_owned()`, a `&Ticket`, and one `&str` two ways |
| 7 | [`Borrow` is the way back](borrow_the_way_back/README.md) | `type Owned: Borrow<Self>`, `HashMap::get`, and how a `Cow` reads and writes | `seats.get("Ada")`, and `to_mut()` twice |
| 8 | [The traps are steps 5 and 6 together](to_owned_traps/README.md) | `Rc` and `Cow` are `Clone`, so the wrapper is what gets copied | a strong count and a `Cow` variant |
| 9 | [`clone_into` refills instead of allocating](clone_into_refills/README.md) | reuse needs room already there | a roomy buffer's capacity |
| 10, optional | [Implementing it — on the referent, or not at all](implementing_it_or_not/README.md) | `E0119`, the unsized wrapper, and the inherent method | two spellings of one call |
| after | [`Clone`, `ToOwned` or `From`?](clone_to_owned_or_from/README.md) | the three side by side, with the heap read off the addresses | — |

Beside the steps:

- [Where `Clone` will not do: code only](where_clone_will_not_do/README.md) — ten pairs of fences, `.clone()` that fails to compile beside `ToOwned` that works, each checked by the compiler on every build.
- [What `Cow` explanations get wrong, run](cow_claims_checked/README.md) — ten claims about `Cow` from articles, books and chat answers, each checked against the compiler; read it after step 7.
- [Helpful resources for the path](to_owned_reading_list/README.md) — book chapters, the Reference, the Rustonomicon, articles and talks for every step, with the ones to read with care.

## Everything else `ToOwned` touches

Explanations of `ToOwned` tend to arrive as a list of neighbouring topics. Each has a place on the path and a page in the library:

| Topic | On the path | In the library |
|---|---|---|
| ownership, moves, the stack and the heap | [before step 1](#find-your-step) | [Ownership](../../18_Ownership/README.md), [Stack and heap](../../18_Ownership/stack_and_heap/README.md), [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) |
| dynamically sized types, `?Sized`, wide pointers | [step 2](types_with_no_size/README.md) | [`str` is unsized](../../14_Strings/str_is_unsized/README.md), [Marker traits](../marker_traits/README.md) |
| owned types and their heap buffers | [step 3](owned_and_borrowed_types/README.md) | [`String` vs `&str`](../../14_Strings/string_vs_str/README.md), [Stack and heap](../../18_Ownership/stack_and_heap/README.md) |
| `Clone`, `Copy`, and `&T` being `Copy` | [step 4](clone_returns_self/README.md) | [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) |
| method resolution, autoref and autoderef, `Deref` | [step 5](the_dot_picks_first/README.md) | [Method resolution](../method_resolution/README.md), [Coercion](../../29_Conversion/coercion/README.md) |
| blanket impls and coherence | [step 6](the_blanket_to_owned/README.md) | [Extension traits](../extension_traits/README.md) |
| smart pointers: `Rc`, `Arc`, `Box<str>` | [step 8](to_owned_traps/README.md) | [`Rc`](../../18_Ownership/reference_counting/README.md), [The third owned form](../../14_Strings/boxed_str/README.md) |
| `Borrow` (and `BorrowMut`, its `&mut` twin) | [step 7](borrow_the_way_back/README.md) | [`Borrow`](../borrow_trait/README.md) |
| `Cow`, clone-on-write | [steps 7](borrow_the_way_back/README.md) and [8](to_owned_traps/README.md) | [`Cow`](../../18_Ownership/clone_on_write/README.md) |
| allocation cost and reuse | [steps 8](to_owned_traps/README.md), [9](clone_into_refills/README.md) and [after](clone_to_owned_or_from/README.md) | [`clone_into`](../clone_into/README.md), [What a clone costs](../../18_Ownership/what_a_clone_costs/README.md) |
| `to_owned` vs `to_string` vs `From`/`Into` | [after the steps](clone_to_owned_or_from/README.md) | [Making a string](../../14_Strings/making_a_string/README.md), [`From` and `Into`](../../29_Conversion/from_and_into/README.md) |
| implementing it for your own type | [step 10](implementing_it_or_not/README.md) | [Implementing `ToOwned`](../implementing_to_owned/README.md) |
| the std documentation page itself | [step 1](clone_vs_to_owned/README.md) | [Reading the `ToOwned` docs](../reading_the_to_owned_docs/README.md) |

## Katas along the path

The steps say what to understand; these make you write it. Each lives on the page its step reads, in the order the steps unlock them.

| After step | Kata | On |
|---|---|---|
| 1 | [Write the function that hands back the owned twin](clone_vs_to_owned/README.md#practice) — one generic line, six arguments, and the `?Sized` that lets `str` in | [Step 1](clone_vs_to_owned/README.md) |
| 2 | [Measure both halves of a reference](../../14_Strings/str_is_unsized/README.md#practice) — `&str`, `&[i32]`, `&dyn Display` and `&i32`, and the `?Sized` that takes all four | [`str` is unsized](../../14_Strings/str_is_unsized/README.md) |
| 3 | [One `&str` parameter, three callers](../../14_Strings/string_vs_str/README.md#practice) — then flip it to `String` and count what each call site pays | [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) |
| 4 | [One `E0382`, three fixes](../../16_Structs/copy_vs_clone/README.md#practice) — and what each costs the caller | [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) |
| 6 | [Give every slice a `.middle()`](../extension_traits/README.md#practice) — one trait, two methods, and only one of them reachable through a dot | [Extension traits](../extension_traits/README.md) |
| 6 | [Predict the owned twin before you run it](../to_owned/README.md#practice) — six receivers, and the two everybody gets wrong | [`ToOwned`](../to_owned/README.md) |
| 7 | [One lookup for every kind of key](../borrow_trait/README.md#practice) — three maps searched by their borrowed forms, zero allocations counted | [`Borrow`](../borrow_trait/README.md) |
| 7 | [Pay only when you have to](../../18_Ownership/clone_on_write/README.md#practice) — a `Cow` that copies only the rows it changes | [`Cow`](../../18_Ownership/clone_on_write/README.md) |
| 8 | [Predict the count four times](../../18_Ownership/reference_counting/README.md#practice) — one roster shared by three tallies, and the edge that leaks | [`Rc`](../../18_Ownership/reference_counting/README.md) |
| 8 | [Freeze a candidate column three ways](../../14_Strings/boxed_str/README.md#practice) — ending on the `.to_owned()` that clones a pointer instead of the text | [The third owned form](../../14_Strings/boxed_str/README.md) |
| 9 | [Four loops that all look like reuse](../clone_into/README.md#practice) — predict the allocations, then count them | [`clone_into`](../clone_into/README.md) |
| 10 | [A slice that promises its order](../implementing_to_owned/README.md#practice) — `Sorted<T>` with `Borrow`, `ToOwned` and a `Cow` | [Implementing `ToOwned`](../implementing_to_owned/README.md) |

Step 5 has no kata of its own; step 6's katas exercise the same lookup order. The full sequence, with every other kata in the library, is [KATAS.md](../../KATAS.md).

## Tests for anything else you read

Most explanations of `ToOwned` get the trait right and one step underneath it wrong. The tell for each:

- **"`&str` implements `ToOwned` to produce a `String`."** `str` does, with a hand-written impl. There is no hand-written `impl ToOwned for &str`: the blanket `impl<T: Clone> ToOwned for T` covers `&str` because `&str` is `Clone`, and that impl produces a `&str` — [step 1](clone_vs_to_owned/README.md#the-impl-is-on-the-type-behind-the), then [step 6](the_blanket_to_owned/README.md#one-str-two-impls).
- **"`.to_owned()` works on a `&str` through autoderef"** — or through autoref. Neither happens: `str`'s `to_owned(&self)` already takes a `&str`, so the receiver is passed as it is. Had a `&` been added or removed, the call would have returned a `&str` — [step 5](the_dot_picks_first/README.md#walk-it-for-to_owned-and-see-where-a-deref-happens).
- **"On a reference, `clone()` just copies the pointer."** True only when the pointee is not `Clone` — [step 5](the_dot_picks_first/README.md). On a `&String` it allocates a new `String`.
- **"`ToOwned` almost always allocates on the heap."** `42_i32.to_owned()` touches no heap, and `.to_owned()` on an `Rc` bumps a count. It allocates when the impl it lands on does — [step 8](to_owned_traps/README.md) and [after the steps](clone_to_owned_or_from/README.md).
- **"After `to_mut()`, writing to a `Cow` never allocates."** The clone happens at most once, on the first `to_mut()`. Growth is a separate cost: the fresh `String` has no spare capacity, so the first `push` reallocates, as it would for any `String` — [step 7](borrow_the_way_back/README.md#checkpoint).
- **"To implement `ToOwned` for your struct, write `type Owned = Self`."** It compiles only while the struct is not `Clone`, is `Clone` under another name, and becomes `E0119` the day someone adds `#[derive(Clone)]` — [step 10](implementing_it_or_not/README.md).
- **"`clone_into` is another way to get owned data."** It is the way to *refill* owned data. Into a `String` with no capacity it saves nothing; into one with enough capacity it reuses the buffer, even at length zero — [step 9](clone_into_refills/README.md).
- **"Prefer `to_owned()` to `to_string()`, because it is faster."** Not since Rust 1.9; the [`ToOwned`](../to_owned/README.md#for-everything-else-they-are-the-same-call) page has the measurements. The argument that survives is that `to_owned` names what changes — the owner.

## If you are coming from another language

- **C++** — steps 2 and 3 are `std::string` and `std::string_view`, except that Rust also names the thing the view points at. Step 4 is a copy constructor you have to call by name. Steps 5 and 6 are where C++ intuition misleads, and not because of spelling: in C++, copying the pointer (`auto q = p;`) and copying the pointee (`auto s = *p;`) look different, while Rust's `r.clone()` is one spelling that the method search resolves to either, with nothing at the call site to say which.
- **Python** — the trap has a Python twin. `b = a` gives the same list a second name, while `b = a[:]` or `copy.copy(a)` gives a new list, and the spelling tells you which you got. Rust's `r.clone()` can be either, with the same spelling: on a `&String` it is `a[:]`, a new `String`; on a `&str` it is `b = a`, the same text under a second reference ([step 5](the_dot_picks_first/README.md#walk-it-for-clone)). What has no Python twin is [step 4](clone_returns_self/README.md): being copyable at all is a trait a type may decline to implement.

## See also

- [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) — the same kind of page, for the other wall
- [`ToOwned`](../to_owned/README.md) — the trait page steps 1, 6 and 7 keep returning to
- [Reading the `ToOwned` docs](../reading_the_to_owned_docs/README.md) — the std page for the trait, block by block, with every claim on it run
- [`Borrow`: look up an owned key with a borrowed one](../borrow_trait/README.md) — step 7 in full, and the promise nothing checks
- [Implementing `ToOwned` for your own type](../implementing_to_owned/README.md) — step 10 in full
- [Traits](../README.md) — the section this path sits in

## Po polsku

Jeśli `ToOwned` wciąż się „nie klei”, to prawie na pewno nie z powodu samej cechy (*trait*). Ona wnosi jedną nową informację — typ powiązany `Owned`, czyli osobny typ dla wersji posiadanej. Zamieszanie siedzi w pięciu pojęciach pod spodem: typy bez znanego rozmiaru (`str`, `[T]`, `Path`), para własność–pożyczka jako **dwa różne typy** (`String` i `str`), `Clone` zwracające `Self`, wyszukiwanie metody po kropce i implementacja zbiorcza (*blanket impl*). Dlatego ponowne czytanie dokumentacji `ToOwned` nie pomaga, a przejście tych kroków po kolei — tak. Każdy krok ma teraz własną stronę.

Krok 1 zaczyna od sedna: implementacja jest na `str`, nie na `&str` — skoro `Self` to `str`, to `&self` to `&str`. Najważniejsze są potem kroki 4–6 razem. Każda referencja współdzielona `&T` jest `Copy`, więc i `Clone`, więc — przez `impl<T: Clone> ToOwned for T` — ma `to_owned()`. Kiedy `Foo` nie jest `Clone`, `(&foo).to_owned()` nie znajduje niczego na `Foo` i spada na implementację dla `&Foo`: dostajesz **kopię wskaźnika**, a z adnotacją typu — `E0308` bez słowa o `Clone`. Na `&String` jest odwrotnie: `String::clone` pasuje pierwsze, więc dostajesz nowy `String`. Te same znaki, dwa wyniki, a rozstrzyga kolejność wyszukiwania — bez żadnej „autodereferencji” w przypadku `&str`.

Każdy krok ma punkt kontrolny: najpierw przewidź wynik, potem rozwiń zweryfikowany wydruk na stronie kroku. Błędna przepowiednia znaczy „zostań na tym kroku” — następny zakłada, że poprzedni już siedzi. Przy każdym kroku są linki do lekcji, do oficjalnej dokumentacji (**Docs**) i do słowniczka, a tabela *Katas along the path* podaje ćwiczenia w tej samej kolejności co kroki. Książki i rozdziały do każdego kroku zebrano na stronie z zasobami.

**Szukaj po polsku:** typy bez znanego rozmiaru · implementacja zbiorcza · `rust to_owned vs clone` · `rust autoref method resolution` · `rust clone on reference returns reference`
