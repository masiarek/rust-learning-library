# Advanced

Pages that assume [the Foundations spine](../01_Foundations/README.md) and then add a second hard thing on top — shared state across threads, `unsafe`, FFI, macros, layout and performance. Nothing here is harder to *read* than a Foundations page; what makes it advanced is that the mistakes cost more and the compiler helps less.

Each page is still one idea with a program you can compile and run beside it.

| Lesson | Level | What it teaches |
|---|---|---|
| [Spawning a thread](spawning_a_thread/README.md) | 201 → 301 | `spawn`, the `JoinHandle` that is the only way to a return value, the `'static` bound behind `E0373`, and `thread::scope` — which borrows what `spawn` cannot, so a fan-out needs no `Arc` at all |
| [Channels](channels/README.md) | 201 → 301 | Handing a value over instead of sharing one: `send` moves, the `Sender` you kept that stops the program exiting, the difference between *empty* and *disconnected*, and when a bound is backpressure rather than a limit |
| [Lock poisoning](mutex_poisoning/README.md) | 301 | Why `Mutex::lock()` returns a `Result` at all, what your `.unwrap()` is deciding on someone's behalf, and the three honest answers |
| [The right to vote is a value](one_person_one_vote/README.md) | 301 | Authentication as a type rather than a boolean — a permission that cannot be forged, a `cast` that spends it, and the hole move semantics still cannot close |
| [Scale the denominator away](scaled_integers/README.md) | 301 | Exact arithmetic without fractions — why a float breaks a tied count, why the rationals that fix it cost a `gcd` apiece, and when you can multiply the denominators out and spend the count in `i128` |
| [What `i128` is exact about](i128_exactness/README.md) | 301 | The three different properties "exact" gets used for, which one a 128-bit integer actually has, what widening costs per operation — and why `fractions.Fraction` is a different trade rather than a slower one |
| [When the denominators compound](compounding_weights/README.md) | 301 | The proportional count the companion trick cannot reach — why its denominators compound out of the previous round's, why the width you need is a property of the ballots, and what a fixed-width rational really runs out of |
| [Did the rounding decide it?](interval_arithmetic/README.md) | 301 | The question the other four do not ask — bracket each rounded term instead of storing it, and find out whether your choice of scale or the ballots picked the winner; sound but not complete, so *decided* is a proof and *undecided* is only about that scale |
| [What `unsafe` turns off](what_unsafe_turns_off/README.md) | 301 | The five powers it grants and the nothing else it turns off — the borrow checker still runs inside the block; `split_at_mut` as a safe API over a small unsafe core; `unsafe fn` vs `unsafe {}` after edition 2024; and why the audit unit is the module, because a safe line three away is what makes the block sound |
| [What a union is](what_a_union_is/README.md) | 301 | Fields on top of each other instead of side by side — why the write is safe and the read is not, why the field type decides whether a bad read is merely wrong or undefined, and why an `enum` is this with the tag already checked |
| [Interior mutability](interior_mutability/README.md) | 301 | Writing through a `&T` — `Cell` and `RefCell` move the borrow check from compile time to run time, which buys the shapes the static checker cannot prove and costs a panic instead of an error. Stub |
| [`Send` and `Sync`](send_and_sync/README.md) | 301 | The two methodless traits that decide what may cross a thread boundary, why nobody writes either, and why the error always names a type three layers inside the closure. Stub |
| [`RwLock` and atomics](rwlock_and_atomics/README.md) | 301 | The two rungs either side of a `Mutex` — and the reason two atomics read together are not atomic together. Stub |
| [The global allocator](the_global_allocator/README.md) | 301 | Where every heap byte comes from — swapping in a counter that turns `String`'s capacity ladder into events you can see, the three rules that keep such a measurement honest, and why the five ways to make a `String` all cost exactly one allocation |

## Planned

Rough order, not a promise. The three that used to sit here — `Send`/`Sync`, `RwLock` and atomics, and interior mutability — are stubs in the table above now, which is the same list with their boundaries written down.

- **FFI beyond a union** — `extern "C"`, `#[repr(C)]`, and what a raw pointer across the boundary obliges you to prove
- **Macros** — `macro_rules!` and the procedural kind, once [what the `!` means](../25_Control_Flow/macros/README.md) has been said at 101

## Po polsku

„Zaawansowany” brzmi po polsku jak zapowiedź trudniejszej składni, i akurat tutaj znaczy coś innego. Żadna z tych stron nie jest trudniejsza w czytaniu niż strony z `01_Foundations` — zaawansowane jest to, że **błąd kosztuje więcej, a kompilator pomaga mniej**. W `unsafe` nie znika borrow checker (działa dalej, w środku bloku), znika tylko kilka konkretnych gwarancji; przy FFI i przy surowych wskaźnikach to ty bierzesz na siebie dowód, którego dotąd wymagał od ciebie kompilator. Wątki są w tym rozdziale z tego samego powodu: `spawn` wymusza `'static`, `thread::scope` pozwala pożyczyć zamiast klonować, a `Mutex::lock()` zwraca `Result` nie bez przyczyny — twoje `.unwrap()` podejmuje wtedy decyzję za kogoś innego.

Rozdział ma jeden garb, którego nie znajdziesz w typowym „advanced Rust”: **cztery strony pod rząd o dokładności liczenia** — skalowanie mianowników, co właściwie jest dokładne w `i128`, co się dzieje, gdy mianowniki narastają z rundy na rundę, i arytmetyka przedziałowa, która jako jedyna pyta, czy o wyniku nie zdecydowało zaokrąglenie. To nie jest teoria liczb dla ozdoby: liczenie głosów na `f64` potrafi rozstrzygnąć remis w złą stronę, a „dokładny” to po polsku, tak samo jak po angielsku, słowo używane na trzy różne sposoby naraz.

Praktyczna uwaga do czytania tabeli: pozycje oznaczone **Stub** to celowo zapisane granice tematu, a nie gotowe lekcje — po polsku najbliższe będzie „szkic”. I nie szukaj polskich odpowiedników nazw z tej tabeli: `Send`, `Sync`, `unsafe`, `union` i cała reszta to słowa kluczowe albo nazwy typów, więc odmieniamy je po polsku („w `unsafe`”, „przez `Mutex`a”), ale nie tłumaczymy.

**Szukaj po polsku:** wielowątkowość w Ruscie · arytmetyka dokładna zamiast zmiennoprzecinkowej · `rust unsafe what it turns off` · `rust thread scope borrow` · `rust mutex poisoning`
