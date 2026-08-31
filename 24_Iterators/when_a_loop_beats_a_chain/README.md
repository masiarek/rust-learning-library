# When a `for` loop beats a chain

**Level:** 201 · working knowledge

**One line:** The fluent style wins for transform-and-keep and loses for everything with real control flow in it — an error that names its row, a work list that grows while you drain it, a break out of two levels, or one pass answering three questions.

Start with the case the chain wins, because it is the common one:

```rust
let shouted: Vec<String> = names.iter().filter(|n| n.len() > 3).map(|n| n.to_uppercase()).collect();
```

One line against five, no `mut`, no empty `Vec` that exists for two statements, and nothing to misread. When the body is *transform some items and keep them*, the chain is simply better, and most bodies are.

The rest of this page is the minority — and it is a bigger minority than the fluent style suggests.

## 1. When the error has to say *where*

```rust
// Chain: correct, and the error has lost the row number.
rows.iter().map(|s| s.parse::<i32>().map_err(|e| e.to_string())).sum()

// Loop: `?` returns from THIS function, so the context is still in scope.
for (n, s) in rows.iter().enumerate() {
    let value = s.parse::<i32>().map_err(|e| format!("row {}: {e} (found {s:?})", n + 1))?;
    total += value;
}
```

```text
chain -> Err("invalid digit found in string")
loop  -> Err("row 3: invalid digit found in string (found \"no\")")
```

Both stop at the same row; only one of them can still say which. The mechanism is worth knowing because it is the same one behind half the "why can't I use `?` here" questions: **`?` inside a closure returns from the closure**, not from the function around it. A chain that wants the index has to thread it through every stage; `enumerate` plus a loop says it once, and the `?` lands where you meant it.

## 2. When the work list grows while you drain it

```rust
let mut stack = vec![1u32];
while let Some(n) = stack.pop() {
    visited.push(n);
    if n < 5 { stack.push(n * 2); stack.push(n * 2 + 1); }
}
```

No iterator can express this. An iterator **borrows the collection for as long as it lives**, so pushing to it mid-iteration is `E0502` — while `stack.pop()` takes one item and hands the borrow straight back, leaving you free to push. That is why every worklist, breadth-first search and graph walk in Rust is a loop, and why reaching for `for x in &stack` there is a borrow error rather than a style choice.

## 3. When you need out of two levels at once

`break 'outer` leaves a nested loop from the inside. There is no adapter for that: the chain equivalent is to flatten and search, which works —

```rust
grid.iter().enumerate().find_map(|(r, row)| row.iter().position(|c| *c == 5).map(|c| (r, c)))
```

— and is arguably nicer here. It stops being nicer the moment the inner loop also has to *update* something outside itself, because `find_map`'s closure cannot both search and mutate. That is the line: a labelled break is a fair fight until the loop body does two jobs.

## 4. When one pass answers several questions

A tuple accumulator can carry three answers through a `fold`:

```rust
scores.iter().fold((0, 0, i32::MIN), |(sum, zeros, best), s| {
    (sum + s, zeros + i32::from(*s == 0), best.max(*s))
})
```

Identical answers to the six-line loop, one expression instead of six statements — and most readers get the loop faster. Two fields is usually still fine (min-and-max in one pass is a good `fold`); three is where the destructuring costs more attention than it saves. This is a judgement, not a rule, and the honest version of it is: **write the fold, read it back the next day, and keep whichever you understood without stopping.**

## What is *not* a reason to choose either

**Speed.** An adapter is a struct with an inlined `next`, and the optimizer flattens a chain into the loop you would have written. If a chain is measurably slower, the cause is an allocation (an intermediate `collect`) or a clone, not the chain. Do not rewrite a readable chain as a loop for performance without a measurement — see [what the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md).

The corollary matters more: because both compile the same, the choice really is about which one a reader understands, and you are allowed to decide that on taste.

## If you are coming from another language

