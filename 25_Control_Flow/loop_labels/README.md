# Loop labels

**Level:** 101 → 201 · working knowledge

**One line:** `'name:` in front of a `for`, `while`, `loop` or plain block lets `break 'name` and `continue 'name` say which one they mean — it replaces the flag variable a nested search otherwise needs, it looks like a lifetime without being one, and it is as close to `goto` as Rust gets.

```rust
fn main() {
    let grid = [[1, 2, 3], [4, -5, 6], [-7, 8, 9]];
    let mut first_negative = None;
    'rows: for (r, row) in grid.iter().enumerate() {
        for (c, &n) in row.iter().enumerate() {
            if n < 0 {
                first_negative = Some((r, c));
                break 'rows; // leaves both loops
            }
        }
    }
    println!("{first_negative:?}"); // Some((1, 1))
}
```

A plain `break` there would leave only the column loop, and the row loop would go on to find `-7` and overwrite the answer — [the trap on the `break` page](../break_expressions/README.md#break-leaves-the-innermost-loop-and-only-that-one). The label moves the decision to the line that knows.

## `break 'outer` leaves every loop out to the label

*Rust in Action* §2.4.5 shows labels with three nested unbounded loops: `'outer: for x in 0.. { for y in 0.. { for z in 0.. { if x + y + z > 1000 { break 'outer; } } } }`. Run with a counter in each loop:

```text title="From the verified output below"
   stopped at (x, y, z) = Some((0, 0, 1001))
   x values seen: [0], y values seen: [0], z passes: 1002
```

It never visits `x = 1`, or even `y = 1`. The innermost loop counts `z` from 0 while `x` and `y` are both still 0, and `z = 1001` is the first value that pushes the sum past 1000 — so the `break 'outer` fires on the 1002nd pass of the first `z` loop, and the two outer loops only ever ran their first pass.

With a plain `break` in the same place, only the `z` loop ends, and `y` moves on. Capped at four values of `y` so it terminates:

```text title="From the verified output below"
   plain break instead: [(0, 0, 1001), (0, 1, 1000), (0, 2, 999), (0, 3, 998)] ...
```

In the book's uncapped version, `y` runs `0..` too, so a plain `break` there never lets the program finish.

## What a label replaces: the flag variable

Without labels, leaving two loops takes a variable the inner loop sets and the outer loop checks:

```rust
fn main() {
    let grid = [[1, 2, 3], [4, -5, 6], [-7, 8, 9]];
    let mut found = None;
    let mut done = false;
    for (r, row) in grid.iter().enumerate() {
        for (c, &n) in row.iter().enumerate() {
            if n < 0 {
                found = Some((r, c));
                done = true;
                break;
            }
        }
        if done {
            break; // the second check, one level out, that the flag needs
        }
    }
    println!("{found:?}"); // Some((1, 1))
}
```

It works, and it is two `mut` bindings and a check in a second place, each of which can be forgotten without a compile error; forget the outer `if done` and you are back to the last match instead of the first. Two versions need no `mut` at all — a [labelled block](#a-labelled-block-is-breakable-without-a-loop) that *is* the answer, and `find_map`:

```rust
fn main() {
    let grid = [[1, 2, 3], [4, -5, 6], [-7, 8, 9]];

    let by_block = 'search: {
        for (r, row) in grid.iter().enumerate() {
            for (c, &n) in row.iter().enumerate() {
                if n < 0 {
                    break 'search Some((r, c));
                }
            }
        }
        None
    };

    let by_chain = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&n| n < 0).map(|c| (r, c)));

    println!("{by_block:?} {by_chain:?}"); // Some((1, 1)) Some((1, 1))
}
```

All four agree (see *section 2* of the output below). [When a `for` loop beats a chain](../../24_Iterators/when_a_loop_beats_a_chain/README.md#3-when-you-need-out-of-two-levels-at-once) draws the line between the label and `find_map`: the chain is fine until the inner loop also has to update something outside itself.

## `continue 'outer` abandons the rest of an outer pass

A labelled `continue` ends the current pass of the *named* loop, including whatever was left of every loop inside it. The book's footnote calls it the less common of the two; it is the natural spelling of *one bad item disqualifies the whole group*:

```rust
fn main() {
    let batches = [[4, 7, 5], [6, -1, 8], [9, 2, 3]];
    let mut kept = Vec::new();
    'batches: for (b, batch) in batches.iter().enumerate() {
        for &reading in batch {
            if reading < 0 {
                continue 'batches; // one bad reading discards the whole batch
            }
        }
        kept.push((b, batch.iter().sum::<i32>()));
    }
    println!("{kept:?}"); // [(0, 16), (2, 14)]
}
```

```text title="From the verified output below"
   readings checked = 8 of 9   <- the reading after the -1 was never looked at
   filter(all >= 0)  = [(0, 16), (2, 14)]
```

The `push` after the inner loop runs only for a batch the inner loop finished, which is what makes this shorter than a flag. When the test is a single `all`, as here, `filter(|b| b.iter().all(|&r| r >= 0))` says the same thing and stops at the same `-1`; the label earns its place when the inner loop does more than test.

## A label is not a lifetime

`'rows`, `'batches` and `'a` are written exactly like lifetimes, and the lexer reads them as the same token (the Reference's grammar calls it `LIFETIME_OR_LABEL`). They live in separate namespaces, though, so one function can use `'a` as both without a warning:

```rust
fn first_long<'a>(words: &[&'a str]) -> &'a str {
    'a: {
        for w in words {
            if w.len() > 3 {
                break 'a w;
            }
        }
        "(none)"
    }
}

