# `Option` vs `Result`

**Level:** 101 · for newcomers

**One line:** `Option<T>` says *there might be nothing here*. `Result<T, E>` says *this might have failed, and if it did, here is why*.

Everything else on this page follows from that one word: **why**.

---

## The whole distinction, in four lines

Neither type is built into the language. Both are ordinary enums you could have written yourself, and both live in the prelude so you never import them:

```rust
enum Option<T>    { Some(T), None }
enum Result<T, E> { Ok(T),   Err(E) }
```

Look at the sad arms. `None` carries nothing. `Err(E)` carries a payload. That is the *entire* structural difference between the two types, and it is also the rule for choosing between them.

**If a caller could sensibly ask "why not?", you need `Result`.** If the answer to "why not?" is obvious from the call itself, `Option` is right.

| The call | Sad answer | Why |
|---|---|---|
| `map.get("zoe")` | `None` | It isn't in the map. There is no other possible reason. |
| `vec.first()` | `None` | The vec is empty. Same — only one story. |
| `File::open(path)` | `Err(e)` | Missing? No permission? Too many open files? The caller may act differently on each. |
| `"4x".parse::<u32>()` | `Err(e)` | Empty string, bad digit, or overflow — three different bugs upstream. |

A second test, if the first leaves you unsure: **is absence even an error?** Asking a map for a key that isn't there is not a failure — you asked a question and got an honest answer. Failing to open a file *is* a failure. `Option` is for absence; `Result` is for failure.

### If you are coming from another language

- **Python.** `Option` is `None`, except the compiler will not let you forget to check it. `Result` is an exception that travels as a return value instead of unwinding the stack — you cannot accidentally let it fly past you.
- **ABAP.** `Result` is `sy-subrc` done properly: a return code you are *forced* to check, that carries the reason with it instead of making you look it up, and that cannot be silently clobbered by the next statement.

The gain in both cases is the same. Absence and failure stop being conventions that discipline enforces and become facts the type system enforces.

---

## Step 1 — Both are just enums

Nothing magic is happening. `Some`, `None`, `Ok`, and `Err` are enum variants, and every method you will meet below is ordinary library code written on top of `match`.

## Step 2 — `match` is the form that always works

```rust
match "5,2,0".parse::<u32>() {
    Ok(n)  => println!("parsed {n}"),
    Err(e) => println!("could not parse — {e}"),
}
```

The compiler rejects this if you leave an arm out. That single fact is the whole safety story; every shortcut below is sugar over it, and when a shortcut does not fit, `match` is always waiting.

## Step 3 — Shortcuts when you care about one arm

```rust
if let Some(name) = winner { … }          // do something only when present

let Some(text) = raw else { return; };    // bind, or bail — the guard clause

quorum.unwrap_or(0)                       // default, computed eagerly
quorum.unwrap_or_else(|| expensive())     // default, computed only if needed
quorum.unwrap_or_default()                // T::default(), for 0 / "" / empty
```

`let … else` is the one to internalise. It keeps the happy path unindented at the left margin, which is what stops Rust error handling from turning into a staircase.

## Step 4 — `map` vs `and_then`, the nesting trap

`map` transforms the value inside and leaves the wrapper alone:

```rust
let trimmed: Option<&str> = raw.map(|s| s.trim());
```

But if your closure *itself* returns an `Option`, `map` nests them and you get `Option<Option<u32>>` — almost never what you want:

```rust
trimmed.map(|s| s.parse::<u32>().ok())        // Some(Some(12))   ✗
trimmed.and_then(|s| s.parse::<u32>().ok())   // Some(12)         ✓
```

`and_then` flattens. (It is `flat_map` under a different name.) `Result` has the identical pair, plus `map_err` for reshaping the error without touching the success.

**Rule of thumb:** if the closure can itself fail or come up empty, reach for `and_then`.

## Step 5 — `?`, the whole point of the design

`?` means: *give me the happy value, or return the sad one from this function right now.*

```rust
fn first_word_len(s: &str) -> Option<usize> {
    let first = s.split_whitespace().next()?;   // None here → return None
    Some(first.len())
}

fn sum_scores(line: &str) -> Result<u32, ParseIntError> {
    let mut total = 0;
    for tok in line.split(',') {
        total += tok.trim().parse::<u32>()?;    // Err here → return that Err
    }
    Ok(total)
}
```