- **Python.** The same argument runs there, with one extra reason on the loop's side that Rust does not have: a comprehension cannot contain a statement, so anything with a `try`, a `break` with cleanup, or two accumulators becomes a loop by force. In Rust a closure *can* hold statements, so the pressure is weaker and the fluent style stretches further — which is exactly why the "when does it stop paying" question needs asking here and answers itself there. What transfers unchanged is the worklist case: mutating a list while iterating it is undefined-ish behaviour in Python (`RuntimeError` for a dict, silently skipped elements for a list) and a compile error in Rust, so both languages push you to `while stack:` / `while let Some(x) = stack.pop()`.
- **ABAP.** Everything here is a `LOOP`, so the interesting direction is the reverse: which of your loops would be better as a chain. The test is the same — if the body is *transform and collect*, the 7.40 comprehension (`VALUE ty( FOR wa IN itab WHERE ( … ) ( … ) )`) is the chain and it reads better; if the body has a `CHECK` that needs to report the row, an `EXIT` from two levels, or three accumulators, the `LOOP` is right and the comprehension will fight you. The `sy-tabix`/`enumerate` pairing is worth noticing too: ABAP hands you the row number for free in every loop, which is the one ergonomic advantage over a Rust chain that has to ask for `enumerate`.
- **JavaScript.** `for...of` versus `.map().filter()` is the same trade with one difference in the fine print: `break` works in `for...of` and there is no way out of a `forEach`, so the JS advice ("use `for...of` when you might need to stop") maps onto Rust's short-circuiting consumers instead — `find`, `any`, `take_while` — which do stop, and are the reason the Rust chain loses this round far less often.

---

## The verified output

