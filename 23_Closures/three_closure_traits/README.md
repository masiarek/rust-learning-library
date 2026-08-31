# The three closure traits

**Level:** 201 · working knowledge

**One line:** `Fn`, `FnMut` and `FnOnce` are a ladder, not a menu — every closure implements `FnOnce`, and what the **body** does with the capture decides how far up it goes.

```rust
fn main() {
    let name = String::from("Ada");
    let reads = || name.len();          // reads a capture      -> Fn
    println!("{} {}", reads(), reads());  // 3 3

    let mut count = 0;
    let mut bumps = || { count += 1; count };   // mutates one   -> FnMut
    println!("{} {}", bumps(), bumps());  // 1 2

    let owned = String::from("cookie");
    let eats = || owned;                // moves one out         -> FnOnce
    println!("{}", eats());               // cookie
}
```

Three closures, three verbs: **read**, **mutate**, **consume**. That is the whole classification.

## The ladder

```rust
pub trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}
pub trait FnMut<Args>: FnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}
pub trait Fn<Args>: FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}
```

The receivers are the definition. `&self` can be had any number of times, so an `Fn` is callable repeatedly and may not mutate. `&mut self` is exclusive but repeatable, so an `FnMut` may change its captures between calls. `self` is consumed, so an `FnOnce` runs at most once — which is exactly what lets its body move a capture out and hand it to you.

Because they are supertraits, the implementations are nested rather than exclusive:

| the body… | `Fn` | `FnMut` | `FnOnce` |
|---|---|---|---|
| only reads its captures | yes | yes | yes |
| mutates a capture | no | yes | yes |
| moves a capture out | no | no | yes |

Read the rows, not the labels: *"this closure is an `FnMut`"* is shorthand for *"`FnMut` is the tightest bound it satisfies"*. It is an `FnOnce` too.

## Which bound to write: the loosest one you can live with

The direction is the opposite of what "loosest" suggests. A parameter bounded by `FnOnce` accepts **every** closure; one bounded by `Fn` accepts only the closures that neither mutate nor consume. Tightening the bound does not make your function safer or faster — it refuses callers.

That is why std's choices look inconsistent until you read them as *"the least this job needs"*:

