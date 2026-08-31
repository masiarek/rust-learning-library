# Letting the compiler reorder a float sum

**Level:** 201 → 301 · deep dive

**One line:** `a + b + c` means `(a + b) + c` and nothing else, because moving the parentheses changes the answer — and Rust 1.98 added five methods that hand the choice of parentheses back to the optimizer.

```rust
// Needs Rust 1.98 or newer.
fn main() {
    let scores = [0.1_f64; 10];

    let strict = scores.iter().copied().fold(0.0, |a, b| a + b);
    let free = scores.iter().copied().fold(0.0_f64, f64::algebraic_add);

    println!("{strict:.17}");  // 0.99999999999999989 — this build and every other
    println!("{free:.17}");    // 0.99999999999999989 unoptimized; 1.00000000000000000 under -O
}
```

Two folds, the same ten numbers, one operation each. The first is pinned to one answer forever. The second is a request.

## Ten tenths do not make one

[`0.1` is not 0.1](../what_a_float_stores/README.md) — it is stored as 0.100000000000000005551115123126, a hair too big. Add ten of those left to right and the running total keeps getting rounded to the nearest `f64`, and the roundings do not cancel:

```text
left to right = 0.99999999999999988898   == 1.0 ? false
in pairs      = 1.00000000000000000000   == 1.0 ? true
```

Both lines are ten additions of the same value. Only the parentheses differ: `t + t + t + …` versus `((t+t) + (t+t)) + ((t+t) + (t+t)) + (t+t)`. Neither is a bug and neither is "more correct" — each is the exactly-rounded result of the operations it actually performed. **Where you put the parentheses decides which errors cancel.**

That is what non-associativity means, and it is not a rounding-scale curiosity. Three values are enough to lose a whole number:

```text
(1e16 + -1e16) + 1 = 1
1e16 + (-1e16 + 1) = 0
```

`-1e16 + 1` is `-1e16` again — 1 is far below the last bit of a number that size, so it rounds away. Multiplication misbehaves the same way, with range instead of precision:

```text
(1e300 * 1e300) * 1e-300 = inf
1e300 * (1e300 * 1e-300) = 1e300
```

## Why `+` will not do it for you

Because of the above, `+` is left-associative and the compiler must honour it exactly. That guarantee costs real speed:

- A sum over a slice is a **serial dependency chain**. Each add waits for the previous one. A modern core can issue several additions at once and has vector registers holding four or eight `f64` at a time, and none of that is reachable while the order is fixed.
- Regrouping into partial sums is what unlocks it — four running totals summed at the end, or one vector accumulator. That is the transformation `-O` cannot apply to your `+`.

So the loop stays scalar. Not because the optimizer is weak, but because it is forbidden to change your answer, and reordering is a change to your answer.

## The five methods

Rust 1.98 stabilized the opt-in, one method per operator:

| method | operator | what it permits |
|---|---|---|
| `algebraic_add` | `+` | regrouping and reassociation |
| `algebraic_sub` | `-` | same |
| `algebraic_mul` | `*` | same, plus fusing a following add into one FMA |
| `algebraic_div` | `/` | same, plus turning `x / c` into `x * (1/c)` |
| `algebraic_rem` | `%` | same |

