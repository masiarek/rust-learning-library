# A borrow is a loan

[References](../README.md) › **A borrow is a loan**

**Level:** 201 · working knowledge

**One line:** Creating a reference records a **loan** on the place it borrows from, carrying the reference's lifetime; the borrow checker then checks two conditions — the reference is used only while that lifetime lasts, and the place is not used against the loan until it expires — and every labelled line of a borrow error is one part of that check.

```rust
let mut v = vec![10, 11];
v.push(12);                     // Vec::push(&mut v, 12): a loan on `v` that ends with the call
let vptr = &mut v[1];           // a loan on `v`, carrying vptr's lifetime 'a
println!("v[1] = {}", *vptr);   // v[1] = 11 — vptr's last use, so 'a ends here
```

Swap the first two statements and the program is `E0499`. The same two borrows are taken either way; what changes is whether the first loan is still outstanding when the second one is.

## The model: a loan and two conditions

The rule — mutate through one reference, or share among many readers, never both at once — is the one [Borrowing](../../borrowing/README.md) teaches. Jung et al. call it the *exclusion principle* and, in §2 of the Stacked Borrows paper, describe how the compiler enforces it:

- Creating a reference (`&x`, `&mut x`, or the one a method call inserts) gives it a **lifetime**, a stretch of the program. It is almost always inferred.
- It also records a **loan** on the place borrowed from, with the same lifetime.
- **Condition (1):** the reference is used only while its lifetime lasts.
- **Condition (2):** the place is not used until the loan has expired. For a shared loan this weakens to *not mutated*.

With a lifetime inferred inside one function body, condition (1) never fails on its own. The compiler lengthens the lifetime until it covers every use of the reference, and no further — the paper's `'a` ends at the last use, which is [where a borrow ends](../../borrowing/README.md#where-a-borrow-ends-the-part-that-decides-everything) under non-lexical lifetimes. So (1) decides **how long** the loan lasts, and (2) is where a program is refused: some use of the place lands inside it.

What counts as a use depends on the loan. Each cell is a real rustc 1.98.0 run:

| During a loan on `v` | `&mut` loan (`let vptr = &mut v[1];`) | `&` loan (`let vptr = &v[1];`) |
|---|---|---|
| read the owner, `v.len()` | `E0502` | allowed |
| take another `&v[1]` | `E0502` | allowed — a second loan |
| mutate, `v.push(12)` | `E0499` | `E0502` |
| the borrowed variable goes out of scope — `r = &mut x;` or `r = &x;` in a block, `r` read after it | `E0597` | `E0597` |

`v.push(12)` is a use of `v` even though no `&` is written: it is `Vec::push(&mut v, 12)`, and taking that `&mut v` is the use.

## Reading a borrow error

The paper's first program, refused:

```text title="Abridged — real rustc output for push_while_loaned.rs"
error[E0499]: cannot borrow `v` as mutable more than once at a time
 --> push_while_loaned.rs:4:5
  |
3 |     let vptr = &mut v[1];
  |                     - first mutable borrow occurs here
4 |     v.push(12);
  |     ^ second mutable borrow occurs here
5 |     println!("v[1] = {}", *vptr);
  |                           ----- first borrow later used here
```

Three labels, one per part of the model:

| rustc's label | In the model |
|---|---|
| `first mutable borrow occurs here` | the loan, recorded by `&mut v[1]` with `vptr`'s lifetime `'a` |
| `second mutable borrow occurs here` | a use of `v` inside `'a` — condition (2) fails here, so the `^` points here |
| `first borrow later used here` | the use of `vptr` that condition (1) stretched `'a` to reach |

The message is the one the paper quotes. The `^` sits under the `push`, not under the `println!` the paper marks as the error line: rustc reports the use that breaks (2), and lists the line that kept the loan open.

**The loan is on `v`, not on `v[1]`.** The paper speaks of a loan on `v[1]`. On a `Vec`, `&mut v[1]` means `&mut *IndexMut::index_mut(&mut v, 1)` — see [index expressions ↗](https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions) — a call that borrows the whole vector, which is why the first label underlines only `v` and the message says `` cannot borrow `v` ``. An array's indexing is built in, so there the loan is on the element — but on any element, since the checker does not compare indices:

