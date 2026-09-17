# Loops without an index

**Level:** 101 → 201 · for newcomers

**One line:** `for i in 0..v.len() { let item = v[i]; }` is legal Rust that fetches each item the long way round — `for item in &v` cannot index past the end, cannot watch the collection change under it, and has no bounds check for anyone to optimize away.

```rust
fn main() {
    let prices = vec![250, 1200, 99, 480];

    let mut total = 0;
    for price in &prices {
        total += price;
    }
    println!("{total}");                             // 2029

    for (i, price) in prices.iter().enumerate() {
        if *price > 1000 {
            println!("index {i} holds {price}");     // index 1 holds 1200
        }
    }
}
```

When the position is part of the answer, `enumerate` hands it over with the item. Neither loop reads `prices[i]`.

## The index version compiles — for `Copy` items

```rust
let prices = [250, 1200, 99, 480];
let mut total = 0;
for i in 0..prices.len() {
    let price = prices[i];                           // a copy of an i32
    total += price;
}
println!("{total}");                                 // 2029
```

It prints the same total. Change the element type to `String` and `let item = collection[i];` stops compiling, because indexing names a place inside the `Vec` and `let` would move the value out of it:

```text title="Abridged — real rustc output for move_out_by_index.rs, without the closing summary"
error[E0507]: cannot move out of index of `Vec<String>`
 --> move_out_by_index.rs:4:20
  |
4 |         let item = collection[i];
  |                    ^^^^^^^^^^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
  |
help: consider borrowing here
  |
4 |         let item = &collection[i];
  |                    +
help: consider cloning the value if the performance cost is acceptable
  |
4 |         let item = collection[i].clone();
  |                                 ++++++++
```

The first help is the item `for item in &collection` would have given you without the index. Clippy says the same about the `i32` version, which rustc accepts silently:

```text title="Abridged — real clippy-driver output for range_of_indices.rs, without the help URL and the warning count"
warning: the loop variable `i` is only used to index `prices`
 --> range_of_indices.rs:4:14
  |
4 |     for i in 0..prices.len() {
  |              ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(clippy::needless_range_loop)]` on by default
help: consider using an iterator
  |
