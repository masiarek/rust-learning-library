# Array or `Vec`?

**Level:** 101 → 201 · for newcomers

**One line:** Use a `Vec` unless the length is a fact about the **problem** rather than about this run — and know the four things an array buys when it is, because "arrays are less flexible" undersells all of them.

```rust
const SCALE: usize = 6;                  // 0..=5, decided by the ballot design
let histogram = [0u32; SCALE];           // a fact about STAR, not about today
let candidates = vec!["Ada", "Ben"];     // a fact about this election
println!("{} {}", histogram.len(), candidates.len());   // 6 2
```

The usual advice — *"if you're unsure, use a `Vec`"* — is right, and it is right for a reason worth knowing: almost every length in a real program comes from **input**, and input length is not knowable when you compile. So the default follows from the data, not from `Vec` being the better type.

## What is actually different

| | `[T; N]` | `Vec<T>` |
|---|---|---|
| length | in the **type** — `[u32; 5]` and `[u32; 3]` are different types | in the **value**, and changeable |
| where the data lives | inline, wherever the array is — usually the stack | on the heap; three numbers on the stack point at it |
| `size_of` | `N * size_of::<T>()`, no header | pointer + length + capacity, whatever `N` is |
| grow / shrink | never | `push`, `pop`, `insert`, `retain` |
| `Copy` | yes, if `T: Copy` | never |
| in a `const` | yes | no — allocation needs a run time |
| needs an allocator | no | yes |

`size_of::<[u32; 5]>()` is 20 — five values and nothing else — while `size_of::<Vec<u32>>()` is three pointer-sized numbers no matter how many elements it holds. Both are measured in the [verified output](#the-verified-output) rather than quoted.

## The four things an array buys

**1. The length is part of the type, so the wrong one cannot be passed.** A function taking `[u8; 32]` cannot be handed 31 bytes, and a function taking `[u32; 6]` cannot be handed a five-bucket histogram:

```text title="Abridged — real rustc output for arrlen.rs"
error[E0308]: mismatched types
 --> arrlen.rs:3:30
  |
3 |     println!("{}", summarise([1, 2, 3, 4, 5]));
  |                    --------- ^^^^^^^^^^^^^^^ expected an array with a size of 6, found one with a size of 5
```

The equivalent `Vec` mistake compiles and produces a report that is merely wrong. This is the case people mean by "a hash is `[u8; 32]`, not `Vec<u8>`" — the length is not a quantity, it is part of what the thing *is*.

**2. An array of `Copy` types is `Copy`.** So a snapshot costs nothing to say:

```rust
let mut live = [4u32, 0, 2];
let snapshot = live;      // a real copy — no .clone(), and `live` is still usable
live[0] += 100;
println!("{live:?} {snapshot:?}");   // [104, 0, 2] [4, 0, 2]
```

The same two lines over a `Vec` are `error[E0382]: borrow of moved value`, and the fix is `.clone()` — a heap allocation you now know you made. That is the trade rather than a wart: the `Vec` version makes the cost visible, the array version makes it zero.

**3. It works in a `const`.** `const WEIGHTS: [u32; 6] = [0, 1, 2, 3, 4, 5];` is a compile-time value with no initialisation code; `const W: Vec<u32> = vec![…]` cannot exist, because there is no allocator at compile time. Lookup tables, opcode tables, day-length tables — all arrays for this reason.

**4. No allocator at all.** Which is why `no_std` and embedded Rust are written in arrays, and why a hot loop that must not touch the heap uses one.

## Where the `Vec` is simply right

The length came from somewhere you cannot see at compile time — and that is most lengths:

```rust
let ballots = ["5,3,0", "4,4,4"];
let n = ballots.len();
// let counts = [0u32; n];   // error[E0435]: attempt to use a non-constant value in a constant
let counts = vec![0u32; n];  // the answer
println!("{}", counts.len());   // 2
```

An array length is a **constant**, full stop. `n` came from the input, so `vec![0; n]` is not a fallback — it is the only thing that expresses what you meant. Add to that: anything that grows or shrinks, anything returned from a function whose size depends on its arguments, and anything large, since an array lives on the stack and `[u8; 10_000_000]` is a stack overflow rather than an error message.

## The question mostly dissolves at the signature

```rust
fn total(counts: &[u32]) -> u32 { counts.iter().sum() }
let fixed = [4u32, 0, 2];
let grown = vec![4u32, 0, 2];
println!("{} {}", total(&fixed), total(&grown));   // 6 6
```

Take `&[T]` and both callers work — a `Vec` derefs to a slice, an array coerces to one. So the choice is about **how you store the data**, not about what your functions accept, and getting it wrong is a local decision you can change later without touching a single signature. [Arrays and slices](../arrays_and_slices/README.md) is that rule in full, and its kata is the one that shows which callers a fixed-length signature turns away.

## If you are coming from another language

**Python.** There is nothing to choose here, because Python's `list` is `Vec` and there is no array in the language — `[1, 2, 3]` grows, and always has. The nearest counterparts to a Rust array are `tuple` (fixed length, but heterogeneous and immutable) and `array.array` / `numpy.ndarray` (fixed element type, still heap, still growable in NumPy's case only by copying).

