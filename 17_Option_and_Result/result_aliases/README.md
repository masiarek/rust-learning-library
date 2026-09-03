# The `Result` you are reading is probably an alias

**Level:** 201 · working knowledge

**One line:** `Result<T, E>` has two parameters, but the signatures you meet show one or none — the `E` has been pinned by a type alias, and expanding it is how you find out what can actually go wrong.

Here is a signature from the standard library:

```rust
fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>
```

One parameter. Yet `Result` takes two, and the missing one is the interesting one — it is the only part of the type that says *what failure looks like*. Nothing has been hidden from you; one line in `std::io` pinned it:

```rust
pub type Result<T> = std::result::Result<T, std::io::Error>;
```

So `io::Result<usize>` **is** `Result<usize, io::Error>`. Not a similar type, not a wrapper — the same type, spelled shorter. A type alias creates no new type and no conversion; it is a nickname the compiler expands before it does anything else. Which is why the two annotations in Step 1 below are interchangeable with no cast at all.

---

## Why anyone does this

Because the `E` repeats. A module with fifteen fallible functions would otherwise write `Result<_, row::Error>` fifteen times, and the reader would learn nothing new on the fourteenth. Pinning it once puts the interesting parameter — the `T` — where the eye already is.

The cost is one hop: a newcomer reading `io::Result<usize>` cannot see the error type at all. That is the trade, and it is worth naming out loud, because it is the reason this page exists. **An alias does not answer "what can go wrong?" — it moves the answer one click away.**

## The three you will meet in `std`

