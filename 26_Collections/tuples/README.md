# Tuples

**Level:** 101 · for newcomers

**One line:** A tuple is a struct whose fields are numbered instead of named — free to write, free to return, and readable for about two fields.

```rust
fn min_max(scores: &[i32]) -> (i32, i32) {
    let mut lo = scores[0];
    let mut hi = scores[0];
    for &s in scores {
        if s < lo { lo = s; }
        if s > hi { hi = s; }
    }
    (lo, hi)
}

fn main() {
    let (lo, hi) = min_max(&[5, 3, 0, 4, 2]);
    println!("{lo} to {hi}");   // 0 to 5
}
```

The type is written the way the value is: `(i32, i32)`. There is no declaration anywhere, which is the entire appeal — a function with two things to say does not have to invent a type to say them in.

## The length and the element types are the type

`(u8, char, bool)` and `(u8, bool, char)` are different types, and neither is `(u8, char)`. That is what makes destructuring safe: the pattern `let (a, b, c) = t;` cannot compile against a two-tuple.

There is no header and no indirection — a tuple is its fields, laid out next to each other in an order the compiler chooses. `size_of::<(i32, i32)>()` is 8.

## The two odd arities

| Written | Is | Note |
|---|---|---|
| `()` | the **empty tuple**, size 0 | what a function with no `->` returns, and what a `;` turns an expression into |
| `(7)` | just `7` | parentheses, not a tuple |
| `(7,)` | a **one-tuple** | the trailing comma is the whole difference |

`()` is called the *unit type*, and meeting it as a tuple explains a lot of error messages: *"expected `()`, found `i32`"* is the compiler saying you left a value where a statement was expected.

## Comparison is field by field, left to right

```rust
fn main() {
    let mut rounds = [(2, "Cara"), (1, "Ben"), (2, "Ada")];
    rounds.sort();
    println!("{rounds:?}");   // [(1, "Ben"), (2, "Ada"), (2, "Cara")]
}
```

`(1, ..)` sorts before `(2, ..)` without the names being read at all; the tie between the two 2s is then broken by `"Ada" < "Cara"`. So a tuple is a **free sort key** — put the field you want to sort by first, and `sort()` does the rest. This is `#[derive(PartialOrd, Ord)]`'s rule too, on any struct: declaration order is comparison order.

## The trap: `.0` is a comment nobody wrote

A tuple's fields have no names, so the meaning lives in the reader's head or in a variable name at the call site — and it does not travel. `row.2` in a function two files away is unreadable, and a tuple whose fields have the same type can be **transposed with no error at all**:

```rust
fn main() {
    // (score, weight) — or was it (weight, score)?
    let row: (u32, u32) = (5, 3);
    let weighted = row.0 * row.1;   // right either way, and that is the problem
    println!("{weighted}");   // 15
}
```

The rule that keeps this from biting: **two fields you destructure on the spot, tuple; anything that travels, name it.** A `struct` with two named fields costs three lines and makes `b.score` impossible to confuse with `b.weight` — and gives you `#[derive(Debug)]` output that says which is which.

## If you are coming from another language

- **Python.** Nearly the same thing, and the habits transfer directly: `lo, hi = min_max(xs)` is the same destructuring, `_` discards the same way, and returning a pair rather than inventing a class is the same instinct. Three differences. Rust's tuple is **fixed-length and typed**, so there is no `*rest` unpacking and no `t + (1,)`; the length mismatch Python raises at run time (`ValueError: too many values to unpack`) is a compile error here. Rust has no `namedtuple`, because a struct already costs three lines and gives you more. And a Python tuple is a heap object with a header, while Rust's is exactly its fields — `(u8, u8)` is two bytes, not a 56-byte `PyTupleObject`.
- **ABAP.** There is no tuple, and the two workarounds are the two things a tuple replaces: `EXPORTING` a second parameter, or declaring a one-off structure in `TYPES BEGIN OF … END OF`. The first is what `(lo, hi)` deletes — one return value instead of a signature with output parameters, and no possibility of forgetting to read one. The second is what this page says to do anyway past two fields: a named structure, where `ls_row-score` cannot be mistaken for `ls_row-weight`. What genuinely does not exist in ABAP is the *anonymous* type — every structure needs a `TYPES` declaration somewhere, and that declaration is exactly the ceremony a tuple exists to skip for the two-line case.
- **Go.** Multiple return values are the same idea with less power: Go's are only a return-position feature, so you cannot put one in a slice, a map or a struct field. A Rust tuple is an ordinary value, which is why `Vec<(String, u32)>` is such a common shape and `[]struct{...}` is Go's answer to it.
- **C/C++.** `std::pair` and `std::tuple`, with `structured bindings` (`auto [lo, hi] = min_max(xs);`) being the C++17 version of `let (lo, hi) = …`. The advice is the same in both languages, and older for C++: `.first`/`.second` past two fields is how a bug hides.