fn main() {
    println!("{}", first_long(&["an", "to", "apple"])); // apple
}
```

The first `'a` is a lifetime: it ties the returned `&str` to the input strings. The second is a label on a block. They have nothing to do with each other — and the [1.63.0 release notes ↗](https://doc.rust-lang.org/releases.html#version-1630-2022-08-11) record the removal of the old warnings about a label and a lifetime sharing a name. Where the two rules part:

- **`'static` is a lifetime and cannot be a label.** `'static: loop { … }` is refused with `error: labels cannot use keyword names`, and the Reference's keyword list says the same.
- **A label does not reach into a closure.** `'outer: for … { let f = |y| { if … { break 'outer; } }; }` is `E0767`, *use of unreachable label*, with a note that labels are unreachable through functions, closures, async blocks and modules.

```text title="Abridged — real rustc output for label_into_closure.rs, the E0767 block only"
error[E0767]: use of unreachable label `'outer`
 --> label_into_closure.rs:5:23
  |
2 |     'outer: for x in 0..3 {
  |     ------ unreachable label defined here
...
5 |                 break 'outer;
  |                       ^^^^^^ unreachable label `'outer`
  |
  = note: labels are unreachable through functions, closures, async blocks and modules
```

The same file also gets `E0267`, *`break` inside of a closure*, and an `unused label` warning on `'outer` — the label the closure could not reach is, as far as rustc is concerned, never used.

## A labelled block is breakable without a loop