Two constraints worth knowing before they bite you:

1. `?` on an `Option` only works inside a function returning `Option`; `?` on a `Result` only inside one returning `Result`. You cannot mix them without converting — see Step 6.
2. On the `Result` side, `?` does **not** just return the error. It returns `Err(From::from(e))`, converting the error into your function's error type. That implicit `From` call is what makes Steps 8 and 9 work, and it is the single most useful thing to understand about `?`.

## Step 6 — Crossing between the two

```rust
maybe.ok_or("no quorum recorded")   // Option<T> → Result<T, E>   you SUPPLY a reason
maybe.ok_or_else(|| build_err())    // same, lazily
res.ok()                            // Result<T, E> → Option<T>   you DISCARD the reason
res.err()                           // Result<T, E> → Option<E>   keep only the reason
```

The asymmetry is the lesson. Going *up* to `Result` you must invent the missing information; going *down* to `Option` you throw information away. That is exactly why `.ok()` deserves a second look in review — it is where error detail goes to die.

## Step 7 — Two facts worth knowing early

**`.transpose()` flips the nesting.** `Option<Result<T, E>>` and `Result<Option<T>, E>` both come up constantly — "I may not have a value, and parsing it may fail" versus "parsing may fail, and it may legitimately yield nothing". One call converts between them.

**`Option` is often free.** `Option<Box<T>>` is the same size as `Box<T>`: the compiler uses the impossible null pointer to represent `None`. You get the null-safety without paying for it, which is why nobody in Rust reaches for a sentinel value like `-1`.

## Step 8 — Designing the `E` in `Result<T, E>`

For a library, name your failures. One variant per thing a caller might reasonably handle differently:

```rust
#[derive(Debug)]
enum BallotError {
    Empty,
    BadScore(ParseIntError),
    OutOfRange { got: u32, max: u32 },
}

impl From<ParseIntError> for BallotError {
    fn from(e: ParseIntError) -> Self { BallotError::BadScore(e) }
}
```

With `Display` and `Error` implemented, plus that `From`, this now works — the `?` converts a `ParseIntError` into a `BallotError` on its own:

```rust
let score: u32 = tok.trim().parse()?;
```

In real projects the `thiserror` crate writes the `Display`/`Error`/`From` boilerplate for you. It generates exactly this; there is nothing extra to learn once you can write it by hand.

## Step 9 — When you don't want to enumerate anything

For an application (as opposed to a library), erase the error type:

```rust
fn load_and_total(line: &str) -> Result<u32, Box<dyn Error>> {
    let ballot = parse_ballot(line)?;   // BallotError    → Box<dyn Error>
    let weight: u32 = "3".parse()?;     // ParseIntError  → Box<dyn Error>
    Ok(ballot.iter().sum::<u32>() * weight)
}
```

Two unrelated error types flow through one function because both convert into the box. The `anyhow` crate is this with better ergonomics and backtraces.

**The convention:** libraries name their errors so callers can match on them; applications erase them because nothing downstream will ever match. Choosing wrong is the most common error-handling mistake in Rust, and it is not subtle once you know the question — *will anyone `match` on this?*

---

## The verified output

Every line below came from actually compiling and running [`option_vs_result.rs`](examples/option_vs_result.rs) — it is regenerated by `tools/run_examples.py` and checked in CI, so it cannot drift away from the code.

