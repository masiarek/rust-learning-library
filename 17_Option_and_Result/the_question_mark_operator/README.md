# The `?` operator

**Level:** 201 · working knowledge

**One line:** `?` is three statements in one character — unwrap the happy value, **return** on the sad one, and convert the error type on the way out — and it is that third job, done silently through `From`, that people are surprised by later.

```rust
fn score(row: &str) -> Result<u32, std::num::ParseIntError> {
    let Some((_name, count)) = row.split_once('=') else {
        return Ok(0);
    };
    Ok(count.parse::<u32>()?)
}

fn main() {
    println!("{:?}", score("Ada=5"));   // Ok(5)
}
```

## What it expands to

```rust
fn expanded(count: &str) -> Result<u32, std::num::ParseIntError> {
    // this whole `match` is what `count.parse::<u32>()?` becomes
    let v = match count.parse::<u32>() {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    };
    Ok(v)
}

fn main() {
    println!("{:?}", expanded("12"));   // Ok(12)
}
```

Read the sad arm carefully, because two separate things happen in it. The `return` leaves **your function**, not the expression — `?` is a control-flow operator wearing punctuation, and a line with three `?` in it has three exits. And `From::from(e)` converts the error into whatever your signature promised, choosing the impl by the return type.

Nothing about that is special-cased for `Result`. `?` is defined by the [`Try` ↗](https://doc.rust-lang.org/std/ops/trait.Try.html) trait, and `Option` implements it too — which is why the same character works there, with the conversion step absent because `None` carries nothing to convert.

## The conversion is the half people miss

This is the whole reason `?` scales past one error type. Give your error an `impl From<TheirError>`, and every `?` in the module starts converting into it without a word at the call site:

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum TallyError {
    Malformed(String),
    NotANumber(ParseIntError),
}

impl From<ParseIntError> for TallyError {
    fn from(e: ParseIntError) -> Self {
        TallyError::NotANumber(e)
    }
}

fn strict_score(row: &str) -> Result<u32, TallyError> {
    let (_name, count) = row
        .split_once('=')
        .ok_or_else(|| TallyError::Malformed(row.to_string()))?;
    Ok(count.parse::<u32>()?)          // ParseIntError in, TallyError out
}

fn main() {
    // the payoff of a named error type: the caller can ask which one it was
    for row in ["Ada=5", "Cara=oops", "Dev"] {
        match strict_score(row) {
            Ok(n) => println!("{row}: {n}"),                          // Ada=5: 5
            Err(TallyError::NotANumber(e)) => println!("{row}: {e}"), // Cara=oops: invalid digit found in string
            Err(TallyError::Malformed(r)) => println!("{row}: no '=' in {r:?}"), // Dev: no '=' in "Dev"
        }
    }
}
```

The last line of `strict_score` returns a `ParseIntError` and the function is declared to return `TallyError`. Nobody wrote a conversion there; the `From` impl was found by type. That is the mechanism behind `thiserror`'s `#[from]` attribute and behind `anyhow`'s catch-all — both of them generate or provide the `From` impls that `?` then finds.

Note the first `?` in that function too: it is applied to an `Option`, and `ok_or_else` is what made it a `Result` first. That pairing — [a transform to change the shape, then `?` to leave](../transforms_instead_of_match/README.md) — is most of what idiomatic error plumbing looks like.

## `?` on an `Option` means something else

Same punctuation, different early exit — it returns `None`:

```rust
fn initial(row: &str) -> Option<char> {
    let (name, _) = row.split_once('=')?;   // returns None from `initial`
    name.chars().next()
}

fn main() {
    println!("{:?}", initial("Ada=5"));   // Some('A')
    println!("{:?}", initial("Dev"));     // None
}
```

You cannot mix the two in one function, and the compiler is unusually helpful about it:

```text title="Abridged — real rustc output for mixing.rs, without the file-and-line header"
error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
  |
1 | fn initial(row: &str) -> Result<char, String> {
  | --------------------------------------------- this function returns a `Result`
2 |     let (name, _) = row.split_once('=')?;
  |                                        ^ use `.ok_or(...)?` to provide an error compatible with `Result<char, String>`
```

*"Use `.ok_or(...)?`"* is the fix and also the reason the rule exists: there is no error value to invent. Rust will not guess one, because the guess would be the most important word in your error message.

## It needs a function that can receive it

`?` returns from the enclosing function, so that function has to have somewhere to put the error:

```text title="Abridged — real rustc output for in_main.rs, without the file-and-line header"
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
  |
1 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
2 |     let n: u32 = "12".parse()?;
  |                              ^ cannot use the `?` operator in a function that returns `()`
help: consider adding return type
  |
1 ~ fn main() -> Result<(), Box<dyn std::error::Error>> {
```

Taking that suggestion is fine and common — see [`main` can return a `Result`](../../02_Errors/main_returns_result/README.md) for what the runtime then prints, which is the `Debug` form and not the `Display` sentence you wrote. Inside a closure the same rule applies to the closure, not to the function around it, and that is a real trip-up: a `?` inside `.map(|x| …)` returns from the *closure*, so the closure must itself return a `Result`.

## `Box<dyn Error>`: one return type that accepts every `?`

Every type implementing `Error` converts into `Box<dyn Error>`, so a function returning one accepts a `?` on anything:

```rust
use std::error::Error;

fn total(rows: &[&str]) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;
    for row in rows {
        let (_name, count) = row.split_once('=').ok_or("row has no '='")?;
        sum += count.parse::<u32>()?;          // a different error type, same `?`
    }
    Ok(sum)
}

fn main() {
    println!("{:?}", total(&["Ada=5", "Ben=2"]).map_err(|e| e.to_string()));   // Ok(7)
    println!("{:?}", total(&["Dev"]).map_err(|e| e.to_string()));              // Err("row has no '='")
}
```

Convenient at the top of a program and lossy in a library: the caller receives something printable and cannot match on which failure it was. The [`anyhow` vs `thiserror`](../../02_Errors/thiserror_vs_anyhow/README.md) split is exactly this decision, made deliberately — a binary can afford the box, a library owes its callers a type.

## What `?` is not

- **Not `unwrap`.** It never panics. It returns, and the caller decides. Reaching for `?` instead of `unwrap` is the single highest-value habit in this section — see [`unwrap` is a TODO](../../02_Errors/unwrap_is_a_todo/README.md).
- **Not a `catch`.** A panic inside the call still unwinds straight past it; `?` only sees values the callee chose to return.
- **Not free of design.** Every `?` is a decision that *this* function does not handle that error and the caller must. A function whose body is all `?` has made that decision several times without saying so, which is right at a boundary and wrong in the middle of your domain logic.

## If you are coming from another language

**Python.** `?` is closest to letting an exception propagate — you do nothing, and the failure leaves. The two differences both cut in Rust's favour and are worth stating exactly:

```python
def score(row):
    name, count = row.split("=")   # ValueError leaves here, invisibly
    return int(count)              # ValueError leaves here too, invisibly
```

Every line in Python is potentially an exit; nothing marks which. In Rust the `?` marks them, so an exit is one character but never an *invisible* one, and a line without `?` cannot leave early. The second difference is the signature: `score`'s Python type tells you nothing about failure, while `Result<u32, TallyError>` lists it — which is what makes an exhaustive `match` at the caller possible at all.

The Python habit that transfers badly is the bare `except:` at the top of a script. Its Rust spelling is `Box<dyn Error>`, and it has the same property: it makes the program run and makes the failures indistinguishable.

**ABAP.** The nearest construct is class-based exceptions, and it maps closely — `RAISING cx_tally_error` in a signature is the `Result`'s error type, and an un-caught exception propagates the way `?` does:

```abap
METHODS score IMPORTING iv_row TYPE string
              RETURNING VALUE(rv_score) TYPE i
              RAISING   cx_tally_error.
```

Three things line up and one does not. `RAISING` is a checked, declared part of the signature, exactly like `E` in `Result<T, E>` — ABAP is unusual among the languages a Rust newcomer arrives from in already having this, so trust the instinct. `CX_ROOT` is `Box<dyn Error>`, with the same trade. And `CATCH cx_tally_error INTO DATA(lx)` is the `match` on `Err(e)`.

What does not line up is the older half of ABAP: `sy-subrc` and `MESSAGE ... RAISING` propagate nothing, so a caller three levels up cannot see the failure at all without every level in between forwarding it by hand. `?` is that forwarding, written once per call rather than once per level, and the compiler refuses to let a level forget.

## The verified output

<!-- output:the_question_mark_operator -->
*Verified output of [`the_question_mark_operator.rs`](examples/the_question_mark_operator.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The same function, written twice
   Ada=5        by match Ok(5)   with ? Ok(5)
   Ben=2        by match Ok(2)   with ? Ok(2)
   Cara=oops    by match Err("err")   with ? Err("err")
   Dev          by match Ok(0)   with ? Ok(0)
   Identical answers. What `?` removed was nine lines of arms that
   did nothing but hand the value on or hand the error back.

2. What `?` expands to
   count.parse::<u32>()?
   ==  match count.parse::<u32>() {
           Ok(v)  => v,
           Err(e) => return Err(From::from(e)),
       }
   Three things, not one: unwrap the happy path, RETURN on the sad
   one, and convert the error while leaving.

3. The conversion is the half people miss
   Ada=5        -> 5
   Ben=2        -> 2
   Cara=oops    -> count is not a number: invalid digit found in string
   Dev          -> row has no '=': "Dev"
   `strict_score` returns TallyError, but its last `?` was applied to
   a ParseIntError, and nobody wrote a conversion at that spot — the
   `impl From<ParseIntError> for TallyError` did it, silently.

4. `?` works on Option too, and means something different
   initial("Ada=5") = Some('A')
   initial("Ben=2") = Some('B')
   initial("Cara=oops") = Some('C')
   initial("Dev") = None
   Here `?` returns None early. Same punctuation, same shape, but
   there is no conversion step: `None` carries nothing to convert.
   You cannot mix them in one function — `?` on an Option inside a
   fn returning Result is E0277, not an automatic Ok/Err guess.

5. Box<dyn Error>: one return type that accepts every `?`
   total(all four rows)   = Err("count is not a number: invalid digit found in string")
   total(the good two)    = Ok(7)
   Every error type implementing Error converts into Box<dyn Error>,
   so `?` accepts all of them. Convenient at the top of a program,
   and lossy in a library: the caller gets a message, not a type.

6. What `?` is not
   It is not `unwrap`. It never panics; it returns.
   It does not catch anything — a panic inside the call still unwinds.
   And it is not free of design: every `?` you write is a decision that
   THIS function does not handle that error, and the caller must.
```
<!-- /output -->

## See also

- [Transforms instead of `match`](../transforms_instead_of_match/README.md) — `ok_or` and `map_err`, the two methods that make a value `?`-able
- [`main` can return a `Result`](../../02_Errors/main_returns_result/README.md) — taking the compiler's suggestion, and what gets printed
- [`unwrap` is a TODO](../../02_Errors/unwrap_is_a_todo/README.md) — the habit `?` replaces
- [`anyhow` vs `thiserror`](../../02_Errors/thiserror_vs_anyhow/README.md) — who should own the error type, and why a library answers differently
- [`From` and `Into`](../../29_Conversion/from_and_into/README.md) — the conversion `?` performs, in its own right
- [The `Result` you are reading is probably an alias](../result_aliases/README.md) — expanding `io::Result<T>` to see what `?` is actually converting
- [The Reference: the question mark operator ↗](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator) · [`std::ops::Try` ↗](https://doc.rust-lang.org/std/ops/trait.Try.html)

## Po polsku

Po polsku `?` bywa nazywany operatorem propagacji błędu i ta nazwa jest dobra, o ile pamięta się, czym propagacja tutaj **nie** jest: `?` to `return`, a nie `throw`. Nie ma rozwijania stosu, nie ma łapania — linia z trzema `?` ma po prostu trzy wyjścia z funkcji, za to wyjścia widoczne gołym okiem. Błąd nie „leci” sam trzy poziomy w górę, jak wyjątek w Javie czy Pythonie: każdy poziom musi postawić u siebie własny `?`, tyle że kosztuje to jeden znak, a kompilator nie pozwoli o nim zapomnieć. W domknięciu (*closure*) ta sama reguła liczy się względem samego domknięcia — `?` w środku `.map(|x| …)` wychodzi z **domknięcia**, a nie z funkcji dookoła, i to jest tu najczęstsza pomyłka.

Druga połowa operatora bywa przeoczana: w rozwinięciu stoi `Err(e) => return Err(From::from(e))`. Konwersja typu błędu dzieje się po cichu, a odpowiednia implementacja `From` dobierana jest po **typie zwracanym przez funkcję**, mimo że w miejscu, gdzie stoi `?`, nikt żadnej konwersji nie napisał. Dlatego jeden `impl From<ParseIntError> for TallyError` sprawia, że każdy `?` w module zaczyna oddawać `TallyError`. Na tym mechanizmie stoją `thiserror` (atrybut `#[from]`) i `anyhow` — one tylko generują albo dostarczają te implementacje `From`, których `?` potem szuka. Sam operator nie jest zresztą wbudowany na stałe w `Result`: definiuje go cecha (*trait*) `Try`, którą implementuje też `Option` — tam kroku konwersji po prostu nie ma, bo `None` nie niesie niczego do przekonwertowania.

Warto od razu rozpoznawać dwa komunikaty `E0277`. Pierwszy — *the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`* — pojawia się przy mieszaniu obu typów w jednej funkcji, a podpowiedź *use `.ok_or(...)?`* jest i lekarstwem, i wyjaśnieniem reguły: kompilator nie wymyśli za ciebie treści błędu, bo to właśnie ona byłaby najważniejszym słowem w komunikacie dla użytkownika. Drugi dotyczy `?` w funkcji zwracającej `()`, najczęściej w `main`, i sam proponuje `fn main() -> Result<(), Box<dyn std::error::Error>>`. `Box<dyn Error>` przyjmuje `?` po dowolnym typie błędu — wygodnie w programie, stratnie w bibliotece, bo kod wywołujący dostaje tekst zamiast typu, po którym dałoby się zrobić `match`. I jeszcze jedno dla czytających starsze materiały: w miejscu dzisiejszego `?` zobaczysz tam makro `try!` — to jego poprzednik sprzed Rusta 1.13, a od edycji 2018 `try` jest słowem zastrzeżonym i makro trzeba by zapisać jako `r#try!`. W nowym kodzie nie ma po nie powodu sięgać.

**Szukaj po polsku:** operator `?` w Ruscie · propagacja błędów · obsługa błędów w Ruscie · `rust ? operator From conversion` · `rust E0277 ? operator Option Result`