```text title="Abridged — real rustc output for two_elements_of_an_array.rs, help line trimmed"
error[E0499]: cannot borrow `v[_]` as mutable more than once at a time
 --> two_elements_of_an_array.rs:4:13
  |
3 |     let a = &mut v[0];
  |             --------- first mutable borrow occurs here
4 |     let b = &mut v[1];
  |             ^^^^^^^^^ second mutable borrow occurs here
5 |     *a += *b;
  |     -------- first borrow later used here
```

## Three more refusals, the same three lines

| Program | The loan | The use that breaks (2) | What held the loan open |
|---|---|---|---|
| a reborrow used once too often | `&mut (*v2)[1]`, on `*v2` | `Vec::push(v2, 12)` | a second read of `vptr` |
| two shared loans, then a push | `&v[1]`, shared | `v.push(12)`, a mutation | the read of `vptr` |
| a `&mut` loan, then a read | `&mut v[1]` | `v.len()`, a read | `*vptr += 1` |

The paper's reborrow compiles as written — it is section 2 of the run below. One more read of `vptr` after the push is refused:

```text title="Abridged — real rustc output for reborrow_used_after_push.rs"
error[E0499]: cannot borrow `*v2` as mutable more than once at a time
 --> reborrow_used_after_push.rs:6:15
  |
4 |     let vptr = &mut (*v2)[1];
  |                     ----- first mutable borrow occurs here
5 |     println!("v[1] = {}", *vptr);
6 |     Vec::push(v2, 12);
  |               ^^ second mutable borrow occurs here
7 |     println!("v[1] = {}", *vptr);
  |                           ----- first borrow later used here
```

The place is `*v2`, not `v`: `vptr`'s loan is on what `v2` points at, nested inside `v2`'s own loan on `v`. Passing `v2` to `push` reborrows it as `&mut *v2`, and that is the use. [Reborrowing](../../reborrowing/README.md) has the mechanics.

The paper's two shared references interleave fine (section 3 of the run). A `push` between the second `let` and the first read is refused:

```text title="Abridged — real rustc output for push_while_shared.rs"
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> push_while_shared.rs:5:5
  |
3 |     let vptr = &v[1];
  |                 - immutable borrow occurs here
4 |     let vptr2 = &v[1];
5 |     v.push(12);
  |     ^^^^^^^^^^ mutable borrow occurs here
6 |     println!("v[1] = {}", *vptr);
  |                           ----- immutable borrow later used here
```

Two loans are outstanding at the `push`, `vptr`'s and `vptr2`'s, and rustc names one of them. Line 4 is not an error of its own: taking a second `&v[1]` is a non-mutating use, which a shared loan allows.

Under a `&mut` loan, condition (2) is not weakened, and a plain read is enough:

```text title="Abridged — real rustc output for read_while_loaned.rs"
error[E0502]: cannot borrow `v` as immutable because it is also borrowed as mutable
 --> read_while_loaned.rs:4:26
  |
3 |     let vptr = &mut v[1];
  |                     - mutable borrow occurs here
4 |     println!("len = {}", v.len());
  |                          ^ immutable borrow occurs here
5 |     *vptr += 1;
  |     ---------- mutable borrow later used here
```

## Going out of scope is a use too

The other half of the checker's work has no second reference in it. From *Programming Rust*, 2nd ed., ch. 5, "Borrowing a Local Variable":

```text title="Abridged — real rustc output for local_in_a_block.rs"
error[E0597]: `x` does not live long enough
 --> local_in_a_block.rs:5:13
  |
4 |         let x = 1;
  |             - binding `x` declared here
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     assert_eq!(*r, 1);
  |     ----------------- borrow later used here
```

| rustc's label | In the model |
|---|---|
| `` binding `x` declared here `` | the place the loan is on |
| `borrowed value does not live long enough` | the loan, recorded by `&x` |
| `` `x` dropped here while still borrowed `` | the end of `x`'s scope: a use of the place inside the loan, condition (2) |
| `borrow later used here` | the `assert_eq!` reads `r`, so condition (1) stretches the loan to it |

`x` is an `i32` and has no destructor, and rustc still says *dropped*: the end of its storage is the use. Declaring `x` before the block moves that use past the last read of `r` — section 4 of the run.