<!-- output:option_vs_result -->
*Verified output of [`option_vs_result.rs`](examples/option_vs_result.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: Two enums; the difference is whether the sad arm carries a reason
  scores.get("ada")   -> Some(5)
  scores.get("zoe")   -> None
      "why not?" has exactly one answer, so None needs no payload.
  "42".parse::<u32>() -> Ok(42)
  "4x".parse::<u32>() -> Err(ParseIntError { kind: InvalidDigit })
      "why not?" has many answers, so Err carries one.

──── Step 2: match — the form that always works
  Option: counted 461 ballots
  Result: could not parse — invalid digit found in string
      The compiler rejects the match if you forget an arm. That is the whole safety story.

──── Step 3: Shortcuts for when you only care about one arm
  if let            -> winner is Ada
  let-else          -> bound "12" for the rest of the function
  unwrap_or         -> 0
  unwrap_or_else    -> 7
  unwrap_or_default -> 0

──── Step 4: map vs and_then — the nesting trap
  map               -> Some("12")
  map (wrong tool)  -> Some(Some(12))   <- Option<Option<u32>>
  and_then          -> Some(12)
  map_err           -> Ok(7)
  map_err           -> Err("bad score: invalid digit found in string")

──── Step 5: `?` — early return in one character
  first_word_len("hello world") -> Some(5)
  first_word_len("     ")       -> None
  sum_scores("5, 2, 0")         -> Ok(7)
  sum_scores("5, x, 0")         -> Err(ParseIntError { kind: InvalidDigit })
      `?` on an Option needs an Option-returning fn; on a Result, a Result-returning fn.

──── Step 6: Crossing between the two
  Option -> Result  .ok_or(reason) -> Err("no quorum recorded")
  Result -> Option  .ok()          -> None   (the reason is thrown away)
      Going up you must SUPPLY a reason; going down you DISCARD one. That asymmetry is the point.

──── Step 7: Two facts worth knowing early
  .transpose() flips the nesting -> Ok(Some(12))
  size_of  i32=4  Option<i32>=8  Box<i32>=8  Option<Box<i32>>=8
      Option<Box<T>> is free: None reuses the null pointer. Safety with no runtime cost.

──── Step 8: Designing the E in Result<T, E>
  "5,2,0" -> ok [5, 2, 0]
  "" -> error: the ballot line was empty
  "5,x,0" -> error: not a number: invalid digit found in string
  "5,9,0" -> error: score 9 is above the 5 cap
      `?` did the ParseIntError -> BallotError conversion. That is `From`, not magic.

──── Step 9: Box<dyn Error> — two unrelated error types in one function
  load_and_total("5,2,0") -> Ok(21)
  load_and_total("5,9,0") -> Err: score 9 is above the 5 cap
      Libraries name their errors (Step 8). Applications usually erase them (Step 9).
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/option_vs_result/examples/option_vs_result.rs -o /tmp/ovr && /tmp/ovr
```

---

## Cheat sheet

Same row = same idea on both types.

| What you want | `Option<T>` | `Result<T, E>` |
|---|---|---|
| Handle both arms | `match` | `match` |
| Handle one arm | `if let Some(x)` | `if let Ok(x)` |
| Bind or bail | `let Some(x) = … else` | `let Ok(x) = … else` |
| Transform the value | `.map(f)` | `.map(f)` |
| Transform, may fail | `.and_then(f)` | `.and_then(f)` |
| Transform the error | — | `.map_err(f)` |
| Fall back to a value | `.unwrap_or(v)` | `.unwrap_or(v)` |
| Fall back lazily | `.unwrap_or_else(f)` | `.unwrap_or_else(f)` |
| Propagate to the caller | `?` | `?` (converts via `From`) |
| Convert to the other | `.ok_or(e)` | `.ok()` |
| Flip the nesting | `.transpose()` | `.transpose()` |
| Is it the happy arm? | `.is_some()` | `.is_ok()` |
| Borrow instead of move | `.as_ref()` | `.as_ref()` |
| Crash if sad | `.expect("why")` | `.expect("why")` |

## Traps

- **`.unwrap()` in anything that ships.** Use `.expect("a message saying why this cannot fail")` instead. The message is you writing down the proof, and it is what you will read in the panic six months from now. Bare `.unwrap()` is fine in tests and throwaway code.
- **Ignoring a returned `Result`.** Both types are `#[must_use]`, so the compiler warns. If you genuinely mean to discard one, write `let _ = …` so the next reader can see it was a decision.
- **`.unwrap_or(expensive())`.** The argument is evaluated whether or not it is needed. Use `.unwrap_or_else(|| expensive())`.
- **Reaching for `.clone()` to escape a borrow.** When you need the inside of an `Option` without consuming it, the answer is usually `.as_ref()` (or `.as_deref()`), not a clone.
- **`Option` where the caller will ask "why not?".** The tell is a caller writing `None` handling that guesses at a cause, or a function whose docs explain in prose what `None` means. If the prose is needed, the type should have carried it.

## See also

- [The Rust Book, ch. 9 — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [`std::option`](https://doc.rust-lang.org/std/option/) and [`std::result`](https://doc.rust-lang.org/std/result/) — the method lists are worth one slow read; most of what you would write by hand is already there
- [Rust by Example — Error handling](https://doc.rust-lang.org/rust-by-example/error.html)
