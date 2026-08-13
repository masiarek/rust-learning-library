# Shadowing and `unwrap`

**Level:** 201 · working knowledge

**One line:** They are unrelated. Shadowing is about **names**; unwrapping is about **values** — and the popular explanation that ties them together is crediting shadowing for something `Copy` is doing.

They do meet in exactly one idiom, and it is worth learning: `let x = x.unwrap_or(…)`, where unwrapping changes the *type* and shadowing lets the *name* survive the change. Everything else said about the pair is folklore.

---

## The two ideas, separately

**Shadowing** is declaring a variable whose name is already in use. The new one hides the old for the rest of the scope; the old one is still there, unchanged, and comes back when the scope ends.

```rust
let score = "42";              // &str
let score = score.len();       // usize — a NEW variable, same name
```

**Unwrapping** is getting the `T` out of an `Option<T>` or `Result<T, E>` — by `match`, `if let`, `?`, [`unwrap`](../what_a_panic_costs/README.md), or [`unwrap_or`](../unwrap_or/README.md).

Nothing in either definition mentions the other. You can shadow without an `Option` anywhere in sight, and unwrap a hundred times without ever reusing a name.

## The claim under test

Here is the explanation that circulates, with the program it comes with:

> "Shadowing is useful in combination with `.unwrap()`. By using shadowing, you can declare a new variable with the same name as the unwrapped value and assign the unwrapped value to it. This allows you to conveniently access and work with the unwrapped value without affecting the original container object. […] This demonstrates how shadowing allows us to hold onto values even after using `.unwrap()`."

```rust
fn main() {
    let maybe_number: Option<i32> = Some(42);

    if let Some(number) = maybe_number {
        let number = number * 2;                  // "shadowing the unwrapped value"
        println!("The doubled value is: {number}");
    } else {
        println!("No value found!");
    }

    let unwrapped_number = maybe_number.unwrap_or(0);
    println!("The unwrapped value is: {unwrapped_number}");
}
```

It prints `84` then `42`, exactly as advertised. Three of its claims are still wrong.

### 1. The first binding is not shadowing

`if let Some(number) = maybe_number` introduces `number` by **pattern binding**. There is no outer `number` for it to hide — the name is new. The only actual shadow in the program is `let number = number * 2;` inside the block, and it shadows the pattern binding, not the `Option`.

### 2. The original survives because of `Copy`, not shadowing

Delete the shadow entirely and `maybe_number` is just as intact on the last line. What kept it alive is that `Option<i32>` is `Copy`, so `if let` copied the value out and left the original where it was.

Swap in a type that is not `Copy` and the same program stops compiling — however you write the inside of the block:

```rust
let maybe_name: Option<String> = Some("Ada".to_string());

if let Some(name) = maybe_name {
    let name = format!("{name} {name}");
    println!("The doubled value is: {name}");
}

let unwrapped_name = maybe_name.unwrap_or_default();   // ← does not compile
```

rustc, on exactly that program:

```text
error[E0382]: use of partially moved value: `maybe_name`
 --> shadowing_move.rs:9:26
  |
4 |     if let Some(name) = maybe_name {
  |                 ---- value partially moved here
...
9 |     let unwrapped_name = maybe_name.unwrap_or_default();
  |                          ^^^^^^^^^^ value used here after partial move
  |
  = note: partial move occurs because value has type `String`, which does not
          implement the `Copy` trait
help: borrow this binding in the pattern to avoid moving the value
  |
4 |     if let Some(ref name) = maybe_name {
  |                 +++
```

The compiler names the mechanism itself, and it is not shadowing. The fixes are all about *ownership* — borrow the option (`&maybe_name`), borrow inside the pattern (`Some(ref name)`), reach for `.as_ref()` or `.as_deref()`, or clone if you must.

### 3. A shadow cannot "hold onto" anything

The claim is backwards. A shadow is scoped: it ends at the closing brace and the outer name returns. It is the *first* thing to disappear, so it is the last mechanism you would reach for to keep a value around.

```rust
let score = 42;
{
    let score = score * 2;   // 84 in here
}
// 42 again out here
```

## So what is shadowing genuinely for?

**Keeping one name while the type changes underneath it.** That is the whole feature, and it is why it turns up next to unwrapping so often:

```rust
fn tally(quorum: Option<u32>) -> u32 {
    let quorum = quorum.unwrap_or(0);   // Option<u32> in, u32 out, same name
    …
}
```

Without it you would be inventing `quorum_opt` and `quorum_value`, and every later line would have to remember which one it is holding. The `Option` version is gone from that point on — not hidden by convention, *inaccessible* — so nothing downstream can use the wrong one.

The same move, three ways you will see it:

```rust
let config = config.unwrap_or_default();      // fall back
let Some(config) = config else { return 1 };  // or bail  (let-else)
let config = config?;                          // or hand it to the caller
```

