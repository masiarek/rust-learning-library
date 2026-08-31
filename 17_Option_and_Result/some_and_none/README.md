# `Some` and `None`: reading an `Option`

**Level:** 101 · for newcomers

**One line:** An `Option<i32>` is a variable that is honest about being empty, and `match` is how you ask which of its two shapes you are holding.

This is the first thing anyone does with an `Option`, and it is worth doing slowly, because every convenience method you meet later — `unwrap_or`, `?`, `map` — is ordinary library code written on top of the `match` below.

---

## The whole type

`Option` is not built into the language. It is an ordinary enum, and you could have written it yourself:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Two shapes. `Some` carries a value of whatever type `T` is; `None` carries nothing. A variable of type `Option<i32>` is therefore *either* an `i32` *or* nothing at all — and, crucially, the type says so out loud, so nobody downstream can forget.

## The kata

Declare a favourite number that may not exist, then handle both shapes:

```rust
let favnum: Option<i32>;

// Uncomment one of these:
// favnum = Some(3);
favnum = None;

match favnum {
    Some(n) => println!("Your favourite number is {n}, good choice"),
    None => println!("You don't have a favourite number... what?!"),
}
```

Three things in that small program are worth pulling apart.

### 1. `match` must cover both shapes, and the compiler checks

Delete the `None` arm and the program does not build:

```text
error[E0004]: non-exhaustive patterns: `None` not covered
 --> e3.rs:3:11
  |
3 |     match favnum {
  |           ^^^^^^ pattern `None` not covered
```

That single error is the entire safety story of `Option`. Not *"remember to check for null"* — a rule you follow — but *"this does not compile"*, which is a rule you cannot fail to follow. Everything else on this page is a convenience over it, and when a convenience does not fit, `match` is always waiting.

When you only care about one shape, [`if let`](../if_let/README.md) is the same code with the arm you did not care about deleted.

### 2. `Some` and `Option::Some` are the same name

Both variants live in the prelude, so the `Option::` prefix is optional and almost nobody writes it. Use it only when another name in scope collides.

The type annotation, on the other hand, is not decoration. `let favnum: Option<i32>;` tells the compiler what `T` is, and without it a bare `None` has nothing to go on:

```text
error[E0282]: type annotations needed for `Option<_>`
 --> e1.rs:2:9
  |
2 |     let favnum = None;
  |         ^^^^^^   ---- type must be known at this point
```

`let favnum = Some(3);` needs no annotation, because the `3` settles it.

### 3. Declaring without assigning is fine

`let favnum: Option<i32>;` with the assignment on a later line is normal Rust, and the compiler proves the variable is set on every path before you read it. That is a general rule, not an `Option` one — and it is why `Option` is usually the *wrong* tool for "I don't have a value yet"; see [initial values](../initial_values/README.md).

## Getting the value out

`unwrap_or` is the shortcut the kata ends on: *give me the value, or this default if there isn't one.*

```rust
favnum.unwrap_or(42)        // 42 when None
favnum.unwrap_or_default()  // T::default() — 0 for i32, "" for String
favnum.unwrap()             // the value, or a panic
```

`unwrap` is the one to be suspicious of, because it is not a getter — it is the same `match` with the `None` arm filled in for you as `panic!`, which is a claim about your program rather than a way of reading a value. Write `.expect("why this cannot be None")` instead wherever you would have written `.unwrap()` in code that ships: the two do the identical thing, and only one of them leaves behind the sentence you will want to read when it fires — see the traps at the end of [`Option` vs `Result`](../option_vs_result/README.md).

## The trap: `Some(0)` is not `None`

This is the part that catches people arriving from almost any other language.

```rust
Some(0).unwrap_or(42)   // -> 0   the voter answered, and the answer was zero
None.unwrap_or(42)      // -> 42  the voter never answered
```

An `Option` distinguishes *"the value is zero"* from *"there is no value"*. Most languages do not, and the shorthand everyone reaches for quietly collapses the two: Python's `favnum or 42` answers `42` for both, because `0` is falsy; ABAP's `IS INITIAL` says the same thing, because an integer that was never set and an integer set to `0` are the same bit pattern.

