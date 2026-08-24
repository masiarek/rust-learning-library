# `Copy` vs `Clone`

**Level:** 101 → 201 · working knowledge

**One line:** `Clone` is a method you call. `Copy` changes what `let b = a;` means. A struct is never `Copy` by accident.

```rust
let b = a;   // moves or copies. Nothing here says which; the type decides.
```

| | `Clone` | `Copy` |
|---|---|---|
| how you use it | `a.clone()` — you write it | nothing; `=` stops moving |
| what it may do | run your code, allocate | a bit-for-bit `memcpy`, always |
| after `let b = a;` | `a` is **moved** — dead | `a` is **copied** — alive |
| visible in the source | yes | **no** |

The last row is why they are separate traits. `Copy` hides duplication, which is only safe when duplication is trivial.

---

## `Copy` is not "cheap to duplicate"

It is **"duplicating is just a `memcpy`, and afterwards nobody owns anything extra."**

One `String` field disqualifies the whole struct. Copying it bit-for-bit duplicates the *pointer*: two owners, two frees. Not discouraged — unrepresentable.

## Three refusals, three codes

```text
error[E0277]: the trait bound `P: Clone` is not satisfied     // Copy requires Clone
error[E0204]: the trait `Copy` cannot be implemented for this type
  |  struct P { s: String }
  |         ^   --------- this field does not implement `Copy`
error[E0184]: `Copy` not allowed on types with destructors    // Drop runs once per value
```

Always `#[derive(Clone, Copy)]` together. `E0204` points at the offending field, which is the fastest way to see what a type owns.

## All-`Copy` fields is not enough

```rust
#[derive(Debug, Clone)]   // no Copy
struct Tally { counted: u32 }
```

One `u32`, still moves. You opt in, not the fields. `Copy` is a promise to callers that adding a `String` later would silently break.

## Which to reach for

1. **A reference.** Most "I need `Copy`" is really "my signature should take `&`".
2. **`Copy`** for small plain data — ids, coordinates, counters — where `&` everywhere is noise.
3. **`.clone()`** last, justified. A real allocation, written where it happens.

[Struct update syntax](../struct_update/README.md) shows it sharpest: one `..base` line copies the `Copy` fields and moves the rest, out of the same value.

## If you are coming from another language

**Python.** Every name binds a reference, so `b = a` never duplicates and this page's question does not arise.

| Python | Rust |
|---|---|
| `b = a` | a reference; both usable | a **move**; `a` is dead |
| `copy.copy` / `copy.deepcopy` | explicit | `.clone()` |
| — | no equivalent | `Copy` |

The move is the new idea. `Clone` is `deepcopy` renamed, and `Copy` names the types where a move and a copy are indistinguishable.

**ABAP.** Structure and internal-table assignment copies deeply, always — every ABAP type behaves like a Rust `Copy` type, so two-owners-of-one-allocation never happens.

| ABAP | Rust |
|---|---|
| `ls_b = ls_a.` | copy | move, unless the type is `Copy` |
| `REF TO` / `CREATE OBJECT` | the exception | the default for anything owning data |
| pass a ref, agree not to touch the original | convention | enforced by the compiler |

`Copy` is how you say "small enough that no agreement is needed".

---

## Practice

**One `E0382`, three fixes, and the field that removes one.** Write `Reading { precinct: u32, turnout: u32 }`, a function taking it **by value**, call it twice.