`let … else` is the one to internalise: after that line `config` is a plain value for the rest of the function, and the failure path left at the top.

### Shadowing is not `mut`

Easy to conflate, genuinely different:

| | `let mut x = …; x = …;` | `let x = …; let x = …;` |
|---|---|---|
| How many variables | one, reused | two; the second hides the first |
| Can the type change? | **no** — `mut` retypes nothing | **yes**, and that is the point |
| Lasts until | reassigned again | the end of the scope |
| Needs `mut` | yes | no — each binding can stay immutable |

The second column is what makes the unwrap idiom possible at all: `Option<u32>` → `u32` is a type change, so `mut` could never have expressed it.

### If you are coming from another language

- **Python.** `x = int(x)` looks like the same move, and reads the same, but Python is *rebinding one name* with no type discipline; Rust is creating a second variable, and the first still exists in the enclosing scope. The practical gain is the one Python cannot offer: after `let x = x.unwrap_or(0)`, the optional version is not merely discouraged, it is unreachable.
- **ABAP.** You cannot do this at all — a `DATA` name is one typed variable for the whole routine, which is why ABAP code grows `lv_amount_str` / `lv_amount` pairs and every later line has to keep them straight. Shadowing removes the pair: same name, and the compiler guarantees you are past the conversion.

---

## The verified output

<!-- output:shadowing_and_unwrap -->
*Verified output of [`shadowing_and_unwrap.rs`](examples/shadowing_and_unwrap.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: The example as it is usually written
  The doubled value is: 84
  The unwrapped value is: 42
      Two questions this raises: what did the shadow do, and why is
      maybe_number still usable on the last line?

──── Step 2: The shadow did nothing to protect the original
  no shadow at all      -> doubled = 84
  maybe_number afterwards -> Some(42)
      Still intact. The shadow was never what kept it alive: `if let`
      COPIED the i32 out, because Option<i32> is Copy.

──── Step 3: Take Copy away and the same code stops compiling
  borrowed, then shadowed -> Ada Ada
  maybe_name afterwards   -> Some("Ada")
      Option<String> is not Copy, so the by-value version is a compile
      error no matter how the inside of the block is written.

──── Step 4: What shadowing IS for: the name stays, the type changes
  before: quorum is None   type core::option::Option<i32>
  after:  quorum is 0      type i32
      This is the one place the two ideas genuinely meet: unwrapping
      changes the TYPE, and shadowing lets the NAME survive the change.
      Without it you would invent quorum_opt / quorum_value pairs.

──── Step 5: A shadow ends with its block; the outer name comes back
  inside the block  -> 84
  after the block   -> 42
      So shadowing cannot 'hold onto' an unwrapped value for later —
      the shadow is the thing that disappears first.

──── Step 6: Shadowing is not mutation
  let mut: 1, type i32
  shadowed three times: 12, type u32
      `mut` reuses one variable and its type is fixed. Each `let` makes a
      NEW variable, so the type may change — which is the whole point.

──── Step 7: The idiom worth copying: let-else
  seats_for(Some(3)) -> 3
  seats_for(None)    -> 1
      Inside the function, `config` is a u32 rather than an Option<u32>
      from that line on, and nothing downstream can forget to unwrap it.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/shadowing_and_unwrap/examples/shadowing_and_unwrap.rs -o /tmp/sau && /tmp/sau
```

## Traps

- **Reading `if let Some(x) = opt` as shadowing.** It is a pattern binding. The distinction matters the moment `opt` is not `Copy`, because then the binding *moves* and the difference is a compile error rather than a naming question.
- **Assuming the original option survives.** It does for `Option<i32>`, `Option<bool>`, `Option<&T>` — anything `Copy`. It does not for `Option<String>`, `Option<Vec<T>>`, or your own structs. Borrow with `&opt` or `.as_ref()` when you need the option afterwards.
- **Shadowing a name you still need.** The old value is not gone, but it is unreachable for the rest of the scope. If you find yourself wanting both, they are two different things and deserve two names.
- **Shadowing across a long function.** Retyping the same name three lines apart is clear; doing it forty lines in makes the reader scroll to find out what `config` currently *is*. Shadowing near the top, once, is the readable form.
- **Reaching for `mut` to change a type.** It cannot. That error (`E0308: mismatched types`) is often a shadow trying to happen.

## See also

- [`unwrap`: the bet you are making](../what_a_panic_costs/README.md) — the other half of the pair: what unwrapping decides on your behalf
- [`unwrap_or`](../unwrap_or/README.md) — the fallback in `let x = x.unwrap_or(…)`, and the three things it costs
- [`Option` vs `Result`](../option_vs_result/README.md) — `let … else`, `?`, and the rest of the ways to stop holding a wrapper
- [The Rust Book, ch. 3.1 — Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)
- [`Option::as_ref`](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref) and [`as_deref`](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref) — the usual answer when a move gets in your way