They exist on `f32` and `f64` (and on the still-unstable `f16` / `f128`), and each is a `const fn`. The [release notes ↗](https://blog.rust-lang.org/2026/08/20/Rust-1.98.0/) put the promise in one sentence: *the exact set of optimizations is not specified*, and the results are *non-deterministic, since the compiler is free to choose different optimizations, but they never cause undefined behavior*.

## Permission, not instruction

The example below is compiled the way every example in this library is compiled — no `-O` — and its `algebraic_add` fold returns exactly what the `+` fold returned. Nothing reordered anything, because at `-O0` there is no optimizer to spend the permission.

Compile the same file with optimization and the second line moves:

```text
$ rustc --edition 2024 -O letting_the_compiler_reorder.rs -o /tmp/lcr && /tmp/lcr
   fold with `+`             = 0.99999999999999988898
   fold with `algebraic_add` = 1.00000000000000000000
```

Same source, same input, same compiler, two answers. That is the feature working, and it is also the whole cost of using it.

## What you trade

You give up **reproducibility across builds**, not correctness. Every value `algebraic_add` can return is the exactly-rounded result of *some* legal grouping of your numbers — so the answer stays inside the window the groupings span, and on this page that window is about 1.1e-16 wide. What you can no longer do is predict which end of it you get. The choice can differ between debug and release, between compiler versions, between target CPUs, and between one inlining decision and another.

Two habits follow:

- **Assert on a tolerance, never on equality.** `(x - 0.8).abs() < 1e-12` survives any regrouping; `x == 0.8` is a bet on the optimizer.
- **If you need a specific grouping, write the grouping.** `algebraic_add` cannot be asked for pairwise summation — it can only be told that pairwise summation is allowed. A hand-written pairwise fold gives you the accuracy *and* keeps the answer fixed. The kata below does exactly that.

## Where this is the wrong tool

- **Money, ballots, anything auditable.** A total that changes when someone flips on LTO is not a total. Count in integers instead, the way [`what_a_float_stores`](../what_a_float_stores/README.md) argues.
- **Regression tests with recorded expectations.** This library's own answer keys are the example: every page here claims what a program *printed*, so a value the optimizer is free to choose could not be recorded in one.
- **Anything you have not measured.** The methods buy vectorization in hot numeric loops. In a loop that runs eleven times they buy nothing and cost you a guarantee.

The fit is the opposite case: a reduction over a large array where the input is approximate anyway — a norm, a dot product, a mean over sensor samples — and the last bit was never carrying information.

## If you are coming from another language

### Python

`sum()` looks like the same left-to-right fold, and until recently it was. It is not any more:

```python
xs = [0.1] * 10

total = 0.0
for x in xs:      # the explicit fold — Rust's `+`
    total += x
print(total)      # 0.9999999999999999

print(sum(xs))    # 1.0
print(math.fsum(xs))  # 1.0
```

CPython 3.12 [changed `sum()` ↗](https://docs.python.org/3/library/functions.html#sum) to *"an algorithm that gives higher accuracy and better commutativity on most builds"* — it carries a compensation term rather than reassociating, so it beats both groupings on this page. `math.fsum()` goes further and is exactly rounded. NumPy's `np.sum` reaches the same 1.0 by a third route, pairwise summation.

So Python already offers you the whole spectrum, chosen by which name you call — and, crucially, **each name is a fixed algorithm**. `sum(xs)` returns the same bits on every machine that runs it. The thing that transfers is the intuition that summation order is a decision; the thing that does not is who makes it. Rust's `algebraic_add` is not `math.fsum` and not `np.sum`: it is a *delegation*, and the delegate may answer differently tomorrow. Note also which way each language chose to move by default — CPython made its ordinary `sum()` quietly more accurate, while Rust keeps `+` bit-exact and makes you ask.

### ABAP

The reason a lifetime of FI code never hit this: `TYPE p DECIMALS 2` is **packed decimal**, base ten, so `0.1` is exactly 0.1 and no grouping of any number of additions can drift. That is not luck or care — it is the type, and it is why `p` is the amount type and `f` is not.

`TYPE f` is IEEE binary floating point, the identical thing as Rust's `f64`, and it has this problem in full. So the bridge is exact: **`f64` is ABAP's `f`, and Rust's standard library has no `p`.** When you would have reached for `TYPE p DECIMALS 2`, reach for an integer count of cents, or a decimal crate — not for `f64` with a rounding step bolted on.

What is new is the direction of the switch. ABAP gives you no way to say "you may reorder this", because with `p` there is nothing to gain by reordering. Rust's `algebraic_add` is a control that only exists for the binary-float type, and only makes sense for a workload where you already accepted approximation.

### C and C++

`-ffast-math` is the same permission, and comparing them shows what Rust deliberately left out.

| | `-ffast-math` | `algebraic_add` |
|---|---|---|
| scope | whole translation unit | one call |
| granularity | all float ops in it | the op you wrote it on |
| assumes no NaN/infinity | yes (`-ffinite-math-only`) | no |
| can produce undefined behavior | yes, once NaN or infinity does arrive | no |
| affects code you did not write | yes — including, on some toolchains, process-wide flush-to-zero | no |

The rows that matter are the last three. `-ffast-math` includes a *claim* — that NaN and infinity never occur — and a program that breaks that claim gets nonsense with no diagnostic. Rust's methods make no such claim: NaN and infinity stay ordinary values, the result is always a real `f32` or `f64`, and the [API proposal ↗](https://github.com/rust-lang/libs-team/issues/532) picked the word *algebraic* over *fast* precisely to mark the difference from the older `f*_fast` intrinsics, which could return poison. You are buying reassociation, not a set of assumptions about your data.

## What the program prints

<!-- output:letting_the_compiler_reorder -->
*Verified output of [`letting_the_compiler_reorder.rs`](examples/letting_the_compiler_reorder.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. THE SAME TEN NUMBERS, TWO GROUPINGS
   left to right = 0.99999999999999988898   == 1.0 ? false
   in pairs      = 1.00000000000000000000   == 1.0 ? true
   nothing was rounded twice and nothing is a bug: 0.1 is not 0.1,
   so where you put the parentheses decides which errors cancel.

2. WHY THE COMPILER IS NOT ALLOWED TO CHOOSE FOR YOU
   (1e16 + -1e16) + 1 = 1
   1e16 + (-1e16 + 1) = 0
   three values, one operator, two answers — one of which lost the 1.
   the same holds for multiplication, where the gap is a range limit:
   (1e300 * 1e300) * 1e-300 = inf
   1e300 * (1e300 * 1e-300) = 1e300
   so `+` and `*` are evaluated left-associatively, always, and an
   optimizer that regrouped a loop for speed would be changing results.

3. THE 1.98 METHODS SAY 'GO AHEAD'
   fold with `+`             = 0.99999999999999988898
   fold with `algebraic_add` = 0.99999999999999988898
   identical here — and that is the lesson, not a failure.
   this program is built without -O, so nothing reordered anything.
   the methods grant permission; the optimizer is what spends it.

4. THE FIVE OF THEM
   algebraic_add  algebraic_sub  algebraic_mul  algebraic_div  algebraic_rem
   on f32 and f64 alike:
     2.5_f32.algebraic_mul(4.0) = 10
     7.0_f64.algebraic_div(2.0) = 3.5
     7.0_f64.algebraic_rem(2.0) = 1
   each returns a real f32/f64, never a poison value and never UB.
   what it does not return is a promise about WHICH valid answer.
```
<!-- /output -->

## Practice

**Make the sum land on the number you would have typed.** Eight ballot scores of `0.1`, and `0.8_f64` as the target.

1. Fold them left to right and compare to `0.8` with `==`. It fails.
2. Write `sum_pairwise(&[f64]) -> f64` that halves the slice and recurses, and compare again. It passes — the grouping the optimizer would have chosen is the one that lands on the literal.
3. Now swap the left-to-right `+` for `algebraic_add` and predict the result *before* running it. Then explain the answer you get in one sentence.
4. Finally, write the assertion you would actually ship — one that holds no matter which grouping the compiler picks.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:letting_the_compiler_reorder_kata -->
*[`letting_the_compiler_reorder_kata.rs`](examples/letting_the_compiler_reorder_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata: sum eight tenths so the answer equals the 0.8 you would have typed.
//!
//!   rustc --edition 2024 letting_the_compiler_reorder_kata.rs -o /tmp/lcrk && /tmp/lcrk

/// Add a slice by halving it, so no partial sum ever gets far ahead of the
/// next value it has to absorb. Order is still fixed — the grouping is just
/// a different fixed one.
fn sum_pairwise(xs: &[f64]) -> f64 {
    match xs.len() {
        0 => 0.0,
        1 => xs[0],
        n => {
            let (left, right) = xs.split_at(n / 2);
            sum_pairwise(left) + sum_pairwise(right)
        }
    }
}

fn main() {
    let scores = [0.1_f64; 8];

    let left_to_right = scores.iter().copied().fold(0.0, |a, b| a + b);
    let pairwise = sum_pairwise(&scores);
    let algebraic = scores.iter().copied().fold(0.0_f64, f64::algebraic_add);

    println!("PART 1 — two groupings, one target");
    println!("  0.8 as written    = {:.20}", 0.8_f64);
    println!("  left to right     = {left_to_right:.20}   == 0.8 ? {}", left_to_right == 0.8);
    println!("  pairwise          = {pairwise:.20}   == 0.8 ? {}", pairwise == 0.8);
    println!("  the pairwise grouping is the one the optimizer would reach for,");
    println!("  and on this input it happens to land on the literal exactly.");

    println!("\nPART 2 — does asking for it get it?");
    println!("  algebraic_add     = {algebraic:.20}   == 0.8 ? {}", algebraic == 0.8);
    println!("  no. `algebraic_add` permits the regrouping, it does not perform it,");
    println!("  and an unoptimized build has no reason to bother. If you need a");
    println!("  particular grouping, write the grouping — that is what PART 1 did.");

    println!("\nPART 3 — what you may rely on");
    let bounds = [left_to_right.min(pairwise), left_to_right.max(pairwise)];
    println!("  every legal answer sits in [{:.20}, {:.20}]", bounds[0], bounds[1]);
    println!("  width of that window = {:e}", bounds[1] - bounds[0]);
    println!("  so an assertion on the WINDOW holds under any regrouping:");
    println!("    (algebraic - 0.8).abs() < 1e-12  ->  {}", (algebraic - 0.8).abs() < 1e-12);
    println!("    algebraic == 0.8                 ->  {}   (a bet on the optimizer)", algebraic == 0.8);
}
```
<!-- /source -->

<!-- output:letting_the_compiler_reorder_kata -->
*Verified output of [`letting_the_compiler_reorder_kata.rs`](examples/letting_the_compiler_reorder_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
PART 1 — two groupings, one target
  0.8 as written    = 0.80000000000000004441
  left to right     = 0.79999999999999993339   == 0.8 ? false
  pairwise          = 0.80000000000000004441   == 0.8 ? true
  the pairwise grouping is the one the optimizer would reach for,
  and on this input it happens to land on the literal exactly.

PART 2 — does asking for it get it?
  algebraic_add     = 0.79999999999999993339   == 0.8 ? false
  no. `algebraic_add` permits the regrouping, it does not perform it,
  and an unoptimized build has no reason to bother. If you need a
  particular grouping, write the grouping — that is what PART 1 did.

PART 3 — what you may rely on
  every legal answer sits in [0.79999999999999993339, 0.80000000000000004441]
  width of that window = 1.1102230246251565e-16
  so an assertion on the WINDOW holds under any regrouping:
    (algebraic - 0.8).abs() < 1e-12  ->  true
    algebraic == 0.8                 ->  false   (a bet on the optimizer)
```
<!-- /output -->

</details>

## See also

- [What a float actually stores](../what_a_float_stores/README.md) — why `0.1` is not 0.1, and the two traits Rust withholds because of it
- [Numbers and bytes](../README.md) — the rest of this section
- [`f64::algebraic_add` ↗](https://doc.rust-lang.org/std/primitive.f64.html#method.algebraic_add) — the reference page for all five

## Po polsku

Ze szkoły pamiętasz dwie własności dodawania: przemienność i łączność (*associativity*). Dla liczb zmiennoprzecinkowych (*floating-point*) zostaje tylko pierwsza — `a + b` to wciąż `b + a` (poza `NaN`), ale `(a + b) + c` i `a + (b + c)` to dwie różne liczby, i o tym jest cała ta lekcja. Te same dziesięć dziesiątych zsumowane od lewej dają 0.99999999999999988898, a zsumowane parami — równo 1.0. Żaden z tych wyników nie jest błędem: każdy jest poprawnie zaokrąglonym rezultatem dokładnie tych działań, które wykonano, a nawiasy decydują o tym, które błędy zaokrągleń się nawzajem zniosą.

Dlatego `+` w Ruscie jest lewostronnie łączne i kompilator musi to uszanować co do bitu. To nie nadgorliwość — trzy wartości wystarczą, żeby zgubić całą jedynkę: `(1e16 + -1e16) + 1` to 1, a `1e16 + (-1e16 + 1)` to 0, bo jedynka leży daleko poniżej ostatniego bitu liczby tego rzędu i po prostu się zaokrągla do zera. Cena tej gwarancji jest realna: sumowanie wycinka to szeregowy łańcuch zależności, każde dodawanie czeka na poprzednie, więc rejestry wektorowe trzymające po cztery czy osiem `f64` stoją bezczynnie. Optymalizator nie jest tu słaby — ma zakaz.

Rust 1.98 dodał pięć metod (`algebraic_add`, `algebraic_sub`, `algebraic_mul`, `algebraic_div`, `algebraic_rem`) i najważniejsze słowo brzmi: **pozwolenie, a nie polecenie**. Bez `-O` nic się nie przegrupuje i przykład z tej strony zwraca dokładnie to samo co zwykłe `+`; dopiero z optymalizacją druga linijka przeskakuje na 1.0. Z C znasz to jako `-ffast-math` i różnica jest zasadnicza: `-ffast-math` działa na całą jednostkę kompilacji i dodatkowo **zakłada**, że `NaN` i nieskończoności nie wystąpią, więc program, który to założenie łamie, dostaje bzdurę bez jednego ostrzeżenia. Metody Rusta nie zakładają niczego — wynik zawsze jest prawdziwym `f32` albo `f64`, nigdy niezdefiniowanym zachowaniem, i słowo *algebraic* zamiast *fast* w nazwie postawiono właśnie po to.

Oddajesz nie poprawność, tylko **powtarzalność między kompilacjami**. Każda wartość, jaką `algebraic_add` może zwrócić, jest poprawnie zaokrąglonym wynikiem jakiegoś dozwolonego pogrupowania, więc odpowiedź na pewno wyląduje w oknie — w zadaniu na końcu strony ma ono szerokość 1.11e-16 — ale nie wiadomo, na którym jego końcu, i wybór może się różnić między kompilacją debug a release, między wersjami kompilatora i między procesorami. Stąd dwa nawyki: asercja na tolerancję (`(x - 0.8).abs() < 1e-12`), nigdy na `==`; a jeśli potrzebujesz konkretnego pogrupowania, to je napisz, bo `algebraic_add` można tylko na nie pozwolić, a nie o nie poprosić. Do pieniędzy, głosów i wszystkiego, co ktoś kiedyś zaudytuje, ta metoda nie służy w ogóle — suma, która zmienia się po włączeniu LTO, nie jest sumą; tam liczy się na liczbach całkowitych.

**Szukaj po polsku:** łączność dodawania · liczby zmiennoprzecinkowe · błąd zaokrąglenia · `rust algebraic_add` · `floating point associativity ffast-math`
