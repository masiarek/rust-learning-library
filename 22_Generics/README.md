# Generics

**One line:** `<T>` lets one definition serve every type, and Rust settles which type that is at compile time — so a generic costs nothing at run time, and the compiler checks your definition before anybody calls it.

Two facts about Rust's version separate it from most others, and between them they explain everything in this section. The parameter is filled in **at compile time**, by stamping out a separate copy of the code per type used — so `Vec<u8>` really is a run of bytes, with no boxing and no tag. And the definition is **type-checked on its own**, against the bounds it declares, which is why an unbounded `T` cannot be printed or compared, and why the error for that arrives in the function you are writing rather than in a stranger's call site.

The section builds one small type — a container, then a linked list — and takes it through five questions in order: what the brackets mean, what to do when the compiler cannot work the type out, what a `T` is allowed to do, what changes when the generic is an enum, and what happens when a generic type contains itself.

| Lesson | Level | What it teaches |
|---|---|---|
| [What a generic is](what_a_generic_is/README.md) | 101 → 201 | `<T>` as a parameter list for types, why `impl<T> Container<T>` says it twice, that two fill-ins are two unrelated types, and that a `Container<T>` is laid out as exactly its `T` |
| [When the compiler cannot infer](when_the_compiler_cannot_infer/README.md) | 201 | `E0282`, the three places to name the missing type, why the compiler's own suggested fix does not compile as printed, and the fact that inference reads the whole body rather than one line |
| [Where the bound goes](where_the_bound_goes/README.md) | 201 → 301 | What an unbounded `T` can do (store, move, drop — that is the list), three spellings of which only two mean the same thing, why a bound on the *struct* locks out callers who never needed it, and the bound `derive` invents |
| [Generic enums](generic_enums/README.md) | 201 | `<T>` on an enum: every variant shares it, a payload-free variant still needs one, two parameters (`Result<T, E>` is not built in), and the tag that costs nothing |
| [A generic recursive type](a_generic_recursive_type/README.md) | 201 → 301 | `E0072` and the pointer that breaks the cycle, why `Option<Box<T>>` costs nothing, why a hand-rolled `End` variant is `Option` with the API removed, and why the payload should not be boxed |

## Generic pages that live elsewhere

The library used generics for a year before this section existed, so several of the deepest pages about them sit with their own subject:

- [Static vs dynamic dispatch](../12_Traits/static_vs_dynamic_dispatch/README.md) — monomorphization measured against a vtable, and the collection that forces `dyn`
- [Marker traits](../12_Traits/marker_traits/README.md) — `Sized` (the bound every `T` already carries), `?Sized`, and `PhantomData` for a parameter the struct does not store
- [Phantom types](../12_Traits/phantom_types/README.md) — the parameter with no data behind it: `E0392`, what a `PhantomData` field claims, and a tag the compiler checks and then deletes
- [What a type annotation does](../15_First_Programs/what_an_annotation_does/README.md) — the turbofish, and the four shapes where `:` decides a type
- [When a struct refuses](../16_Structs/when_a_struct_refuses/README.md) — eight struct errors, including the `E0282` from a function inside `impl<T>` that never mentions `T`
- [Nullable pointers](../17_Option_and_Result/nullable_pointers/README.md) — the niche that makes `Option<Box<T>>` free
- [`Some` and `None`](../17_Option_and_Result/some_and_none/README.md) — `Option<T>` is an ordinary generic enum, four lines of std

## Not yet written

Listed here rather than as empty pages, so the gaps are visible: **generic functions in anger** (the `largest`/`smallest` pair, and returning a reference to dodge a `Copy` bound), **lifetimes as generic parameters** (`struct Excerpt<'a>` — the same brackets, a different kind of thing inside), **associated types versus type parameters** (`Iterator::Item` and why it is not `Iterator<T>`), **`impl Trait` in return position** and what it hides, **const generics** (`[T; N]`, and the arrays that finally became one type), **default type parameters** (`Add<Rhs = Self>`), **blanket impls** (`impl<T: Display> ToString for T`, and the coherence rules that bound them), and **variance** — why `&'static str` is accepted where `&'a str` is wanted, and where that stops.

## Po polsku

Typy generyczne (*generics*) to jeden z niewielu działów, dla których istnieje polskie tłumaczenie Tour of Rust — i jednocześnie ten, w którym polska intuicja najczęściej zawodzi, bo czytelnik przychodzi tu albo z Javy, albo z C++, a Rust bierze po jednej rzeczy z każdego z tych światów. Od strony maszyny zachowuje się jak szablon w C++: `<T>` jest wypełniane **w czasie kompilacji**, przez wygenerowanie osobnej kopii kodu dla każdego użytego typu (monomorfizacja, *monomorphization*), więc `Vec<u8>` to naprawdę ciąg bajtów — bez opakowywania (*boxing*), bez znacznika typu i bez kosztu w czasie działania. Od strony sprawdzania zachowuje się jak Java: definicja jest sprawdzana **sama w sobie**, wobec zadeklarowanych ograniczeń (*bounds*), a nie dopiero w miejscu wywołania.

Konsekwencja tego drugiego zaskakuje głównie osoby po C++: nieograniczony `T` nie umie prawie nic — nie wypiszesz go przez `{}` ani nie porównasz, dopóki nie dopiszesz `T: Display` albo `T: PartialEq`. W zamian komunikat o błędzie ląduje w funkcji, którą właśnie piszesz, a nie kilkanaście poziomów głębiej w cudzej bibliotece, przy tworzeniu instancji szablonu.

Ten dział prowadzi jeden mały typ — najpierw pojemnik, potem listę jednokierunkową — przez pięć pytań po kolei: co znaczą nawiasy ostre, co zrobić, gdy kompilator nie potrafi wywnioskować typu (`E0282`), co wolno `T`-owi i gdzie postawić ograniczenie, co się zmienia, gdy generyczne jest wyliczenie, i co się dzieje, gdy typ generyczny zawiera sam siebie (`E0072`). Kody błędów są tu najlepszym hasłem wyszukiwania: po polsku o `E0072` nie ma praktycznie nic, po angielsku jest wszystko.

**Szukaj po polsku:** typy generyczne w Ruscie · monomorfizacja · ograniczenia typów generycznych · `rust generics monomorphization` · `rust E0282 type annotations needed`