The habit that transfers badly is reaching for the list every time, which in Rust means a heap allocation for a three-element lookup table that could have been a `const`. And one that transfers well: Python's own advice for the fixed case is `tuple`, for the same reason Rust's is an array — *this collection's shape is part of what it is*. A Python programmer who already writes `RGB = (255, 0, 0)` rather than a list has the instinct; in Rust it becomes `const RGB: [u8; 3]`, and the compiler enforces it rather than relying on the reader.

| Python | Rust |
|---|---|
| `[1, 2, 3]` — grows | `vec![1, 2, 3]` |
| `(1, 2, 3)` — fixed, heterogeneous | `(1, 2, 3)` — a [tuple](../tuples/README.md), same idea |
| no fixed-length homogeneous type | `[1u32, 2, 3]` — an array |
| `len()` on either | `.len()` on either |
| a list of a million ints — a million heap objects | `Vec<i64>` — one allocation, a million machine words |

**ABAP.** The split maps onto ABAP's own, and closely enough to be useful. A `Vec<T>` is an **internal table** (`DATA lt_scores TYPE STANDARD TABLE OF i`) — it grows with `APPEND`, shrinks with `DELETE`, and lives on the heap. A Rust array is closer to a **fixed-length structure** or a `TYPE c LENGTH n` field: the length is part of the type, declared once, and the runtime will not let you extend it.

Two differences worth having in mind. First, ABAP has no `Copy` distinction — assigning an internal table copies it (with copy-on-write underneath), so the `let snapshot = live;` question does not arise; ABAP's version of the choice is `DATA(lt_copy) = lt_orig` versus `FIELD-SYMBOLS` / reference variables, which is *borrowing*, not array-versus-`Vec`. Second, ABAP's tables carry the feature Rust puts in a different type entirely: `SORTED` and `HASHED` tables with a `KEY`, which in Rust are [`BTreeMap`](../sorted_collections/README.md) and [`HashMap`](../the_hashmap/README.md) rather than a flavour of `Vec`. So an ABAP developer choosing "which table type" is answering a question Rust splits across three types, and the array/`Vec` decision is only the first third of it.

---

## Practice

**Two lengths in one election, and only one of them is a fact about the problem.**

A STAR ballot scores 0–5 — six buckets, in every election ever held. The candidate list is different every time. Build the score histogram and the per-candidate totals, choosing the storage for each, and count four ballots into them.

Then show the three consequences of the choice. Take a snapshot of the histogram mid-count with a plain `let`, and say why it works here and what rustc says when you try it on the `Vec`. Write a `total` that takes the fixed length, and name the error a five-bucket argument produces. Finally, try to size the per-candidate array from `candidates.len()` and read what the compiler calls that.

