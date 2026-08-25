# Traits

**One line:** A trait is how one piece of code talks about many types at once — and it is the feature the rest of Rust is built out of, which is why `Copy`, `Display`, `Iterator`, `From` and `Default` all turn up long before anybody explains what they have in common.

Rust has no classes and no inheritance. What it has instead is this: a type declares its data, an `impl` block gives that type its own functions, and a **trait** names behaviour that any number of unrelated types can promise to provide. Everything a generic function knows about its argument, it knows because a trait bound said so.

The section starts from the declaration and works outward — what goes in a trait, how a call finds it, and what happens when a function has to return one.

| Lesson | Level | What it teaches |
|---|---|---|
| [What a trait is](what_a_trait_is/README.md) | 101 → 201 | The declaration itself: abstract methods, default bodies, associated constants, and `Self` — plus why it is neither a base class nor quite an interface |
| [A trait must be in scope](trait_in_scope/README.md) | 201 | The `use` that makes the methods appear, `E0599` when it is missing, and the three spellings of one call — including the fully-qualified form that is the *only* way past an inherent method of the same name |
| [Returning a trait](returning_a_trait/README.md) | 201 | Why `-> Trait` cannot compile, `impl Trait` when one type comes back, `Box<dyn Trait>` when several can, and the second pointer that makes `&dyn` sixteen bytes |
| [Static vs dynamic dispatch](static_vs_dynamic_dispatch/README.md) | 201 → 301 | `<P: Trait>` vs `&dyn Trait`: monomorphization against a vtable, the heterogeneous collection that decides it, and `E0038` — which the compiler now calls *dyn compatibility* and every book still calls *object safety* |
| [Supertraits](supertraits/README.md) | 201 | `trait Shout: Display` is a bound, not a parent — what the default bodies may spend, the `E0277` that lands on the `impl` line, and trait upcasting, stable since 1.86 |
| [Marker traits](marker_traits/README.md) | 201 | Traits with no methods: `Sized` as the bound you never wrote, `?Sized`, `Send`/`Sync` as auto traits nobody implements, and `PhantomData` |
| [`ToOwned`](to_owned/README.md) | 201 | `Clone` generalized to borrowed data: why `str` cannot be `Clone` at all, why `.clone()` on a `&str` quietly does nothing, and the `Rc` that `to_owned` does not deep-copy |
| [Links and videos](resources/README.md) | reference | The reading behind the section |

Three trait pages live outside this folder, because they are met long before anyone goes looking for a traits section: [`Copy` vs `Clone`](../01_Foundations/copy_vs_clone/README.md), [Debug and Display](../01_Foundations/debug_vs_display/README.md), and [the `Default` trait](../03_Command_Line/the_default_trait/README.md). They stay where they are; this section links them rather than moving them.

## Not yet written

The topics below are the rest of the map, in rough order of when you need them. They are listed here rather than as empty pages so the gaps are visible without pretending to be lessons: **trait bounds** (`T: Trait`, `where`, and multiple bounds with `+`), **associated types** and how they differ from generic parameters, the **orphan rule** and coherence, **blanket impls**, **negative impls**, **trait aliases**, and **async traits**.