Since [Rust 1.65.0 ↗](https://doc.rust-lang.org/releases.html#version-1650-2022-11-03) (2022-11-03), a label can go on a plain block, and `break 'label value` leaves it early with a value:

```rust
fn main() {
    for input in ["", "12x", "42"] {
        let verdict = 'check: {
            if input.is_empty() {
                break 'check "empty";
            }
            if !input.bytes().all(|b| b.is_ascii_digit()) {
                break 'check "not a number";
            }
            "ok"
        };
        println!("{input:?} -> {verdict}"); // "" -> empty, "12x" -> not a number, "42" -> ok
    }
}
```

It is a block with early exits and one value at the end — the shape a helper function with several `return`s has, without the function. The release notes list it as *label-break-value*. Three rules, each with its own error:

**`break` into a labelled block must name it.** Unlike a loop, a labelled block is never the target of a bare `break`. And once a labelled block sits inside a loop, a bare `break` or `continue` meant for the loop is refused too — rustc will not guess whether you meant the block or the loop:

```rust
fn main() {
    'nums: for n in 0..5 {
        'check: {
            if n == 3 {
                // break;   // error[E0695]: unlabeled `break` inside of a labeled block
                break 'nums; // name the loop instead
            }
            if n == 1 {
                break 'check;
            }
            println!("{n}"); // 0, then 2
        }
    }
}
```

```text title="Real rustc output for break_through_a_block.rs, where the loop has no label"
error[E0695]: unlabeled `break` inside of a labeled block
 --> break_through_a_block.rs:5:17
  |
5 |                 break;
  |                 ^^^^^ `break` statements that would diverge to or through a labeled block need to bear a label

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0695`.
```

The same file with `continue;` in place of `break;` is also `E0695`, *unlabeled `continue` inside of a labeled block*. (`rustc --explain E0696` gives three erroneous examples. Compiled verbatim on 1.98.0, only the `continue 'b;` one is `E0696`; the two bare `continue;` ones are `E0695`.)

**A labelled block cannot be `continue`d.** A block has no next pass:

```text title="Real rustc output for continue_a_labeled_block.rs"
error[E0696]: `continue` pointing to a labeled block
 --> continue_a_labeled_block.rs:5:13
  |
3 | /     'check: {
4 | |         if n > 2 {
5 | |             continue 'check;
  | |             ^^^^^^^^^^^^^^^ labeled blocks cannot be `continue`'d
... |
8 | |     }
  | |_____- labeled block the `continue` points to

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0696`.
```

**`break 'label value` works on a labelled `loop` and a labelled block, and nowhere else.** A label on a `for` or `while` does not change what that loop can return: `break 'search n` out of `'search: for` is still `E0571`, exactly as on [the `break` page](../break_expressions/README.md#break-with-a-value-loop-and-labelled-blocks-only).

## What rustc warns about

A label nothing breaks to is `unused_labels`, warn-by-default and part of the `unused` group (`rustc -W help` lists both):

```text title="Real rustc output for unused_label.rs"
warning: unused label
 --> unused_label.rs:2:5
  |
2 |     'outer: for x in 0..3 {
  |     ^^^^^^
  |
  = note: `#[warn(unused_labels)]` (part of `#[warn(unused)]`) on by default

warning: 1 warning emitted
```

A label that reuses the name of one already in scope compiles, and the inner one wins — the Reference says labels follow the shadowing rules of local variables:

```rust
fn main() {
    'a: for x in 0..2 {
        'a: for y in 0..2 {
            if y == 1 {
                break 'a; // the inner 'a: the x loop carries on
            }
            println!("{x} {y}"); // 0 0, then 1 0
        }
    }
}
```

```text title="Real rustc output for shadowed_label.rs"
warning: label name `'a` shadows a label name that is already in scope
 --> shadowed_label.rs:3:9
  |
2 |     'a: for x in 0..2 {
  |     -- first declared here
3 |         'a: for y in 0..2 {
  |         ^^ label `'a` already in scope

warning: unused label
 --> shadowed_label.rs:2:5
  |
2 |     'a: for x in 0..2 {
  |     ^^
  |
  = note: `#[warn(unused_labels)]` (part of `#[warn(unused)]`) on by default

warning: 2 warnings emitted
```

Read the second warning in that file carefully: `unused label` lands on the **outer** `'a`, which confirms the `break 'a` went to the inner one. And the shadowing warning has **no lint name** — no `#[warn(…)]` note under it — so there is nothing to put in an `#[allow(…)]`. Measured: even `#![allow(warnings)]` at the top of the file leaves it printed; only `rustc -A warnings` on the command line silences it. Rename the label.

A label that does not exist at all is an error, `E0426`, *use of undeclared label*.

## No `goto`, and the cleanup it was for is `Drop`

*Rust in Action* §2.4.5 notes that Rust has no `goto`, and points to loop labels for the one pattern where C programmers still reach for it: jumping to a cleanup section when something fails.

**`goto` is not even reserved.** The Reference's [reserved keywords ↗](https://doc.rust-lang.org/reference/keywords.html#reserved-keywords) — words kept back for possible future use — are `abstract`, `become`, `box`, `do`, `final`, `gen`, `macro`, `override`, `priv`, `try`, `typeof`, `unsized`, `virtual` and `yield`. `goto` is not on the list, so it is an ordinary identifier, while `do` has to be escaped as a [raw identifier](../../15_First_Programs/raw_identifiers/README.md):

```rust
fn main() {
    let goto = "an ordinary name";
    let r#do = "a reserved word, escaped";
    println!("{goto} / {}", r#do); // an ordinary name / a reserved word, escaped
}
```

`let goto = 1;` compiles on 1.98.0 in all four editions, 2015 through 2024. `let do = 2;` does not:

```text title="Real rustc output for do_is_reserved.rs"
error: expected identifier, found reserved keyword `do`
 --> do_is_reserved.rs:2:9
  |
2 |     let do = 2;
  |         ^^ expected identifier, found reserved keyword
  |
help: escape `do` to use it as an identifier
  |
2 |     let r#do = 2;
  |         ++

error: aborting due to 1 previous error
```