Finish by passing both to the same function.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:array_or_vec_kata -->
*[`array_or_vec_kata.rs`](examples/array_or_vec_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one election has two lengths, and only one of them is a fact
//! about the problem rather than about this run.

const SCALE: usize = 6; // 0..=5 -- decided by the ballot design, not by the input

fn print_histogram(counts: &[u32]) -> String {
    counts.iter().enumerate().map(|(s, n)| format!("{s}={n}")).collect::<Vec<_>>().join("  ")
}

fn main() {
    println!("=== the two lengths ===");
    println!("  the score scale is 0..=5 in every STAR election ever held  -> [u32; {SCALE}]");
    println!("  the candidate list is different every time                 -> Vec<String>");

    let candidates: Vec<String> = ["Ada", "Ben", "Cara"].iter().map(|s| s.to_string()).collect();
    let ballots = ["5,3,0", "4,4,4", "0,0,5", "2,5,1"];
    println!("  this election: {} candidates, {} ballots", candidates.len(), ballots.len());

    println!("\n=== count into the fixed-length one ===");
    let mut histogram = [0u32; SCALE];
    for line in ballots {
        for field in line.split(',') {
            histogram[field.parse::<usize>().unwrap()] += 1;
        }
    }
    println!("  histogram = {}", print_histogram(&histogram));

    println!("\n=== consequence 1: the array is Copy, so a snapshot is free ===");
    let before = histogram;          // no .clone(), no move
    histogram[5] += 1000;            // the original goes on changing
    println!("  before    = {}", print_histogram(&before));
    println!("  after     = {}", print_histogram(&histogram));
    println!("  the same two lines over a Vec do not compile. rustc opens with");
    println!("    error[E0382]: borrow of moved value: `live`");
    println!("  and closes with");
    println!("    help: consider cloning the value if the performance cost is acceptable");
    println!("  between them it says the move happened because Vec<u32> is not Copy.");
    println!("  So the fix is `let before = live.clone();` -- a heap allocation you");
    println!("  now know you made, which is the difference the two types are for.");
    histogram[5] -= 1000;

    println!("\n=== consequence 2: a wrong length is a compile error, not a bad report ===");
    fn total(counts: [u32; SCALE]) -> u32 { counts.iter().sum() }
    println!("  total(histogram) = {}", total(histogram));
    println!("  total([0u32; 5]) -> error[E0308]: mismatched types");
    println!("                      expected an array with a size of 6, found one with a size of 5");
    println!("  a Vec of the wrong length would have produced a report that was merely wrong");

    println!("\n=== consequence 3: a run-time length cannot be an array length ===");
    let n = candidates.len();
    let mut per_candidate = vec![0u32; n];   // the only thing that works here
    for line in ballots {
        for (i, field) in line.split(',').enumerate() {
            per_candidate[i] += field.parse::<u32>().unwrap();
        }
    }
    for (name, total) in candidates.iter().zip(&per_candidate) {
        println!("  {name:<5} {total}");
    }
    println!("  `let c = [0u32; n];` -> error[E0435]: attempt to use a non-constant value in a constant");
    println!("  an array length is a constant; n came from the input, so it is vec![0; n]");

    println!("\n=== and both end up at the same signature ===");
    println!("  print_histogram(&histogram)     -> {}", print_histogram(&histogram));
    println!("  print_histogram(&per_candidate) -> {}", print_histogram(&per_candidate));
    println!("  one fn taking &[u32], two callers -- the decision was about storage, not API");
}
```
<!-- /source -->

<!-- output:array_or_vec_kata -->
*Verified output of [`array_or_vec_kata.rs`](examples/array_or_vec_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== the two lengths ===
  the score scale is 0..=5 in every STAR election ever held  -> [u32; 6]
  the candidate list is different every time                 -> Vec<String>
  this election: 3 candidates, 4 ballots

=== count into the fixed-length one ===
  histogram = 0=3  1=1  2=1  3=1  4=3  5=3

=== consequence 1: the array is Copy, so a snapshot is free ===
  before    = 0=3  1=1  2=1  3=1  4=3  5=3
  after     = 0=3  1=1  2=1  3=1  4=3  5=1003
  the same two lines over a Vec do not compile. rustc opens with
    error[E0382]: borrow of moved value: `live`
  and closes with
    help: consider cloning the value if the performance cost is acceptable
  between them it says the move happened because Vec<u32> is not Copy.
  So the fix is `let before = live.clone();` -- a heap allocation you
  now know you made, which is the difference the two types are for.

=== consequence 2: a wrong length is a compile error, not a bad report ===
  total(histogram) = 12
  total([0u32; 5]) -> error[E0308]: mismatched types
                      expected an array with a size of 6, found one with a size of 5
  a Vec of the wrong length would have produced a report that was merely wrong

=== consequence 3: a run-time length cannot be an array length ===
  Ada   11
  Ben   12
  Cara  10
  `let c = [0u32; n];` -> error[E0435]: attempt to use a non-constant value in a constant
  an array length is a constant; n came from the input, so it is vec![0; n]

=== and both end up at the same signature ===
  print_histogram(&histogram)     -> 0=3  1=1  2=1  3=1  4=3  5=3
  print_histogram(&per_candidate) -> 0=11  1=12  2=10
  one fn taking &[u32], two callers -- the decision was about storage, not API
```
<!-- /output -->

</details>

## The verified output

<!-- output:array_or_vec -->
*Verified output of [`array_or_vec.rs`](examples/array_or_vec.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== what is actually different ===
  size_of::<[u32; 5]>()              20
    five u32 values                  20   <- five values inline, no header
  size_of::<Vec<u32>>() == 3 * usize true
     a Vec is pointer + length + capacity on the stack; the data is elsewhere
  size_of::<[u32; 5000]>()           20000
     ...and that whole block is on the STACK when you make one

=== case 1: the length is part of the type, so a wrong one cannot be passed ===
  summarise([u32; 6]) = 23
  summarise([u32; 5]) -> error[E0308]: mismatched types
                         expected an array with a size of 6, found one with a size of 5
  a Vec<u32> of the wrong length is a run-time surprise; this is a compile error

=== case 2: an array of Copy types is Copy ===
  live     = [104, 0, 2, 1, 7, 9]
  snapshot = [4, 0, 2, 1, 7, 9]   <- untouched, and `live` is still usable
  Vec needs .clone() for the same thing: [4, 0, 2]

=== case 3: an array works in a const context; a Vec does not ===
  const WEIGHTS: [u32; 6]  = [0, 1, 2, 3, 4, 5]
  const TOP_WEIGHT         = 5   <- indexed at compile time
  `const W: Vec<u32> = vec![..]` does not compile: allocation needs a run time

=== case 4: no allocation at all ===
  an array needs no allocator, so it is what no_std and embedded code use,
  and what a hot loop uses to avoid touching the heap at all.

=== where the Vec is simply right ===
  parsed 4 ballots -> counts = [3, 1, 1, 1, 3, 3]
  the number of ballots is not known until the input is read, and
  `let n = ballots.len(); let c = [0u32; n];` does not compile --
    error[E0435]: attempt to use a non-constant value in a constant
  an array length is a constant; `vec![0; n]` is the answer

=== and the question mostly dissolves at the signature ===
  report(&fixed) = 0:4 1:0 2:2 3:1 4:7 5:9
  report(&grown) = 0:4 1:0 2:2 3:1 4:7 5:9
  one function, `&[u32]`, and both callers work -- so the choice is about
  how you STORE the data, not about what your functions accept
```
<!-- /output -->

## See also

- [Arrays and slices](../arrays_and_slices/README.md) — the array in full, and why `&[T]` is what belongs in a signature
- [`Vec`](../the_vec/README.md) — the three numbers, the doubling growth, and the two removals
- [Tuples](../tuples/README.md) — the other fixed-length option, for values of *different* types
- [Copy vs Clone](../../16_Structs/copy_vs_clone/README.md) — why an array of `Copy` is `Copy` and a `Vec` never is
- [Values](../../15_First_Programs/values/README.md) — where the widths in the `size_of` table come from
- [`BTreeMap` and `BTreeSet`](../sorted_collections/README.md) — when the thing you wanted was a keyed table, not a list

## Po polsku

Domyślną odpowiedzią jest `Vec`, a powód warto znać, bo nie brzmi „tablice są gorsze”: **prawie każda długość w prawdziwym programie pochodzi z danych wejściowych**, a długości wejścia nie da się poznać w czasie kompilacji. Tablica `[T; N]` ma długość **w typie** — `[u32; 5]` i `[u32; 3]` to dwa różne typy — leży w miejscu, w którym leży sama tablica (zwykle na stosie), nigdy nie rośnie i jest `Copy`, o ile `T` jest `Copy`. `Vec<T>` trzyma na stosie trzy liczby (wskaźnik, długość, pojemność), a dane na stercie, rośnie i kurczy się, i `Copy` nie jest nigdy.

Cztery rzeczy, które kupuje tablica, i żadna z nich nie jest tylko „mniejszą elastycznością”. Po pierwsze, **długość jest częścią typu**, więc złej nie da się przekazać: funkcja biorąca `[u8; 32]` nie przyjmie 31 bajtów, a błąd brzmi *„expected an array with a size of 6, found one with a size of 5”*. Ten sam błąd popełniony na `Vec` skompiluje się i wyprodukuje po prostu zły raport — dlatego skrót mieszający jest typu `[u8; 32]`, a nie `Vec<u8>`: długość nie jest tam wielkością, tylko częścią tego, czym ta rzecz jest. Po drugie, **tablica typów `Copy` sama jest `Copy`**, więc `let snapshot = live;` robi prawdziwą kopię, a oryginał dalej działa; te same dwie linijki na `Vec` to `error[E0382]: borrow of moved value` i trzeba dopisać `.clone()` — czyli alokację, o której teraz wiadomo. Po trzecie, **tablica działa w kontekście `const`** (`const WEIGHTS: [u32; 6] = …`), a `Vec` nie, bo w czasie kompilacji nie ma alokatora — stąd wszystkie tablice przeglądowe są tablicami. Po czwarte, tablica **nie potrzebuje alokatora w ogóle**, dlatego pisze się nimi kod `no_std` i wbudowany.

`Vec` jest po prostu właściwy, gdy długość przyszła skądś, czego nie widać w czasie kompilacji — a to większość przypadków. `let n = ballots.len(); let c = [0u32; n];` nie skompiluje się: `error[E0435]: attempt to use a non-constant value in a constant`. Długość tablicy jest **stałą**, kropka, więc `vec![0; n]` nie jest obejściem, tylko jedynym zapisem tego, o co chodziło. Do tego dochodzi wszystko, co rośnie lub maleje, oraz wszystko duże — tablica leży na stosie, a `[u8; 10_000_000]` to przepełnienie stosu, nie komunikat o błędzie.

Na koniec rzecz, która ten dylemat w dużej mierze rozpuszcza: w sygnaturze pisze się `&[T]` i działają oba wywołania, bo `Vec` dereferencjuje się do wycinka, a tablica się do niego dopasowuje. Wybór dotyczy więc **sposobu przechowywania**, a nie tego, co przyjmują twoje funkcje — i jest decyzją lokalną, którą można później zmienić, nie ruszając ani jednej sygnatury. Dla czytelnika z ABAP-a: `Vec<T>` to tabela wewnętrzna (`STANDARD TABLE OF`), a tablica jest bliżej struktury o stałej długości albo pola `TYPE c LENGTH n`. Uwaga na dwie różnice — w ABAP-ie nie ma rozróżnienia `Copy`, bo przypisanie tabeli kopiuje ją (z kopiowaniem przy zapisie pod spodem), a odpowiednikiem pożyczania są `FIELD-SYMBOLS` i zmienne referencyjne; oraz tabele `SORTED` i `HASHED` z kluczem to w Ruscie nie odmiany `Vec`, tylko osobne typy — `BTreeMap` i `HashMap`. Wybór „rodzaju tabeli” w ABAP-ie odpowiada więc pytaniu, które Rust rozdziela na trzy typy, a dylemat tablica-czy-`Vec` jest tylko pierwszą jego trzecią.

**Szukaj po polsku:** tablica a wektor · długość w typie · typ `Copy` · tablica przeglądowa `const` · `rust array vs Vec` · `rust E0435 non-constant value`
