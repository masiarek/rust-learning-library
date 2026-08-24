# Struct update syntax, and the partial move

**Level:** 101 → 201 · working knowledge

**One line:** `..base` fills every field you did not name, by **moving** them one at a time. The base ends up *partially* dead; `Copy` decides which half survives.

```rust
let user2 = User { email: "another@example.com".to_string(), ..user1 };
```

As English: *"and the rest like `user1`"*. As Rust: an **assignment of each remaining field**. Assignment moves.

---

## Two things it is not

```rust
let b = Ballot { notes: "…".into(), ..a };   // VALUES from another instance of the SAME type
struct Rectangle { top_left: Point }         // a TYPE, as a field's type — unrelated
```

Not a copy constructor either — `..base` runs none of your code. Base intact *and* a duplicate is `.clone()`.

## The rule

**`..base` moves exactly the fields you did not name — and only the non-`Copy` ones go dead.**

| Field | Named? | `Copy`? | After |
|---|---|---|---|
| `email` | yes | — | untouched on `user1` |
| `active` | no | yes | copied; still readable |
| `sign_in_count` | no | yes | copied; still readable |
| `username` | no | no | **moved**; reading it is `E0382` |

```text title="Real rustc output"
error[E0382]: borrow of moved value: `user1.username`
  |
5 |     let user2 = User { email: "b@example.com".to_string(), ..user1 };
  |                 ---------------------------------------------------- value moved here
7 |     println!("{}", user1.username);
  |                    ^^^^^^^^^^^^^^ value borrowed here after move
  |
  = note: move occurs because `user1.username` has type `String`,
          which does not implement the `Copy` trait
```

It names **`user1.username`**, not `user1`. The borrow checker tracks this per field. Clearest place in the library to watch [`Copy`](../copy_vs_clone/README.md) work — one line copies two fields and moves a third.

## Keeping the base whole

1. **Name every non-`Copy` field.** Then `..base` carries only `Copy` fields.
2. **`..base.clone()`.** The cost is written down.
3. **`..Default::default()`.** The base is a temporary nobody holds, so nothing is stranded. This is why config structs are built this way.

## Syntax

`..base` comes **last**, no trailing comma:

```text
error: cannot use a comma after the base struct
   |       ..my_instance,
   |       ^^^^^^^^^^^^^- help: remove this comma
   |
   = note: the base struct must always be the last field
```

## If you are coming from another language

**Python.** `dataclasses.replace(user1, email=…)`, or `{**d, "email": …}` for dicts.

```python
user2 = dataclasses.replace(user1, email="b@example.com")
user1.username   # still fine
```

Python copies a *reference*, so the string is shared and the original is untouched. Rust relocates the owned data. `..base.clone()` is the line that matches Python's behaviour, and the difference between the two is the allocation Python was doing for you.

**ABAP.** `MOVE-CORRESPONDING ls_source TO ls_target`, or `CORRESPONDING #( ls_source )`.

```abap
MOVE-CORRESPONDING ls_source TO ls_target.
" ls_source unchanged, deep components included
```

Always a copy. Rust's `..` looks like the same operation and is not, for any field owning heap data. Nothing in `..user1` signals that `user1` lost something.

---

## Practice

**Predict which half survives.** A struct with two `Copy` fields and two `String` fields; build a second value naming only *one* `String`. Before compiling, write down per field whether it is still readable, and why.

