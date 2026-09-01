# The wildcard `_`

**Level:** 201 · working knowledge

**One line:** `_` is a pattern that matches anything and **binds nothing** — so it moves nothing, owns nothing, and drops nothing, and every surprising thing it does follows from that one sentence.

```rust
let ballot = String::from("5,3,0");
let _ = ballot;             // NOT a move — `_` never took ownership
println!("{ballot}");       // 5,3,0
```

Replace the underscore with a name and the same two lines stop compiling:

```text title="Abridged — real rustc output for underscore_moves.rs"
error[E0382]: borrow of moved value: `other`
 --> underscore_moves.rs:8:16
  |
7 |     let _x = other;                 // a real binding: this IS a move
  |              ----- value moved here
8 |     println!("{other}");            // E0382
  |                ^^^^^ value borrowed here after move
```

`_x` is an ordinary binding that happens to start with an underscore. `_` is not a short name — it is the absence of one.

## It changes when a *temporary* dies, and never when an owned value does

This is the pair to hold in your head, because the two lines look identical and the difference decides whether your lock is held:

| | What it does | When the value drops |
|---|---|---|
| `let _ = make();` | matches a **temporary**, binds nothing | **at the semicolon** — nothing owns it |
| `let _g = make();` | binds the temporary | end of the enclosing block |
| `let s = make(); let _ = s;` | matches a **place** that already has an owner | end of the block, as `s`, unchanged |
| `_ = s;` (no `let`) | the same, as an assignment | end of the block, as `s`, unchanged |

So `_` never extends a lifetime and never shortens one. It declines to participate, and whoever already owned the value keeps owning it. When nobody owned it — when the right-hand side was a freshly-made temporary — declining to participate means it dies at the end of the statement.

That is [the `let _ = mutex.lock();` bug](../../12_Traits/drop_and_raii/README.md) in one row of a table: the guard is a temporary, `_` refuses it, the lock releases before the protected work starts, and nothing warns.

## `_ = expr;` is an assignment, not a binding

Since Rust 1.59 the left of an `=` may be a pattern, so the `let` is optional:

```rust
#[must_use]
fn tally(scores: &[u8]) -> u32 { scores.iter().map(|s| u32::from(*s)).sum() }

_ = tally(&[5, 3, 0]);       // deliberately discarded, and it says so
```

Without it, the `#[must_use]` fires — and the compiler suggests the older spelling:

```text title="Abridged — real rustc output for discarded_result.rs"
warning: unused return value of `tally` that must be used
 --> discarded_result.rs:4:5
  |
4 |     tally(&[5, 3, 0]);
  |     ^^^^^^^^^^^^^^^^^
  |
help: use `let _ = ...` to ignore the resulting value
```

The two forms mean the same thing and have the same drop timing. `let _ = …` is older and is what rustc suggests; `_ = …` reads as what it is, an assignment to nowhere, and does not introduce a `let` that binds nothing.

## `_` and `_name` answer two different questions

| | Is it a binding? | Owns the value? | What you are saying |
|---|---|---|---|
| `_` | no | no | there is no value here to name |
| `_name` | **yes** | **yes** | a value I am deliberately not reading |

Both silence [`unused_variables`](../../15_First_Programs/what_a_warning_is_asking/README.md), which is why they get treated as interchangeable, and they are not. Only `_name` can hold a guard. Only `_name` can be renamed back into use later. And only `_` can appear where a binding would be wrong — inside a `match` arm, a closure parameter, a tuple you are half-unpacking.

## The same glyph, five jobs

```rust
let pair = (7u8, 9u8);
let (_, second) = pair;                              // one field, unnamed
for _ in 0..3 {}                                     // the loop variable, unwanted
let scores = [5u8, 3, 0];
let ones: Vec<u8> = scores.iter().map(|_| 1).collect();   // a closure argument, ignored
let refs: Vec<_> = ones.iter().collect();                 // a TYPE hole, not a pattern
let list = [1u8, 2, 3, 4];
let ends = match list { [first, .., last] => (first, last) };   // `..` is the REST
println!("{second} {ones:?} {} {ends:?}", refs.len());   // 9 [1, 1, 1] 3 (1, 4)
```

