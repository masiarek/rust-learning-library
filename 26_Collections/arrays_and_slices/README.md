# Arrays and slices

**Level:** 101 → 201 · for newcomers

**One line:** `[T; N]` is a separate type for every length, which is why you almost never write it in a signature — `&[T]` moves the length out of the type and into the value, and one function then serves every caller.

```rust
fn total(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

fn main() {
    let five = [5u32, 3, 0, 4, 2];
    let three = [1u32, 2, 3];
    println!("{} {}", total(&five), total(&three));   // 14 6
    println!("{}", total(&five[1..3]));               // 3
}
```

One function, three callers, and the third is *part of* an array it does not own.

## The array: length in the type

`[u32; 5]` and `[u32; 3]` are as different as `u32` and `String`. The elements sit inline — `size_of::<[u32; 5]>()` is 20, five values and no header — and if `T: Copy` then the array is `Copy` too, so `let mut sorted = five;` copies all five and leaves the original alone.

## The slice: length in the value

| | size | holds |
|---|---|---|
| `&[u32; 5]` | 8 | a pointer |
| `&[u32]` | 16 | a pointer **and** a length |

That second row is a *fat pointer*, and it is the whole trick. The length stopped being compile-time information and became a number carried alongside the address, so a slice can point at an array, a `Vec`, or a run of either.

Ranges are half-open — `five[1..3]` is indices 1 and 2 — and `..` on its own is the whole thing. `&v[..]` is the idiom for "this `Vec`, as a slice".

## Out of bounds is a panic, not a wrong answer

```rust
fn main() {
    let five = [5u32, 3, 0, 4, 2];
    println!("{:?}", five.get(9));   // None
    println!("{:?}", five.get(1));   // Some(3)
}
```

`five[9]` does not compile at all — rustc constant-folds the index and refuses with *"this operation will panic at runtime"*, under the deny-by-default `unconditional_panic` lint. With an index it cannot see, the check happens at run time and the program aborts.

The split is the same one `HashMap` makes: **`[i]` is a claim, `.get(i)` is a question.** C reads past the end and carries on with whatever was there; Rust stops. Every indexing operation costs a comparison, which the optimizer removes wherever it can prove the index is in range — which is most of the time, and is why `for x in &arr` is faster than `for i in 0..arr.len()`.

## The methods are on the slice

Almost nothing is defined on the array type itself. `first`, `last`, `contains`, `sort`, `windows`, `chunks`, `split_at`, `iter` — all of these live on `[T]`, and the array and `Vec` reach them by deref coercion. So learning slice methods once covers both.

```rust
fn main() {
    let five = [5u32, 3, 0, 4, 2];
    println!("{:?}", five.windows(2).collect::<Vec<_>>());
    // [[5, 3], [3, 0], [0, 4], [4, 2]]
    println!("{:?}", five.chunks(2).collect::<Vec<_>>());
    // [[5, 3], [0, 4], [2]]
}
```

`windows` overlaps and never yields a short one; `chunks` does not overlap and the last one may be short. Reaching for the wrong one is the most common off-by-one in this corner of std.

## The trap: `&Vec<T>` in a signature

Writing `fn total(scores: &Vec<u32>)` compiles and looks equivalent. It is not: it refuses arrays, refuses slices, refuses `&v[1..]`, and buys nothing at all, because the only things it can do with a `&Vec` are the things `&[T]` already offers. **Take `&[T]` unless you need to push.** Clippy has a lint for it (`ptr_arg`), which is how most people find out.

## If you are coming from another language

