# Printing borrows: why `println!` never moves

**Level:** 101 · for newcomers

**One line:** `println!` is handed `&s`, never `s`, and so is every other formatting macro — so a `String` can be printed as often as you like, and the move error only comes from something that takes the `String` by value.

```rust
fn main() {
    let s = "foo".to_string();   // or String::from("foo"): the same String
    println!("{s}");             // foo
    println!("{s}");             // foo
}
```

`s` is a `String`, and a `String` [moves rather than copies](../copy_or_move/README.md). There is no `E0382` here because nothing moves `s`: both lines only read it.

---

## The trap: reading `println!` as a function call

`println!("{s}")` looks like a call to a function named `println` with `s` as its argument, and a function with a `String` parameter would take `s` away from you. The `!` says it is a [macro ↗](https://doc.rust-lang.org/book/ch20-05-macros.html): it is replaced by other code before the borrow checker runs, and that code decides how `s` is used.

## What it turns into

rustc will print that code — [Printing the HIR](../../20_Compilers/printing_the_hir/README.md) takes the command apart. `-Zunpretty` is a nightly flag, and `RUSTC_BOOTSTRAP=1` makes the stable compiler accept it — fine for looking, not for building:

```bash
RUSTC_BOOTSTRAP=1 rustc --edition 2024 -Zunpretty=hir main.rs
```

For `let s = String::from("foo"); println!("{s}");` on 1.98.0:

```text title="Abridged — the prelude lines above main are dropped"
fn main() {
    let s = String::from("foo");
    {
        ::std::io::_print({
                super let args = (&s,);
                super let args = [format_argument::new_display(args.0)];
                unsafe { format_arguments::new(b"\xc0\x01\n\x00", &args) }
            });
    };
}
```

`(&s,)` is the line to read: the macro builds a tuple of **references** to its arguments, and everything after it works through those. The rest — `super let`, `format_argument`, the byte string — is compiler-internal and changes between releases. `-Zunpretty=expanded` stops one step earlier, at `format_args!("{0}\n", s)`, where the `&` is not visible yet.

## The compiler calls it an immutable borrow

A mutable borrow that is still in use blocks `println!`:

```rust
fn main() {
    let mut s = String::from("foo");
    let r = &mut s;
    // println!("{s}");   // error[E0502] — transcript below
    r.push('!');
    println!("{s}");      // foo!
}
```

With that line uncommented:

```text
error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
 --> main.rs:4:16
  |
3 |     let r = &mut s;
  |             ------ mutable borrow occurs here
4 |     println!("{s}");   // error[E0502] — transcript below
  |                ^ immutable borrow occurs here
5 |     r.push('!');
  |     - mutable borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0502`.
```

The caret sits under `{s}`, labelled *immutable borrow occurs here*. Put `let t = s;` on that line instead — a real move — and the error is `E0505`, *cannot move out of `s` because it is borrowed*.

## Every formatting macro borrows

[The example](#the-verified-output) prints a `Ticket` that is neither `Copy` nor `Clone` and prints a line when it is dropped, and uses it again after each of these:

| Macro | What it does with its arguments |
|---|---|
| [`println!` ↗](https://doc.rust-lang.org/std/macro.println.html), [`print!` ↗](https://doc.rust-lang.org/std/macro.print.html) | borrows |
| [`format!` ↗](https://doc.rust-lang.org/std/macro.format.html) | borrows, returns a new `String` |
| [`write!` ↗](https://doc.rust-lang.org/std/macro.write.html) | borrows, writes into the buffer you pass |
| [`assert!` ↗](https://doc.rust-lang.org/std/macro.assert.html) with a message | borrows, formats the message only on failure |
| [`panic!` ↗](https://doc.rust-lang.org/std/macro.panic.html) | borrows — checked even in a branch that never runs |
| [`assert_eq!` ↗](https://doc.rust-lang.org/std/macro.assert_eq.html) | borrows both sides |

The ticket's drop line does not appear until something takes it by value.

## What does move

Something that takes the value itself. Three shapes you will meet.

**A by-value parameter** — `fn take(t: Ticket)`, and likewise `let t = s;`, `drop(s)` and `vec![s]`. In the output, `take(t)` prints `~ ticket #7 dropped` before control is back in `main`.

**An argument expression that consumes.** The macro borrows the *result* of each argument, so if computing the argument moves `s`, the move is over before `println!` gets there:

```rust
fn main() {
    let s = String::from("foo");
    println!("{}", s + "!");   // foo! — the + takes s by value
    // println!("{s}");        // error[E0382] — transcript below
}
```

```text title="Abridged — a note pointing into the toolchain's own source file is dropped"
error[E0382]: borrow of moved value: `s`
  --> main.rs:4:16
   |
 2 |     let s = String::from("foo");
   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
 3 |     println!("{}", s + "!");   // foo! — the + takes s by value
   |                    ------- `s` moved due to usage in operator
 4 |     println!("{s}");        // error[E0382] — transcript below
   |                ^ value borrowed here after move
   |
help: consider cloning the value if the performance cost is acceptable
   |
 3 |     println!("{}", s.clone() + "!");   // foo! — the + takes s by value
   |                     ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

The example makes the order visible: `label(u)` takes a ticket by value, and its drop line is printed *before* the line `println!` was asked to print.

**`dbg!`**, the printing macro that takes its argument by value. It hands the value back so it can wrap an expression, which means a bare `dbg!(s);` moves `s` and drops it at the semicolon:

```rust
fn main() {
    let s = String::from("foo");
    dbg!(s);              // moves s, prints to stderr, drops it
    // println!("{s}");   // error[E0382] — transcript below
}
```

```text
error[E0382]: borrow of moved value: `s`
 --> main.rs:4:16
  |
2 |     let s = String::from("foo");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     dbg!(s);              // moves s, prints to stderr, drops it
  |     ------- value moved here
4 |     println!("{s}");   // error[E0382] — transcript below
  |                ^ value borrowed here after move
  |
help: consider borrowing instead of transferring ownership
  |
3 |     dbg!(&s);              // moves s, prints to stderr, drops it
  |          +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

The `help` line is the habit: `dbg!(&s)`. [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md) has the rest.

## `"foo".to_string()` or `String::from("foo")`?

The same `String` — the example builds one each way and prints `a == b: true` — so neither spelling changes anything above. Which to prefer is style, argued and measured on [Making a `String`](../../14_Strings/making_a_string/README.md) and [`ToOwned`](../../12_Traits/to_owned/README.md).

## If you are coming from another language

- **Python.** `print(s)` passes a reference to the object, and nothing in Python is ever moved, so the question never comes up. Rust agrees for `println!`, which is handed a reference. The half Python has no counterpart for is the function: `def take(s)` shares the object with its caller, while Rust's `fn take(s: String)` takes it, and the caller's `s` is unusable afterwards.
- **C.** `printf("%s", s)` passes a copy of the pointer and reads the text through it — the same shape as `println!` holding `&s`. What changed is the checking: `printf` trusts that `%s` meets a `char *` still pointing at live memory, while `format_args!` checks each argument's type at compile time and the borrow checker checks the value is still alive.
- **C++.** `std::cout << s` binds a `const std::string&`, a borrow in all but name. The difference is the by-value parameter: `void take(std::string s)` **copies**, leaving the caller's `s` intact, and after `take(std::move(s))` a later use of `s` still compiles, against a string in a valid but unspecified state. Rust's `take(s)` moves by default, and the later use is `E0382`.

## The verified output

<!-- output:printing_borrows -->
*Verified output of [`printing_borrows.rs`](examples/printing_borrows.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── The two spellings from the question
  foo foo
  foo foo
  a == b: true

──── Every formatting macro borrows: t is used again after each one
  println!   ticket #7
  print!     ticket #7
  format!    ticket #7
  write!     ticket #7
  assert!    passed, so its message was never formatted
  panic!     compiled, in a branch that did not run
  assert_eq! passed, and a and b are still here: foo foo
  no drop line yet: t still owns ticket #7

──── A function that takes it by value: the move
  take() owns ticket #7 now
  ~ ticket #7 dropped
  back in main; t was moved, so this line cannot name it

──── An argument that consumes: the move happens before println! runs
  ~ ticket #8 dropped
  ticket #8, labelled
  the drop line came first: label(u) returned before the print began
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 18_Ownership/printing_borrows/examples/printing_borrows.rs -o /tmp/pb && /tmp/pb
```

## See also

- [Copy or move?](../copy_or_move/README.md) — which types move at all, and why `"hi"` does not
- [Ownership and moves](../ownership_and_moves/README.md) — what a move transfers
- [Borrowing](../borrowing/README.md) — the many-readers-or-one-writer rule behind the `E0502` above
- [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md) — the printing macro that moves
- [The format language](../../14_Strings/the_format_language/README.md) — what goes inside the braces

## Documentation

- [`println!` ↗](https://doc.rust-lang.org/std/macro.println.html) and [`format_args!` ↗](https://doc.rust-lang.org/std/macro.format_args.html) — the macro the others expand to; its *Argument lifetimes* section covers storing what it builds
- [`fmt::Arguments` ↗](https://doc.rust-lang.org/std/fmt/struct.Arguments.html) — the value `format_args!` builds, holding the references
- [`std::fmt`, named parameters ↗](https://doc.rust-lang.org/std/fmt/index.html#named-parameters) — why `{s}` with no argument list reads the variable `s` in scope
- [`Display` ↗](https://doc.rust-lang.org/std/fmt/trait.Display.html) — the trait `{}` calls
- [`dbg!` ↗](https://doc.rust-lang.org/std/macro.dbg.html) — its docs say it *"moves and takes ownership"* of the expression
- [`E0382` ↗](https://doc.rust-lang.org/error_codes/E0382.html), [`E0502` ↗](https://doc.rust-lang.org/error_codes/E0502.html) and [`E0505` ↗](https://doc.rust-lang.org/error_codes/E0505.html) — the three errors on this page
- [`String` ↗](https://doc.rust-lang.org/std/string/struct.String.html), [`impl From<&str> for String` ↗](https://doc.rust-lang.org/std/string/struct.String.html#impl-From%3C%26str%3E-for-String), [`ToString` ↗](https://doc.rust-lang.org/std/string/trait.ToString.html), [`ToOwned` ↗](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html), [`From` ↗](https://doc.rust-lang.org/std/convert/trait.From.html) and [`Into` ↗](https://doc.rust-lang.org/std/convert/trait.Into.html) — the spellings of a new `String`
- [`drop` ↗](https://doc.rust-lang.org/std/mem/fn.drop.html) and [`vec!` ↗](https://doc.rust-lang.org/std/macro.vec.html) — two more things that take the value
- The Book: [What is ownership? ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) and [References and borrowing ↗](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust by Example: formatted print ↗](https://doc.rust-lang.org/rust-by-example/hello/print.html)

## Po polsku

Pytanie, od którego zaczyna się ta strona: dlaczego dwa wywołania `println!("{s}")` na tym samym `String` nie dają błędu o przeniesionej wartości? Bo `println!` to makro, a nie funkcja, i każde makro formatujące — `print!`, `format!`, `write!`, `panic!`, `assert_eq!` — **pożycza** swoje argumenty. Rozwija się do kodu, który bierze `&s`, a nie `s`, więc wartość zostaje u właściciela i można ją drukować dowolnie wiele razy. Kompilator mówi to wprost: przy aktywnej referencji mutowalnej `println!` daje `E0502`, błąd o **pożyczeniu**, a nie o przeniesieniu.

Przeniesienie własności robi dopiero coś, co przyjmuje wartość: parametr typu `String`, `let t = s;`, `drop(s)`, `vec![s]`, argument w rodzaju `s + "!"` — oraz jedno makro, `dbg!`, które zabiera wartość i ją zwraca. Stąd nawyk `dbg!(&s)`.

`"foo".to_string()` i `String::from("foo")` dają ten sam `String`; wybór jest kwestią stylu i niczego na tej stronie nie zmienia.

**Szukaj po polsku:** czy `println!` pożycza zmienną · makra formatujące w Ruscie · `format_args!` · `rust println borrow or move` · `E0382 borrow of moved value`
