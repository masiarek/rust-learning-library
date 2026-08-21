# Advanced

Pages that assume [the Foundations spine](../01_Foundations/README.md) and then add a second hard thing on top — shared state across threads, `unsafe`, FFI, macros, layout and performance. Nothing here is harder to *read* than a Foundations page; what makes it advanced is that the mistakes cost more and the compiler helps less.

Each page is still one idea with a program you can compile and run beside it.

| Lesson | Level | What it teaches |
|---|---|---|
| [Lock poisoning](mutex_poisoning/README.md) | 301 | Why `Mutex::lock()` returns a `Result` at all, what your `.unwrap()` is deciding on someone's behalf, and the three honest answers |
| [The right to vote is a value](one_person_one_vote/README.md) | 301 | Authentication as a type rather than a boolean — a permission that cannot be forged, a `cast` that spends it, and the hole move semantics still cannot close |
| [Scale the denominator away](scaled_integers/README.md) | 301 | Exact arithmetic without fractions — why a float breaks a tied count, why the rationals that fix it cost a `gcd` apiece, and when you can multiply the denominators out and spend the count in `i128` |
| [What `i128` is exact about](i128_exactness/README.md) | 301 | The three different properties "exact" gets used for, which one a 128-bit integer actually has, what widening costs per operation — and why `fractions.Fraction` is a different trade rather than a slower one |

## Planned

Rough order, not a promise:

- **`Send` and `Sync`** — the two traits that decide what may cross a thread boundary, and why you almost never write them yourself
- **`Arc<Mutex<T>>` and its alternatives** — channels, `RwLock`, atomics, and the question of which one your problem actually is
- **Interior mutability** — `Cell`, `RefCell`, and the runtime borrow check you are opting into
- **`unsafe`** — what it does and does not turn off, and why the unit of review is the module rather than the block