**The C pattern the book means** releases resources in reverse order, and every failure jumps to the right point in that list:

```c
int load(const char *path) {
    int rc = -1;
    FILE *file = fopen(path, "r");
    if (!file) goto out;
    char *buffer = malloc(4096);
    if (!buffer) goto close_file;
    if (!fgets(buffer, 4096, file)) goto free_buffer;
    rc = atoi(buffer);
free_buffer:
    free(buffer);
close_file:
    fclose(file);
out:
    return rc;
}
```

**In Rust the cleanup needs no jump**, because it is not code at the end of the function — it is the [`Drop`](../../12_Traits/drop_and_raii/README.md) of each value that owns a resource, and it runs on every way out of the scope. Here with a guard that prints when it is dropped:

```rust
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("drop: {}", self.0);
    }
}

fn load(fail_at: u8) -> Result<u32, String> {
    let _file = Guard("close file");
    if fail_at == 1 {
        return Err("could not open".to_string());
    }
    let _buffer = Guard("free buffer");
    let text = if fail_at == 2 { "4x" } else { "42" };
    let n: u32 = text.parse().map_err(|e| format!("{text:?}: {e}"))?;
    Ok(n)
}

fn main() {
    for fail_at in [0, 1, 2] {
        println!("{:?}", load(fail_at));
    }
    // drop: free buffer, drop: close file, Ok(42)
    // drop: close file, Err("could not open")
    // drop: free buffer, drop: close file, Err("\"4x\": invalid digit found in string")
}
```

```text title="Section 6 of the verified output below"
   load(fail_at = 0):
      drop: free buffer
      drop: close file
      result: Ok(42)
   load(fail_at = 1):
      drop: close file
      result: Err("could not open")
   load(fail_at = 2):
      drop: free buffer
      drop: close file
      result: Err("\"4x\": invalid digit found in string")
```

A normal return, an early `return`, and a `?` — and each time exactly the guards that existed were dropped, newest first, which is the order the C labels spell out by hand. `fail_at = 1` never created the buffer, so there is no `free buffer` line, and nobody had to jump past it.

If you want the C function's *shape* — one exit point, a result the code after it can inspect — without writing a separate function, a labelled block is it, and the same drops run at its closing brace (section 6 of the output below):

```rust
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("drop: {}", self.0);
    }
}

fn main() {
    let cell = -7;
    let outcome = 'work: {
        let _file = Guard("close file");
        let _buffer = Guard("free buffer");
        if cell < 0 {
            break 'work Err("negative cell");
        }
        Ok(cell)
    };
    println!("{outcome:?}"); // after "drop: free buffer" and "drop: close file": Err("negative cell")
}
```

So the book's advice holds, with a correction it could not make in 2021: labelled blocks were not stable until 1.65, so a label then had to go on a loop, and the pattern meant a `loop` that runs once. Today the label gives you the jump, and `Drop` makes the cleanup at the end of the jump unnecessary.

## The verified output