<!-- output:when_a_loop_beats_a_chain -->
*Verified output of [`when_a_loop_beats_a_chain.rs`](examples/when_a_loop_beats_a_chain.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. When the error needs to say WHERE
   chain -> Err("invalid digit found in string")
   loop  -> Err("row 3: invalid digit found in string (found \"no\")")
   Both stop at the same row. Only the loop still knows which row it
   was, because `?` inside a closure returns from the CLOSURE — so a
   chain that wants the index has to carry it through every stage.
   `enumerate` plus a loop says it once.

2. When the work list grows while you drain it
   visited [1, 3, 7, 6, 2, 5, 4, 9, 8]
   No iterator can express this: an iterator borrows the collection
   for as long as it lives, so pushing while iterating is E0502.
   `while let Some(x) = stack.pop()` takes one item and gives the
   borrow straight back, which is why every worklist, BFS and
   graph walk in Rust is a loop.

3. When you need out of TWO levels at once
   labelled break -> Some((1, 1))
   find_map       -> Some((1, 1))
   Same answer, and here the chain is arguably nicer. It stops being
   nicer the moment the inner loop also has to update something
   outside it — `find_map`'s closure cannot both search and mutate.

4. When one pass has to answer several questions
   loop  -> sum 15, zeros 1, best 5
   fold  -> sum 15, zeros 1, best 5
   Identical answers. The fold is one expression and the loop is six
   lines, and most readers get the loop faster — a tuple accumulator
   with three fields is where the fluent style stops paying.

5. And the other direction, honestly
   chain -> ["BERNADETTE", "CARA"]
   loop  -> ["BERNADETTE", "CARA"]
   One line against five, no state to misread, and the `mut` and the
   empty Vec never exist. When the body is transform-and-keep, the
   chain wins outright — which is most of the time.

6. What is NOT a reason to pick either
   Speed. Both compile to the same loop: the adapters are structs
   with inlined `next` methods and the optimizer flattens them. If a
   chain is slower it is because it allocates (an intermediate
   collect) or clones, not because it is a chain.
```
<!-- /output -->

---

## See also

- [Adapters by job](../adapters_by_job/README.md) — the chain vocabulary this page is deciding against
- [`fold` and `reduce`](../fold_and_reduce/README.md) — the tuple accumulator in case 4, and when it is worth it
- [Iterators are lazy](../iterators_are_lazy/README.md) — why a chain does not walk the data once per adapter
- [Borrowing](../../18_Ownership/borrowing/README.md) — the rule that makes case 2 a compile error rather than a choice
- [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) — why speed is not the tiebreaker
- [`while let`](../../17_Option_and_Result/while_let/README.md) — the loop form case 2 is written with

## Sources

The Book's [iterators chapter ↗](https://doc.rust-lang.org/book/ch13-04-performance.html) makes the performance half of this argument with a benchmark; the rest is a judgement call, and this page tries to say which parts are which.

## Po polsku

Zacznijmy od przypadku, w którym łańcuch adapterów wygrywa, bo to przypadek najczęstszy: *przekształć i zbierz*. `names.iter().filter(…).map(…).collect()` to jedna linia zamiast pięciu, bez `mut` i bez pustego wektora, który istnieje przez dwie instrukcje. Ta strona jest o mniejszości — większej, niż sugeruje popularność stylu łańcuchowego. Cztery sytuacje, w których zwykła pętla `for` jest po prostu lepsza:

- **Błąd musi powiedzieć, w którym wierszu.** `?` postawiony w domknięciu (*closure*) wraca z domknięcia, a nie z otaczającej funkcji — to ta sama przyczyna, co w połowie pytań „dlaczego nie mogę tu użyć `?`”. Łańcuch potrafi tylko `Err("invalid digit found in string")`; pętla z `enumerate` mówi indeks raz i zwraca `Err("row 3: … (found \"no\")")`.
- **Lista pracy rośnie, kiedy ją opróżniasz.** Iterator pożycza kolekcję na cały swój czas życia, więc dopisanie czegoś w trakcie iterowania to `E0502`. `while let Some(n) = stack.pop()` bierze jeden element i natychmiast oddaje pożyczenie, dzięki czemu można dołożyć następne. Dlatego każde przeszukiwanie grafu i każda kolejka zadań w Ruscie jest pętlą — to nie kwestia gustu, tylko borrow checkera.
- **Wyjście z dwóch poziomów naraz.** `break 'outer` nie ma odpowiednika wśród adapterów. `find_map` bywa tu nawet ładniejszy, dopóki wewnętrzna pętla ma *tylko* szukać — jego domknięcie nie potrafi równocześnie szukać i zmieniać czegoś na zewnątrz siebie.
- **Jeden przebieg, kilka odpowiedzi.** `fold` z krotką w akumulatorze uniesie trzy wyniki naraz. Dwa pola (minimum i maksimum w jednym przebiegu) to wciąż dobry `fold`; przy trzech rozbieranie krotki kosztuje więcej uwagi, niż oszczędza.

Ostatni punkt nie jest regułą, tylko oceną, i uczciwa wersja tej oceny brzmi: napisz `fold`, przeczytaj go następnego dnia i zostaw tę wersję, którą zrozumiałeś bez zatrzymania się.

Czego na tej liście nie ma: **szybkości**. Adapter to struktura ze wstawianą w miejscu wywołania metodą `next`, a optymalizator spłaszcza cały łańcuch do dokładnie tej pętli, którą i tak byśmy napisali. Jeśli łańcuch mierzalnie zwalnia, winna jest alokacja (pośredni `collect`) albo klonowanie, nie sam łańcuch. Warto to podkreślić, bo polski czytelnik przychodzi tu zwykle od pythonowych wyrażeń listowych, gdzie granica przebiega gdzie indziej: w Pythonie wyrażenie listowe nie zniesie żadnej instrukcji, więc `try` czy `break` wypycha do pętli siłą. W Ruscie domknięcie instrukcje zniesie, styl łańcuchowy rozciąga się więc znacznie dalej — i właśnie dlatego pytanie, kiedy przestaje się opłacać, trzeba zadać sobie świadomie.

**Szukaj po polsku:** pętla czy łańcuch adapterów · `?` w domknięciu · `rust question mark inside closure` · `rust modify vec while iterating E0502` · `rust labeled break outer loop`