---

## The verified output

<!-- output:tuples -->
*Verified output of [`tuples.rs`](examples/tuples.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Two values out of one function, with no type declared
   min_max([5, 3, 0, 4, 2]) = (0, 5)
   or keep the pair whole: both.0 = 0, both.1 = 5
   The type is written the way the value is: (i32, i32).

2. Length and element types are part of the type
   (u8, char, bool) = (5, A, true)
   size_of::<(u8, char, bool)>() = 8
   size_of::<(i32, i32)>()       = 8
   A tuple has no header and no indirection: it is its fields,
   laid out next to each other, in an order the compiler picks.

3. The two odd arities
   () is the empty tuple, size 0 — what every function without
      a `->` returns, and what a `;` turns an expression into.
   (7,) is a ONE-tuple, and the trailing comma is load-bearing:
      (7) is just 7 in parentheses. one.0 = 7, unit = ()

4. Comparison is field by field, left to right
   sorted: [(1, "Ben"), (2, "Ada"), (2, "Cara")]
   (1, ..) < (2, ..) settles the first two without reading the name;
   the tie between the 2s is broken by "Ada" < "Cara".
   Ordering a tuple is a free sort key — put the field you want
   to sort by first.

5. Where a tuple stops being readable
   tuple:  let (lo, hi) = min_max(&scores);        -> (0, 5)
   struct: range_of(&scores).lowest / .highest   -> 0 / 5
   Two fields you destructure on the spot: tuple. Three or more,
   or a value that travels, or `.2` appearing in another function:
   name the fields. `.0` is a comment nobody wrote.
```
<!-- /output -->

## Practice

**Four fields, and the transposition that compiles.** Write a ballot as a tuple — voter, candidate, score, and whether it arrived by post — then write a function that totals the postal and walk-in scores separately, destructuring the tuple in the `for` pattern. Now write the same four fields as a struct and the same function against that.

Two things to do before you look at the solution. Swap two fields **in the tuple's type declaration** and see whether the compiler catches it — then answer why it caught this one, and construct a version it would not catch. And write down what `#[derive(Debug)]` prints for each of the two shapes; only one of them tells the reader what the numbers mean.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:tuples_kata -->
*[`tuples_kata.rs`](examples/tuples_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: swap, ignore, nest, and the arity where it breaks down.
//!
//!   rustc --edition 2024 tuples_kata.rs -o /tmp/tk && /tmp/tk

/// A ballot: voter, candidate, score, and whether it arrived by post.
type BallotTuple = (&'static str, &'static str, u8, bool);

#[derive(Debug)]
struct Ballot {
    voter: &'static str,
    candidate: &'static str,
    score: u8,
    postal: bool,
}

fn totals_tuple(rows: &[BallotTuple]) -> (u32, u32) {
    let mut postal = 0;
    let mut walk_in = 0;
    for &(_, _, score, is_postal) in rows {
        if is_postal {
            postal += u32::from(score);
        } else {
            walk_in += u32::from(score);
        }
    }
    (postal, walk_in)
}

fn totals_struct(rows: &[Ballot]) -> (u32, u32) {
    let mut postal = 0;
    let mut walk_in = 0;
    for b in rows {
        if b.postal {
            postal += u32::from(b.score);
        } else {
            walk_in += u32::from(b.score);
        }
    }
    (postal, walk_in)
}

fn main() {
    println!("1. Swap without a temporary");
    let (mut a, mut b) = (1, 2);
    (a, b) = (b, a);
    println!("   after (a, b) = (b, a): a = {a}, b = {b}");
    println!("   The right-hand tuple is built first, so no third name is needed.");

    println!();
    println!("2. `_` in a pattern discards without binding");
    let row: BallotTuple = ("Ada", "Cara", 5, true);
    let (voter, _, score, _) = row;
    println!("   let (voter, _, score, _) = row;  ->  {voter} scored {score}");
    println!("   An unused binding warns; an `_` does not, because it says so.");

    println!();
    println!("3. Nesting, and destructuring through it");
    let by_round = ((1, "Ada"), (2, "Ben"));
    let ((r1, w1), (r2, w2)) = by_round;
    println!("   round {r1}: {w1}, round {r2}: {w2}");
    println!("   by_round.1.0 = {} — legal, and the reason this is the last", by_round.1.0);
    println!("   arity anyone should write.");

    println!();
    println!("4. The four-field tuple, and the same data named");
    let tuple_rows: [BallotTuple; 4] = [
        ("Ada", "Cara", 5, true),
        ("Ben", "Cara", 3, false),
        ("Cara", "Ada", 4, true),
        ("Dan", "Ada", 2, false),
    ];
    let struct_rows: Vec<Ballot> = tuple_rows
        .iter()
        .map(|&(voter, candidate, score, postal)| Ballot { voter, candidate, score, postal })
        .collect();

    println!("   first row, named: {} -> {}", struct_rows[0].voter, struct_rows[0].candidate);
    let (p1, w1) = totals_tuple(&tuple_rows);
    let (p2, w2) = totals_struct(&struct_rows);
    println!("   totals_tuple  -> postal {p1}, walk-in {w1}");
    println!("   totals_struct -> postal {p2}, walk-in {w2}");
    println!("   Same answer. The difference is in the two function bodies:");
    println!("     for (_, _, score, is_postal) in rows       <- position");
    println!("     for b in rows: b.score, b.postal            <- name");
    println!("   Swap `score` and `postal` in the tuple's TYPE and the first");
    println!("   body still compiles if the types happen to line up. Here they");
    println!("   do not (u8 vs bool), so this one is caught — which is luck,");
    println!("   not design: two u8 fields swapped compile fine and count wrong.");
    println!("   Field names cannot be transposed by accident.");

    println!();
    println!("5. What the struct also bought");
    println!("   {:?}", struct_rows[0]);
    println!("   `#[derive(Debug)]` prints the field names. A tuple prints");
    println!("   (\"Ada\", \"Cara\", 5, true) and leaves the reader to guess.");
}
```
<!-- /source -->

<!-- output:tuples_kata -->
*Verified output of [`tuples_kata.rs`](examples/tuples_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Swap without a temporary
   after (a, b) = (b, a): a = 2, b = 1
   The right-hand tuple is built first, so no third name is needed.

2. `_` in a pattern discards without binding
   let (voter, _, score, _) = row;  ->  Ada scored 5
   An unused binding warns; an `_` does not, because it says so.

3. Nesting, and destructuring through it
   round 1: Ada, round 2: Ben
   by_round.1.0 = 2 — legal, and the reason this is the last
   arity anyone should write.

4. The four-field tuple, and the same data named
   first row, named: Ada -> Cara
   totals_tuple  -> postal 9, walk-in 5
   totals_struct -> postal 9, walk-in 5
   Same answer. The difference is in the two function bodies:
     for (_, _, score, is_postal) in rows       <- position
     for b in rows: b.score, b.postal            <- name
   Swap `score` and `postal` in the tuple's TYPE and the first
   body still compiles if the types happen to line up. Here they
   do not (u8 vs bool), so this one is caught — which is luck,
   not design: two u8 fields swapped compile fine and count wrong.
   Field names cannot be transposed by accident.

5. What the struct also bought
   Ballot { voter: "Ada", candidate: "Cara", score: 5, postal: true }
   `#[derive(Debug)]` prints the field names. A tuple prints
   ("Ada", "Cara", 5, true) and leaves the reader to guess.
```
<!-- /output -->

</details>

---

## See also

- [Arrays and slices](../arrays_and_slices/README.md) — the other compound type built into the language, and the one that is all the same type
- [What a struct is](../../16_Structs/what_a_struct_is/README.md) — the named version, and where this page says to go
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the one-field tuple struct, which is a different feature with a similar look
- [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) — matching on a tuple to cover two questions in one `match`
- [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) — where `()` comes from, and the semicolon that produces it

## Sources

[Primitives: Tuples ↗](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html) in Rust by Example, and the [`tuple` primitive ↗](https://doc.rust-lang.org/std/primitive.tuple.html) page in std, which is where the trait implementations (and the twelve-element limit on them) are written down.

## Po polsku

Krotka (*tuple*) to struktura, której pola są ponumerowane zamiast nazwane — i to jedno z niewielu pojęć, dla których polskie słowo jest w pełni ustalone: tak samo mówi się o krotkach w Pythonie, tak samo nazywa je polski Tour of Rust, który *tuple struct* oddaje jako „struktura krotkowa”. Właśnie dlatego warto wyliczyć, czym rustowa krotka różni się od pythonowej, bo słowo jest to samo, a zasady inne. Długość i typy pól **są** typem: `(u8, char, bool)` i `(u8, bool, char)` to dwa różne typy, żaden z nich nie jest `(u8, char)`, więc rozpakowanie `let (a, b, c) = t;` po prostu nie skompiluje się przy dwuelementowej krotce — pythonowy `ValueError: too many values to unpack` przenosi się tu z czasu działania do czasu kompilacji. Nie ma też pythonowego rozpakowania z `*rest` ani doklejania `t + (1,)`, a sama wartość nie jest obiektem na stercie z nagłówkiem, tylko dokładnie swoimi polami obok siebie: `size_of::<(i32, i32)>()` to 8.

Dwie krańcowe długości mają w Ruscie własne znaczenie. `()` to pusta krotka, o rozmiarze 0, nazywana typem jednostkowym (*unit type*) — zwraca ją każda funkcja bez `->` i w nią właśnie średnik zamienia wyrażenie. Kiedy zrozumiesz, że `()` to krotka, przestaje dziwić cała rodzina komunikatów w rodzaju *„expected `()`, found `i32`”*: kompilator mówi wtedy, że zostawiłeś wartość tam, gdzie spodziewał się instrukcji. Na drugim krańcu `(7)` to zwykłe 7 w nawiasach, a `(7,)` to krotka jednoelementowa — reguła przecinka jest identyczna jak w Pythonie i to akurat przenosi się jeden do jednego.

Krotki porównuje się leksykograficznie, pole po polu, od lewej: `(1, ..)` wyprzedza `(2, ..)` bez zaglądania w nazwiska, a remis między dwiema dwójkami rozstrzyga `"Ada" < "Cara"`. Krotka jest więc **darmowym kluczem sortowania** — postaw na początku to pole, po którym chcesz sortować, i `sort()` zrobi resztę; ta sama reguła rządzi `#[derive(PartialOrd, Ord)]` na strukturze, gdzie kolejność deklaracji pól jest kolejnością porównywania. I na koniec pułapka, która tej wygodzie towarzyszy: `.0` to komentarz, którego nikt nie napisał. Znaczenie pól mieszka w głowie czytelnika i **nie podróżuje** razem z wartością, a krotka o polach tego samego typu daje się przestawić zupełnie bez błędu — w ćwiczeniu z tej strony zamiana `score` z `postal` została wykryta tylko dlatego, że jedno jest `u8`, a drugie `bool`; dwa pola `u8` zamienione miejscami skompilują się bez słowa i policzą źle. Zasada, która przed tym chroni, jest prosta: **dwa pola rozpakowane na miejscu — krotka; cokolwiek, co wędruje dalej — nazwij pola w strukturze.** W zamian `#[derive(Debug)]` wypisze, co jest czym.

**Szukaj po polsku:** krotka w Ruscie · typ jednostkowy · rozpakowanie krotki · `rust tuple vs struct` · `rust expected () found i32`
