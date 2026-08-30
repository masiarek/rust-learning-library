# Traits

**One line:** A trait is how one piece of code talks about many types at once — and it is the feature the rest of Rust is built out of, which is why `Copy`, `Display`, `Iterator`, `From` and `Default` all turn up long before anybody explains what they have in common.

Rust has no classes and no inheritance. What it has instead is this: a type declares its data, an `impl` block gives that type its own functions, and a **trait** names behaviour that any number of unrelated types can promise to provide. Everything a generic function knows about its argument, it knows because a trait bound said so.

The section starts from the declaration and works outward — what goes in a trait, how a call finds it, and what happens when a function has to return one.

| Lesson | Level | What it teaches |
|---|---|---|
| [What a trait is](what_a_trait_is/README.md) | 101 → 201 | The declaration itself: abstract methods, default bodies, associated constants, and `Self` — plus why it is neither a base class nor quite an interface |
| [A trait must be in scope](trait_in_scope/README.md) | 201 | The `use` that makes the methods appear, `E0599` when it is missing, and the three spellings of one call — including the fully-qualified form that is the *only* way past an inherent method of the same name |
| ["No method named …"](no_method_named/README.md) | 201 | The other three things `E0599` means: the method nobody wrote, the trait nobody implemented, and the blanket impl whose bound your type misses — told apart by the `help:` line, which is absent in the first case and names the wrong trait in the last |
| [Operators are traits](operators_are_traits/README.md) | 201 | `a + b` is `Add::add(a, b)`: `type Output` letting `+` change the type, the four separate impls one `*` can need, why `3 * p` is not free once `p * 3` exists, and the test for whether an operator should exist at all |
| [Returning a trait](returning_a_trait/README.md) | 201 | Why `-> Trait` cannot compile, `impl Trait` when one type comes back, `Box<dyn Trait>` when several can, and the second pointer that makes `&dyn` sixteen bytes |
| [Static vs dynamic dispatch](static_vs_dynamic_dispatch/README.md) | 201 → 301 | `<P: Trait>` vs `&dyn Trait`: monomorphization against a vtable, the heterogeneous collection that decides it, the enum that gets you one without either, the type check `dyn` never performs, and `E0038` — which the compiler now calls *dyn compatibility* and every book still calls *object safety* |
| [Supertraits](supertraits/README.md) | 201 | `trait Shout: Display` is a bound, not a parent — what the default bodies may spend, the `E0277` that lands on the `impl` line, and trait upcasting, stable since 1.86 |
| [Marker traits](marker_traits/README.md) | 201 | Traits with no methods: `Sized` as the bound you never wrote, `?Sized`, `Send`/`Sync` as auto traits nobody implements, and `PhantomData` |
| [Phantom types](phantom_types/README.md) | 201 → 301 | A type parameter with no data behind it: `E0392` and the `PhantomData` field that answers it, what the field claims about variance and drop, `impl Ballot<Star>` as a specialization for one tag, and the three things the tag costs you |
| [`Drop`, and what RAII buys](drop_and_raii/README.md) | 201 | The destructor that runs at a place you can point to: three drop orders, two of them opposites, `E0040` on an explicit call, and the `let _ =` that releases a guard before the work starts |
| [`ToOwned`](to_owned/README.md) | 201 | `Clone` generalized to borrowed data: why `str` cannot be `Clone` at all, why `.clone()` on a `&str` quietly does nothing, and the `Rc` that `to_owned` does not deep-copy |
| [`clone_into`](clone_into/README.md) | 201 | The provided method beside `to_owned`: refilling a buffer instead of allocating one, counted — plus the three ways it does not pay, and the backwards argument order that blocked it for five years |
| [The comparison traits](comparison_traits/README.md) | 201 | `==` and `<` are traits, and the `Partial` half is why `f64` has both while having neither `Eq` nor `Ord` — plus the derived `Ord` that sorts by field declaration order. Stub |
| [`Read` and `Write`](read_and_write/README.md) | 201 | The two traits between your code and every byte source there is — `&[u8]` is a `Read`, which is the whole testing story, and a `BufWriter` that flushes on drop throws the error away. Stub |
| [Links and videos](resources/README.md) | reference | The reading behind the section |

Three trait pages live outside this folder, because they are met long before anyone goes looking for a traits section: [`Copy` vs `Clone`](../16_Structs/copy_vs_clone/README.md), [Debug and Display](../15_First_Programs/debug_vs_display/README.md), and [the `Default` trait](../03_Command_Line/the_default_trait/README.md). They stay where they are; this section links them rather than moving them.

**Trait bounds** live outside it too, in [Where the bound goes](../22_Generics/where_the_bound_goes/README.md) — because the question they answer is a question about a generic (*what may this `T` do?*) rather than about a trait, and because the page's real subject is the placement: on the `impl` that spends the bound, never on the struct.

## Not yet written

The topics below are the rest of the map, in rough order of when you need them. They are listed here rather than as empty pages so the gaps are visible without pretending to be lessons: **associated types** and how they differ from generic parameters, the **orphan rule** and coherence, **blanket impls**, **negative impls**, **trait aliases**, and **async traits**.
