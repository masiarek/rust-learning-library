# The never type `!`

**Level:** 201 · working knowledge

**One line:** `!` is the type of an expression that does not finish — `panic!()`, `loop {}`, `return`, `process::exit` — and because no value of it can exist, it fits into a slot of *any* type, which is what lets a diverging arm sit in a `match` beside a `u32`.

```rust
fn give_up(reason: &str) -> ! {
    panic!("{reason}");
}

let n = 3u32;
let score: u32 = match n {
    0 => give_up("a zero score is not a score"),  // type `!`
    k => k,                                       // type u32
};
println!("{score}");   // 3
```

The two arms have different types and the `match` still type-checks. That is not a special case for `panic!`; it is the whole behaviour of `!`.

## Counting values puts it in order

| Type | Values it has | What a slot of that type means |
|---|---|---|
| `u8` | 256 | one of 256 numbers |
| `bool` | 2 | true or false |
| [`()`](../the_unit_type/README.md) | 1 | there is nothing to hand back |
| `!` | **0** | **control never gets here** |

`()` and `!` are the pair that get muddled, and the distance between them is one value. `()` is something you *have* — you can put it in a `Vec`, return it, match on it. `!` is something nobody can have, ever. Since a value of type `!` cannot be produced, a slot expecting one can never be wrong, so the compiler lets an expression of type `!` stand in for a value of any type at all. `()` gets no such licence, and that asymmetry is almost the only thing you need to remember:

```rust
let n = 3u32;
let a: u32 = match n { 0 => panic!("zero"), k => k };   // fine: `!` coerces
// let b: u32 = match n { 0 => println!("zero"), k => k };   // E0308
```

Uncomment the second and the compiler is precise about why:

```text title="Abridged — real rustc output for never_coercion.rs"
error[E0308]: mismatched types
 --> never_coercion.rs:4:33
  |
4 |     let b: u32 = match n { 0 => println!("zero"), k => k };
  |                                 ^^^^^^^^^^^^^^^^ expected `u32`, found `()`
```

`println!` finished, and handed back `()`. That is a value of the wrong type. `panic!` never finished, so there is nothing to be the wrong type.

## Four ways to produce one