- **Python.** A Python list is Rust's `Vec`, and Rust's array is the thing Python does not have — a fixed-length, stack-allocated block whose length the compiler knows. The slicing syntax is nearly identical and half-open in both, `xs[1:3]` versus `&xs[1..3]`, so the off-by-one instincts transfer. Two real differences: a Python slice **copies**, and a Rust slice **borrows** — `&v[1..]` is a view into `v`, so `v` cannot be mutated while it is alive, which is a compile error rather than the aliasing surprise it would be in Python. And negative indices do not exist: `xs[-1]` is `xs.last()`, which returns `Option` because the list may be empty, and that `Option` is Python's `IndexError` moved from run time to the type.
- **ABAP.** An internal table is closest to `Vec`, and `[T; N]` has no real counterpart — the nearest thing is a fixed-size field like `TYPE c LENGTH 5`, where the length is likewise part of the type. What transfers well is the reading habit: `READ TABLE itab INDEX i` sets `sy-subrc` and you check it, which is exactly `.get(i)` returning `Option`. Rust's `arr[i]` is the version that skips the check and dumps instead — `TABLE_INVALID_INDEX` is the same failure with a different name. The genuine difference is the slice: ABAP has no borrowed view over part of a table, so a helper that works on a range takes `FROM`/`TO` indices and works on the whole table, which is exactly the aliasing bug slices exist to make impossible.
- **C.** An array decaying to a bare pointer is what a slice fixes. `void total(uint32_t *scores, size_t n)` is a slice split into two arguments that the compiler cannot check agree; `&[u32]` is those two arguments welded into one value. Every buffer overrun that has ever come from passing the wrong `n` is unrepresentable here.
- **Java / C#.** `int[]` carries its own length, so it is closer to a slice than to a C array — but there is no view type, so a sub-range means `Arrays.copyOfRange` (a copy) or passing offsets around. `Span<T>` in modern C# is Rust's slice, arrived at from the same direction.

---

## The verified output