4 -     for i in 0..prices.len() {
4 +     for <item> in &prices {
```

Its sibling lint, `explicit_counter_loop`, catches the other hand-rolled shape — a `let mut i = 0;` bumped at the bottom of a `for` — and suggests `.iter().enumerate()`.

## `0..v.len()` is read once, so the collection can change under it

The range is built before the first turn. Nothing ties it to the `Vec` afterwards, so the index loop compiles with a `push` or a `remove` in its body — and neither does what it looks like. Sections 2 and 3 of the output below:

- **Push inside it:** starting from `[1, 2, 3]` and pushing `x * 10` for every odd `x`, the loop makes **3** turns and leaves `[1, 2, 3, 10, 30]`. The pushed values were never visited; the range was already `0..3`.
- **Remove inside it:** removing every `0` from `[5, 0, 0, 4]` removes the first zero at index 1, shifts the second zero into index 1, moves on to index 2 — and never sees it. At index 3 the `Vec` is only three long: `index out of bounds: the len is 3 but the index is 3`. The panic left `[5, 0, 4]` behind.

Iterate directly and neither program compiles, because `for x in &queue` holds a borrow of `queue` for the whole loop:

```text title="Abridged — real rustc output for mutate_while_walking.rs, without the closing summary"
error[E0502]: cannot borrow `queue` as mutable because it is also borrowed as immutable
 --> mutate_while_walking.rs:5:13
  |
3 |     for x in &queue {
  |              ------
  |              |
  |              immutable borrow occurs here
  |              immutable borrow later used here
4 |         if x % 2 == 1 {
5 |             queue.push(x * 10);
  |             ^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

That refusal is the book's safety argument, and it is the stronger half: the index loop is not *unsafe* — every out-of-range read is a panic, not a wrong byte — but it lets a loop and a mutation disagree about the length, and the direct loop does not. The fixes are the shapes the borrow checker is pointing at: `retain(|&r| r != 0)` for the removal (`[5, 4]`), and for a work list that grows while you walk it, a `while` loop — [When a `for` loop beats a chain](../../24_Iterators/when_a_loop_beats_a_chain/README.md) has one. [Iterator invalidation](../../31_C_and_Cpp/iterator_invalidation/README.md) is the same refusal told from the C++ side, where the program compiles and quietly skips.

## An index needs the right bound, and `take(n)` is not the same fix

`for i in 0..n { s += v[i]; }` with `n` passed in separately is the C shape: the loop and the slice each have a length, and nothing makes them agree. With `v = [1, 2, 3]`, `n = 2` gives 3 and `n = 5` panics at index 3.

Clippy flags it, and its suggestion is not a rewrite of the same function:

```text title="Abridged — real clippy-driver output for index_to_n.rs, without the help URL and the warning count"
warning: the loop variable `i` is only used to index `v`
 --> index_to_n.rs:3:14
  |
3 |     for i in 0..n {
  |              ^^^^
  |
  = note: `#[warn(clippy::needless_range_loop)]` on by default
help: consider using an iterator
  |
3 -     for i in 0..n {
3 +     for <item> in v.iter().take(n) {
```

`v.iter().take(n).sum()` with `n = 5` returns **6**: `take` stops at whichever end comes first, so the panic that said *the caller passed a bad `n`* becomes a total over fewer items than asked for. If `n > v.len()` is a bug, keep saying so — `assert!(n <= v.len())`, or `v[..n].iter()`, which panics before the loop starts with `range end index 5 out of range for slice of length 3`.

## What the index was for, and the shape that says so

| the index was for | write instead | on `[12, 15, 11, 18, 20, 19]` |
|---|---|---|
| the position as well as the item | [`.iter().enumerate()`](../../24_Iterators/zip_and_enumerate/README.md) | with `max_by_key`: the warmest is index 4, at 20 |
| two sequences in step | [`.iter().zip(&other)`](../../24_Iterators/zip_and_enumerate/README.md) | `["Thu 18", "Fri 20", "Sat 19"]` with the day names, filtered to `>= 18` |
| each item and the next one | [`.windows(2)`](../../26_Collections/slice_methods/slice_windows/README.md) | rises `[3, -4, 7, 2, -1]` |
| fixed-size groups | [`.chunks(2)`](../../26_Collections/slice_methods/slice_chunks/README.md) | pair sums `[27, 29, 39]` |
| every k-th item | [`.iter().step_by(2)` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by) | `[12, 11, 20]` |
| walking backwards | [`.iter().rev()`](../../24_Iterators/double_ended_and_exact_size/README.md) | `[19, 20, 18, 11, 15, 12]` |

The two index loops the output checks these against (`temps[i] - temps[i - 1]`, `temps[i] + temps[i + 1]`) give the same answers here, and the second stops agreeing the moment the length is odd. On `[1, 2, 3]`, `chunks(2)` returns `[3, 3]` — the last chunk is short — while the index loop panics reaching for `odd[3]`.

## When an index is the honest answer

When one step needs **two positions at once**:

```rust
let mut sorted = [12, 15, 11, 18, 20, 19];
for i in 0..sorted.len() {
    for j in i + 1..sorted.len() {
        if sorted[j] < sorted[i] {
            sorted.swap(i, j);
        }
    }
}
println!("{sorted:?}");                              // [11, 12, 15, 18, 19, 20]
```

An iterator hands out one element per turn, and `swap(i, j)` names two places in one call. The kata below has another: moving the zeros to the end in place, where a *read* position advances every turn and a *write* position only on a keep. Neither loop draws `needless_range_loop`, because `i` is used for something other than one index. (For sorting itself, [`sort`](../../26_Collections/slice_methods/slice_sort/README.md) exists.)

## The bounds check: real in a debug build, and gone at `opt-level=3` when the bound is `v.len()`

*Rust in Action* §2.4.1 argues that indexing pays for a bounds check that direct iteration does not. The claim is about generated code, so it was measured in generated code — three functions summing a `&[u64]`:

```rust
#![crate_type = "lib"]

#[unsafe(no_mangle)]
pub fn sum_by_len(v: &[u64]) -> u64 {
    let mut s = 0;
    for i in 0..v.len() {
        s += v[i];
    }
    s
}

#[unsafe(no_mangle)]
pub fn sum_by_n(v: &[u64], n: usize) -> u64 {
    let mut s = 0;
    for i in 0..n {
        s += v[i];
    }
    s
}

#[unsafe(no_mangle)]
pub fn sum_by_iter(v: &[u64]) -> u64 {
    let mut s = 0;
    for x in v {
        s += x;
    }
    s
}
```

Save that as `bounds.rs`, and this beside it as `count_bounds_checks.py`. It compiles the file at both levels with `--emit asm` and counts the `call`s to `panic_bounds_check` between each function's label and the next symbol:

```python
import re
import subprocess

for level in (0, 3):
    subprocess.run(
        ["rustc", "--edition", "2024", "-C", f"opt-level={level}",
         "--emit", "asm", "-o", "bounds.s", "bounds.rs"],
        check=True,
    )
    counts, current = {}, None
    for line in open("bounds.s"):
        if m := re.match(r"_?(sum_by_\w+):", line):
            current = m[1]
            counts[current] = 0
        elif re.match(r"_*R\w+:", line):
            current = None
        elif current and "call" in line and "panic_bounds_check" in line:
            counts[current] += 1
    for name in ("sum_by_len", "sum_by_n", "sum_by_iter"):
        print(f"opt-level={level}  {name}  calls panic_bounds_check: {counts[name]}")
```

```text title="Real run — python3 count_bounds_checks.py, rustc 1.98.0, x86_64-apple-darwin, 2026-09-16; the same six counts from asm built by rustc 1.98.0 in rust:1.98-slim (x86_64-unknown-linux-gnu)"
opt-level=0  sum_by_len  calls panic_bounds_check: 1
opt-level=0  sum_by_n  calls panic_bounds_check: 1
opt-level=0  sum_by_iter  calls panic_bounds_check: 0
opt-level=3  sum_by_len  calls panic_bounds_check: 0
opt-level=3  sum_by_n  calls panic_bounds_check: 1
opt-level=3  sum_by_iter  calls panic_bounds_check: 0
```

At `opt-level=0` — what `cargo build` and `cargo run` give you — both index loops compare `i` against the length on every turn. At `opt-level=3` the loop over `0..v.len()` has no check left, because the range itself proves every `i` is in bounds. The loop over `0..n` keeps one, and *where* it keeps it is the interesting part:

```text title="rustc 1.98.0, x86_64-apple-darwin, -C opt-level=3 — sum_by_n abridged: .cfi lines, the n == 0 return and the vectorized middle removed, comments added"
_sum_by_n:
	testq	%rdx, %rdx              # n == 0: return 0
	je	LBB2_1
	leaq	-1(%rdx), %rax          # rax = n - 1
	cmpq	%rax, %rsi              # len <= n - 1 ?
	jbe	LBB2_11                 #   then panic, before any element is read
	...
LBB2_11:
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	l_anon.d1f3c3f877804adb66d8eaeb5422ed21.1(%rip), %rdx
	movq	%rsi, %rdi              # the index it reports: len, the first bad one
	callq	__RNvNtCsl7QZrza34zr_4core9panicking18panic_bounds_check
LBB2_5:
	addq	(%rdi,%rcx,8), %rax     # the scalar loop: no compare against len
	incq	%rcx
LBB2_9:
	cmpq	%rcx, %rdx
	jne	LBB2_5
	retq
```

One compare, **before** the loop, not one per element. The optimizer may move the panic that early because the partial sum it skips is never observable, and it reports the same index the unoptimized loop would have reached. The direct loop has no check at either level, since there is no index to check.

So the book's claim holds for debug builds and for bounds the compiler cannot connect to the slice, and in these three functions the cost at `opt-level=3` is at most one compare per call. Speed is the weaker reason to drop the index; the length disagreements in the two sections above are the stronger one. [What the check costs](../../31_C_and_Cpp/buffer_overruns/README.md#what-the-check-costs) has the C comparison, [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) the same `--emit asm` experiment on a different loop, and [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) the `get_unchecked` that removes a check by promise instead of by proof.

## If you are coming from another language

**Python.** `for i in range(len(xs))` is the same shape with the same fix, and `enumerate` and `zip` are the same functions. What transfers exactly is the timing: `range(len(xs))` is built once, like `0..v.len()` — on Python 3.14.7, appending inside `for i in range(len(queue))` makes 3 turns over a list that ends up 5 long. What differs is the *direct* loop. Python's `for x in queue` sees the appends and makes 5 turns, where Rust's `for x in &queue` refuses to compile, so the Rust habit is not "the direct loop handles growth" but "the direct loop forbids it". The table's rows have Python names too: `itertools.pairwise` is `windows(2)`, `itertools.batched(xs, 2)` is `chunks(2)` (and also leaves a short last batch), `xs[::2]` is `step_by(2)`, `reversed(xs)` is `rev()`. Python's `xs[i]` is always checked and raises `IndexError`, so the bounds-check section below has no Python counterpart.

**C.** The index loop is the C idiom because C has nothing else: `for (size_t i = 0; i < n; i++) sum += a[i];`, no check at any optimization level, and `n` a separate value that can disagree with the buffer — the [buffer overrun](../../31_C_and_Cpp/buffer_overruns/README.md) in one line. `sum_by_n` above is that loop in Rust, with the check C never had. The slice is what changes the question: `&[u64]` carries its length, so `for x in v` has no `n` to get wrong, and `0..v.len()` has one the optimizer can see. C++'s range-based `for (auto x : v)` is the direct loop, and erasing from `v` inside it is the [iterator invalidation](../../31_C_and_Cpp/iterator_invalidation/README.md) the borrow checker refuses.

**ABAP.** The direct loop is already the ABAP habit. `LOOP AT itab INTO wa` hands you each row with `sy-tabix` as the position — `enumerate`, numbered from 1 — and the index form — a `DO` loop counted up to `lines( itab )`, with `READ TABLE itab INTO wa INDEX sy-index` inside — is what you write only for a stride or two positions at once, which is also when Rust keeps its index. The out-of-range read differs. `READ TABLE … INDEX` past the end sets `sy-subrc = 4` and leaves `wa` as it was, so an unchecked `sy-subrc` quietly reuses the previous row; a table expression `itab[ idx ]` raises `CX_SY_ITAB_LINE_NOT_FOUND` instead. Rust's `v[i]` is the second behaviour and `v.get(i)` the first with the return code folded into an `Option`. `DELETE itab` inside `LOOP AT itab` is legal ABAP and skips rows the same way the index loop above does; in Rust the direct loop will not compile with it.

## Practice

**Five index loops, and the one to keep.** Rewrite each of these without an index, and check each rewrite against the original:

1. Number the lines of a file from 1 — `"1: [package]"` and so on.
2. Count the positions where a student's answers differ from the key.
3. Find the largest rise between consecutive readings.
4. Sum every third reading, starting with the first, written originally as a `while` loop with `i += 3`.
5. Average each pair of readings, when the number of readings is odd.

Then run three of them on inputs the original author did not expect: an answer sheet shorter than the key (2), a single reading (3), and the odd count (5). Say which rewrite and which original disagree, and which of the two answers is the honest one.

Finally, move every `0` in `[0, 3, 0, 5, 7, 0, 2]` to the end, in place, keeping the other values in order — and say why this one keeps its indices.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:loops_without_an_index_kata -->
*[`loops_without_an_index_kata.rs`](examples/loops_without_an_index_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: five index loops rewritten without an index, each checked
//! against the original, and the one loop that should keep its indices.
//!
//!   rustc --edition 2024 loops_without_an_index_kata.rs -o /tmp/lwaik && /tmp/lwaik

fn main() {
    let lines = ["[package]", "name = \"demo\"", "", "[dependencies]"];
    let expected = ['b', 'a', 'd', 'c', 'a'];
    let actual = ['b', 'c', 'd', 'c', 'b'];
    let readings = [12, 15, 11, 18, 20, 19, 25];

    println!("1. Number the lines from 1");
    let mut by_index = Vec::new();
    for i in 0..lines.len() {
        by_index.push(format!("{}: {}", i + 1, lines[i]));
    }
    let by_enumerate: Vec<String> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{}: {line}", i + 1))
        .collect();
    let by_counter: Vec<String> = (1..).zip(&lines).map(|(n, line)| format!("{n}: {line}")).collect();
    println!("   {by_enumerate:?}");
    println!("   same as the index loop: {}; (1..).zip(&lines) gives the same again: {}", by_enumerate == by_index, by_counter == by_index);

    println!();
    println!("2. Count the answers that differ");
    let mut wrong_by_index = 0;
    for i in 0..expected.len() {
        if expected[i] != actual[i] {
            wrong_by_index += 1;
        }
    }
    let wrong = expected.iter().zip(&actual).filter(|(e, a)| e != a).count();
    println!("   zip + filter + count = {wrong}, index loop = {wrong_by_index}");
    let short = ['b', 'c'];
    let wrong_short = expected.iter().zip(&short).filter(|(e, a)| e != a).count();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let attempt = std::panic::catch_unwind(|| {
        let mut n = 0;
        for i in 0..expected.len() {
            if expected[i] != short[i] {
                n += 1;
            }
        }
        n
    });
    std::panic::set_hook(hook);
    let index_says = match attempt {
        Ok(n) => format!("{n}"),
        Err(_) => "a panic (index out of bounds)".to_string(),
    };
    println!("   against a 2-answer sheet: zip says {wrong_short} and stops quietly; the index loop gives {index_says}");

    println!();
    println!("3. The largest rise between consecutive readings");
    let mut max_by_index = i32::MIN;
    for i in 1..readings.len() {
        max_by_index = max_by_index.max(readings[i] - readings[i - 1]);
    }
    let max_rise = readings.windows(2).map(|w| w[1] - w[0]).max();
    println!("   windows(2) + max = {max_rise:?}, index loop = {max_by_index}");
    let one = [7];
    let mut one_by_index = i32::MIN;
    for i in 1..one.len() {
        one_by_index = one_by_index.max(one[i] - one[i - 1]);
    }
    let one_rise = one.windows(2).map(|w| w[1] - w[0]).max();
    println!("   on [7]: windows says {one_rise:?}; the index loop says {one_by_index}, a made-up number");

    println!();
    println!("4. Sum every third reading, starting with the first");
    let mut every_third_by_index = 0;
    let mut i = 0;
    while i < readings.len() {
        every_third_by_index += readings[i];
        i += 3;
    }
    let every_third: i32 = readings.iter().step_by(3).sum();
    println!("   step_by(3) + sum = {every_third}, index loop = {every_third_by_index}");

    println!();
    println!("5. Average each pair of readings");
    let mut averages_by_index = Vec::new();
    for i in (0..readings.len() - 1).step_by(2) {
        averages_by_index.push((readings[i] + readings[i + 1]) as f64 / 2.0);
    }
    let averages: Vec<f64> = readings
        .chunks(2)
        .map(|c| c.iter().sum::<i32>() as f64 / c.len() as f64)
        .collect();
    println!("   chunks(2)  -> {averages:?}");
    println!("   index loop -> {averages_by_index:?}   <- 7 readings: the index loop had to drop the last one");

    println!();
    println!("6. The one that keeps its indices: move every zero to the end, in place");
    let mut values = [0, 3, 0, 5, 7, 0, 2];
    let mut write = 0;
    for read in 0..values.len() {
        if values[read] != 0 {
            values.swap(write, read);
            write += 1;
        }
    }
    println!("   {values:?}");
    println!("   two positions move independently -- `read` every turn, `write` only on a keep --");
    println!("   and swap needs both at once. No adapter says that more clearly than two indices.");
}
```
<!-- /source -->

<!-- output:loops_without_an_index_kata -->
*Verified output of [`loops_without_an_index_kata.rs`](examples/loops_without_an_index_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Number the lines from 1
   ["1: [package]", "2: name = \"demo\"", "3: ", "4: [dependencies]"]
   same as the index loop: true; (1..).zip(&lines) gives the same again: true

2. Count the answers that differ
   zip + filter + count = 2, index loop = 2
   against a 2-answer sheet: zip says 1 and stops quietly; the index loop gives a panic (index out of bounds)

3. The largest rise between consecutive readings
   windows(2) + max = Some(7), index loop = 7
   on [7]: windows says None; the index loop says -2147483648, a made-up number

4. Sum every third reading, starting with the first
   step_by(3) + sum = 55, index loop = 55

5. Average each pair of readings
   chunks(2)  -> [13.5, 14.5, 19.5, 25.0]
   index loop -> [13.5, 14.5, 19.5]   <- 7 readings: the index loop had to drop the last one

6. The one that keeps its indices: move every zero to the end, in place
   [3, 5, 7, 2, 0, 0, 0]
   two positions move independently -- `read` every turn, `write` only on a keep --
   and swap needs both at once. No adapter says that more clearly than two indices.
```
<!-- /output -->

</details>

## The verified output

<!-- output:loops_without_an_index -->
*Verified output of [`loops_without_an_index.rs`](examples/loops_without_an_index.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== 1. the same total, with and without an index ===
  for i in 0..prices.len() { prices[i] } -> 2029
  for price in &prices                   -> 2029
  and when the position IS wanted: enumerate -> index 1 holds 1200

=== 2. 0..v.len() is evaluated once, before the first turn ===
  pushing inside the loop compiles: 3 turns, queue = [1, 2, 3, 10, 30]
  the pushed 10 and 30 were never visited -- the range was already 0..3

=== 3. and removing inside it skips one element, then runs off the end ===
  start [5, 0, 0, 4], remove every 0 by index
  visited indices [0, 1, 2, 3], readings left as [5, 0, 4]
  panic: index out of bounds: the len is 3 but the index is 3
  retain(|&r| r != 0) -> [5, 4]

=== 4. an index needs the right bound, and take(n) is not the same fix ===
  sum_first_by_index(&[1, 2, 3], 2) = 3
  sum_first_by_take (&[1, 2, 3], 2) = 3
  sum_first_by_index(&[1, 2, 3], 5) -> panic: index out of bounds: the len is 3 but the index is 3
  sum_first_by_take (&[1, 2, 3], 5) = 6   <- stops at the end, no panic

=== 5. what the index was for, and the shape that says so ===
  temps = [12, 15, 11, 18, 20, 19]
  position     enumerate    warmest is index 4, 20 degrees
  neighbours   windows(2)   [3, -4, 7, 2, -1]   same as temps[i] - temps[i - 1]: true
  groups       chunks(2)    [27, 29, 39]          same as temps[i] + temps[i + 1]: true
    on [1, 2, 3]: chunks(2) -> [3, 3] (a short last chunk); the index form -> panic: index out of bounds: the len is 3 but the index is 3
  every k-th   step_by(2)   [12, 11, 20]
  backwards    iter().rev() [19, 20, 18, 11, 15, 12]
  two in step  zip          ["Thu 18", "Fri 20", "Sat 19"]

=== 6. where an index is honest: two positions at once ===
  swap(i, j) inside two index loops -> [11, 12, 15, 18, 19, 20]
  an iterator hands out one element at a time; swap needs two places in one call
```
<!-- /output -->

## See also

- [`for` loops](../for_loops/README.md) — the loop this page is about using directly, and what `&` in front of the collection does
- [`zip` and `enumerate`](../../24_Iterators/zip_and_enumerate/README.md) — the two adapters that replace most indices, and the row `zip` silently drops
- [When a `for` loop beats a chain](../../24_Iterators/when_a_loop_beats_a_chain/README.md) — including the growing work list, where a `while` is the right loop
- [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) — `[i]` asserts, `.get(i)` asks
- [Slice methods](../../26_Collections/slice_methods/README.md) — `windows`, `chunks`, `swap` and the rest of what a slice can do without an index
- [Buffer overruns](../../31_C_and_Cpp/buffer_overruns/README.md) — the bug a bounds check exists for, and what it costs
- [Iterator invalidation](../../31_C_and_Cpp/iterator_invalidation/README.md) — removing while walking, in C++ and in Rust
- [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) — reading `--emit asm` at two optimization levels
- [Flow control](../flow_control/README.md) — the chapter's constructs side by side

## Sources

*Rust in Action* (Tim McNamara, Manning 2021), §2.4.1, the subsection on avoiding an index variable — its performance and safety arguments are each re-run above, and [What *Rust in Action* says about flow control, run](../flow_control_claims_checked/README.md) collects the verdicts. Clippy's [`needless_range_loop` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#needless_range_loop) and [`explicit_counter_loop` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#explicit_counter_loop); Python's [`itertools` ↗](https://docs.python.org/3/library/itertools.html) for `pairwise` and `batched`.

## Po polsku

Pętla `for i in 0..v.len() { let item = v[i]; }` jest poprawnym Rustem, ale sięga po element okrężną drogą — i kompiluje się tylko dla elementów `Copy`: dla `Vec<String>` zapis `let item = v[i];` to `E0507` („cannot move out of index”), a pierwsza podpowiedź kompilatora, `&v[i]`, daje dokładnie to, co `for item in &v` dałoby bez indeksu. Clippy nazywa ten kształt `needless_range_loop`, a ręcznie prowadzony licznik obok pętli — `explicit_counter_loop`, z podpowiedzią `.iter().enumerate()`.

Ważniejszy od wydajności jest argument bezpieczeństwa. Zakres `0..v.len()` wylicza się **raz**, przed pierwszym obiegiem, więc pętla z indeksem kompiluje się nawet wtedy, gdy w jej ciele wektor rośnie albo maleje: dopisane elementy nigdy nie zostaną odwiedzone, a usuwanie przesuwa elementy pod indeksem — jeden zostaje pominięty, a na końcu program panikuje z komunikatem „index out of bounds: the len is 3 but the index is 3”. Pętla bezpośrednia, `for x in &v`, trzyma pożyczkę przez cały czas, więc ta sama modyfikacja to `E0502` już przy kompilacji. Osobna pułapka: podpowiedź Clippy dla `for i in 0..n` brzmi `v.iter().take(n)`, a to **nie** jest ta sama funkcja — przy `n` większym niż długość zamiast paniki dostajemy po cichu sumę z mniejszej liczby elementów.

Zamiast indeksu wybiera się kształt, który mówi, do czego indeks służył: `enumerate` (pozycja), `zip` (dwie sekwencje naraz), `windows(2)` (sąsiedzi), `chunks(n)` (grupy — ostatnia bywa krótsza), `step_by(k)` (co k-ty), `rev()` (od końca). Indeks zostaje uczciwym wyborem wtedy, gdy jeden krok potrzebuje **dwóch pozycji naraz**, jak w `swap(i, j)`. A sprawdzanie granic, o którym pisze *Rust in Action*? Zmierzone w asemblerze: przy `opt-level=0` obie pętle z indeksem wołają `panic_bounds_check` w każdym obiegu; przy `opt-level=3` pętla po `0..v.len()` nie ma już żadnego sprawdzenia, pętla po `0..n` ma jedno porównanie **przed** pętlą, a pętla bezpośrednia nie ma go na żadnym poziomie optymalizacji.

**Szukaj po polsku:** pętla bez indeksu · sprawdzanie granic tablicy · unieważnienie iteratora · okna i kawałki wycinka · `rust needless_range_loop` · `rust bounds check elimination` · `rust windows chunks step_by`