1. Check. Then trigger the failing read and watch `E0382` name the field, not the value.
2. Keep the base whole three ways. Which for a config struct, which for amending one record?
3. Add a trailing comma after `..base` — a different error from every other one here.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:struct_update_kata -->
*[`struct_update_kata.rs`](examples/struct_update_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: predict which half of the base survives a `..base`.
//!
//!   rustc --edition 2024 struct_update_kata.rs -o /tmp/suk && /tmp/suk

#[derive(Debug, Default)]
struct Ballot {
    precinct: u32,       // Copy
    counted: bool,       // Copy
    voter: String,       // NOT Copy
    notes: String,       // NOT Copy
}

fn main() {
    let original = Ballot {
        precinct: 12,
        counted: false,
        voter: "Ada".to_string(),
        notes: "handed in late".to_string(),
    };

    // Only `notes` is named, so `..original` supplies the other THREE.
    let amended = Ballot { notes: "resolved".to_string(), ..original };

    println!("Predict, field by field, what is still readable on `original`:\n");
    println!("  precinct  Copy      -> copied   -> {} still readable", original.precinct);
    println!("  counted   Copy      -> copied   -> {} still readable", original.counted);
    println!("  voter     NOT Copy  -> MOVED    -> reading it is E0382");
    println!("  notes     NOT Copy  -> not taken (we named it) -> {:?} still readable", original.notes);
    println!("\n  amended = {amended:?}");

    println!("\nThe rule in one line:");
    println!("  `..base` moves exactly the fields you did NOT name,");
    println!("  and only the non-Copy ones among those actually go dead.");

    println!("\nThree ways to keep the base whole:");
    let base = Ballot { precinct: 3, counted: true,
                        voter: "Ben".to_string(), notes: "n/a".to_string() };

    // 1. Name every non-Copy field yourself.
    let a = Ballot { voter: "Cara".to_string(), notes: "n/a".to_string(), ..base };
    println!("  1. name every non-Copy field   -> base alive: {:?}", base.voter);

    // 2. Clone the base into the update position.
    let b = Ballot { precinct: 99, ..clone_ballot(&base) };
    println!("  2. clone into the base slot    -> base alive: {:?}", base.voter);

    // 3. Use a temporary nobody holds.
    let c = Ballot { voter: "Dan".to_string(), ..Default::default() };
    println!("  3. ..Default::default()        -> nothing to strand");

    println!("\n  {a:?}\n  {b:?}\n  {c:?}");

    println!("\nAnd the syntax trap, which is its own error:");
    println!("  Ballot {{ precinct: 1, ..base, }}");
    println!("    error: cannot use a comma after the base struct");
    println!("    note: the base struct must always be the last field");
}

// Ballot does not derive Clone here on purpose — this spells out that the
// "clone" in option 2 is ordinary code, not something `..` does for you.
fn clone_ballot(b: &Ballot) -> Ballot {
    Ballot {
        precinct: b.precinct,
        counted: b.counted,
        voter: b.voter.clone(),
        notes: b.notes.clone(),
    }
}
```
<!-- /source -->

<!-- output:struct_update_kata -->
*Verified output of [`struct_update_kata.rs`](examples/struct_update_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Predict, field by field, what is still readable on `original`:

  precinct  Copy      -> copied   -> 12 still readable
  counted   Copy      -> copied   -> false still readable
  voter     NOT Copy  -> MOVED    -> reading it is E0382
  notes     NOT Copy  -> not taken (we named it) -> "handed in late" still readable

  amended = Ballot { precinct: 12, counted: false, voter: "Ada", notes: "resolved" }

The rule in one line:
  `..base` moves exactly the fields you did NOT name,
  and only the non-Copy ones among those actually go dead.

Three ways to keep the base whole:
  1. name every non-Copy field   -> base alive: "Ben"
  2. clone into the base slot    -> base alive: "Ben"
  3. ..Default::default()        -> nothing to strand

  Ballot { precinct: 3, counted: true, voter: "Cara", notes: "n/a" }
  Ballot { precinct: 99, counted: true, voter: "Ben", notes: "n/a" }
  Ballot { precinct: 0, counted: false, voter: "Dan", notes: "" }

And the syntax trap, which is its own error:
  Ballot { precinct: 1, ..base, }
    error: cannot use a comma after the base struct
    note: the base struct must always be the last field
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:struct_update -->
*Verified output of [`struct_update.rs`](examples/struct_update.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. What it saves you
   user2 = User { email: …, ..user1 }
   User { active: true, sign_in_count: 1, username: "someusername123", email: "another@example.com" }
   `..base` must come LAST, and takes no trailing comma.

2. It is an assignment, so it MOVES — and it moves per FIELD
   user1.active        true   <- bool is Copy, so it was copied
   user1.sign_in_count 1   <- u64 is Copy, so it was copied
   user1.username      -- moved out, and now unusable:
     error[E0382]: borrow of moved value: `user1.username`
     note: move occurs because `user1.username` has type `String`,
           which does not implement the `Copy` trait
   user1.email         someone@example.com   <- NOT moved: user2 supplied its own
   user2.username      someusername123   <- this is user1's String, relocated
   So `user1` is not dead. It is PARTIALLY moved, field by field.

3. Name every non-Copy field and the base survives intact
   base is still whole:  User { active: true, sign_in_count: 7, username: "ada", email: "ada@example.com" }
   and the new one:      User { active: true, sign_in_count: 7, username: "ben", email: "ben@example.com" }

4. `..Default::default()` never strands anything
   User { active: false, sign_in_count: 0, username: "cara", email: "" }
   The base is a temporary nobody holds, so there is no binding
   left half-moved. That is why config structs use this form.

5. It is not a copy constructor
   `..base` does not clone, and it does not call any of your code.
   If you want `base` intact and a full duplicate, that is `.clone()`.
```
<!-- /output -->

## See also

- [STRUCTS.md](../../STRUCTS.md) · [`Copy` vs `Clone`](../copy_vs_clone/README.md) · [What a struct is](../what_a_struct_is/README.md) · [Ownership and moves](../ownership_and_moves/README.md)