| Alias | Expands to | Note |
|---|---|---|
| [`io::Result<T>` ↗](https://doc.rust-lang.org/std/io/type.Result.html) | `Result<T, io::Error>` | Everywhere in file, socket, and stdin code |
| [`fmt::Result` ↗](https://doc.rust-lang.org/std/fmt/type.Result.html) | `Result<(), fmt::Error>` | **No parameters at all** — both slots pinned |
| [`thread::Result<T>` ↗](https://doc.rust-lang.org/std/thread/type.Result.html) | `Result<T, Box<dyn Any + Send>>` | What a joined thread or `catch_unwind` hands back |

`fmt::Result` is the one that looks least like a `Result` and teaches the most. It has no parameters because a `Display` impl has nothing to return on success and only one way to fail — so `T` is `()` and `E` is `fmt::Error`, and every `fmt` impl you will ever write ends in `Ok(())`.

## Writing your own

The convention is one alias per crate or module, declared next to the error it pins:

```rust
#[derive(Debug)]
pub enum Error { Empty, NotANumber(String), OutOfRange { got: u32, max: u32 } }

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse(line: &str) -> Result<Vec<u32>> { … }
```

Two details that trip people up:

- **Write the right-hand side as `std::result::Result`.** You are defining something called `Result` in this scope; if you refer to the prelude's `Result` by bare name on the right, you are referring to the alias you are in the middle of defining.
- **Callers outside the module still have the normal one.** They write `row::Result<T>` (or import it), and their own `Result<T, E>` keeps working. You have shadowed the name inside your module, not replaced it globally.

## `Ok(())` — the same trick on the other slot

`Result<(), E>` is the mirror image: **it worked, and there is nothing to hand back.** The unit type `()` is Rust's "no value" — zero bytes, one possible value — so `Ok(())` reads as *succeeded, no news*. You will type it constantly: every `Display` impl, every function that writes somewhere, and `main` itself.

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let row = parse("5,9,0")?;   // `?` works in main, because main returns Result
    println!("{row:?}");
    Ok(())
}
```

That signature is worth having in your fingers: it is the shortest way to use `?` at the top level. But **one trap comes with it**, and it surprises everyone once. When `main` returns `Err`, the runtime prints `Error: ` followed by the error's **`Debug`** form and exits with status 1 — not the `Display` form you carefully wrote. Given the error in Step 4 below, the terminal shows:

```text
Error: OutOfRange { got: 9, max: 5 }
```

not `score 9 is above the 5 cap`. (Verified by running it; it is `Termination` for `Result`, which is [documented to use `Debug` ↗](https://doc.rust-lang.org/std/process/trait.Termination.html).) If a human is meant to read that message, print it yourself and call [`std::process::exit` ↗](https://doc.rust-lang.org/std/process/fn.exit.html), or use a wrapper type whose `Debug` delegates to `Display` — which is exactly what `anyhow::Error` does. That trap is one of *four* default paths that print `Debug` without asking; [Debug and Display](../../15_First_Programs/debug_vs_display/README.md) counts them, and its kata prices both of those two fixes against each other.

## `Infallible` — an `E` with no values at all

At the far end of the `E` slot sits [`std::convert::Infallible` ↗](https://doc.rust-lang.org/std/convert/enum.Infallible.html), an enum with **no variants**. You cannot construct one, so `Err(_)` cannot be built, so a `Result<T, Infallible>` is always `Ok`.

It exists because traits force the shape on you. `FromStr` requires an `Err` type even for `String`, whose parse cannot fail; `TryFrom<u32> for u64` requires one even though every `u32` fits. Both name `Infallible` and the signature stops lying.

And it costs nothing. Because the compiler knows the `Err` variant is uninhabited, it drops the tag: `size_of::<Result<u64, Infallible>>()` is **8**, the same as a bare `u64`, where `Result<u64, io::Error>` is 16. Step 5 prints all three.

## Reading the `E` slot back

Once you expand the alias, the second parameter tells you how much the author knew about their own failures — and how much you are allowed to do about them:

| `E` is… | What it tells the caller |
|---|---|
| `Infallible` | Cannot fail. The `Result` is here to satisfy a trait. |
| A concrete type (`io::Error`) | One failure type; interrogate it (`.kind()`) rather than matching variants |
| Your own enum | A fixed menu — the caller can `match` and act differently per variant |
| `Box<dyn Error>` / `anyhow::Error` | Anything at all; nobody downstream is expected to match |

The last row is the design decision, not just a spelling: **libraries name their errors so callers can match on them; applications erase them because nothing downstream will.** An `anyhow::Result<T>` in a *published library* is the common mistake — it looks tidy in the signature and leaves every user of the crate with an error they can only print.

### If you are coming from another language

- **Python.** An alias is `IOResult = Result[T, IOError]` in a `.pyi` — pure documentation, no runtime object. The difference is who reads it: in Python the type is advice, so a wrong alias is a lint; here the compiler expands it before checking anything, so a mismatch is a build failure and `?` still needs a real `From` impl to cross between error types.
- **ABAP.** Closest to a `TYPES: ty_result TYPE …` in an interface — one name for a structure everyone returns. What changes is `Ok(())`: the ABAP habit of returning a filled structure whose fields you must inspect becomes a type that is *either* the value *or* the reason, never both, and never a struct you forgot to check. `Result<(), E>` is the subroutine that only ever needed `sy-subrc`.

---

## Practice

**Expand the alias.** Define your own `type Result<T> = std::result::Result<T, RowError>;`, write two functions against it — one returning a value, one returning `Ok(())` — and print what each does on good and bad input.

Then read the `E` back the way you would in an unfamiliar crate: follow the alias to its definition and list the error's variants. That list is the real answer to *what can go wrong here*, and it is the thing the one-parameter signature hid. Add an `Infallible` function to see the other extreme — an error type with no values, so the failing arm cannot be reached.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:result_aliases_kata -->
*[`result_aliases_kata.rs`](examples/result_aliases_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: expand the alias and find out what can go wrong.
//!
//!   rustc --edition 2024 result_aliases_kata.rs -o /tmp/rak && /tmp/rak

use std::convert::Infallible;
use std::fmt;

#[derive(Debug)]
pub enum RowError {
    NoRows,
    BadRow { row: usize },
}

impl fmt::Display for RowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RowError::NoRows => write!(f, "no rows were supplied"),
            RowError::BadRow { row } => write!(f, "row {row} is not a number"),
        }
    }
}

/// The alias: one parameter instead of two, with the E pinned once.
pub type Result<T> = std::result::Result<T, RowError>;

/// Reads as `-> Result<u32>`, and is really `-> std::result::Result<u32, RowError>`.
fn total(rows: &[&str]) -> Result<u32> {
    if rows.is_empty() {
        return Err(RowError::NoRows);
    }
    let mut sum = 0;
    for (i, r) in rows.iter().enumerate() {
        sum += r.trim().parse::<u32>().map_err(|_| RowError::BadRow { row: i })?;
    }
    Ok(sum)
}

/// `Ok(())` — the same trick in the other slot. Nothing to return, but it can fail.
fn check(rows: &[&str]) -> Result<()> {
    total(rows)?;
    Ok(())
}

/// An E with no values at all: this function cannot fail, and the type says so.
fn always_ok(n: u32) -> std::result::Result<u32, Infallible> {
    Ok(n * 2)
}

fn main() {
    println!("The alias hides the E; the behaviour is unchanged:");
    for rows in [vec!["5", "3"], vec![], vec!["5", "x"]] {
        match total(&rows) {
            Ok(n) => println!("  {rows:?} -> {n}"),
            Err(e) => println!("  {rows:?} -> {e}"),
        }
    }

    println!("\nOk(()) — the success that carries nothing:");
    println!("  check(['5','3']) -> {:?}", check(&["5", "3"]));
    println!("  check([])        -> {:?}", check(&[]));

    println!("\nInfallible — an error type with no values:");
    println!("  always_ok(21) -> {:?}", always_ok(21));
    println!("      There is no way to build the Err, so the match arm for it");
    println!("      cannot be reached — which is the compiler's own reason for");
    println!("      letting `let Ok(n) = always_ok(21);` be irrefutable.");

    println!("\nReading the E back is a matter of following one line:");
    println!("  `type Result<T> = std::result::Result<T, RowError>;`");
    println!("  so `-> Result<u32>` fails with RowError, whose variants are the");
    println!("  actual list of things that can go wrong: NoRows, BadRow.");
    println!("  std::io::Result<T>, std::fmt::Result and ParseResult are the same");
    println!("  trick, and the E is the thing you were looking for in each.");
}
```
<!-- /source -->

<!-- output:result_aliases_kata -->
*Verified output of [`result_aliases_kata.rs`](examples/result_aliases_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The alias hides the E; the behaviour is unchanged:
  ["5", "3"] -> 8
  [] -> no rows were supplied
  ["5", "x"] -> row 1 is not a number

Ok(()) — the success that carries nothing:
  check(['5','3']) -> Ok(())
  check([])        -> Err(NoRows)

Infallible — an error type with no values:
  always_ok(21) -> Ok(42)
      There is no way to build the Err, so the match arm for it
      cannot be reached — which is the compiler's own reason for
      letting `let Ok(n) = always_ok(21);` be irrefutable.

Reading the E back is a matter of following one line:
  `type Result<T> = std::result::Result<T, RowError>;`
  so `-> Result<u32>` fails with RowError, whose variants are the
  actual list of things that can go wrong: NoRows, BadRow.
  std::io::Result<T>, std::fmt::Result and ParseResult are the same
  trick, and the E is the thing you were looking for in each.
```
<!-- /output -->

</details>

---

## The verified output

Every line below came from actually compiling and running [`result_aliases.rs`](examples/result_aliases.rs) — regenerated by `tools/run_examples.py` and checked in CI, so it cannot drift away from the code.

<!-- output:result_aliases -->
*Verified output of [`result_aliases.rs`](examples/result_aliases.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: io::Result<T> looks wrong until you expand it
  io::Result<()>              -> Ok(())
  Result<(), std::io::Error>  -> Ok(())
  the two annotations are interchangeable -> Ok(())
  buffer now holds: "Ada,Ben,Cara\n5,2,0\n"
      type Result<T> = Result<T, Error>;   <- one line in std::io

──── Step 2: The aliases you will actually meet
  std::io::Result<T>      = Result<T, std::io::Error>
  std::fmt::Result        = Result<(), std::fmt::Error>   <- NO parameters at all
  std::thread::Result<T>  = Result<T, Box<dyn Any + Send>>
  a Display impl returning fmt::Result -> [5, 2, 0]
      fmt::Result pins BOTH slots, which is why `Ok(())` ends every fmt impl.

──── Step 3: Writing your own alias
  parse("5,2,0") -> Ok([5, 2, 0])
  parse("") -> Err: the input line was empty
  parse("5,x,0") -> Err: "x" is not a score
  parse("5,9,0") -> Err: score 9 is above the 5 cap
      pub type Result<T> = std::result::Result<T, Error>;
      Inside the module `Result` now means YOUR Result. Outside it is
      `row::Result<T>`, and the prelude's two-parameter one is untouched.

──── Step 4: Ok(()) — the other slot emptied out
  a function that only succeeds or fails -> Ok(())
  Display of the error  -> score 9 is above the 5 cap
  Debug of the error    -> OutOfRange { got: 9, max: 5 }
      A failing `fn main() -> Result<(), E>` prints the DEBUG form after
      "Error: " and exits 1. Your careful Display impl is not used.

──── Step 5: Infallible — the error type that has no values
  String::from_str("Ada")  -> Ok("Ada")    (FromStr::Err  = Infallible)
  u64::try_from(3u32)      -> Ok(3)        (TryFrom::Error = Infallible)
  size_of  Infallible=0  ()=0
  size_of  u64=8  Result<u64, Infallible>=8  Result<u64, io::Error>=16
      Infallible is an enum with NO variants, so Err(_) cannot be built.
      The compiler knows it, drops the tag, and Result<u64, Infallible>
      costs exactly what a u64 costs. A promise with no runtime price.

──── Step 6: Reading the E slot back
  Result<T, Infallible              > cannot fail; the Result is there for a trait's sake
  Result<T, io::Error               > one concrete failure, ask it for .kind()
  Result<T, RowError (your enum)    > a fixed menu; the caller can match on it
  Result<T, Box<dyn Error>          > anything at all; nobody downstream will match
      This is the only question the second parameter answers, and an
      alias does not remove the answer — it just moves it one hop away.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 17_Option_and_Result/result_aliases/examples/result_aliases.rs -o /tmp/ra && /tmp/ra
```

---

## Traps

- **Passing two arguments to a one-parameter alias.** Once `type Result<T>` is in scope, `Result<u32, ParseIntError>` no longer compiles — the error is `type alias takes 1 generic argument but 2 generic arguments were supplied`, which reads like nonsense until you remember which `Result` you are looking at. Fix by dropping the second argument, or by naming `std::result::Result` explicitly.
- **Assuming an alias is a new type.** It is not, so you cannot `impl` a trait on it, and it gives you no extra type safety. If you want a distinct type — one the compiler will keep apart from its inner type — you want a [newtype](../../16_Structs/newtype_score/README.md) (a one-field `struct`), not an alias. That is the whole of the choice: an alias is a nickname the compiler expands and forgets, a newtype is a type it will hold you to.
- **`main`'s `Debug` output.** Covered above, and worth repeating because it is the one that gets shipped: your `Display` impl is not what the user sees.
- **`anyhow::Result` in a library.** Convenient for you, a dead end for your callers. Name the errors in a library; erase them in a binary.
- **Reading `io::Result` as "a different, simpler `Result`".** It has every method the two-parameter one has, because it *is* the two-parameter one. If a method seems missing, check [`std::result::Result` ↗](https://doc.rust-lang.org/std/result/enum.Result.html) — that is where the list lives.

## See also

- [`Option` vs `Result`](../option_vs_result/README.md) — the prior page: which of the two you want, and everything `?` does
- [A score is not a number: the newtype](../../16_Structs/newtype_score/README.md) — the other answer to "I want a name for this type", and the one that actually changes what compiles
- [matklad — Study of `std::io::Error` ↗](https://matklad.github.io/2020/10/15/study-of-std-io-error.html) — a slow read of the single error type behind `io::Result`, and the best short lesson available on designing an `E`
- [The Rust Book, ch. 9 — Error Handling ↗](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [`std::result` ↗](https://doc.rust-lang.org/std/result/) — the method list the aliases inherit

## Po polsku

`Result<T, E>` ma dwa parametry, ale w podpisach ze standardowej biblioteki widać jeden albo żaden — bo `E` zostało przypięte aliasem typu (*type alias*). `io::Result<usize>` **to jest** `Result<usize, io::Error>` — nie typ podobny i nie opakowanie, tylko ten sam typ zapisany krócej. Alias nie tworzy nowego typu i nie wymaga żadnej konwersji — kompilator rozwija go, zanim cokolwiek sprawdzi. Ma to jedną cenę, o której warto pamiętać przy czytaniu cudzego kodu: drugi parametr jest jedyną częścią typu mówiącą, **co może pójść nie tak**, a alias przesuwa tę odpowiedź o jedno kliknięcie dalej. Rozwinięcie aliasu nie jest więc ciekawostką, tylko normalnym krokiem w czytaniu obcego crate'a.

Przy definiowaniu własnego aliasu czekają dwie pułapki naraz. Po prawej stronie trzeba napisać pełną ścieżkę `std::result::Result`, bo sama nazwa `Result` odnosi się w tym miejscu już do aliasu, który właśnie definiujesz. Druga to przesłanianie (*shadowing*) na poziomie typów: wewnątrz twojego modułu `Result` znaczy odtąd twój jednoparametrowy `Result`, więc `Result<u32, ParseIntError>` przestaje się kompilować, a komunikat — `type alias takes 1 generic argument but 2 generic arguments were supplied` — brzmi jak bzdura, dopóki nie przypomnisz sobie, na który `Result` właśnie patrzysz. Na zewnątrz modułu nic się nie zmienia: tam nadal jest zwykły dwuparametrowy `Result`, a twój nazywa się `row::Result<T>`.

Drugą stroną tej samej sztuczki jest `Ok(())` — „udało się i nie ma czego oddać”. `fmt::Result` przypina **oba** sloty naraz (`Result<(), fmt::Error>`), dlatego każda implementacja `Display` kończy się na `Ok(())`. I tu czai się niespodzianka, która łapie każdego raz: kiedy `fn main() -> Result<(), E>` zwróci `Err`, środowisko uruchomieniowe wypisuje `Error: ` i formę **`Debug`**, a nie `Display`. Na ekranie pojawia się `Error: OutOfRange { got: 9, max: 5 }`, a nie starannie napisane `score 9 is above the 5 cap`. Jeśli ten komunikat ma czytać człowiek, wypisz go sam i zakończ przez `std::process::exit`.

Na drugim krańcu slotu `E` stoi `Infallible` — wyliczenie (*enum*) **bez wariantów**, czyli typ, którego nie da się skonstruować. Skoro nie da się zbudować `Err(_)`, taki `Result` jest zawsze `Ok`; istnieje po to, żeby dopełnić kształt narzucony przez cechę (*trait*), która żąda typu błędu tam, gdzie błędu nie ma — `FromStr` dla `String`, `TryFrom<u32>` dla `u64`. I nic nie kosztuje: `size_of::<Result<u64, Infallible>>()` to **8**, tyle co samo `u64`, podczas gdy `Result<u64, io::Error>` waży 16 — kompilator wie, że wariant `Err` jest pusty, i wyrzuca znacznik.

**Szukaj po polsku:** alias typu · obsługa błędów w Ruscie · `rust type alias Result` · `rust io::Result error type` · `rust main returns Result prints Debug`
