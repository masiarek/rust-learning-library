# Advanced

Pages that assume [the Foundations spine](../01_Foundations/README.md) and then add a second hard thing on top — shared state across threads, `unsafe`, FFI, macros, layout and performance. Nothing here is harder to *read* than a Foundations page; what makes it advanced is that the mistakes cost more and the compiler helps less.

Each page is still one idea with a program you can compile and run beside it.

| Lesson | Level | What it teaches |
|---|---|---|
| [Lock poisoning](mutex_poisoning/README.md) | 301 | Why `Mutex::lock()` returns a `Result` at all, what your `.unwrap()` is deciding on someone's behalf, and the three honest answers |
| [The right to vote is a value](one_person_one_vote/README.md) | 301 | Authentication as a type rather than a boolean — a permission that cannot be forged, a `cast` that spends it, and the hole move semantics still cannot close |
| [Scale the denominator away](scaled_integers/README.md) | 301 | Exact arithmetic without fractions — why a float breaks a tied count, why the rationals that fix it cost a `gcd` apiece, and when you can multiply the denominators out and spend the count in `i128` |
| [What `i128` is exact about](i128_exactness/README.md) | 301 | The three different properties "exact" gets used for, which one a 128-bit integer actually has, what widening costs per operation — and why `fractions.Fraction` is a different trade rather than a slower one |
| [When the denominators compound](compounding_weights/README.md) | 301 | The proportional count the companion trick cannot reach — why its denominators compound out of the previous round's, why the width you need is a property of the ballots, and what a fixed-width rational really runs out of |
| [Did the rounding decide it?](interval_arithmetic/README.md) | 301 | The question the other four do not ask — bracket each rounded term instead of storing it, and find out whether your choice of scale or the ballots picked the winner; sound but not complete, so *decided* is a proof and *undecided* is only about that scale |
| [What a union is](what_a_union_is/README.md) | 301 | Fields on top of each other instead of side by side — why the write is safe and the read is not, why the field type decides whether a bad read is merely wrong or undefined, and why an `enum` is this with the tag already checked |
| [The global allocator](the_global_allocator/README.md) | 301 | Where every heap byte comes from — swapping in a counter that turns `String`'s capacity ladder into events you can see, the three rules that keep such a measurement honest, and why the five ways to make a `String` all cost exactly one allocation |

## Planned

Rough order, not a promise:

- **`Send` and `Sync`** — the two traits that decide what may cross a thread boundary, and why you almost never write them yourself
- **`Arc<Mutex<T>>` and its alternatives** — channels, `RwLock`, atomics, and the question of which one your problem actually is
- **Interior mutability** — `Cell`, `RefCell`, and the runtime borrow check you are opting into
- **`unsafe`** — what it does and does not turn off, and why the unit of review is the module rather than the block