Four of those five are the pattern this page is about. `Vec<_>` is not: the `_` in a **type** is an inference hole, asking the compiler to work the type out, and it has nothing to do with discarding a value. Same character, unrelated feature — worth naming because the two read alike and a reader who conflates them expects `Vec<_>` to throw something away.

`..` is the other near-neighbour. `_` matches **exactly one** thing without naming it; `..` matches **however many are left**. `[first, _, last]` only fits a three-element array; `[first, .., last]` fits any array of two or more.

## If you are coming from another language

**Python.** The convention looks identical and the mechanism is not. Python's `_` is an ordinary variable name with a social meaning — `for _ in range(n)` really does bind `_` to each value, `_, second = pair` really does keep the first element alive, and in the REPL `_` is the last result. Nothing in the language treats it specially. Rust's `_` is a *pattern*, enforced by the compiler: it creates no binding at all, so there is no name to hold a reference and no reference to keep the value alive. That is the whole difference, and it shows up exactly once, painfully: the `with lock:` you would reach for to scope a resource is `let _guard = …` in Rust, and writing `let _ = …` there releases the lock immediately. Two more differences worth having: Python's `match` statement (3.10+) *does* use `_` as a real wildcard pattern with Rust's meaning, so that one transfers cleanly; and `_ = f()` is legal in both, but in Python it is a plain assignment keeping the result alive under the name `_`, while in Rust nothing is kept.

**ABAP.** There is no wildcard binding, and the closest habits pull in the wrong direction. A `FIELD-SYMBOLS` assignment or an unused `EXPORTING` parameter always has a name, so "ignore this value" is expressed by declaring a variable you never read — which is exactly `_name` and never `_`. The genuinely useful transfer is the opposite one: `_` is how you say *this is deliberate* in the source instead of in a code-inspector pragma. Where ABAP writes `##NEEDED` or an SLIN exemption beside a variable to stop the checker complaining, Rust puts the same statement into the code itself, where it is the compiler's input rather than an annotation the compiler ignores. And `_` has one job ABAP's `-` placeholder in `SELECT` lists does not: it is the *only* way to satisfy an exhaustive `CASE`, since a Rust [`match`](../../25_Control_Flow/match_expressions/README.md) has no implicit fall-through and no optional `WHEN OTHERS`.

## The verified output