When a type fixes the lifetime instead of a later use, `borrow later used here` gives way to the annotation, and the model is unchanged:

```text title="Abridged — real rustc output for static_annotation.rs"
error[E0597]: `x` does not live long enough
 --> static_annotation.rs:3:27
  |
2 |     let x = 1;
  |         - binding `x` declared here
3 |     let r: &'static i32 = &x;
  |            ------------   ^^ borrowed value does not live long enough
  |            |
  |            type annotation requires that `x` is borrowed for `'static`
4 |     assert_eq!(*r, 1);
5 | }
  | - `x` dropped here while still borrowed
```

The annotation makes the loan last for `'static`; the drop at the end of `main` falls inside it. [`&'static str`](../../../14_Strings/static_str/README.md) meets this one on a `String`, and [What `&'a T` claims](../../what_a_reference_claims/README.md) is the lifetime side in full. The owner being assigned to or moved while borrowed, `E0506` and `E0505`, is condition (2) again — see [Borrowed state](../../borrowed_state/README.md).

## At run time there is no loan

*Programming Rust* (ch. 5, "Reference Safety") makes the point that lifetimes exist only while compiling: at run time a reference is an address and nothing more. The last section of the run measures it. A `&i32` borrowed for a few lines and a `&'static i32` are both one word. A wide reference, `&[i32]` or `&str`, is two words — an address and a length — so *an address and nothing more* holds exactly only for a thin reference. The second word is run-time data; the lifetime is still not there.

That absence is the paper's starting point. From §3 on, Stacked Borrows rebuilds the checker's rules as a run-time model that deliberately uses no lifetimes, so that `unsafe` code can be held to them, and §7 evaluates it with Miri.

## Checkpoint

**Predict before you open the answer.** `let r; { let x = 1; r = &x; println!("{}", *r); }` — `r` is declared outside the block, and `x` dies at its `}`. Does it compile?

<details markdown="1">
<summary><strong>The answer</strong></summary>

<!-- output:a_borrow_is_a_loan -->
*Verified output of [`a_borrow_is_a_loan.rs`](examples/a_borrow_is_a_loan.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── 1. push first, then the loan: nothing is outstanding at the push
  v[1] = 11
  v    = [10, 11, 12]   (the owner is free again)
  Swap the push and the `let`: E0499, the push uses `v` during 'a.

──── 2. A reborrow: vptr's loan sits inside v2's
  v[1] = 11
  v    = [10, 11, 12]
  One more read of vptr after the push: E0499 on `*v2`.

──── 3. Two shared loans on one element, used interleaved
  v[1] = 11
  v.len() = 2   (a read of the owner under shared loans)
  v[1] = 11
  With `let mut v` and v.push(12) before the first read: E0502, a mutation during 'a.

──── 4. The referent outlives the loan: `x` declared outside the block
  *r = 1
  With `let x = 1;` inside the block: E0597, `x` dropped while borrowed.

──── Checkpoint. `r` declared outside, `x` inside, last read inside. Compiles?
  yes: *r = 1   (read before the `}`, so the loan ended first)
  Where `r` is declared does not matter. Its last read sets the loan's end.

──── 5. At run time a reference carries no lifetime
  &i32 borrowed for a few lines   1 word
  &'static i32                    1 word
  &[i32]  address + length        2 words
  &str    address + length        2 words
  The length is run-time data; the lifetime was checked and erased.
```
<!-- /output -->

</details>

## Practice

**Reference to a local variable.** Take the program under [Going out of scope is a use too](#going-out-of-scope-is-a-use-too). Say which line takes the loan, which line breaks which condition, and why. Then make it compile two ways: once by moving `x` out of the block, and once by keeping `x` inside it and no longer holding a reference past the `}`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:a_borrow_is_a_loan_kata -->
*[`a_borrow_is_a_loan_kata.rs`](examples/a_borrow_is_a_loan_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: reference to a local variable.
//!
//! The refused program, from ch. 5 of Programming Rust, 2nd ed.:
//!
//!     let r;
//!     {
//!         let x = 1;
//!         r = &x;              // loan on `x` starts
//!     }                        // `x` dropped: a use of the place, condition (2)
//!     assert_eq!(*r, 1);       // a use of `r`, so condition (1) stretches the loan here
//!
//! E0597. Condition (1) makes the loan last until the assert; the drop at `}`
//! falls inside it, and condition (2) forbids that. Two ways out: move the drop
//! past the last use, or end the loan before the drop.
//!
//!   rustc --edition 2024 a_borrow_is_a_loan_kata.rs -o /tmp/a_borrow_is_a_loan_kata && /tmp/a_borrow_is_a_loan_kata

/// Fix 1: declare `x` outside the block. It now drops at the end of this
/// function, after the last use of `r`, so no use of `x` falls inside the loan.
fn move_x_out() -> i32 {
    let x = 1;
    let r;
    {
        r = &x;
    }
    assert_eq!(*r, 1);
    *r
}

/// Fix 2: keep `x` in the block, but keep an `i32` instead of a `&i32`.
/// The reference is last used inside the block, so its loan has ended by the `}`.
fn copy_the_value_out() -> i32 {
    let n;
    {
        let x = 1;
        let r = &x;
        n = *r; // i32 is Copy: the value leaves the block, the loan does not
    }
    assert_eq!(n, 1);
    n
}

fn main() {
    println!("fix 1, `x` declared outside the block: *r = {}", move_x_out());
    println!("fix 2, value copied out of the block:  n  = {}", copy_the_value_out());
}
```
<!-- /source -->