<!-- output:arrays_and_slices -->
*Verified output of [`arrays_and_slices.rs`](examples/arrays_and_slices.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The length is part of the type
   five  : [u32; 5] = [5, 3, 0, 4, 2]
   three : [u32; 3] = [1, 2, 3]
   [u32; 5] and [u32; 3] are as different as u32 and String. A
   function taking [u32; 5] rejects a four-element array.
   size_of::<[u32; 5]>() = 20  — five values, no header

2. A slice is a view: pointer plus length, and no ownership
   size_of::<&[u32; 5]>() = 8  — a plain pointer
   size_of::<&[u32]>()    = 16 — pointer AND length
   The length moved out of the type and into the value, which is
   why one function can serve every length.
   total(&five) = 14, total(&three) = 6
   total(&five[1..3]) = 3 — same function, part of the array

3. Ranges are half-open: the end is not included
   five[1..3] = [3, 0]   (indices 1 and 2)
   five[..2]  = [5, 3]      five[3..] = [4, 2]
   five[..]   = [5, 3, 0, 4, 2]  — the whole thing, as a slice

4. Out of bounds is a panic, not a wrong answer
   five[9] does not even compile — rustc constant-folds the index
   and refuses: `error: this operation will panic at runtime`.
   five[i] where i = 9 -> panicked
   five.get(9) -> None
   five.get(1) -> Some(3)
   `[i]` asserts the index is in range; `.get(i)` asks. C reads
   past the end and keeps going; Rust stops the program.

5. The methods live on the slice, so the array gets them free
   five.first() = Some(5), five.last() = Some(2)
   five.contains(&4) = true
   a copy, sorted: [0, 2, 3, 4, 5]   (the original is still [5, 3, 0, 4, 2])
   `five` is Copy because u32 is, so `let mut sorted = five` copied it.
   windows(2): [[5, 3], [3, 0], [0, 4], [4, 2]]
   chunks(2):  [[5, 3], [0, 4], [2]]
```
<!-- /output -->

## Practice

**One function, four callers, and the signature that turns three away.** Write `average` twice: once taking `&[u32; 5]`, once taking `&[u32]`. Then call each with a five-element array, a two-element array, a `Vec`, and a sub-range of the first array, and write down which calls compile.

The slice version has a case the fixed version does not: the length can be zero. Decide what it returns for an empty slice, and — before you look — write down what the unguarded version prints for `0 / 0` in floating point, because it does not panic and it does not stop.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:arrays_and_slices_kata -->
*[`arrays_and_slices_kata.rs`](examples/arrays_and_slices_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one function, four callers, and the signature that rejects three.
//!
//!   rustc --edition 2024 arrays_and_slices_kata.rs -o /tmp/aask && /tmp/aask

/// The signature that only ever serves one caller.
fn average_fixed(scores: &[u32; 5]) -> f64 {
    f64::from(scores.iter().sum::<u32>()) / 5.0
}

/// The signature to write instead.
fn average(scores: &[u32]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }
    Some(f64::from(scores.iter().sum::<u32>()) / scores.len() as f64)
}

/// Runs of equal values, without allocating a Vec per run.
fn longest_run(scores: &[u32]) -> usize {
    let mut best = 0;
    let mut current = 0;
    let mut previous: Option<u32> = None;
    for &s in scores {
        current = if Some(s) == previous { current + 1 } else { 1 };
        previous = Some(s);
        best = best.max(current);
    }
    best
}

fn main() {
    let ballot: [u32; 5] = [5, 3, 3, 3, 2];
    let short = [4u32, 4];
    let owned: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    let empty: [u32; 0] = [];

    println!("1. The fixed-length signature, and who it turns away");
    println!("   average_fixed(&ballot) = {:.2}", average_fixed(&ballot));
    println!("   average_fixed(&short)   does not compile:");
    println!("     expected `&[u32; 5]`, found `&[u32; 2]`   [E0308]");
    println!("   average_fixed(&owned)   does not compile either — a Vec is not");
    println!("     an array, however many elements it happens to hold.");

    println!();
    println!("2. The slice signature, and the same four callers");
    println!("   average(&ballot)     = {:?}", average(&ballot).map(|a| (a * 100.0).round() / 100.0));
    println!("   average(&short)      = {:?}", average(&short));
    println!("   average(&owned)      = {:?}", average(&owned));
    println!("   average(&ballot[1..]) = {:?}", average(&ballot[1..]).map(|a| (a * 100.0).round() / 100.0));
    println!("   average(&empty)      = {:?}   <- the length can be zero, so the", average(&empty));
    println!("   function has to say what it does about that. `&[u32; 5]` never");
    println!("   had to, which is the one thing it bought.");

    println!();
    println!("3. `len()` is a run-time value, so the empty case is a real case");
    println!("   The fixed version divides by the 5 in its own type and cannot");
    println!("   be handed nothing. The slice version divides by len(), so 0/0");
    println!("   is reachable: in floating point that is NaN, silently.");
    let bad = f64::from(empty.iter().sum::<u32>()) / empty.len() as f64;
    println!("   without the guard: {bad}   <- prints, compares false to itself");
    println!("   with the guard:    {:?}", average(&empty));

    println!();
    println!("4. What the slice methods give you for free");
    println!("   longest_run({ballot:?}) = {}", longest_run(&ballot));
    println!("   same, via windows(2): {}", 1 + ballot.windows(2).filter(|w| w[0] == w[1]).count());
    println!("   (that shortcut is only right because this array has ONE run of");
    println!("   repeats — count adjacent equal pairs and you have counted every");
    println!("   run at once, not the longest. windows is a tool, not an answer.)");
    println!("   ballot.split_at(2) = {:?}", ballot.split_at(2));
    println!("   ballot.iter().rev().collect::<Vec<_>>() = {:?}", ballot.iter().rev().collect::<Vec<_>>());
}
```
<!-- /source -->

<!-- output:arrays_and_slices_kata -->
*Verified output of [`arrays_and_slices_kata.rs`](examples/arrays_and_slices_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The fixed-length signature, and who it turns away
   average_fixed(&ballot) = 3.20
   average_fixed(&short)   does not compile:
     expected `&[u32; 5]`, found `&[u32; 2]`   [E0308]
   average_fixed(&owned)   does not compile either — a Vec is not
     an array, however many elements it happens to hold.

2. The slice signature, and the same four callers
   average(&ballot)     = Some(3.2)
   average(&short)      = Some(4.0)
   average(&owned)      = Some(3.5)
   average(&ballot[1..]) = Some(2.75)
   average(&empty)      = None   <- the length can be zero, so the
   function has to say what it does about that. `&[u32; 5]` never
   had to, which is the one thing it bought.

3. `len()` is a run-time value, so the empty case is a real case
   The fixed version divides by the 5 in its own type and cannot
   be handed nothing. The slice version divides by len(), so 0/0
   is reachable: in floating point that is NaN, silently.
   without the guard: NaN   <- prints, compares false to itself
   with the guard:    None

4. What the slice methods give you for free
   longest_run([5, 3, 3, 3, 2]) = 3
   same, via windows(2): 3
   (that shortcut is only right because this array has ONE run of
   repeats — count adjacent equal pairs and you have counted every
   run at once, not the longest. windows is a tool, not an answer.)
   ballot.split_at(2) = ([5, 3], [3, 3, 2])
   ballot.iter().rev().collect::<Vec<_>>() = [2, 3, 3, 3, 5]
```
<!-- /output -->

</details>

---

## See also

- [`Vec`](../the_vec/README.md) — the growable one, which derefs to exactly the slice type on this page
- [Grids and nested `Vec`s](../vec_of_vecs/README.md) — `[[T; N]; M]` against `Vec<Vec<T>>`, and `chunks` turning one block back into rows
- [Tuples](../tuples/README.md) — the other built-in compound type, for fields of *different* types
- [String slices](../../14_Strings/string_slices/README.md) — `&str` is `&[u8]` with a promise about its contents, and the same half-open ranges
- [Borrowing](../../18_Ownership/borrowing/README.md) — why a slice cannot outlive what it points at
- [Stack and heap](../../18_Ownership/stack_and_heap/README.md) — where the array's elements actually are

## Sources

[Primitives: Arrays and Slices ↗](https://doc.rust-lang.org/rust-by-example/primitives/array.html) in Rust by Example; the [`slice` ↗](https://doc.rust-lang.org/std/primitive.slice.html) and [`array` ↗](https://doc.rust-lang.org/std/primitive.array.html) primitive pages in std, which are where the method list actually lives.

## Po polsku

Tablica (*array*) `[T; N]` nosi długość **w typie**: `[u32; 5]` i `[u32; 3]` to dwa osobne typy, tak samo różne jak `u32` i `String`. To pierwsza rzecz, która zaskakuje po Pythonie czy Javie, gdzie tablica jest jedna, a długość to zwykła właściwość obiektu. Elementy leżą w niej jeden przy drugim, bez żadnego nagłówka (`size_of::<[u32; 5]>()` to 20), najczęściej na stosie — a jeśli `T: Copy`, to cała tablica też jest `Copy`, więc `let mut sorted = five;` kopiuje pięć wartości i zostawia oryginał nietknięty. Odpowiednikiem, który rośnie, jest wektor (`Vec`), a nie tablica.

Wycinek (*slice*) `&[T]` przenosi długość **z typu do wartości** i na tym polega cały trik tej lekcji. `&[u32; 5]` zajmuje 8 bajtów (sam adres), a `&[u32]` szesnaście: adres **i** długość — stąd angielska nazwa *fat pointer*. Dzięki temu jedna funkcja `fn total(scores: &[u32])` obsługuje tablicę dowolnej długości, wektor i fragment `&five[1..3]`. Uwaga na nawyk przyniesiony z Pythona: tam `xs[1:3]` **kopiuje**, a w Ruscie `&xs[1..3]` to **pożyczenie** — żywy widok na cudze dane, więc dopóki wycinek istnieje, źródła nie wolno zmienić, i mówi o tym kompilator, a nie dziwne zachowanie w czasie działania. Zakresy są w obu językach półotwarte: `five[1..3]` to indeksy 1 i 2.

Resztę strony da się zapamiętać jako dwie pary przeciwieństw. Pierwsza: **`[i]` to twierdzenie, `.get(i)` to pytanie**. `five[9]` w ogóle się nie kompiluje — rustc wylicza stały indeks i odrzuca kod lintem `unconditional_panic` z komunikatem *„this operation will panic at runtime”* — przy indeksie znanym dopiero w czasie działania program kończy się paniką, a `five.get(9)` po prostu zwraca `None`. Druga: `windows(2)` kontra `chunks(2)` — okna zachodzą na siebie i nigdy nie są krótsze, kawałki nie zachodzą, ale ostatni bywa krótszy; pomylenie ich to klasyczny błąd o jeden (*off-by-one*) w tym zakątku std. I jedna rada, której polskie kursy zwykle nie dają: w sygnaturze funkcji pisz `&[T]`, nigdy `&Vec<T>` — ta druga forma nie daje nic ponadto, a odrzuca tablice i wycinki; clippy zgłasza to jako `ptr_arg`.

**Szukaj po polsku:** tablice w Ruscie · wycinek w Ruscie · `rust slice vs array` · `rust &[T] vs &Vec<T> ptr_arg` · `rust windows vs chunks`
