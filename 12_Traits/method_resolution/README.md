# Method resolution

**Level:** 201 · working knowledge

**One line:** `x.f()` is not a lookup on `x`'s type — it is a search that dereferences and borrows until something named `f` fits, and the first thing it finds wins.

```rust
fn main() {
    let name = String::from("Ada");
    println!("{}", name.len());          // 3   <- str::len, one deref down
    println!("{}", (&&&name).len());     // 3   <- same method, three refs peeled off
}
```

`String` has no `len`. `str` does, and it takes `&self` — so the call you wrote became `str::len(&*name)`, with both the deref and the borrow inserted. That insertion is what makes the dot operator feel like it works on everything.

## The search

For `x.f()` where `x: T`, the compiler builds a list of **candidate receiver types**:

1. Start with `T`.
2. Dereference repeatedly — `T`, `*T`, `**T`, … — adding each to the list, following `Deref` impls as well as plain references.
3. At the very end, try one unsized coercion (`[T; N]` → `[T]`).

Then it walks that list **in order**, and for each candidate `U` looks for a method whose receiver is `U`, then `&U`, then `&mut U`. The first match ends the search.

Two things follow, and both are the subject of the rest of this page. **Nearer wins:** a method one rung up beats an identical one further down, however much better the deeper one fits. And **inherent beats trait** at the same rung, which is the rule [A trait must be in scope](../trait_in_scope/README.md) is about.

For a `Vec<i32>` the ladder reads `Vec<i32>` → `[i32]`, which is why `v.iter()`, `v.first()` and `v.sort()` — all slice methods — answer a call made on a `Vec`.

## The trap: an inherent method shadows the target's

Give a type a `Deref` impl and it inherits the target's whole method surface. Add an inherent method with a name the target already uses, and every call silently changes meaning:

```rust
struct Tally(Vec<i32>);

impl Deref for Tally {
    type Target = Vec<i32>;
    fn deref(&self) -> &Vec<i32> { &self.0 }
}

impl Tally {
    fn len(&self) -> String { format!("{} entries", self.0.len()) }   // added later
}
```

Now `t.len()` is a `String`. It was a `usize` before that `impl` block existed, nothing warns, and every caller that was doing arithmetic on it stops compiling somewhere else entirely.

| Spelling | Reaches | Returns |
|---|---|---|
| `t.len()` | `Tally::len` — first rung | `String` |
| `(*t).len()` | `Vec::len` — deref written out | `usize` |
| `Vec::len(&t)` | `Vec::len` — named, not searched | `usize` |
| `t.first()` | `[i32]::first` — nothing shadows it | `Option<&i32>` |

`Vec::len(&t)` works because a function argument *is* a coercion site, so `&Tally` coerces to `&Vec<i32>` on the way in — a different mechanism from the receiver search, doing the same job. See [Coercion](../../29_Conversion/coercion/README.md).

The practical rule for anyone writing a `Deref` impl: **the target's method names are now yours to collide with.** This is the main reason `Deref` is discouraged for anything that is not a smart pointer, and the reason the Book recommends writing `Rc::clone(&a)` rather than `a.clone()` — naming the type says which `clone` you meant instead of leaving it to the ladder.

## It crosses as many pointers as it needs

```rust
let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
shared.borrow_mut().push(4);      // Rc -> RefCell::borrow_mut, then Vec::push
```

No `*` anywhere, and three types involved. This is also where the search's convenience turns into its worst error message: when nothing matches, `E0599` reports the method as missing from the *original* type, with a `help:` line listing what it tried — see ["No method named …"](../no_method_named/README.md) for reading it.

## If you are coming from another language

- **C++.** `operator->` chaining is the near relative — `a->b` re-applies `operator->` until it reaches a raw pointer — and `Deref` is doing the same thing for the dot. The differences are worth holding onto: Rust's search also inserts the *borrow* (`&` or `&mut`), which C++ has no need for; and overload resolution does not exist here, so the ambiguity C++ resolves by argument types is resolved in Rust purely by position on this ladder. Where C++ would report an ambiguous call, Rust silently picks the nearer one.
- **Python.** Attribute lookup walks the MRO and then `__getattr__`, which is a similar "keep looking" story with a very different failure mode: Python resolves at call time and tells you at run time, so shadowing a method is something you find in production. Rust's ladder is resolved at compile time, so the `Tally::len` shadowing above cannot produce a wrong answer — it produces a type error at whoever was using the result. The habit to port is the same though: adding a method to a base class, or an inherent method to a `Deref` type, is an API change even though nothing was removed.
- **ABAP.** Method resolution is much flatter — a method belongs to a class or an interface, and `me->method( )` versus `if_x~method( )` is the whole disambiguation story, with the interface-alias spelling as the escape hatch. That alias spelling is the closest thing to `<Fox as Shout>::shout(&fox)`. What has no counterpart is the deref half: ABAP references are dereferenced explicitly with `->*` and nothing chains them for you, so the "which type is this method actually on?" question does not arise.