<!-- output:a_borrow_is_a_loan_kata -->
*Verified output of [`a_borrow_is_a_loan_kata.rs`](examples/a_borrow_is_a_loan_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
fix 1, `x` declared outside the block: *r = 1
fix 2, value copied out of the block:  n  = 1
```
<!-- /output -->

</details>

## If you are coming from another language

- **C++.** The paper's first program is `std::vector<int> v = {10, 11}; int& vptr = v[1]; v.push_back(12);` followed by a read of `vptr`. Apple clang 21 compiles it with `-Wall -Wextra` and prints nothing. Built with `-fsanitize=address`, the read is reported as `heap-use-after-free`: the capacity was 2, so `push_back` grew it to 4 and moved the buffer. [Iterator invalidation](../../../31_C_and_Cpp/iterator_invalidation/README.md) is the same hazard with an iterator in place of the reference. Clang 23's opt-in `-Wlifetime-safety` is built on the same loans, plus liveness, in Polonius's vocabulary, and warns on a `push_back` like this one — see [Lifetime safety in Clang](../../../31_C_and_Cpp/lifetime_safety_in_clang/README.md). Two things change in Rust. The check is an error, not an opt-in warning. And condition (2) needs no list of operations that may reallocate: every mutation of `v` counts, so `v[0] = 7` while `&v[1]` is live is `E0502` too, though the C++ equivalent is safe.
- **Python.** `vptr = v[1]` binds a name to the `int` object, not to the slot in the list, so `v.append(12)` then `print(vptr)` prints `11`, and a later `v[1] = 99` is not seen through `vptr` either. There are no interior pointers to invalidate; the paper notes that garbage-collected languages generally do not offer them. Rust offers them, and the loans are what make them checkable.

## See also

- [Borrowing](../../borrowing/README.md) — the rule the model enforces, and [where a borrow ends](../../borrowing/README.md#where-a-borrow-ends-the-part-that-decides-everything), which sets the length of every loan on this page
- [Reborrowing](../../reborrowing/README.md) — the nested loan behind the paper's third program
- [Borrowing something forever](../../borrowing_forever/README.md#a-destructor-is-a-use) — a loan that lasts as long as the value, so the closing brace that runs `Drop` is the use that fails
- [Borrowed state](../../borrowed_state/README.md) — `E0506` and `E0505`: the owner assigned to or moved during a loan
- [A stack slot is reused](../../a_stack_slot_is_reused/README.md) — `E0515`, and what would be at the address if the loan were allowed to outlive its frame
- [What `&'a T` claims](../../what_a_reference_claims/README.md) — the three claims a lifetime makes, and which one `E0597` is
- [Lifetime annotations](../../lifetime_annotations/README.md) — naming the lifetime a loan carries, when the compiler cannot infer the relationship
- [Interior mutability](../../../09_Advanced/interior_mutability/README.md) — `Cell` and `RefCell`, the exception to the exclusion principle the paper takes up in §5
- [Lifetime safety in Clang](../../../31_C_and_Cpp/lifetime_safety_in_clang/README.md) — loans, origins and liveness, as Polonius and Clang 23 state them
- Error index: [E0499 ↗](https://doc.rust-lang.org/error_codes/E0499.html) · [E0502 ↗](https://doc.rust-lang.org/error_codes/E0502.html) · [E0597 ↗](https://doc.rust-lang.org/error_codes/E0597.html)

## Sources

- Ralf Jung, Hoang-Hai Dang, Jeehoon Kang, Derek Dreyer, *Stacked Borrows: An Aliasing Model for Rust*, Proc. ACM Program. Lang. 4, POPL, Article 41, January 2020 — §2 "An Introduction to Rust": [paper ↗](https://plv.mpi-sws.org/rustbelt/stacked-borrows/paper.pdf) · [project page ↗](https://plv.mpi-sws.org/rustbelt/stacked-borrows/). Its programs are verified here on rustc 1.98.0: the three it accepts are sections 1–3 of the run (the shared-reference one with a `v.len()` read added, and without its unneeded `mut`, which rustc warns about as `unused_mut`); the three it rejects are the `push_while_loaned.rs`, `reborrow_used_after_push.rs` and `push_while_shared.rs` transcripts.
- Jim Blandy, Jason Orendorff, Leonora F. S. Tindall, *Programming Rust*, 2nd ed., O'Reilly 2021 — ch. 5 "References", "Reference Safety", "Borrowing a Local Variable". In the library's [book list](../../../10_Resources/books/README.md).
- Every transcript on this page is a real rustc 1.98.0 `--edition 2024` compile of the file its title names; only the `aborting` and `For more information` lines, and one `help:` line, are dropped.

## Po polsku

Kontroler pożyczeń (*borrow checker*) pilnuje **zasady wykluczania**: dane można modyfikować przez jedną referencję albo współdzielić między wielu czytelników, ale nigdy jedno i drugie naraz. Artykuł o Stacked Borrows (§2) opisuje, jak to sprawdza. Utworzenie referencji nadaje jej **czas życia** i zapisuje **pożyczkę** na miejscu, z którego pożyczono, z tym samym czasem życia. Potem są dwa warunki: (1) referencja jest używana tylko w swoim czasie życia, (2) pożyczone miejsce nie jest używane, dopóki pożyczka trwa — a przy pożyczce współdzielonej tylko nie jest modyfikowane.

Warunek (1) przy wnioskowanym czasie życia sam nigdy nie zawodzi: kompilator wydłuża czas życia do ostatniego użycia referencji. Ustala więc, **jak długo** trwa pożyczka. Błąd powstaje w warunku (2), gdy jakieś użycie miejsca wypada w środku pożyczki — i każda etykieta komunikatu to jeden element modelu. *first mutable borrow occurs here* to pożyczka, *second mutable borrow occurs here* to użycie łamiące warunek (2), *first borrow later used here* to użycie, do którego warunek (1) rozciągnął pożyczkę. `v.push(12)` też jest użyciem `v`, bo to `Vec::push(&mut v, 12)`. Na `Vec` pożyczka dotyczy całego `v`, a nie `v[1]`: indeksowanie to wywołanie `IndexMut::index_mut(&mut v, 1)`.

Koniec zasięgu zmiennej to również użycie. `let r; { let x = 1; r = &x; } assert_eq!(*r, 1);` daje `E0597`: *`x` dropped here while still borrowed* to warunek (2), a `assert_eq!` trzyma pożyczkę otwartą. Poprawki są dwie: zadeklarować `x` przed blokiem albo skopiować wartość z bloku i nie trzymać referencji za `}`. W czasie wykonania pożyczki nie ma — referencja to adres, a referencja szeroka (`&[T]`, `&str`) to adres i długość, ale nigdy czas życia.

**Szukaj po polsku:** kontroler pożyczeń · pożyczka i czas życia · zasada wykluczania · `rust borrow checker loan` · `rust E0499 E0502 E0597` · `Stacked Borrows`
