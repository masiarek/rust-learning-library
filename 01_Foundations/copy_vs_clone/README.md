# `Copy` vs `Clone`

**Level:** 101 → 201 · working knowledge

**One line:** `Clone` gives you a method you call by name; `Copy` changes what `let b = a;` *means* — and a struct is never `Copy` by accident, because every field has to be `Copy` **and** you have to opt in.

```rust
let b = a;
```

That line either **moves** `a` or **copies** it, and nothing in the line says which. The type decides, and the difference is whether `a` is still usable on the next line.

---

## The one difference that matters

| | `Clone` | `Copy` |
|---|---|---|
| how you use it | `a.clone()` — you write it | nothing — `=` just stops moving |
| what it may do | run your code, allocate, be slow | a bit-for-bit `memcpy`, always |
| after `let b = a;` | `a` is **moved** — dead | `a` is **copied** — alive |
| visible in the source | yes | **no** |

That last row is the whole reason the two are separate traits. Making a type `Copy` makes duplication *invisible*, which is only safe when duplication is *trivial*. `Clone` is for everything else, and its verbosity is the feature: an allocation you can see is an allocation you can question.

## `Copy` is not "cheap to duplicate"

It is **"duplicating this is just a `memcpy`, and afterwards nobody owns anything extra."**

That is why a `String` field poisons it for the whole struct. Copying a `String` bit-for-bit would duplicate its *pointer*, giving two owners of one heap allocation and two frees at the end of scope. `Copy` does not forbid that as a style rule — it makes it unrepresentable.

## The three refusals, each with its own code

```text
error[E0277]: the trait bound `P: Clone` is not satisfied
  |  impl Copy for P {}
  = note: required by a bound in `Copy`
```

`Copy` **requires** `Clone` as a supertrait, so you cannot have one without the other. Always `#[derive(Clone, Copy)]` together.

```text
error[E0204]: the trait `Copy` cannot be implemented for this type
  |  struct P { s: String }
  |         ^   --------- this field does not implement `Copy`
```

One non-`Copy` field is enough. The compiler points at the field, which makes this the fastest way to find out what is actually owned inside a type you did not write.

```text
error[E0184]: the trait `Copy` cannot be implemented for this type;
              the type has a destructor
```

A `Drop` impl runs **once per value**. If the value could be copied, it would run per copy, on the same resource. So `Copy` and `Drop` are mutually exclusive, and that is not a limitation — it is the same double-free argument as above, arriving from the other direction.

## All-`Copy` fields is not enough

```rust
#[derive(Debug, Clone)]   // note: no Copy
struct Tally { counted: u32 }
```

One `u32`, and it still moves. Nothing about the fields opts a struct in — **you** do, and the deliberateness is the point: `Copy` is a promise to every caller that passing the value by value costs them nothing. Adding a `String` field later would silently break every call site that relied on it, so Rust makes you say it out loud first.

## Which to reach for

1. **A reference first.** `fn report(r: &Reading)` copies nothing and asks for the least. Most "I need `Copy`" moments are really "my signature should have taken `&`".
2. **`Copy` for small plain data** — coordinates, ids, counters — where an `&` at every call site would be noise. Rough guide: a couple of machine words, no field that owns anything.
3. **`.clone()` last**, and justify it. It is a real allocation, written where it happens.

## Where this shows up

[Struct update syntax](../struct_update/README.md) is the sharpest demonstration in the library: one `..base` line copies the `Copy` fields and moves the non-`Copy` ones, out of the *same value*, and the error names the field rather than the value. If this page is abstract, that one is the same idea with the compiler pointing at it.

## Coming from another language

- **Python.** Every assignment binds a reference, so `b = a` never duplicates and `copy.deepcopy` is the explicit escape. Rust's `Clone` is roughly the `deepcopy` — and Rust's *move* has no Python equivalent at all, which is the genuinely new idea. `Copy` then names the small set of types where a move and a copy are indistinguishable.
- **ABAP.** Assignment of a structure copies it, always. Rust's `Copy` types behave exactly that way, and everything else does not — the closest ABAP analogue to a move is passing a reference and then agreeing, by convention, not to touch the original. Rust makes that agreement the compiler's job.

---

## Practice

**One `E0382`, three fixes, and the field that removes one of them.** Write a small `Reading { precinct: u32, turnout: u32 }`, a function taking it **by value**, and call that function twice. Read the error.

Now fix it three ways — derive `Copy`, `.clone()` at the call site, and change the signature to take `&Reading` — and rank them for this type. Say what each costs the *caller*, not the callee.

Then change `precinct` to a `String` and try all three again. One is now impossible; get its error and explain, in one sentence about heap allocations, why that follows rather than being an arbitrary rule.

Finally, predict which of `#[derive(Clone)]`, `#[derive(Copy)]` and `impl Drop` can coexist on one type, then check.

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

- [STRUCTS.md](../../STRUCTS.md) — the map: every struct lesson in reading order
- [Struct update syntax](../struct_update/README.md) — `Copy` deciding, field by field, inside one line
- [Ownership and moves](../ownership_and_moves/README.md) — what a move is, and what `Copy` opts out of
- [Borrowing](../borrowing/README.md) — the fix that beats both, most of the time