<!-- output:the_wildcard -->
*Verified output of [`the_wildcard.rs`](examples/the_wildcard.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: `_` binds nothing, so it moves nothing
  let ballot = String::from("5,3,0");
  let _ = ballot;
  ballot is still usable afterwards: "5,3,0"
      Replace `_` with any name — `let _x = ballot;` — and the
      next line stops compiling with E0382, moved value. The
      underscore is not a short name. It is the absence of one.

──── Step 2: ...which is why it changes WHEN a temporary dies
  A: let _  = make("temp");     a temporary, matched by `_`
      drop(temp)
      ...end of the statement reached
  B: let _g = make("temp");     the same temporary, bound
      ...end of the statement reached
      drop(temp)
      A dropped BEFORE the next line ran; B survived to the end
      of its block. Nothing owned A, so it died as a temporary
      at the semicolon. That one character is the most expensive
      mistake in Rust: `let _ = mutex.lock();` unlocks instantly.

──── Step 3: But it does NOT shorten the life of a value that is already owned
  C: let s = make("named"); let _ = s;
      ...after `let _ = s;`
      drop(named)
      `s` still owns it, so it drops at the end of the block, as
      `s` always would. A and C look identical and differ in one
      thing: whether the right-hand side was a temporary or a
      place. `_` never extends a lifetime and never shortens one;
      it declines to participate, and the value keeps its owner.

──── Step 4: `_ = expr;` with no `let` is an assignment, not a binding
  #[must_use] fn tally(&[u8]) -> u32
  _ = tally(&[5, 3, 0]);      the warning is answered, in one line
      Without it, `tally(&[5, 3, 0]);` warns:
        warning: unused return value ... that must be used
      `let _ = ...` says the same thing and is older; `_ = ...`
      is destructuring assignment, and it reads as what it is —
      an assignment to nowhere. Same drop timing as `let _ =`.

──── Step 5: `_` and `_name` answer two different questions
  for _ in &scores       3 iterations, no binding made
  let _unread = tally(..)  a real binding, silent about being unused
      `_`      = 'there is no value here to name'  — no binding
      `_name`  = 'a value I am deliberately not reading' — a binding,
                 which owns, drops at the end of scope, and can be
                 renamed back into use later. Only one of the two
                 is safe to put a lock guard in, and it is `_name`.

──── Step 6: The other places the same glyph shows up
  let (_, second) = pair;        second = 9
  [first, .., last]              1..4   `..` is 'the rest',
                                 `_` is 'exactly one, unnamed'
  |_| 1u8                        a closure ignoring its argument
  Vec<_>                         [1, 1, 1] — a DIFFERENT `_`: an
                                 inference hole in a TYPE, not a
                                 pattern. Same character, unrelated
                                 feature; it asks the compiler to
                                 fill the type in rather than to
                                 discard a value.
```
<!-- /output -->

## See also

- [Irrefutable patterns](../irrefutable_patterns/README.md) — `_` is one, which is why it is legal in a plain `let`
- [`Drop` and RAII](../../12_Traits/drop_and_raii/README.md) — the lock-guard bug this page's drop table explains
- [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) — `_` versus `_name`, from the `unused_variables` side
- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) — what "drops at the end of the block" means once values can move
- [`match` expressions](../../25_Control_Flow/match_expressions/README.md) — the `_` arm, and why exhaustiveness makes it load-bearing
- [Destructuring structs](../destructuring_structs/README.md) — `..` for the rest, the pattern `_` is confused with
- [Wildcard pattern ↗](https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern) · [Destructuring assignment ↗](https://doc.rust-lang.org/reference/expressions/underscore-expr.html)

## Po polsku

`_` to **wzorzec uniwersalny** (*wildcard pattern*), czasem nazywany symbolem wieloznacznym: pasuje do wszystkiego i **niczego nie wiąże**. Druga połowa tego zdania jest cała treścią strony. Skoro nie powstaje żadne wiązanie, to nic nie przejmuje własności — dlatego `let _ = karta;` **nie jest przeniesieniem** i `karta` nadal działa w następnej linijce, podczas gdy `let _x = karta;` już przenosi i kolejne użycie kończy się `E0382`. Podkreślnik nie jest krótką nazwą; jest brakiem nazwy.

Stąd bierze się jedyna naprawdę kosztowna pułapka. Dwie linijki wyglądają tak samo, a różni je to, co stoi po prawej stronie. Jeśli jest to **wartość tymczasowa** — wynik wywołania, jak `mutex.lock()` — to nikt jej nie przejmuje i ginie **na średniku**, czyli zanim zacznie się chroniona praca; blokada zwalnia się natychmiast i nic nie ostrzega. Jeśli po prawej stoi **zmienna, która już ma właściciela**, nie zmienia się nic: wartość zniknie na końcu bloku, tak jak zawsze. `_` nigdy nie przedłuża ani nie skraca życia wartości — on po prostu odmawia udziału, a właściciel zostaje ten sam. Lekarstwem jest jeden znak: `let _g = …`.

Dwie rzeczy warto jeszcze rozróżnić, bo wyglądają podobnie. `_name` to **prawdziwe wiązanie** z wiodącym podkreślnikiem — przejmuje własność, żyje do końca bloku i tylko ono nadaje się na strażnika blokady; wspólne mają z `_` wyłącznie to, że oba uciszają ostrzeżenie `unused_variables`. A `_` w **typie** (`Vec<_>`) to zupełnie inna funkcja języka: dziura do wypełnienia przez wnioskowanie typów, nie odrzucenie wartości. Ten sam znak, dwa niezwiązane mechanizmy.

**Szukaj po polsku:** wzorzec uniwersalny · symbol wieloznaczny · podkreślnik we wzorcu · `rust wildcard pattern` · `rust let _ drops immediately` · `rust _ vs _name`