1. Fix three ways — derive `Copy`, `.clone()` at the call site, take `&Reading`. Rank them by what each costs the *caller*.
2. Change `precinct` to `String` and retry all three. One is impossible: get the error, explain it in one sentence about heap allocations.
3. Predict which of `#[derive(Clone)]`, `#[derive(Copy)]`, `impl Drop` can coexist. Check.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:copy_vs_clone_kata -->
*[`copy_vs_clone_kata.rs`](examples/copy_vs_clone_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one E0382, three fixes, and the field that forbids one of them.
//!
//!   rustc --edition 2024 copy_vs_clone_kata.rs -o /tmp/cvck && /tmp/cvck

// ---- Round 1: all fields Copy, so all three fixes are available -----------
#[derive(Debug, Clone, Copy)]
struct Reading {
    precinct: u32,
    turnout: u32,
}

fn report(r: Reading) -> String { format!("p{} turnout {}", r.precinct, r.turnout) }
fn report_ref(r: &Reading) -> String { format!("p{} turnout {}", r.precinct, r.turnout) }

// ---- Round 2: one String field, and Copy is now impossible ----------------
#[derive(Debug, Clone)]
struct Named {
    precinct: String, // <- this one field forbids Copy for the whole struct
    turnout: u32,
}

fn name_it(n: Named) -> String { format!("{} turnout {}", n.precinct, n.turnout) }
fn name_it_ref(n: &Named) -> String { format!("{} turnout {}", n.precinct, n.turnout) }

fn main() {
    println!("The bug: calling a by-value function twice.");
    println!("    let r = Reading {{ .. }};");
    println!("    report(r); report(r);");
    println!("    error[E0382]: use of moved value: `r`\n");

    println!("Fix 1 — derive Copy. The call sites do not change at all.");
    let r = Reading { precinct: 7, turnout: 431 };
    println!("    {}", report(r));
    println!("    {}   <- same `r`, copied again", report(r));

    println!("\nFix 2 — clone at the call site. Visible, and it costs.");
    println!("    {}", report(r.clone()));

    println!("\nFix 3 — take a reference. Nothing is duplicated at all.");
    println!("    {}", report_ref(&r));

    println!("\nWhich to ship? Fix 3, then Fix 1, and Fix 2 last.");
    println!("  A reference asks for the least and copies nothing, so it is the");
    println!("  default. Copy is right for a small, plain-data value where the");
    println!("  `&` would be noise. Clone at a call site is the one to justify:");
    println!("  it is a real allocation, and it is often papering over a signature");
    println!("  that should have taken `&` in the first place.");

    println!("\nRound 2 — swap `precinct` to a String, and one fix disappears.");
    let n = Named { precinct: "Riverside".to_string(), turnout: 431 };
    println!("    #[derive(Copy)] is now:");
    println!("      error[E0204]: the trait `Copy` cannot be implemented for this type");
    println!("        this field does not implement `Copy`");
    println!("    ...because Copy is a bit-for-bit duplicate, and duplicating a");
    println!("    String's pointer would give two owners of one allocation.");
    println!("    That is the double-free `Copy` exists to make unrepresentable.");
    println!("\n    So only two fixes remain:");
    println!("      clone     {}", name_it(n.clone()));
    println!("      reference {}", name_it_ref(&n));
    println!("      and `n` is still alive: {n:?}");

    println!("\nThe rule to carry away:");
    println!("  Copy is not 'cheap to duplicate' — it is 'duplicating is JUST a");
    println!("  memcpy, with nobody owning anything afterwards'. Any field that");
    println!("  owns a resource makes that false for the whole struct.");
}
```
<!-- /source -->

<!-- output:copy_vs_clone_kata -->
*Verified output of [`copy_vs_clone_kata.rs`](examples/copy_vs_clone_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The bug: calling a by-value function twice.
    let r = Reading { .. };
    report(r); report(r);
    error[E0382]: use of moved value: `r`

Fix 1 — derive Copy. The call sites do not change at all.
    p7 turnout 431
    p7 turnout 431   <- same `r`, copied again

Fix 2 — clone at the call site. Visible, and it costs.
    p7 turnout 431

Fix 3 — take a reference. Nothing is duplicated at all.
    p7 turnout 431

Which to ship? Fix 3, then Fix 1, and Fix 2 last.
  A reference asks for the least and copies nothing, so it is the
  default. Copy is right for a small, plain-data value where the
  `&` would be noise. Clone at a call site is the one to justify:
  it is a real allocation, and it is often papering over a signature
  that should have taken `&` in the first place.

Round 2 — swap `precinct` to a String, and one fix disappears.
    #[derive(Copy)] is now:
      error[E0204]: the trait `Copy` cannot be implemented for this type
        this field does not implement `Copy`
    ...because Copy is a bit-for-bit duplicate, and duplicating a
    String's pointer would give two owners of one allocation.
    That is the double-free `Copy` exists to make unrepresentable.

    So only two fixes remain:
      clone     Riverside turnout 431
      reference Riverside turnout 431
      and `n` is still alive: Named { precinct: "Riverside", turnout: 431 }

The rule to carry away:
  Copy is not 'cheap to duplicate' — it is 'duplicating is JUST a
  memcpy, with nobody owning anything afterwards'. Any field that
  owns a resource makes that false for the whole struct.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:copy_vs_clone -->
*Verified output of [`copy_vs_clone.rs`](examples/copy_vs_clone.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The difference is what `let b = a;` MEANS
   Precinct is Copy: after `let _also = p;`, p is fine -> Precinct { id: 7, registered: 431 }
   Ballot is not:   after `let moved = b;`, `b` is E0382
                    the value lives on as `moved` -> Ballot { voter: "Ada", scores: [5, 2, 0] }
   Same syntax. Different meaning. The TYPE decides.

2. Passing to a function is the same question
   consume_precinct(p) = 438 (id+registered), and p survives -> Precinct { id: 7, registered: 431 }
   consume_ballot(moved) = "Ada cast 3" and `moved` does not survive
   ...which is why `.clone()` is in that line at all.

3. All-Copy fields is NOT enough — you have to opt in
   Tally holds one u32 and still is not Copy, because it does
   not derive Copy. consume_tally(t) MOVES it:
   consume_tally(t) = 12
   `t` is now dead. Opting in is deliberate: making a type Copy
   is a promise to your callers that you cannot quietly take back.

4. Clone is a method you call; Copy is something the compiler does
   original  Ballot { voter: "Ben", scores: [4] }
   duplicate Ballot { voter: "Ben", scores: [4] }   <- a second heap allocation, on purpose
   `Copy` never allocates: it is a bit-for-bit copy, nothing else.

5. The three refusals, each with its own code
   impl Copy for P {} without Clone
     error[E0277]: the trait bound `P: Clone` is not satisfied
     -> `Copy` requires `Clone`. Always derive both together.
   #[derive(Copy)] on a struct holding a String
     error[E0204]: the trait `Copy` cannot be implemented for this type
       this field does not implement `Copy`
   #[derive(Copy)] on a struct that also impls Drop
     error[E0184]: `Copy` not allowed on types with destructors
     -> a destructor runs once per value; copies would run it twice.
```
<!-- /output -->

## See also

- [STRUCTS.md](../../STRUCTS.md) · [Struct update syntax](../struct_update/README.md) · [Ownership and moves](../ownership_and_moves/README.md) · [Borrowing](../borrowing/README.md)
