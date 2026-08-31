# `unwrap_or_else`: the fallback that is built only if it is needed

**Level:** 201 · working knowledge

**One line:** Passing a closure instead of a value buys three things — the default is not computed on the happy path, it may consume what it captures, and on a `Result` it is handed the error, so falling back need not mean forgetting.

The first of those is the one everybody quotes. The third is the one that changes how you write a parser.

```rust
Option::unwrap_or_else(self, f: F) -> T   where F: FnOnce()  -> T
Result::unwrap_or_else(self, f: F) -> T   where F: FnOnce(E) -> T
```

Two signatures, not one. The `Option` closure takes nothing, because `None` carries nothing; the `Result` closure takes the error **by value**. Write the `Option` shape against a `Result` out of habit and the compiler is precise about it:

```text
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
  |
3 |     println!("{}", r.unwrap_or_else(|| 0));
  |                      ^^^^^^^^^^^^^^ -- takes 0 arguments
  |
help: consider changing the closure to take and ignore the expected argument
  |
3 |     println!("{}", r.unwrap_or_else(|_| 0));
```

Take that `help` at face value only if you mean it — see [the lab coat](#the-lab-coat-a-closure-that-ignores-its-argument) below.

---

## The payoff: fall back *and* keep the reason

A bad row in a ballot file must not stop the count. It also must not vanish. `unwrap_or_else` is the only member of the family that can do both, because the closure is handed the thing every other member drops:

```rust
let s = score(raw).unwrap_or_else(|why| {
    log.push(format!("row {}: {raw:?} — {why}", i + 1));
    0
});
```

```text
total    : 10   (3 of 6 rows counted as 0)
  row 3: "4x" — invalid digit found in string
  row 5: "" — cannot parse integer from empty string
  row 6: "12" — 12 is outside the 0-5 range
```

`unwrap_or(0)` produces the same total and no second list: same arithmetic, no audit. And note the log distinguishes three causes that all arrive as the number `0` — the distinction [`.ok()` throws away](../none_on_error/README.md) before you can act on it.

The stronger version of the same move is to stop defaulting altogether and return a value that *remembers* it was defaulted:

```rust
r.map(Score::Cast).unwrap_or_else(|why| Score::Defaulted { to: 0, why })
```

Now the total is still computable and "which rows did we trust?" is still answerable, which is the property a defaulted `0` destroys.

## Lazy, and free

The closure runs only on the sad path — six lookups with two misses build the default twice — and it costs nothing to have it there. A closure is a struct holding its captures, so one that captures nothing is **zero bytes** and compiles to the branch you would have written by hand:

```text
size_of_val(&|| 50)             = 0
size_of_val(&move || seats * 2) = 2
size_of_val(&move || fallback)  = 24  (a String moved in)
```

`FnOnce` is the loosest of the three call traits, which is why the last one is allowed: a closure that hands out its captured `String` can be called at most once, and `unwrap_or_else` calls it at most once. (A `Fn` bound would reject it, and that is the practical difference between the three traits — not "how the closure looks" but "how many times, and may it consume".)

When the fallback is just a constructor, pass the function itself — a function name is already something callable, so no `||` is needed:

```rust
maybe_name.unwrap_or_else(String::new)
maybe_list.unwrap_or_else(Vec::new)
maybe_list.unwrap_or_default()          // the same call, when it is exactly T::default
```

## The two `_else` methods do opposite things

This is the confusion worth spending a paragraph on, because both names contain `else` and they sit next to each other in the docs.

| Call | Returns | Use it to |
|---|---|---|
| `or_else(\|\| …)` | `Option<T>` / `Result<T, F>` | try **another source** and stay in the wrapper |
| `unwrap_or_else(\|\| …)` | `T` | **end** the chain with a concrete value |
| `map_or_else(d, f)` | `U` | handle both branches at once, one closure each |

They compose exactly as that reads — sources first, the settled value last:

```rust
let port = from_flag("port")
    .or_else(|| from_env("port"))      // still an Option: there may be more sources
    .unwrap_or_else(compiled_in_default); // now a u16: the chain is over
```

Reach for the wrong one and it is `error[E0308]: expected u16, found Option<u16>` — an easy fix, but only if you know which of the two you meant.

## The lab coat: a closure that ignores its argument

```rust
bad.unwrap_or(0)          // 0
bad.unwrap_or_else(|_| 0) // 0
```

Identical in every way that matters: same value, same discarded error, same cost. The second merely *looks* more careful, and a reviewer skimming for `unwrap_or` will not find it. If there is genuinely nothing to do with the error, write `unwrap_or(0)` and say so plainly. If there is something — a log line, a counter, a tag on the value — the closure is where it goes.

`|_|` in a fallback is worth treating as a small smell in review: it is the exact place a swallowed error is easiest to hide, precisely because the code looks like it is handling something.

## The closure that panics

`.unwrap_or_else(|| panic!("…"))` turns up in real code, and which of two things it is depends on the type you called it on.

**On an `Option` it is `expect` spelled long.** Same panic, more typing, and a reviewer grepping for the places this program can die will not find it. Write `expect`.

**On a `Result` it does one thing `expect` cannot.** The closure is handed the error, so the message is yours to compose — rather than a fixed string with the error's `Debug` form appended:

```rust
"4x".parse::<u32>().expect("unreadable score");

"4x".parse::<u32>()
    .unwrap_or_else(|e| panic!("ballot line 12: {e} — rerun the export"));
```

The two messages that come out:

```text
unreadable score: ParseIntError { kind: InvalidDigit }
ballot line 12: invalid digit found in string — rerun the export
```

`expect` puts `Debug` at the end; the closure puts `Display` in the middle of a sentence, beside the context — which row, which file, what to do — that makes the panic actionable rather than merely accurate. That is the whole trade, and the only reason to prefer the longer form.

(`panic!` is a macro, so it is always `|| panic!(…)`. Unlike `default_roster`, it cannot be passed bare.)

## Two things it does *not* fix

**It still takes `self`.** Laziness is about the default, not the receiver — the option is moved just as it is by `unwrap_or`, and the compiler will tell you so in some detail if the closure also wants to look at it:

```text
error[E0382]: borrow of moved value: `names`
note: `Option::<T>::unwrap_or_else` takes ownership of the receiver `self`, which moves `names`
help: consider calling `.as_ref()` to borrow the value's contents
```

**It is not available in a `const`.** Neither is `unwrap_or`; both are only *conditionally* const, still unstable — and `unwrap_or_else` additionally needs the closure itself to be const-callable. In a `const`, `unwrap()` and `expect()` are the whole family. ([The `const fn` in the signature](../unwrap_or/README.md#the-const-fn-in-the-signature-is-not-yet-true) has the details and the error text.)

## If you are coming from another language

- **Python** — `d.get(k, default)` has no lazy form at all; the idiomatic replacements are `d.get(k) or expensive()` (lazy, but it also fires on `0` and `""`) or an explicit `if k not in d`. The closer analogue to the *payoff* section is `try: … except ValueError as e: log.warning(...); score = 0`, which is the same two jobs — supply a value, keep the reason — spread over four lines and a block. The compiler's contribution is that the fallback is an *expression*: it cannot be accidentally skipped, reordered, or left after a `return`.
- **ABAP** — this is `READ TABLE … / IF sy-subrc <> 0. APPEND … TO lt_log. lv_score = 0. ENDIF.` written as one expression. The `sy-subrc` version works, and its weakness is structural: the check, the log, and the default are three separate statements that a later edit can separate further, and nothing fails if someone reads `lv_score` before the `IF`. `unwrap_or_else` makes the fallback part of the value's own construction, so there is no window in which the variable exists but has not been decided.
- **JavaScript** — `x ?? fallback()` is lazy and, unlike `||`, does not fire on `0` or `""`; what it has no equivalent of is the `Result` form, where the fallback is computed *from* the failure.

---

## Practice

**Fall back, but keep the reason.** Total a column of rows where a bad row counts as 0 — and the report still has to say which rows were bad and why. Use the fact that on a `Result`, the closure is handed the error.

Write it with `unwrap_or(0)` first and then try to produce the notes. This is the one job `unwrap_or`, `unwrap_or_default` and `expect` all structurally cannot do: by the time they act, the error is gone.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:unwrap_or_else_kata -->
*[`unwrap_or_else_kata.rs`](examples/unwrap_or_else_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: fall back without forgetting why you had to.
//!
//!   rustc --edition 2024 unwrap_or_else_kata.rs -o /tmp/uoek && /tmp/uoek

/// A tally that must produce a number for the report no matter what, but must
/// not pretend the bad row never happened.
fn counted(raw: &str, notes: &mut Vec<String>) -> u32 {
    raw.trim().parse::<u32>().unwrap_or_else(|e| {
        // On a Result, the closure is HANDED the error. This is the thing
        // `unwrap_or` and `expect` cannot do.
        notes.push(format!("row {raw:?} counted as 0 — {e}"));
        0
    })
}

fn main() {
    let rows = ["12", "7", "twelve", "", "3"];
    let mut notes = Vec::new();

    let total: u32 = rows.iter().map(|r| counted(r, &mut notes)).sum();

    println!("Total across {} rows -> {total}", rows.len());
    println!("\nAnd the report can still say what it did with the rest:");
    for n in &notes {
        println!("  {n}");
    }

    println!("\nThe same fallback written three ways, and what each forgets:");
    let bad = "twelve".parse::<u32>();
    println!("  unwrap_or(0)            -> {}   (the reason is gone)", bad.clone().unwrap_or(0));
    println!("  unwrap_or_default()     -> {}   (the reason is gone)", bad.clone().unwrap_or_default());
    println!(
        "  unwrap_or_else(|e| …)   -> {}   (kept: {})",
        bad.clone().unwrap_or_else(|_| 0),
        bad.unwrap_err()
    );
}
```
<!-- /source -->

<!-- output:unwrap_or_else_kata -->
*Verified output of [`unwrap_or_else_kata.rs`](examples/unwrap_or_else_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Total across 5 rows -> 22

And the report can still say what it did with the rest:
  row "twelve" counted as 0 — invalid digit found in string
  row "" counted as 0 — cannot parse integer from empty string

The same fallback written three ways, and what each forgets:
  unwrap_or(0)            -> 0   (the reason is gone)
  unwrap_or_default()     -> 0   (the reason is gone)
  unwrap_or_else(|e| …)   -> 0   (kept: invalid digit found in string)
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:unwrap_or_else -->
*Verified output of [`unwrap_or_else.rs`](examples/unwrap_or_else.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: Two shapes: no argument on Option, the error on Result
  None.unwrap_or_else(|| 50)                -> 50
  Err(e).unwrap_or_else(|e| e.len() as u16) -> 20
      The Option closure takes nothing, because None carries nothing.
      The Result closure takes the error BY VALUE — it is the only
      fallback in the family that ever sees it. Swap the two by habit
      and the compiler is specific: E0593, closure is expected to take
      1 argument, but it takes 0.

──── Step 2: The payoff: fall back AND keep the reason
  raw rows: ["5", "3", "4x", "2", "", "12"]
  total    : 10   (3 of 6 rows counted as 0)
    row 3: "4x" — invalid digit found in string
    row 5: "" — cannot parse integer from empty string
    row 6: "12" — 12 is outside the 0-5 range
      This is the shape to reach for when a bad row must not stop the
      count but must not vanish either. `unwrap_or(0)` would produce
      the same total and no second list — same arithmetic, no audit.

──── Step 3: Only built when it is needed — and it need not be a closure
  6 races, 2 without a configured roster -> default_roster() ran 2 times
  None.unwrap_or_else(String::new)    -> ""
  None.unwrap_or_else(Vec::new)       -> []
  None::<Vec<u8>>.unwrap_or_default() -> []
      `unwrap_or_else(Vec::new)` passes the function itself — no `||`,
      because a function name already IS something callable. And when
      the function you would name is exactly T::default, the shorter
      spelling of the same call is unwrap_or_default().

──── Step 4: The two `_else` methods do opposite things
  port          flag=None then env=Some(8080) -> or_else gives Some(8080), unwrap_or_else gives 8080
  metrics_port  flag=None then env=None -> or_else gives None, unwrap_or_else gives 9000
  Ok(443).map_or_else(|e| .., |v| ..)  -> listening on 443
  Err(e).map_or_else(|e| .., |v| ..)   -> failed: not a port
      or_else STAYS inside the wrapper: Option -> Option, so you can
      chain a second and third source. unwrap_or_else LEAVES it: the
      result is a plain u16 and the chain is over. Writing one where
      you meant the other is E0308 — expected `u16`, found `Option<u16>`.
      map_or_else handles both sides at once: one closure per branch.

──── Step 5: The lab coat: a closure that ignores its argument
  bad.clone().unwrap_or(0)          -> 0
  bad.clone().unwrap_or_else(|_| 0) -> 0
      Identical, in every way that matters: same value, same discarded
      error, same cost. The second one just looks more careful. If the
      underscore is there because there is genuinely nothing to do with
      the error, write unwrap_or(0) and say so plainly. If there IS —
      a log line, a counter, a tag on the value — the closure is the
      place for it, and that is Step 2.
  a fallback that remembers it was one:
    cast      counts 4
    defaulted counts 0, because row 3: '4x' is not a number
    cast      counts 5
    total 9, and the sum can still say which rows it trusted.

──── Step 6: A closure is a struct, and FnOnce means it may eat its captures
  size_of_val(&|| 50)             = 0
  size_of_val(&move || seats * 2) = 2
  size_of_val(&move || fallback)  = 24  (a String moved in)
  missing.unwrap_or_else(that closure) -> "(unnamed)"
  Some(7).unwrap_or_else(|| 50)   -> 7
      A closure capturing nothing is zero bytes and compiles to the
      same code as the branch you would have written by hand, so the
      laziness is free. FnOnce (rather than Fn) is what lets the last
      one hand its captured String straight out as the fallback: it is
      called at most once, so it is allowed to consume what it holds.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 17_Option_and_Result/unwrap_or_else/examples/unwrap_or_else.rs -o /tmp/uoe && /tmp/uoe
```

## See also

- [`unwrap_or`](../unwrap_or/README.md) — the eager sibling, and the three costs of a default you already have
- [Returning `None` on error](../none_on_error/README.md) — what `.ok()` discards, which is exactly what this closure is handed
- [What a panic costs](../what_a_panic_costs/README.md) — the other answer to "and if it is missing?", and when it is the right one
- [`Option` vs `Result`](../option_vs_result/README.md) — why only one of the two has an error to pass you
- [The three closure traits](../../23_Closures/three_closure_traits/README.md) — why this one takes an `FnOnce`, and what a bound of `Fn` would have refused
- [`Option::unwrap_or_else` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) · [`Result::unwrap_or_else` ↗](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else) · [`or_else` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.or_else) · [`map_or_else` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or_else)

## Po polsku

`unwrap_or_else` poznaje się zwykle jako „leniwą” wersję `unwrap_or`: wartość zapasowa (*fallback*) powstaje dopiero wtedy, gdy naprawdę jest potrzebna, i nic przy tym nie kosztuje — domknięcie (*closure*), które niczego nie przechwytuje, ma `size_of_val` równe 0 i kompiluje się do tego samego rozgałęzienia, które napisałoby się ręcznie. To prawda, ale najmniej ciekawa z trzech rzeczy, które ta metoda daje. Istotniejsze jest to, że **podpisy są dwa, nie jeden**: domknięcie dla `Option` nie przyjmuje niczego, bo `None` niczego nie niesie, a domknięcie dla `Result` dostaje **błąd przez wartość**. Odruchowe napisanie `|| 0` tam, gdzie jest `Result`, kończy się bardzo konkretnym `error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments` — a podpowiedź kompilatora, żeby zmienić to na `|_| 0`, warto przyjąć tylko wtedy, gdy naprawdę nie ma nic do zrobienia z tym błędem.

Z tego drugiego podpisu bierze się jedyna rzecz, której nie potrafi żadna inna metoda z tej rodziny: **podstawić wartość domyślną i jednocześnie zapamiętać, dlaczego trzeba było**. Zły wiersz w pliku z głosami nie może przerwać liczenia, ale nie może też zniknąć bez śladu — `unwrap_or(0)` da dokładnie tę samą sumę i żadnej listy uwag, bo w chwili, w której działa, błędu już nie ma. Polskie materiały o obsłudze błędów zwykle zatrzymują się na haśle „nie używaj `unwrap`, podstaw wartość domyślną”; wart zapamiętania jest krok dalej — wartość, która sama pamięta, że jest domyślna (`Score::Defaulted { to: 0, why }`), zamiast zera nieodróżnialnego od prawdziwego wyniku.

Na koniec dwie pułapki nazewnicze. `or_else` i `unwrap_or_else` brzmią niemal tak samo i leżą obok siebie w dokumentacji, a robią rzeczy przeciwne: `or_else` **zostaje** w opakowaniu (`Option` → `Option`, więc można dołożyć kolejne źródło), `unwrap_or_else` z niego **wychodzi** i kończy łańcuch konkretną wartością — pomylenie ich to `error[E0308]: expected u16, found Option<u16>`. I sytuacja odwrotna: `.unwrap_or_else(|_| 0)` na `Result` jest co do joty tym samym co `unwrap_or(0)` — ta sama wartość, ten sam wyrzucony błąd, ten sam koszt — tylko wygląda staranniej. To najwygodniejsze miejsce, żeby po cichu zgubić błąd, więc `|_|` w wartości zapasowej warto na przeglądzie kodu traktować jak drobny sygnał ostrzegawczy.

**Szukaj po polsku:** wartość domyślna · leniwe wyliczanie · domknięcia w Ruscie · obsługa błędów w Ruscie · `rust unwrap_or vs unwrap_or_else` · `rust E0593 closure is expected to take 1 argument`