| Expression | Why it has type `!` |
|---|---|
| [`panic!(..)`](../../25_Control_Flow/macros/README.md) | the value never arrives |
| `loop {}` with no `break` | control never leaves the loop |
| `return` · `break` · `continue` | control goes somewhere other than here |
| [`std::process::exit(..)` ↗](https://doc.rust-lang.org/std/process/fn.exit.html) | the process is gone |

`unreachable!()`, `todo!()` and `unimplemented!()` are all `panic!` underneath, so all three have type `!` too — which is why any of them can stand in for a value of any type while the function around it is still being written.

## The semicolon rule has an exception, and this is it

```rust
fn f() -> String { panic!(); }    // compiles
fn g() -> String { loop {} }      // compiles
```

[A trailing semicolon makes a block evaluate to `()`](../a_block_is_an_expression/README.md), and normally `fn f() -> String { ...; }` is `E0308`. Not here. A block whose control flow never reaches the end has type `!` regardless of punctuation, so the missing-return error you brace for never appears. Every diverging statement inside a block propagates outwards the same way.

## You can return `!`, and that is all you can do with it

`fn f() -> !` has been stable since 1.0. `!` written anywhere *else* in a type is still experimental:

```text title="Abridged — real rustc output for never_in_type_position.rs"
error[E0658]: the `!` type is experimental
 --> never_in_type_position.rs:1:22
  |
1 | fn f() -> Result<(), !> { Ok(()) }
  |                      ^
  |
  = note: see issue #35121 <https://github.com/rust-lang/rust/issues/35121> for more information
```

`fn f(x: !)` gets the same error. So `!` is a promise a signature may make about its own control flow, not yet a type you may store, nest or pass — and every `!` you will meet in real code is immediately after an `->`. (Verified against the compiler this library is pinned to, `rustc 1.98.0`; the tracking issue is [rust-lang/rust#35121 ↗](https://github.com/rust-lang/rust/issues/35121).)

## `Infallible` is the stable stand-in

When you want to say *this `Result` cannot be an `Err`*, the type to reach for is [`std::convert::Infallible` ↗](https://doc.rust-lang.org/std/convert/enum.Infallible.html) — an ordinary enum with **no variants**, which therefore has no values, exactly like `!`:

```rust
use std::convert::Infallible;

fn parse_score(raw: u8) -> Result<u8, Infallible> {
    Ok(raw.min(5))
}

let value = match parse_score(9) {
    Ok(v) => v,
    Err(e) => match e {},   // zero variants, so zero arms is exhaustive
};
println!("{value}");   // 5
```

`match e {}` is the part worth staring at. An [exhaustive match](../../13_Enums/what_an_enum_is/README.md) has to cover every variant; `Infallible` has none; so an empty `match` covers all of them and produces a value of whatever type the other arm did. That is how you open a cannot-fail `Result` without an [`unwrap`](../../17_Option_and_Result/unwrap_or/README.md) that a reader has to take on trust — the compiler proves the `Err` arm is dead rather than you asserting it.

`Infallible` is also the error type of the `TryFrom` impls that cannot fail, which is why it turns up in [conversion](../../29_Conversion/README.md) signatures you did not write.

## Where it is already working for you

- **`let else`** — [the `else` block must diverge](../../30_Pattern_Matching/let_else/README.md), and *diverge* is precisely *has type `!`*. That is the whole rule, stated in terms of this page.
- **`?`** — the early return in [the `?` operator](../../17_Option_and_Result/the_question_mark_operator/README.md) is a `return`, so the expression `?` expands to has an arm of type `!`.
- **`match` arms that bail** — `Err(e) => return Err(e)`, `None => continue`, `_ => unreachable!()`. Each is an arm that type-checks against its neighbours for this reason and no other.

## If you are coming from another language

**Python.** The counterpart is `typing.NoReturn` (spelled `typing.Never` since 3.11), and the difference is who enforces it. Annotate `def die(msg: str) -> NoReturn:` and mypy will narrow types after a call to it exactly as rustc does — but nothing at run time checks that the function really never returns, and an unannotated `def die()` that always raises gets no narrowing at all. In Rust the annotation is the *only* way to write the function, so the information cannot go missing. The habit that transfers is `sys.exit()` and `raise` in the middle of an expression-shaped `if`; the habit that does not is the `return None` you add at the end to keep a linter quiet, which in Rust would change the type from `!` to `Option<T>` and break every caller relying on the divergence.

**ABAP.** There is no type-level counterpart at all — `MESSAGE ... TYPE 'A'`, `LEAVE PROGRAM`, `RAISE EXCEPTION` and an endless `DO ... ENDDO` all end control flow, and none of them appear in a signature. The nearest familiar thing is the *discipline*: a `FORM` or method whose only exit is `RAISE EXCEPTION` still has to be given a `RETURNING` parameter or an `EXPORTING` one that is never filled, and every caller has to write the dead assignment anyway. Rust's `!` is that dead code deleted and replaced by a promise the compiler reads. Going the other way, ABAP's `sy-subrc` habit — check the return code or lose the failure — is what [`Result`](../../17_Option_and_Result/option_vs_result/README.md) makes unforgettable; `!` is the other half, for the case where there is no code to check because control is not coming back.

## The verified output

<!-- output:the_never_type -->
*Verified output of [`the_never_type.rs`](examples/the_never_type.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: Four stable ways to produce a `!`
  panic!(..)          the value never arrives
  loop {}             with no `break`, control never leaves
  return / break / continue    control goes somewhere else
  std::process::exit(..)       the process is gone
      Each is an EXPRESSION whose type is `!`. Not 'returns
      nothing' — that is `()`, and it is a value you have.

──── Step 2: `!` fits into every type; `()` fits into none
  match n { 0 => give_up(..), k => k }  =>  3
      The two arms have types `!` and `u32`, and the match
      still type-checks as u32 — `!` coerces into anything.
      Swap the diverging arm for `println!("zero")` and it stops:
        error[E0308]: mismatched types
           |     0 => println!("zero"),
           |          ^^^^^^^^^^^^^^^^ expected `u32`, found `()`
      That error is the whole lesson. `()` is a value of the wrong
      type. `!` is no value at all, so there is nothing to mismatch.

──── Step 3: A semicolon does NOT turn `!` into `()`
  fn f() -> String { panic!(); }    compiles
  fn g() -> String { loop {} }      compiles
      The rule you already know — a trailing semicolon makes a
      block evaluate to `()` — has an exception, and this is it.
      A block whose control flow never reaches the end has type
      `!`, punctuation or not. So the missing-return error you
      expect here never appears.

──── Step 4: You can RETURN `!`, but you cannot write it anywhere else
  fn f() -> ! { .. }          stable since 1.0
  fn f() -> Result<(), !> {}  NOT stable:
        error[E0658]: the `!` type is experimental
        note: see issue #35121 for more information
  fn f(x: !) {}               the same error
      So `!` is a promise a signature may make about its own
      control flow, and not yet a type you may store, nest or
      pass. Every `!` you meet in real code is after an `->`.

──── Step 5: `Infallible` is the stable stand-in
  fn parse_score(u8) -> Result<u8, Infallible>
  parse_score(9) = 5
      `Infallible` is `enum Infallible {}` — an ordinary enum with
      no variants, so it has no values either, and `match e {}`
      is EXHAUSTIVE with zero arms. That is how you unwrap a
      cannot-fail Result without an `unwrap` that could panic.
      It is what `Result<T, !>` would mean, spelled in stable Rust.

──── Step 6: Where `!` is already working for you
  give_up(..) -> !   ran, and unwound: caught = true
  unreachable!()  todo!()  unimplemented!()   all have type `!`,
      which is why any of them may stand in for a value of any
      type while you are still writing the function around it.
  let Some(x) = opt else { return; };
      `let else` requires the else block to diverge — and
      'diverge' is exactly 'has type `!`'.
```
<!-- /output -->

## See also

- [The unit type `()`](../the_unit_type/README.md) — the one-value type `!` is confused with, and the census this page finishes
- [A block is an expression](../a_block_is_an_expression/README.md) — the semicolon rule this page is the exception to
- [`let else`](../../30_Pattern_Matching/let_else/README.md) — "the `else` must diverge", restated
- [The `loop` keyword](../../25_Control_Flow/the_loop_keyword/README.md) — where `loop {}` gets its type, and `break v` gives it one instead
- [`expect`](../../17_Option_and_Result/expect/README.md) — the everyday diverging call, and the message that says why you believed it could not happen
- [Never type ↗](https://doc.rust-lang.org/reference/types/never.html) · [`Infallible` ↗](https://doc.rust-lang.org/std/convert/enum.Infallible.html) · [tracking issue #35121 ↗](https://github.com/rust-lang/rust/issues/35121)

## Po polsku

`!` to **typ pusty** (*never type*, „typ nigdy") — typ, który nie ma **żadnej** wartości. Warto ustawić go obok `()`, bo te dwa są najczęściej mylone, a dzieli je dokładnie jedna wartość: `()` ma jedną, `!` nie ma ani jednej. `()` to coś, co **masz** — można to zwrócić, włożyć do wektora, dopasować wzorcem. Wartości typu `!` nie da się wytworzyć nigdy, i stąd bierze się cała jego użyteczność: skoro nikt nie może takiej wartości podać, to miejsce oczekujące jej nie może się pomylić — więc kompilator pozwala wyrażeniu typu `!` wystąpić tam, gdzie spodziewany jest **dowolny** typ. `()` takiego przywileju nie ma i to jest praktycznie jedyna rzecz do zapamiętania.

Typ `!` mają wyrażenia, które **nie kończą się** normalnie: `panic!(..)`, `loop {}` bez `break`, `return`, `break`, `continue` oraz `std::process::exit(..)`. Dlatego ramię `match`a z `panic!()` stoi spokojnie obok ramienia zwracającego `u32` — a gdy zamienić je na `println!("zero")`, kompilator natychmiast protestuje: `expected u32, found ()`. Różnica nie jest kosmetyczna. `println!` **się skończył** i oddał wartość niewłaściwego typu; `panic!` nie skończył się wcale, więc nie ma czego porównywać.

Dwie rzeczy zaskakują przy pierwszym spotkaniu. Po pierwsze, reguła o średniku ma tutaj wyjątek: `fn f() -> String { panic!(); }` kompiluje się mimo średnika, bo blok, do którego końca sterowanie nigdy nie dochodzi, ma typ `!` niezależnie od interpunkcji. Po drugie, `!` wolno napisać **wyłącznie** po `->`; `Result<(), !>` czy `fn f(x: !)` to wciąż `error[E0658]: the !  type is experimental`. Stabilnym zamiennikiem jest `std::convert::Infallible` — zwykłe wyliczenie **bez wariantów**, więc `match e {}` z zerem ramion jest wyczerpujące i pozwala otworzyć `Result`, który nie może zawieść, bez `unwrap`, któremu czytelnik musiałby uwierzyć na słowo.

**Szukaj po polsku:** typ pusty · typ nigdy · rozbieżność sterowania · `rust never type` · `rust Infallible` · `rust E0658 never type is experimental`