The distinction is not academic. In the [star-voting-library ↗](https://github.com/masiarek/star-voting-library) a ballot that scores a candidate **0** and a ballot that leaves them **blank** tabulate identically — both count as zero — but they mean opposite things about the voter, and any report that wants to say *"how many people declined to score anyone"* needs the two kept apart. `Option<Score>` is that distinction, made a property of the type rather than of the reader's memory.

## The trap: `match favnum` moves the value

The kata ends by using `favnum` again after the match:

```rust
println!("Your fav num is {}", favnum.unwrap_or(42));
```

That compiles — but only because `i32` is `Copy`, so the match copied the value instead of taking it. Write the identical code with a `String` inside and it stops compiling:

```text
error[E0382]: use of partially moved value: `favname`
 --> e2.rs:7:20
  |
4 |         Some(n) => println!("{}", n),
  |              - value partially moved here
...
7 |     println!("{}", favname.unwrap_or("nobody".to_string()));
  |                    ^^^^^^^ value used here after partial move
  |
  = note: partial move occurs because value has type `String`, which does not implement the `Copy` trait
```

`match` on a value consumes it. The fix is to match on a reference — `match &favname` — which borrows instead, and the arm binds `&String` rather than `String`. `favname.as_ref()` does the same thing as a method call (`Option<String>` → `Option<&String>`), and `as_deref()` goes one better (`Option<&str>`), which is what you want when the next thing you do is `unwrap_or("nobody")`.

Worth naming the shape of this bug: **a `Copy` type hides it**. The kata works, you learn the pattern, and the compiler only tells you the rule the first time you try it on a `String` — by which point it looks like a new and arbitrary complaint rather than the thing that was always true.

---

## If you are coming from another language

- **Python.** `Option` is the `None` you already use, with the check moved from your discipline into the compiler: you cannot reach the value without saying what happens when it is missing. `unwrap_or(42)` is `favnum if favnum is not None else 42` — deliberately *not* `favnum or 42`, which also swallows `0`, `""` and `[]`. Python 3.10's `match` looks like Rust's and is not the same instrument: it falls through silently when no case matches, so a forgotten case is a runtime surprise rather than a build failure.
- **ABAP.** There is no null for elementary types — every variable already holds its initial value, so `IS INITIAL` cannot tell *"quantity 0"* from *"quantity never supplied"*, and that ambiguity has to be carried in a flag field or in a comment. `Option<T>` is exactly that missing distinction, made part of the type. The closest thing ABAP has is an optional parameter with `IS SUPPLIED`, and the comparison is instructive: `IS SUPPLIED` works only for parameters, only inside the method, and only if you remember to ask. `Some` and `None` travel with the value everywhere it goes, and the compiler asks on your behalf.

---

## Practice

**A favourite number.** Declare a variable `favnum` that is either a number or nothing at all, assign one of the two shapes to it, and print a different sentence for each using a `match`. Then print it a second way, falling back to `42` when there is no number.

Try it before opening this. Two things are worth getting wrong on purpose first: delete the `None` arm and read what the compiler says, and change the type to `Option<String>` and see which line stops compiling. (That second one is [K2](../shadowing_and_unwrap/README.md#practice)'s whole subject, error text and all, if you want it worked through.)

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:some_and_none_kata -->
*[`some_and_none_kata.rs`](examples/some_and_none_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a favourite number that may not exist.
//!
//! Rust's standard library defines:
//!
//!     enum Option<T> {
//!         Some(T),
//!         None,
//!     }
//!
//! So `favnum` below is *either* an i32 *or* nothing, and the `match` has to say
//! what happens in both cases — that is the exercise.
//!
//!   rustc --edition 2024 some_and_none_kata.rs -o /tmp/sank && /tmp/sank

fn main() {
    // Declared here, assigned below: legal, and the compiler proves it is set
    // before the match reads it.
    let favnum: Option<i32>;

    // Swap these two lines to see the other half.
    favnum = Some(3);
    // favnum = None;

    match favnum {
        Some(n) => println!("Your favourite number is {n}, good choice"),
        None => println!("You don't have a favourite number... what?!"),
    }

    // `unwrap_or` reads it a second way: the value, or 42 if there isn't one.
    // This line compiles only because i32 is Copy — the match above did not
    // consume `favnum`. With an Option<String> it would not.
    println!("Your favourite number, or a stand-in: {}", favnum.unwrap_or(42));

    // The same program with the other assignment, so one run shows both shapes.
    println!();
    describe(None);
}

fn describe(favnum: Option<i32>) {
    match favnum {
        Some(n) => println!("Your favourite number is {n}, good choice"),
        None => println!("You don't have a favourite number... what?!"),
    }
    println!("Your favourite number, or a stand-in: {}", favnum.unwrap_or(42));
}
```
<!-- /source -->

<!-- output:some_and_none_kata -->
*Verified output of [`some_and_none_kata.rs`](examples/some_and_none_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Your favourite number is 3, good choice
Your favourite number, or a stand-in: 3

You don't have a favourite number... what?!
Your favourite number, or a stand-in: 42
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:some_and_none -->
*Verified output of [`some_and_none.rs`](examples/some_and_none.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Some and None: reading an Option

──── Step 1: One variable, two possible shapes, one match
  Some(3) -> your favourite number is 3, good choice
  None    -> you don't have a favourite number... what?!
      Same variable, same match, both shapes handled. Delete an arm and
      the program stops compiling — 'I forgot the None case' is not a
      bug you can ship.

──── Step 2: `Some(3)` and `Option::Some(3)` are the same thing
  Some(3) == Option::Some(3) -> true
  None    == Option::None    -> true
      Both variants are in the prelude, so the `Option::` prefix is only
      needed when some other name in scope collides. Write the short form.
  The annotation is not decoration: `let x = None;` on its own does not
      compile, because nothing in that line says what T is.

──── Step 3: Getting the value out
  Some(3).unwrap_or(42)          -> 3
  None.unwrap_or(42)             -> 42
  None.unwrap_or_default()       -> 0
  None.unwrap_or_else(|| 7 * 6)  -> 42
  None.unwrap()                  -> panicked (caught here only to keep this demo running)
      `unwrap` and `expect` are the same call; only `expect` leaves behind
      the sentence you will want six months from now. Neither belongs in
      code that ships unless you can write down why None is impossible.

──── Step 4: `Some(0)` is not `None` — the trap Python and ABAP both set
  Some(0).unwrap_or(42) -> 0
  None.unwrap_or(42)    -> 42
  Some(0).is_some() -> true
  Some(0) == None   -> false
      Python's `favnum or 42` answers 42 for BOTH of these, because 0 is
      falsy. ABAP says the same with IS INITIAL: 0 and 'never set' are one
      value. Rust keeps them apart, which is the difference between a
      ballot that scored a candidate 0 and one that left them blank.

──── Step 5: The trap: `match favnum` MOVES the value, and i32 hides it
  match fav_num  -> Some(3)
  fav_num is still usable afterwards -> 3
      i32 is Copy, so the match copied it. Nothing moved, nothing broke.
  match &fav_name -> Some(Ada)
  fav_name is still usable afterwards -> Ada
      String is NOT Copy. Written the first way — `match fav_name` — the
      match would move it and the next line would not compile. Matching on
      a reference borrows instead, which is why `&` is there.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 17_Option_and_Result/some_and_none/examples/some_and_none.rs -o /tmp/san && /tmp/san
```

---

## Cheat sheet

| What you want | Write |
|---|---|
| Handle both shapes | `match favnum { Some(n) => …, None => … }` |
| Handle only the present case | [`if let Some(n) = favnum { … }`](../if_let/README.md) |
| The value, or a default | `.unwrap_or(42)` |
| The value, or a default that costs something | `.unwrap_or_else(\|\| …)` — and never `.unwrap_or(expensive())`, which computes it either way |
| The value, or `0` / `""` / empty | `.unwrap_or_default()` |
| Just "is there one?" | `.is_some()` / `.is_none()` |
| The value, and absence is a bug | `.expect("why None cannot happen here")` |
| The value, in a test | `.unwrap()` |
| To read it without consuming it | `match &favnum`, `.as_ref()`, `.as_deref()` |

## See also

- [`Option` vs `Result`](../option_vs_result/README.md) — the next question: when is `None` not enough of an answer?
- [`if let`](../if_let/README.md) — the one-arm shortcut, and what the compiler stops checking when you take it
- [Partial functions](../partial_functions/README.md) — why the standard library hands you `Option`s in the first place
- [Initial values](../initial_values/README.md) — the job `Option` looks right for and usually is not
- [`Option` is a one-item collection](../option_as_collection/README.md) — `match` is not the only way to look inside
- [`std::option` ↗](https://doc.rust-lang.org/std/option/) — the full method list; one slow read is worth more than any tutorial
- [The Rust Book, ch. 6 — Enums and Pattern Matching ↗](https://doc.rust-lang.org/book/ch06-00-enums.html)

## Po polsku

`Option<T>` nie jest wbudowany w język — to zwyczajne wyliczenie (*enum*) o dwóch wariantach: `Some(T)` niesie wartość, `None` nie niesie niczego. Tour of Rust nazywa je „Opcją”, ale w kodzie zawsze pisze się `Option`, `Some` i `None`, i wszystkie trzy nazwy siedzą w preludium, więc przedrostek `Option::` jest zbędny. Cała gwarancja bezpieczeństwa mieści się w jednym komunikacie: usuń ramię `None` z `match`, a program się nie zbuduje — `error[E0004]: non-exhaustive patterns: None not covered`. To jest różnica między zasadą „pamiętaj, żeby sprawdzić, czy nie ma `null`-a”, której da się nie dotrzymać, a regułą, której złamać nie sposób, bo kompilator nie przepuści kodu. Adnotacja typu też nie jest ozdobą: `let favnum = None;` nie ma z czego wywnioskować `T` i kończy się na `error[E0282]: type annotations needed`, podczas gdy `let favnum = Some(3);` rozstrzyga sprawę samą trójką.

Najważniejsza pułapka tej strony brzmi: **`Some(0)` to nie `None`**. Kto pracuje na co dzień w SQL-u albo w ABAP-ie, ma tu przewagę i utrudnienie naraz — przewagę, bo różnica między `NULL` a zerem jest znajoma (`IS NULL` to co innego niż `= 0`), a utrudnienie, bo w językach, w których na co dzień się pracuje, skrót „albo wartość, albo domyślna” zwykle zjada zero po cichu: pythonowe `favnum or 42` odpowiada `42` w obu przypadkach, bo `0` jest fałszywe, a ABAP-owe `IS INITIAL` nie odróżni zera od wartości nigdy nieustawionej. `Option` przenosi tę różnicę z bazy danych do zwykłej zmiennej i to bez logiki trójwartościowej — `Some(0) == None` daje po prostu `false`, a `Some(0).is_some()` daje `true`. W bibliotece wyborczej, z której pochodzą przykłady, jest to różnica między kartą, na której wyborca wystawił kandydatowi **0**, a kartą, na której zostawił pustą kratkę: obie liczą się do wyniku tak samo, a mówią o wyborcy rzeczy przeciwne.

Druga pułapka jest własnościowa i na `i32` w ogóle nie widać: `match favnum` **konsumuje** wartość, a przykład z ulubioną liczbą działa wyłącznie dlatego, że `i32` jest `Copy`, więc `match` skopiował ją i zostawił oryginał na miejscu. Napisz to samo dla `Option<String>`, a linia po `match`u przestanie się kompilować — `error[E0382]: use of partially moved value`. Lekarstwem jest dopasowanie do referencji: `match &favname` pożycza zamiast przejmować, a ramię wiąże `&String`; te same skutki mają `.as_ref()` (`Option<String>` → `Option<&String>`) i `.as_deref()` (`Option<&str>`), z czego to ostatnie jest zwykle tym, czego chcesz przed `unwrap_or("nobody")`. Warto nazwać kształt tego błędu: typ `Copy` go ukrywa, więc kompilator upomni się dopiero przy pierwszej wartości typu `String` i zabrzmi to jak nowy kaprys, a nie jak reguła, która obowiązywała od początku. I jeszcze drobiazg z tej samej okolicy: `.unwrap()` i `.expect("dlaczego None jest tu niemożliwe")` robią dokładnie to samo, tylko po tym drugim zostaje zdanie, które będziesz chciał przeczytać, kiedy program naprawdę spanikuje.

**Szukaj po polsku:** typ opcjonalny · dopasowanie wzorców · obsługa braku wartości · `rust Option Some None` · `rust E0004 non-exhaustive patterns` · `rust match on reference E0382`