## The verified output

<!-- output:method_resolution -->
*Verified output of [`method_resolution.rs`](examples/method_resolution.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The receiver you did not write: `&` is inserted for you
   seats.len()                 -> 3
   the method takes &self, so the receiver was `&seats`

2. Deref first, then borrow — `String` has no `len`, `str` does
   name.len()                  -> 3
   name.to_uppercase()         -> ADA
   both live on `str`, one deref down

3. The ladder is walked as far as it goes: Vec -> [i32]
   seats.iter().sum()          -> 6   (`iter` is a slice method)
   seats.first()               -> Some(3)

4. Extra references cost nothing — they are peeled off first
   (&&&name).len()             -> 3

5. The trap: an inherent method on a Deref wrapper shadows the target's
   t.len()      -> "3 entries"  <- Tally::len, and it returns a String
   (*t).len()   -> 3            <- Vec::len, reached by dereferencing first
   Vec::len(&t) -> 3            <- the same method, named instead of found
   t.first()    -> Some(3)      <- unshadowed, so it still reaches the slice

6. The ladder crosses as many smart pointers as it needs
   Rc -> RefCell -> Vec        -> [1, 2, 3, 4]
```
<!-- /output -->

## See also

- [A trait must be in scope](../trait_in_scope/README.md) — the inherent-beats-trait rule, and the three spellings that name a method instead of searching for one
- ["No method named …"](../no_method_named/README.md) — what `E0599` means when the ladder runs out
- [Coercion](../../29_Conversion/coercion/README.md) — the argument-side insertion, which is how `Vec::len(&t)` compiles
- [Reborrowing](../../18_Ownership/reborrowing/README.md) — the receiver the search inserts is a reborrow, not a move
- [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) — the single most-used rung of this ladder

## Sources

The Reference on [method-call expressions ↗](https://doc.rust-lang.org/reference/expressions/method-call-expr.html), which states the candidate-list construction and the `U` / `&U` / `&mut U` order; [`std::ops::Deref` ↗](https://doc.rust-lang.org/std/ops/trait.Deref.html), whose own docs carry the warning about `Deref` on non-pointer types.

## Po polsku

Wywołanie `x.f()` nie jest zwykłym odczytem metody z typu `x`. To **wyszukiwanie**: kompilator buduje listę kandydatów na odbiorcę — najpierw `T`, potem kolejne wyłuskania (`*T`, `**T`, …, podążając też za implementacjami `Deref`), a na samym końcu jedno rozmiarowanie `[T; N]` → `[T]` — po czym przechodzi tę listę **po kolei** i dla każdego kandydata `U` szuka metody o odbiorcy `U`, następnie `&U`, następnie `&mut U`. Pierwsze trafienie kończy poszukiwania. Dlatego `name.len()` na `String` trafia w `str::len` (bo `String` nie ma `len`), a `(&&&name).len()` działa tak samo — nadmiarowe referencje są zdejmowane po drodze.

Z tego wynika pułapka, którą warto znać, zanim napisze się własne `impl Deref`. Typ z `Deref` **dziedziczy całą powierzchnię metod celu**, więc dodanie własnej metody o nazwie, której cel już używa, po cichu zmienia znaczenie każdego wywołania: `t.len()` zaczyna zwracać `String` zamiast `usize`, nic nie ostrzega, a błąd kompilacji pojawia się dopiero u tego, kto z wyniku korzystał. Do metody celu wciąż można dotrzeć — przez `(*t).len()` albo `Vec::len(&t)` — ale trzeba wiedzieć, że jest co omijać. Stąd praktyczna zasada: `Deref` implementuje się dla wskaźników inteligentnych, nie dla zwykłych opakowań, a `Rc::clone(&a)` pisze się zamiast `a.clone()` właśnie po to, żeby nazwać typ, a nie zdać się na drabinę.

Czytelnikowi znającemu C++ najbliższym odpowiednikiem jest łańcuchowanie `operator->`, z dwiema różnicami: Rust wstawia też **pożyczenie** (`&` albo `&mut`), a przeciążania metod tu nie ma, więc niejednoznaczność, którą C++ rozstrzyga typami argumentów, w Ruscie rozstrzyga wyłącznie pozycja na drabinie — cicho, na korzyść bliższego kandydata.

**Szukaj po polsku:** rozstrzyganie wywołania metody · automatyczne wyłuskanie odbiorcy · przesłanianie metody przez `Deref` · `rust method resolution order` · `rust autoref autoderef`
