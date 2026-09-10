# Lifetime safety in Clang — the borrow checker's first rule, retrofitted

**Level:** 301 · for C and C++ programmers

**One line:** Clang 23 warns when a pointer is used after the object it points at has died, naming the same three places `rustc` names for `E0597` — because it runs the same rule, modelled on Rust's borrow checker — and it stops exactly where Rust's second rule, *one writer or many readers*, begins.

The analysis is Google's: an [RFC to LLVM in May 2025 ↗](https://discourse.llvm.org/t/rfc-intra-procedural-lifetime-analysis-in-clang/86291) by Utkarsh Saxena, Dmytro Hrybenko, Yitzhak Mandelbaum, Jan Voung and Kinuko Yasuda, and the temporal-safety half of Mandelbaum's C++Now 2026 talk, [*A Path to Practically Safe C++* ↗](https://www.youtube.com/watch?v=fi6csDXvve0). Clang's [own documentation ↗](https://clang.llvm.org/docs/LifetimeSafety.html) says the design is "inspired by Polonius, the Rust borrow checker".

## The program

The talk's demo, with the includes and a `main` added:

```cpp
#include <iostream>
#include <string>
#include <string_view>

void foo() {
    std::string_view view;
    {
        std::string small = "small scoped string";
        view = small;
    }
    std::cout << view;
}

int main() { foo(); }
```

`view` points into `small`'s characters, `small` is destroyed at the closing brace, and `view` is read after it. A [use-after-free](../use_after_free/README.md) through a `string_view`: AddressSanitizer reports `stack-use-after-scope` on any run that reaches the line, and `-Wall -Wextra` print nothing — on Apple's clang 21 and on Clang 23 alike.

## What Clang 23 says

```text title="Real output — clang 23.1.0 on Compiler Explorer, -std=c++20 -Wlifetime-safety"
<source>:9:16: warning: local variable 'small' does not live long enough [-Wlifetime-safety-use-after-scope]
    9 |         view = small;
      |                ^~~~~
<source>:10:5: note: local variable 'small' is destroyed here
   10 |     }
      |     ^
<source>:11:18: note: later used here
   11 |     std::cout << view;
      |                  ^~~~
1 warning generated.
```

## What rustc says

The same program in Rust — `let view: &str;`, assigned `&small` inside a block and read after it:

```text title="Real rustc output — scoped_string.rs, --edition 2024"
error[E0597]: `small` does not live long enough
 --> scoped_string.rs:5:16
  |
4 |         let small = String::from("small scoped string");
  |             ----- binding `small` declared here
5 |         view = &small;
  |                ^^^^^^ borrowed value does not live long enough
6 |     }
  |     - `small` dropped here while still borrowed
7 |     println!("{view}");
  |                ---- borrow later used here
```

The same three places, in the same order — what the talk calls a *3-point diagnostic*:

| The point | Clang 23 | `rustc` |
|---|---|---|
| where the pointer takes hold | `local variable 'small' does not live long enough` | `borrowed value does not live long enough` |
| where the object dies | `local variable 'small' is destroyed here` | `` `small` dropped here while still borrowed `` |
| where the pointer is read | `later used here` | `borrow later used here` |

One word differs, at the front of the first line: `warning` against `error`.

## The rule underneath

The talk builds the check from three facts, each computed at every point in a function:

- **Invalidation** — where an object ends: a destructor at a closing brace, `delete`, `unique_ptr::reset`, a `push_back` that may reallocate.
- **Points-to sets** — for each pointer, every object it *may* point at here. Clang's RFC calls them origin sets holding loans, which is Polonius's vocabulary.
- **Liveness** — whether the pointer can still be read before something overwrites it.

A violation is all three at one point: an object is invalidated, a points-to set still contains it, and that set is live. [Polonius ↗](https://rust-lang.github.io/polonius/), the next formulation of Rust's borrow checker, states the same rule as Datalog in [its reference implementation ↗](https://github.com/rust-lang/polonius/blob/master/polonius-engine/src/output/naive.rs):

```text title="From polonius-engine/src/output/naive.rs — rules 7 and 8, as its comments give them"
loan_live_at(Loan, Point) :-
  origin_contains_loan_on_entry(Origin, Loan, Point),
  origin_live_on_entry(Origin, Point).

errors(Loan, Point) :-
  loan_invalidated_at(Loan, Point),
  loan_live_at(Loan, Point).
```

A *loan* is one borrow of an object, an *origin* is a points-to set, and `origin_live_on_entry` is liveness. The talk's slogan — *a live points-to set should not contain an expired object* — is rule 8 in English.

## Liveness — the program both accept

The talk's second example:

```cpp
#include <iostream>
#include <memory>

int foo() {
    int* p;
    {
        std::unique_ptr<int> x = std::make_unique<int>(5);
        p = x.get();
    }
    std::unique_ptr<int> y = std::make_unique<int>(42);
    p = y.get();
    std::cout << *p;
    return 0;
}

int main() { return foo(); }
```

For three lines `p` holds the address of a freed `int`, and Clang 23 prints nothing: nothing reads `p` until it has been pointed at `y`. In the talk's words the first value is *not alive*, and the second assignment *kills* it.

`rustc` accepts the same shape — `liveness()` in the example further down — and reports what it saw, as a lint rather than an error:

```text title="Real rustc output — liveness_warning.rs, --edition 2024; a warning only, and the program prints 42"
warning: value assigned to `p` is never read
 --> liveness_warning.rs:5:9
  |
5 |         p = &x;
  |         ^^^^^^ this value is reassigned later and never used
...
8 |     p = &y;
  |     ------ `p` is overwritten here before the previous value is read
  |
  = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default
```

That is [non-lexical lifetimes](../../18_Ownership/borrowing/README.md): a borrow lasts to its last use, not to the end of the scope that declared the reference.

## Points-to — a pointer that may hold either

The talk's third:

```cpp
#include <memory>

int foo(int i, bool cond) {
    int* p = &i;
    if (cond) {
        std::unique_ptr<int> x = std::make_unique<int>(5);
        p = x.get();
    } // p points to {i, x}
    return *p;
}

int main(int argc, char**) { return foo(0, argc > 1); }
```

```text title="Real output — clang 23.1.0 on Compiler Explorer, -std=c++20 -Wlifetime-safety"
<source>:7:13: warning: local variable 'x' does not live long enough [-Wlifetime-safety-use-after-scope]
    7 |         p = x.get();
      |             ^
<source>:8:5: note: local variable 'x' is destroyed here
    8 |     } // p points to {i, x}
      |     ^
<source>:7:13: note: result of call to 'get' aliases the storage of local variable 'x'
    7 |         p = x.get();
      |             ^~~~~~~
<source>:9:13: note: later used here
    9 |     return *p;
      |             ^
1 warning generated.
```

After the `if`, the two paths merge and `p`'s set is `{i, x}` — with `x` dead on one of them. Run with no arguments, this program never takes that path at all. Both checkers object anyway, because the rule is about what the set *may* hold:

```text title="Real rustc output — points_to_two.rs, --edition 2024"
error[E0597]: `x` does not live long enough
 --> points_to_two.rs:5:13
  |
4 |         let x = Box::new(5);
  |             - binding `x` declared here
5 |         p = &x;
  |             ^^ borrowed value does not live long enough
6 |     } // p points to {i, x}
  |     - `x` dropped here while still borrowed
7 |     *p
  |     -- borrow later used here
```

The fix is the same in both languages: declare `x` before the `if`, as `either()` does below.

## Across a call — the contract in the signature

The check reads one function at a time — *compositional* rather than inter-procedural, in the talk's words — so a call is judged by the callee's declared contract, never by its body. In C++ the contract is `[[clang::lifetimebound]]`: *the result may point into this parameter*.

```cpp
#include <iostream>
#include <string>
#include <string_view>

std::string_view first_word(std::string_view s [[clang::lifetimebound]]) {
    return s.substr(0, s.find(' '));
}

int main() {
    std::string_view w = first_word(std::string("hello world"));  // temporary dies at ;
    std::cout << w << '\n';
}
```

```text title="Real output — clang 23.1.0 on Compiler Explorer, -std=c++20 -Wlifetime-safety"
<source>:10:37: warning: object backing the pointer will be destroyed at the end of the full-expression [-Wdangling-gsl]
   10 |     std::string_view w = first_word(std::string("hello world"));  // temporary dies at ;
      |                                     ^~~~~~~~~~~~~~~~~~~~~~~~~~
<source>:10:37: warning: temporary object does not live long enough [-Wlifetime-safety-use-after-scope]
   10 |     std::string_view w = first_word(std::string("hello world"));  // temporary dies at ;
      |                                     ^~~~~~~~~~~~~~~~~~~~~~~~~~
<source>:10:63: note: temporary object is destroyed here
   10 |     std::string_view w = first_word(std::string("hello world"));  // temporary dies at ;
      |                                                               ^
<source>:10:26: note: result of call to 'first_word' aliases the storage of temporary object
   10 |     std::string_view w = first_word(std::string("hello world"));  // temporary dies at ;
      |                          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<source>:11:18: note: later used here
   11 |     std::cout << w << '\n';
      |                  ^
2 warnings generated.
```

The first warning is Clang's older check, which sees one statement at a time; the second is the new one, following the result through the call because of the attribute. Leave the attribute off and the new check says nothing at the call — measured on Clang 23.1 with a two-argument `longer(a, b)`, both declared-only and defined in the same file.

`rustc` refuses the same call:

```text title="Real rustc output — temporary_string.rs, --edition 2024"
error[E0716]: temporary value dropped while borrowed
 --> temporary_string.rs:6:25
  |
6 |     let w = first_word(&String::from("hello world"));
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
  |                         |
  |                         creates a temporary value which is freed while still in use
7 |     println!("{w}");
  |                - borrow later used here
  |
help: consider using a `let` binding to create a longer lived value
  |
6 ~     let binding = String::from("hello world");
7 ~     let w = first_word(&binding);
  |
```

`fn first_word(s: &str) -> &str` carries the contract the attribute states — [lifetime elision](../../18_Ownership/lifetime_annotations/README.md#most-signatures-need-none) wrote one `'a` on both sides for you — and in Rust it is not optional: return a reference without saying where it points and the function itself is `E0106`. Checking each body on its own against its callees' signatures is how `rustc` has always worked; the talk's proposal is to get C++ there with annotations standing in for types.

## The shapes both accept

Declare the owner where it outlives every use, and both checkers are satisfied:

```rust
fn either(cond: bool) -> i32 {
    let i = 1;
    let x = Box::new(5); // declared outside the `if`, so it outlives p
    let mut p = &i;
    if cond {
        p = &x;
    }
    *p
}
```

<!-- output:lifetime_safety_in_clang -->
*Verified output of [`lifetime_safety_in_clang.rs`](examples/lifetime_safety_in_clang.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
liveness()        -> 42
either(false)     -> 1
either(true)      -> 5
first_word(&text) -> hello
```
<!-- /output -->

## Where they part — one writer or many readers

Two C++ programs, one line apart:

```cpp
#include <cstdio>
#include <vector>

int main() {
    std::vector<int> v = {1, 2, 3, 4};
    int& first = v[0];
    v.push_back(5); // may reallocate: 'first' now dangles
    std::printf("%d\n", first);
}
```

With `v[1] = 7;` in place of the `push_back`, Clang 23 is silent. With the `push_back`:

```text title="Real output — clang 23.1.0 on Compiler Explorer, -std=c++20 -Wlifetime-safety"
<source>:6:18: warning: local variable 'v' is later invalidated [-Wlifetime-safety-invalidation]
    6 |     int& first = v[0];
      |                  ^
<source>:7:7: note: local variable 'v' is invalidated here
    7 |     v.push_back(5); // may reallocate: 'first' now dangles
      |     ~~^~~~~~~~~~~~
<source>:8:25: note: later used here
    8 |     std::printf("%d\n", first);
      |                         ^~~~~
1 warning generated.
```

`rustc` refuses both:

| After `first` borrows `v[0]` … | Clang 23 `-Wlifetime-safety` | `rustc` |
|---|---|---|
| `v.push_back(5)`, then read `first` | a warning, from a check Clang's docs mark experimental | `E0502` |
| `v[1] = 7`, then read `first` | nothing | `E0502` |

The second row is Rust's other rule. `v[1] = 7` needs `&mut v`, and while `first` is live nothing else may touch `v` — whether or not the write could move the buffer:

```text title="Real rustc output — write_while_borrowed.rs, --edition 2024"
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> write_while_borrowed.rs:4:5
  |
3 |     let first = &v[0];
  |                  - immutable borrow occurs here
4 |     v[1] = 7; // a write -- but not one that can move the buffer
  |     ^ mutable borrow occurs here
5 |     println!("{first}");
  |                ----- immutable borrow later used here
  |
  = help: use `.split_at_mut(position)` to obtain two mutable non-overlapping sub-slices
```

That program is memory-safe in C++, and Rust refuses it anyway. What the refusal buys is not needing a list of dangerous operations. Clang has to know that `push_back` can reallocate and `operator[]` cannot; to `rustc` every `&mut` counts as an invalidation, so a method nobody has annotated is covered on the day it is written — and the same exclusivity, held across threads, is what rules out [data races](../data_races/README.md). Clang's docs name what the design leaves out, "exclusivity enforcement (alias-xor-mutability)", and the RFC lists "limited forms of exclusivity" as future work.

## What the check is not

- **A proof.** Clang's docs call it "designed for bug finding, not verification". It is off unless you pass `-Wlifetime-safety` — the docs suggest starting with `-Wlifetime-safety-permissive` — and at a call it cannot see into, it assumes nothing happened.
- **Aware of moves.** A moved-from owner still counts as destroyed at its closing brace, so the `-moved` warnings in the strict group can flag a pointer whose object lives on under a new owner. The docs' advice is to take the pointer from the new owner, after the move.
- **In most compilers yet.** Measured with the talk's program on Compiler Explorer: Clang 20.1 knows neither flag; 21.1 accepts `-Wexperimental-lifetime-safety` and prints nothing; 22.1 warns only once a hidden `-Xclang -fexperimental-lifetime-safety` is added; **23.1.0** is the first release where `-Wlifetime-safety` works. Apple's clang 21, which the other pages in this section use, prints nothing even with the hidden flag.
- **Matched by Polonius on the Rust side — not yet.** Stable `rustc` runs NLL; Polonius is on nightly as `-Zpolonius=next`. Every program on this page gets the same verdict from both (`rustc` 1.100.0-nightly, 2026-08-27). Where they differ is a program like [NLL's problem case #3 ↗](https://rust-lang.github.io/rfcs/2094-nll.html#problem-case-3-conditional-control-flow-across-functions), a conditional return of a borrow: `E0499` on stable 1.98.0, accepted by `-Zpolonius=next`.

## If you are coming from another language

- **C++** — If you have fixed a `-Wlifetime-safety` warning, you can already read `E0597`: same three places, same order. Three things change. The check is an error you cannot switch off. Every signature carries its contract, so there is no unannotated function to be optimistic about. And exclusivity comes with it — the `v[1] = 7` row above, which is the part that feels wrong for the first fortnight, and the part that makes the rest checkable without a list of dangerous operations.
- **Python** — A reference keeps its object alive, so nothing on this page can dangle for you; the whole analysis answers a question reference counting never has to ask. The one place Python enforces the *second* rule is a buffer that can move: a `bytearray` with a live `memoryview` refuses to grow, with `BufferError: Existing exports of data: object cannot be re-sized` (Python 3.14.7). That is `E0502` for one type, checked at run time.
- **ABAP** — The runtime owns every object and every table line, so reading freed memory is not a failure ABAP lets you meet. What transfers is a habit you may already have: not holding a reference or field symbol into an internal table across a statement that changes that table. Rust turns the habit into the `E0502` above, and checks it for you.

## Practice

**One contract, two spellings.** Write `longer(a, b)`, returning whichever string is longer, with the lifetimes it needs — and the C++ declaration Clang 23 needs in order to check its callers the same way. Call it with both strings alive; then move one into an inner block, use the result after the block, and predict the error. Finally tie the result to `a` alone, and find the call that now compiles.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:lifetime_safety_in_clang_kata -->
*[`lifetime_safety_in_clang_kata.rs`](examples/lifetime_safety_in_clang_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one contract, two spellings.
//!
//!   rustc --edition 2024 lifetime_safety_in_clang_kata.rs -o /tmp/k && /tmp/k

// The result may point into `a` or into `b`, so both must outlive it.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// The result points into `a` only; `b` may die as soon as the call returns.
fn first<'a>(a: &'a str, _b: &str) -> &'a str {
    a
}

fn main() {
    println!("THE CONTRACT, TWO SPELLINGS");
    println!("  Rust: fn longer<'a>(a: &'a str, b: &'a str) -> &'a str");
    println!("  C++:  std::string_view longer(std::string_view a [[clang::lifetimebound]],");
    println!("                                std::string_view b [[clang::lifetimebound]]);");
    println!("  Both say the result may point into a or into b. Leave the lifetimes");
    println!("  out and rustc refuses the function (E0106). Leave the attributes out");
    println!("  and C++ compiles it, and -Wlifetime-safety assumes the result borrows");
    println!("  from nothing -- so it stays silent at every caller.");
    println!();

    let a = String::from("borrow");
    let b = String::from("checker");
    println!("BOTH ARGUMENTS ALIVE");
    println!("  longer(&a, &b) = {}", longer(&a, &b));
    println!();

    println!("ONE ARGUMENT DIES FIRST");
    println!("  let r;");
    println!("  {{ let b = String::from(\"checker\"); r = longer(&a, &b); }}");
    println!("  println!(\"{{r}}\");");
    println!("  -> E0597: `b` does not live long enough. Refused even on a call where");
    println!("     the body would have returned `a`: the checker judges a call by the");
    println!("     signature and never reads the body. That is the compositional");
    println!("     analysis the talk proposes for C++, and how rustc has always worked.");
    println!();

    let r;
    {
        let short_lived = String::from("checker");
        r = first(&a, &short_lived);
    }
    println!("THE RESULT TIED TO `a` ONLY");
    println!("  first(&a, &short_lived) outlives the block: r = {r}");
    println!("  In C++ that is [[clang::lifetimebound]] on `a` alone. Clang 23 checks");
    println!("  that claim against the body too -- under -Wlifetime-safety-all, as a");
    println!("  warning you opt into; rustc checks the body against 'a on every build.");

    assert_eq!(longer(&a, &b), "checker");
    assert_eq!(r, "borrow");
}
```
<!-- /source -->

<!-- output:lifetime_safety_in_clang_kata -->
*Verified output of [`lifetime_safety_in_clang_kata.rs`](examples/lifetime_safety_in_clang_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
THE CONTRACT, TWO SPELLINGS
  Rust: fn longer<'a>(a: &'a str, b: &'a str) -> &'a str
  C++:  std::string_view longer(std::string_view a [[clang::lifetimebound]],
                                std::string_view b [[clang::lifetimebound]]);
  Both say the result may point into a or into b. Leave the lifetimes
  out and rustc refuses the function (E0106). Leave the attributes out
  and C++ compiles it, and -Wlifetime-safety assumes the result borrows
  from nothing -- so it stays silent at every caller.

BOTH ARGUMENTS ALIVE
  longer(&a, &b) = checker

ONE ARGUMENT DIES FIRST
  let r;
  { let b = String::from("checker"); r = longer(&a, &b); }
  println!("{r}");
  -> E0597: `b` does not live long enough. Refused even on a call where
     the body would have returned `a`: the checker judges a call by the
     signature and never reads the body. That is the compositional
     analysis the talk proposes for C++, and how rustc has always worked.

THE RESULT TIED TO `a` ONLY
  first(&a, &short_lived) outlives the block: r = borrow
  In C++ that is [[clang::lifetimebound]] on `a` alone. Clang 23 checks
  that claim against the body too -- under -Wlifetime-safety-all, as a
  warning you opt into; rustc checks the body against 'a on every build.
```
<!-- /output -->

</details>

## See also

- [Use-after-free](../use_after_free/README.md) — the bug itself, and the C program that printed someone else's data
- [Iterator invalidation](../iterator_invalidation/README.md) — `push_back` and `erase`, and the `E0502` they meet
- [Borrowing](../../18_Ownership/borrowing/README.md) — the two rules, and why a borrow ends at its last use
- [Lifetimes at the call site](../../18_Ownership/lifetimes_at_the_call_site/README.md) — a signature read as a contract
- [Safe Buffers](../safe_buffers/README.md) — the spatial half of the same talk
- [Lifetime Safety Analysis ↗](https://clang.llvm.org/docs/LifetimeSafety.html) — Clang's docs: every flag, the annotations, the limitations
- [The talk's demo on Compiler Explorer ↗](https://godbolt.org/z/3j15s78nh) — the same program with a `[[clang::lifetimebound]]` function in the middle of the chain
- [C and C++](../README.md) — the nine bugs, and the other reply

## Po polsku

Ta strona dotyczy bezpieczeństwa czasowego (*temporal safety*): wskaźnika nie wolno użyć po tym, jak obiekt, na który wskazuje, przestał istnieć. Clang 23 ma analizę, która to sprawdza (`-Wlifetime-safety`), zbudowaną wprost na wzór rustowego borrow checkera (kontrolera pożyczeń). Dokumentacja Clanga mówi otwarcie, że inspiracją był Polonius.

Model opiera się na trzech pojęciach, które lepiej znać po angielsku, bo tak nazywają je i Clang, i Polonius. Pożyczka (*loan*) to wzięcie adresu obiektu. Zbiór możliwych celów wskaźnika (*points-to set*, u Clanga i w Poloniusie *origin*) to wszystkie pożyczki, które wskaźnik może w danym miejscu trzymać. Żywotność (*liveness*) mówi, czy wskaźnik zostanie jeszcze odczytany. Błąd jest wtedy, gdy w jednym punkcie programu obiekt przestaje istnieć, a żywy zbiór wciąż go zawiera — i to jest dokładnie reguła, którą `rustc` zgłasza jako `E0597`. Oba kompilatory wskazują nawet te same trzy miejsca: gdzie wzięto adres, gdzie obiekt zniknął i gdzie wskaźnik został użyty.

Różnica jest jedna, ale zasadnicza. Rust ma drugą regułę — wielu czytelników albo jeden piszący (*aliasing XOR mutability*) — więc odrzuca także `v[1] = 7`, gdy trzymasz referencję do `v[0]`, choć w C++ ten zapis jest bezpieczny. Clang tej reguły nie ma i ostrzega tylko przy operacjach, o których wie, że unieważniają wskaźniki, jak `push_back`. Poza tym to ostrzeżenie, które trzeba włączyć, a nie błąd kompilacji.

**Szukaj po polsku:** kontroler pożyczeń · wiszący wskaźnik · czas życia obiektu · `Clang lifetime safety` · `-Wlifetime-safety` · `Polonius borrow checker` · `rust E0597`