<!-- output:loop_labels -->
*Verified output of [`loop_labels.rs`](examples/loop_labels.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. break 'outer leaves every loop out to the label
   stopped at (x, y, z) = Some((0, 0, 1001))
   x values seen: [0], y values seen: [0], z passes: 1002
   The z loop alone carried the sum past 1000: x = 1 and y = 1 never happened.
   plain break instead: [(0, 0, 1001), (0, 1, 1000), (0, 2, 999), (0, 3, 998)] ...
   Each break ends one z loop, and y just moves on.

2. The flag variable a label replaces, and two ways with no mut at all
   flag variable      Some((1, 1))
   break 'rows        Some((1, 1))
   labelled block     Some((1, 1))
   find_map           Some((1, 1))

3. continue 'batches abandons the rest of an outer pass
   kept (batch, sum) = [(0, 16), (2, 14)]
   readings checked = 8 of 9   <- the reading after the -1 was never looked at
   filter(all >= 0)  = [(0, 16), (2, 14)]

4. A label is not a lifetime: one function uses 'a as both
   first_long(&["an", "to", "apple"]) = "apple"
   first_long(&["an", "to"])          = "(none)"

5. A labelled block: break out of a block that is not a loop
   "" -> empty
   "12x" -> not a number
   "42" -> ok

6. No goto, and cleanup needs no jump
   let goto = "an ordinary name";   let r#do = "a reserved word, escaped";
   load(fail_at = 0):
      drop: free buffer
      drop: close file
      result: Ok(42)
   load(fail_at = 1):
      drop: close file
      result: Err("could not open")
   load(fail_at = 2):
      drop: free buffer
      drop: close file
      result: Err("\"4x\": invalid digit found in string")
   The same shape without a function, as a labelled block:
      drop: free buffer
      drop: close file
      outcome: Err("negative cell")
```
<!-- /output -->

## If you are coming from another language

**Python.** No labels. [PEP 3136 ↗](https://peps.python.org/pep-3136/) proposed labelled `break` and `continue` in 2007 and was rejected; Guido van Rossum's rejection judged code that needs them rare and the feature likelier to be misused than well used. So the Python ways out of two loops are the ones this page replaces: a flag, an exception, or moving the loops into a function so `return` can leave both. The function-and-`return` version transfers to Rust unchanged and is still sometimes the clearest. Python's `with` is the counterpart of `Drop` for cleanup — but only for the block you remembered to wrap; a Rust value runs its `drop` whether or not the caller wrote anything.

**C.** Nested loops are left with a flag or a `goto`, and `goto cleanup` is the idiomatic resource-release pattern shown above. What transfers is the reverse-order release list; what the compiler now enforces is that it cannot be skipped — a Rust function has no path out of a scope that forgets a `drop`, where a C `return` in the middle forgets the `free` silently. Labelled loops are also coming to C: WG14's [N3355 *Named loops* ↗](https://www.open-std.org/jtc1/sc22/wg14/www/docs/n3355.htm) puts an ordinary C label in front of a loop so `break name;` and `continue name;` can target it, and [Clang's C status page ↗](https://clang.llvm.org/c_status.html) lists it under C2y as available from Clang 22.

**JavaScript and Java.** Both have exactly this feature, spelled without the quote: `outer: for (…) { … break outer; … continue outer; }`. Both also allow `break label` out of a labelled plain block — which is Rust's labelled block — and both refuse `continue` to one, as Rust does with `E0696`: measured, Node 20 says *Illegal continue statement: 'blk' does not denote an iteration statement* and `javac` 25 says *not a loop label: blk*. The one surprise for Java readers runs the other way: Java **reserves** `goto` and never uses it (`int goto = 1;` does not compile), while Rust does not reserve it at all. What Rust adds is the value: neither language's labelled block can produce one.

**ABAP.** There are no labels. `EXIT` leaves the innermost `DO`, `WHILE` or `LOOP AT` only, and `CONTINUE` and `CHECK` act on the innermost loop only, so leaving or skipping an outer pass from inside an inner loop is a flag set before the inner `EXIT` and tested after the inner `ENDDO` — the flag version above, line for line. The cleanup half does not transfer as neatly as it looks. `TRY … CLEANUP … ENDTRY` runs its `CLEANUP` block only when an exception leaves the `TRY` uncaught — not on a normal finish and not on `RETURN` or `EXIT` — so resources released there are released on one path out of several. A Rust `Drop` runs on all of them, which is the difference that makes the `goto` pattern unnecessary rather than merely renamed.

## Practice

**Three ways out of a grid of shelves.** A 4 × 4 grid `shelves[s][slot]` holds stock counts; `0` means the slot is empty. Use `[[3, 5, 2, 8], [4, 0, 6, 1], [7, 7, 7, 7], [0, 2, 9, 3]]`.

1. Find the first empty slot as `Option<(usize, usize)>` three ways — with a flag variable, with a labelled `break`, and with `find_map` and no loop — and print all three.
2. Total the stock on the shelves that have **no** empty slot, using `continue 'shelves` to abandon a shelf the moment you meet a `0`. Then write the same total as a chain.
3. Before running it, predict what your labelled-`break` version from step 1 prints if you delete the label from the `break` (and from the loop, so rustc does not warn about an unused label) — and how many times the answer variable gets written.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:loop_labels_kata -->
*[`loop_labels_kata.rs`](examples/loop_labels_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three ways out of a grid of shelves.
//!
//!   rustc --edition 2024 loop_labels_kata.rs -o /tmp/llk && /tmp/llk

fn main() {
    // shelves[s][slot] = how many items are in that slot; 0 means empty
    let shelves = [
        [3, 5, 2, 8],
        [4, 0, 6, 1],
        [7, 7, 7, 7],
        [0, 2, 9, 3],
    ];

    println!("1. The first empty slot, three ways");

    // (a) a flag variable, checked once in each loop
    let mut by_flag = None;
    let mut done = false;
    for (s, shelf) in shelves.iter().enumerate() {
        for (slot, &count) in shelf.iter().enumerate() {
            if count == 0 {
                by_flag = Some((s, slot));
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }

    // (b) a label, so one break leaves both loops
    let mut by_label = None;
    'shelves: for (s, shelf) in shelves.iter().enumerate() {
        for (slot, &count) in shelf.iter().enumerate() {
            if count == 0 {
                by_label = Some((s, slot));
                break 'shelves;
            }
        }
    }

    // (c) no loop at all: the search is a value
    let by_chain = shelves
        .iter()
        .enumerate()
        .find_map(|(s, shelf)| shelf.iter().position(|&count| count == 0).map(|slot| (s, slot)));

    println!("   (a) flag       {by_flag:?}");
    println!("   (b) label      {by_label:?}");
    println!("   (c) find_map   {by_chain:?}");

    println!();
    println!("2. Total stock on the shelves that have no empty slot");
    let mut full_shelves = Vec::new();
    let mut total = 0;
    'shelves: for (s, shelf) in shelves.iter().enumerate() {
        for &count in shelf {
            if count == 0 {
                continue 'shelves; // the rest of this shelf does not matter
            }
        }
        full_shelves.push(s);
        total += shelf.iter().sum::<i32>();
    }
    println!("   full shelves {full_shelves:?}, total stock {total}");

    let chain_total: i32 = shelves
        .iter()
        .filter(|shelf| shelf.iter().all(|&count| count > 0))
        .map(|shelf| shelf.iter().sum::<i32>())
        .sum();
    println!("   the same with filter(all > 0): {chain_total}");

    println!();
    println!("3. The prediction: (b) with the label taken off the break");
    let mut unlabelled = None;
    let mut assignments = 0;
    for (s, shelf) in shelves.iter().enumerate() {
        for (slot, &count) in shelf.iter().enumerate() {
            if count == 0 {
                unlabelled = Some((s, slot));
                assignments += 1;
                break;
            }
        }
    }
    println!("   answer {unlabelled:?}, written {assignments} times");
    println!("   The break ended each shelf's slot loop, and the shelf loop went");
    println!("   on to shelf 3, whose empty slot replaced the right answer.");
}
```
<!-- /source -->

<!-- output:loop_labels_kata -->
*Verified output of [`loop_labels_kata.rs`](examples/loop_labels_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The first empty slot, three ways
   (a) flag       Some((1, 1))
   (b) label      Some((1, 1))
   (c) find_map   Some((1, 1))

2. Total stock on the shelves that have no empty slot
   full shelves [0, 2], total stock 46
   the same with filter(all > 0): 46

3. The prediction: (b) with the label taken off the break
   answer Some((3, 0)), written 2 times
   The break ended each shelf's slot loop, and the shelf loop went
   on to shelf 3, whose empty slot replaced the right answer.
```
<!-- /output -->

</details>

## See also

- [Flow control](../flow_control/README.md) — the chapter overview, and where labels sit among the other jumps
- [`break`](../break_expressions/README.md) — the unlabelled trap this page fixes, and `break value`
- [`continue`](../continue_expressions/README.md) — the single-loop version of `continue 'outer`
- [The `loop` keyword](../the_loop_keyword/README.md) — the loop whose `break 'label value` becomes the loop's value
- [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) — the rule a labelled block extends with early exits
- [When a `for` loop beats a chain](../../24_Iterators/when_a_loop_beats_a_chain/README.md) — `break 'outer` against `find_map`, and where each wins
- [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) — the drop order the guards above print, and the `let _ =` trap
- [Raw identifiers](../../15_First_Programs/raw_identifiers/README.md) — `r#do`, and why the reserved list depends on the edition
- [*Rust in Action*, claims checked](../flow_control_claims_checked/README.md) — the book's §2.4 run claim by claim

## Sources

- The Rust Reference: [loop labels ↗](https://doc.rust-lang.org/reference/expressions/loop-expr.html#loop-labels) (shadowing follows local variables) · [labelled block expressions ↗](https://doc.rust-lang.org/reference/expressions/loop-expr.html#labeled-block-expressions) · [reserved keywords ↗](https://doc.rust-lang.org/reference/keywords.html#reserved-keywords)
- [Rust release notes ↗](https://doc.rust-lang.org/releases.html) — 1.65.0 stabilises breaking from labelled blocks ("label-break-value"); 1.63.0 removes the label/lifetime shadowing warnings; 1.41.0 adds the unused-label warning. Also [Announcing Rust 1.65.0 ↗](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0/)
- Error index: [E0695 ↗](https://doc.rust-lang.org/error_codes/E0695.html) · [E0696 ↗](https://doc.rust-lang.org/error_codes/E0696.html) · [E0767 ↗](https://doc.rust-lang.org/error_codes/E0767.html) · [E0426 ↗](https://doc.rust-lang.org/error_codes/E0426.html)
- *Rust in Action* (Tim McNamara, Manning 2021), §2.4.5 — the triple-nested `'outer` loop, its footnote on `continue`, and the note on `goto`
- [PEP 3136 ↗](https://peps.python.org/pep-3136/) and [its rejection ↗](https://mail.python.org/pipermail/python-3000/2007-July/008663.html) · [WG14 N3355, *Named loops* ↗](https://www.open-std.org/jtc1/sc22/wg14/www/docs/n3355.htm) · [SAP ABAP Keyword Documentation: `CLEANUP` ↗](https://help.sap.com/doc/abapdocu_752_index_htm/7.52/en-US/abapcleanup.htm)

## Po polsku

`break` bez etykiety opuszcza **tylko** najbliższą pętlę, dlatego z pętli zagnieżdżonej wychodzi się etykietą: `'rows: for … { for … { break 'rows; } }`. Apostrof wygląda jak czas życia (*lifetime*), ale nim nie jest — to osobna przestrzeń nazw, więc w jednej funkcji `'a` może być jednocześnie czasem życia i etykietą bloku, bez ostrzeżenia. Etykieta zastępuje zmienną-flagę w rodzaju `let mut done = false;`, którą trzeba ustawić w pętli wewnętrznej i sprawdzić w zewnętrznej — a o tym drugim sprawdzeniu łatwo zapomnieć. Przykład z książki *Rust in Action* (trzy nieskończone pętle i `break 'outer`, gdy `x + y + z > 1000`) zatrzymuje się w punkcie `(0, 0, 1001)`: wewnętrzna pętla po `z` sama przekracza próg, więc `x = 1` nigdy się nie pojawia. Z samym `break` skończyłaby się tylko pętla po `z`, a `y` szedłby dalej bez końca.

`continue 'outer` porzuca resztę obiegu pętli zewnętrznej — to naturalny zapis reguły „jeden zły element dyskwalifikuje całą grupę”. Od wersji 1.65 etykietę można postawić także na zwykłym bloku: `'check: { … break 'check wartość; … }` to blok z wczesnymi wyjściami i jedną wartością na końcu. Takiego bloku nie da się `continue`'ować (`E0696`), a gdy leży wewnątrz pętli, `break` i `continue` bez etykiety są odrzucane (`E0695`) — kompilator nie zgaduje, o który cel chodziło. Etykieta nie sięga do wnętrza domknięcia (`E0767`). Etykieta, której nic nie używa, to ostrzeżenie `unused_labels`; etykieta przesłaniająca inną o tej samej nazwie daje ostrzeżenie bez nazwy lintu, którego nie wycisza nawet `#![allow(warnings)]`.

Rust nie ma `goto` — i to słowo nie jest nawet zarezerwowane: `let goto = 1;` się kompiluje, a zarezerwowane `do` trzeba zapisać jako `r#do`. Wzorzec z C, czyli `goto` do sekcji sprzątającej na końcu funkcji, w Ruscie nie potrzebuje skoku, bo sprzątanie to `Drop` każdej wartości posiadającej zasób, uruchamiany przy **każdym** wyjściu z zakresu: normalnym, `return`, `?` i `break` z bloku z etykietą. W ABAP-ie etykiet nie ma (wyjście z dwóch poziomów to flaga), a blok `CLEANUP` w `TRY … ENDTRY` działa tylko wtedy, gdy wyjątek opuszcza `TRY` nieobsłużony — nie przy zwykłym zakończeniu i nie przy `RETURN` czy `EXIT`.

**Szukaj po polsku:** etykieta pętli · wyjście z pętli zagnieżdżonej · blok z etykietą · `rust labeled break outer loop` · `rust label break value block` · `rust goto` · `rust drop guard cleanup`