| std API | bound | why |
|---|---|---|
| [`Option::unwrap_or_else` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) | `FnOnce() -> T` | runs at most once, so the fallback may be an owned value moved out of the closure |
| [`Iterator::map` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) | `FnMut(Item) -> B` | runs per item, and is allowed to carry a running total between them |
| [`slice::sort_by_key` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key) | `FnMut(&T) -> K` | same: called many times, may keep state |
| [`thread::spawn` ↗](https://doc.rust-lang.org/std/thread/fn.spawn.html) | `FnOnce() -> T + Send + 'static` | the body runs once; `Send + 'static` is a separate promise about the captures |

If you write `F: Fn(..)` and your body only calls `f` once, you have narrowed the door for nothing.

## `move` is not what makes a closure `FnOnce`

The sentence to unlearn is some version of *"`FnOnce` is implemented when the closure consumes what it captured, which is what `move` denotes"*. Both halves are off, and the run below settles them in opposite directions:

- **`FnOnce` is not conditional.** It is the bottom of the ladder, so *every* closure implements it — including the one that just reads a `usize`. Only the *tightest* bound is conditional.
- **`move` decides where the captures live, not what the body may do to them.** A `move` closure that only reads is an `Fn` and can be called as often as you like; a closure with no `move` at all is an `FnOnce` the moment its body returns a capture by value:

```rust
fn once<F: FnOnce() -> String>(f: F) -> String { f() }

fn main() {
    let cookie = String::from("cookie");
    let no_move_but_once = || cookie;          // no `move`, still FnOnce
    println!("{}", once(no_move_but_once));      // cookie

    let biscuit = String::from("biscuit");
    let move_but_fn = move || biscuit.len();   // `move`, still Fn
    println!("{} {}", move_but_fn(), move_but_fn());  // 7 7
}
```

The second closure is called twice, which an `FnOnce` cannot be. [What `move` actually changes](../the_move_keyword/README.md) is its own page.

## Two errors that are this classification talking

**Calling an `FnOnce` twice** is not a special rule about closures. `call_once` takes `self`, so the call *moves* the closure — and the second call is an ordinary use-after-move:

```text title="Abridged — real rustc output, without the file-and-line headers"
error[E0382]: use of moved value: `consumer`
  |
4 |     consumer();
  |     ---------- `consumer` moved due to this call
5 |     consumer();
  |     ^^^^^^^^ value used here after move
  |
note: closure cannot be invoked more than once because it moves the variable `consumable` out of its environment
note: this value implements `FnOnce`, which causes it to be moved when called
```

**An `FnMut` closure needs a `mut` binding.** `call_mut` takes `&mut self`, so calling it borrows the closure mutably — and the compiler's message names the capture that put it in that category:

```text title="Abridged — real rustc output, without the file-and-line header"
error[E0596]: cannot borrow `bumps` as mutable, as it is not declared as mutable
  |
3 |     let bumps = || count += 1;
  |                    ----- calling `bumps` requires mutable binding due to mutable borrow of `count`
4 |     bumps();
  |     ^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
3 |     let mut bumps = || count += 1;
  |         +++
```

`let mut` on a closure looks strange the first time. It is not saying the closure is editable; it is saying the closure **is** the mutable state, and calling it is a mutation.

## If you are coming from another language

- **Python.** There is no counterpart, and the gap is the interesting part. Every Python callable is the `Fn` case forever: a lambda can be called any number of times, it can mutate anything it can reach (with `nonlocal` for names in the enclosing scope), and nothing anywhere records that a callable has *given away* what it captured. The Rust closure `|| owned` — return the captured `String` and cease to exist — has no Python spelling, because in Python handing out a reference does not remove your own. The `FnMut` case is the familiar counter closure with `nonlocal count`, and it maps well; what does not transfer is that in Rust the *binding* has to be `mut`, since calling the thing is a write. And where Python hands the caller a docstring convention, Rust hands them a bound: `F: FnOnce` in a signature is a promise to the caller that their closure will be called at most once, checked at compile time.
- **ABAP.** The nearest structure is again a local class, and the three traits fall out of what its method does to its own attributes: a `METHODS run` that only reads them is `Fn`; one that writes them is `FnMut`; one that hands an attribute out and leaves the object unusable is `FnOnce` — and ABAP has no way to say that last one, so the convention is a `lv_consumed` flag and a `RAISE EXCEPTION` on the second call. That is the difference worth carrying: Rust moves the check from your runtime guard to the compiler, and the error is the ordinary use-after-move above rather than a dump in production. The other half maps onto `IMPORTING` versus `CHANGING`: an `Fn` closure is a method whose parameters are all `IMPORTING` and whose attributes are read-only; an `FnMut` is one with `CHANGING` state. ABAP's `FUNCTIONAL METHODS` naming is a coincidence of vocabulary, not the same idea — it means "returns a value", nothing about purity.
- **C++.** A lambda with a mutable capture needs `mutable` on the lambda, which is the same statement as Rust's `let mut` above, made in the same place for the same reason: calling it is not a `const` operation. What C++ has no equivalent of is `FnOnce`; a `std::function` can always be called again, and moving out of a capture leaves the lambda holding a moved-from object that the type system will still let you call. `std::move_only_function` (C++23) is the closest, and it arrived twenty years later.

---

## The verified output

<!-- output:three_closure_traits -->
*Verified output of [`three_closure_traits.rs`](examples/three_closure_traits.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three closures, three things the body does with the capture
   || name.len()          reads it     -> 3
   the capture survives the call: name = "Ada"
   || { count += 1; count }  mutates it   -> 1 then 2
   the mutation is real: count = 2
   || owned               moves it out -> "cookie"
   ...and `eats` is now spent. Calling it again is E0382.

2. The ladder: Fn is an FnMut is an FnOnce
   trait FnMut<A>: FnOnce<A>   and   trait Fn<A>: FnMut<A>
   the reading closure, through F: Fn      -> (8, 8)
   the same closure, through F: FnMut      -> (8, 8)
   the same closure, through F: FnOnce     -> "8 chars"
   so a bound of FnOnce accepts ALL closures; a bound of Fn accepts
   only the ones that neither mutate nor consume. Take the loosest
   bound your body can live with, or you refuse callers for nothing.

3. What decides it is the BODY, not the `move` keyword
   `|| cookie`        (no move)  is FnOnce: "cookie"
   `move || biscuit.len()`       is Fn:     (7, 7)
   called twice, which an FnOnce could not be. `move` chose where the
   String lives; the body chose which traits the closure gets.

4. An FnMut closure IS the state — and it needs a `mut` binding
   record("Ada")  -> 1
   record("Ben")  -> 2
   record("Cara") -> 3
   seen = ["Ada", "Ben", "Cara"]
   without `let mut record`, the call is E0596 — the closure has to be
   borrowed mutably to be called at all.

5. Where std picked each one, and why
   Option::unwrap_or_else(self, f: F)   F: FnOnce() -> T
       runs at most once, so it may hand over an owned fallback.
   Iterator::map(self, f: F)            F: FnMut(Self::Item) -> B
       runs per item and is allowed to carry a running total.
   Iterator::any(&mut self, f: F)       F: FnMut(Self::Item) -> bool
   slice::sort_by_key(&mut self, f: F)  F: FnMut(&T) -> K
   thread::spawn(f: F)                  F: FnOnce() -> T + Send + 'static
       the thread body runs once; `Send + 'static` is a separate promise.
   Each is the loosest bound that job can accept. Nothing here is about
   speed: the bound decides which closures a caller may hand you.
```
<!-- /output -->

---

## See also

- [What a closure is](../what_a_closure_is/README.md) — the anonymous struct these three traits are implemented on
- [The `move` keyword](../the_move_keyword/README.md) — the other half of the sentence corrected above
- [`unwrap_or_else`](../../17_Option_and_Result/unwrap_or_else/README.md) — `FnOnce` in std, and why the fallback can be an owned value
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — why moving a capture out ends the closure, and why a `Copy` capture never does
- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — `call_once` taking `self` is that rule, applied to the closure itself
- [Supertraits](../../12_Traits/supertraits/README.md) — the `Fn: FnMut: FnOnce` relationship, as a language feature

## Sources

[`Fn` ↗](https://doc.rust-lang.org/std/ops/trait.Fn.html), [`FnMut` ↗](https://doc.rust-lang.org/std/ops/trait.FnMut.html) and [`FnOnce` ↗](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) in std — the three declarations above are those pages' own signatures. The Book's [closure chapter ↗](https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits) covers the same ladder.

## Po polsku

`Fn`, `FnMut` i `FnOnce` to **drabina, a nie menu** — i nazwy mylą tu po polsku bardziej niż po angielsku, bo `FnOnce` odruchowo czyta się jako „domknięcie jednorazowe”, czyli jakiś osobny gatunek domknięć (*closures*). Tymczasem `FnOnce` implementuje **każde** domknięcie, łącznie z tym, które tylko czyta jeden `usize`. Warunkowy jest wyłącznie szczebel *najwyższy*, na jaki dane domknięcie wchodzi, więc zdanie „to domknięcie jest `FnMut`” to skrót od „`FnMut` to najciaśniejsze ograniczenie, jakie ono spełnia” — `FnOnce` też spełnia. O tym, jak wysoko się wejdzie, decyduje jeden czasownik w ciele: czyta, zmienia czy konsumuje przechwyconą wartość.

Definicją są odbiorniki metod i najlepiej nauczyć się ich na pamięć: `call_once(self)`, `call_mut(&mut self)`, `call(&self)`. `&self` można mieć dowolnie wiele razy, więc `Fn` wywołuje się bez ograniczeń i niczego nie zmienia; `&mut self` jest wyłączne, ale powtarzalne, więc `FnMut` może zmieniać swoje przechwycone wartości między wywołaniami; `self` zostaje skonsumowane, więc `FnOnce` wykonuje się najwyżej raz — i właśnie dlatego wolno mu wyprowadzić przechwyconą wartość na zewnątrz. `Fn: FnMut: FnOnce` to relacja cechy nadrzędnej (*supertrait*), nie dziedziczenie: implementacje się zagnieżdżają, zamiast wykluczać.

Przy pisaniu własnej sygnatury kierunek jest odwrotny do intuicji wyniesionej z języków obiektowych, gdzie „ostrzej” znaczy „bezpieczniej”. Parametr z ograniczeniem `FnOnce` przyjmuje **wszystkie** domknięcia, a z ograniczeniem `Fn` — tylko te, które ani nie zmieniają, ani nie konsumują przechwyconych wartości. Zacieśnienie ograniczenia niczego nie przyspiesza ani nie uszczelnia; **jedynie odmawia wywołującym**. Dlatego bierz najluźniejsze ograniczenie, jakie zniesie twoje ciało funkcji, a wybory ze standardowej biblioteki czytaj jako „minimum, którego wymaga to zadanie”: `Option::unwrap_or_else` bierze `FnOnce`, bo uruchamia się co najwyżej raz (i dzięki temu wartość zapasowa może być czymś posiadanym na własność), `Iterator::map` i `slice::sort_by_key` biorą `FnMut`, bo wołane są wielokrotnie i wolno im nieść licznik, a `thread::spawn` — `FnOnce() -> T + Send + 'static`, gdzie `Send + 'static` jest osobną obietnicą o przechwyconych wartościach, a nie częścią tej klasyfikacji.

Zdanie do oduczenia się brzmi mniej więcej tak: „`FnOnce` dostaje domknięcie, które konsumuje przechwyconą wartość, a oznacza się to słowem `move`”. Obie połowy są nietrafione i wydruk na tej stronie obala je w przeciwnych kierunkach: `|| cookie` **bez** `move` jest `FnOnce`, a `move || biscuit.len()` jest `Fn` i daje się wywołać dwa razy (7 i 7). `move` rozstrzyga, **gdzie mieszkają** przechwycone wartości; klasyfikację ustala ciało. Na koniec dwa błędy, które są tą samą klasyfikacją mówiącą wprost: drugie wywołanie `FnOnce` to zwykłe użycie po przeniesieniu własności (`E0382`), bo `call_once` bierze `self` — żadnej specjalnej reguły dla domknięć tu nie ma; a `E0596` żąda `let mut` przy domknięciu, które coś zmienia. To `mut` wygląda dziwnie tylko dopóty, dopóki czyta się je jako „domknięcie da się edytować”. Ono mówi co innego: domknięcie **jest** tym mutowalnym stanem, a jego wywołanie jest zapisem.

**Szukaj po polsku:** różnice Fn FnMut FnOnce · cecha nadrzędna · najluźniejsze ograniczenie · `rust Fn FnMut FnOnce difference` · `rust E0596 closure requires mutable binding` · `rust FnOnce cannot be called twice`
