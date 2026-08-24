# Struct update syntax, and the partial move

**Level:** 101 → 201 · working knowledge

**One line:** `..base` fills in every field you did not name — by **moving** them one at a time, so the base is not dead afterwards, it is *partially* dead, and exactly which half survives is decided field by field by `Copy`.

```rust
let user2 = User {
    email: "another@example.com".to_string(),
    ..user1
};
```

Read as English that says *"and the rest like `user1`"*, which is what it does. Read as Rust it is an **assignment of each remaining field**, and assignment in Rust moves. That second reading is the one that decides what compiles next.

---

## First, the two things it is not

**It is not nested structs.** *Reusing fields from another struct* and *a field whose type is another struct* sound alike and share nothing:

```rust
let b = Ballot { notes: "…".into(), ..a };   // VALUES, from another instance of the SAME type
struct Rectangle { top_left: Point }         // a TYPE, as the type of a field
```

The first is this page. The second is just a field, and needs no special syntax at all.

**It is not a copy constructor.** `..base` calls none of your code, clones nothing, and cannot convert anything. If you want the base intact *and* a duplicate, that is `.clone()`.

## The rule, in one line

**`..base` moves exactly the fields you did not name — and only the non-`Copy` ones among those actually go dead.**

Everything else on this page follows from that. Take a four-field `User` and write `User { email: …, ..user1 }`:

| Field | Named? | `Copy`? | After |
|---|---|---|---|
| `email` | yes | — | untouched on `user1` — you supplied your own |
| `active` | no | yes | **copied**; still readable on `user1` |
| `sign_in_count` | no | yes | **copied**; still readable on `user1` |
| `username` | no | no | **moved**; reading it is `E0382` |

So `user1` is not gone. Three of its four fields are still perfectly readable, and one is not:

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

Note what the compiler names: not `user1`, but **`user1.username`**. The borrow checker tracks this per field, and that precision is the whole behaviour. This is also the cleanest place to *see* what `Copy` buys you — the same line copies two fields and moves one, and the only difference is the field's type.

## Three ways to keep the base whole

1. **Name every non-`Copy` field yourself.** Then `..base` carries only `Copy` fields and nothing goes dead.
2. **Clone into the base slot** — `..base.clone()`. Honest about the cost, because the cost is now written down.
3. **`..Default::default()`.** The base is a temporary nobody holds, so there is no binding left half-moved. This is why config structs are built this way, and it is the form you will write most often.

## Two rules of syntax

`..base` must come **last**, and takes **no trailing comma**. The second has its own error, which is friendlier than most:

```text
error: cannot use a comma after the base struct
   |       ..my_instance,
   |       ^^^^^^^^^^^^^- help: remove this comma
   |
   = note: the base struct must always be the last field
```

## Coming from another language

- **Python.** The nearest thing is `dataclasses.replace(user1, email=...)` or `{**d, "email": ...}` — and both of those leave the original completely intact, because Python copies a *reference*. Rust's version relocates the owned data instead. If you want Python's behaviour, that is `..base.clone()`, and the difference is exactly the allocation you are now choosing to pay for.
- **ABAP.** `MOVE-CORRESPONDING` is the close cousin, and it is a **copy** — the source structure is untouched afterwards. Rust's `..` looks like the same operation and is not, for the fields that own heap data.

---

## Practice

**Predict which half of the base survives.** Take a struct with two `Copy` fields and two `String` fields, build a second value from it naming only *one* of the `String`s, and — before compiling — write down for each of the four fields whether it is still readable on the base and why. Then check, and make the failing read happen so you see `E0382` name the field rather than the value.

Then make the base survive three different ways, and say which you would ship for a config struct and which for amending one record.

Finally, add a trailing comma after `..base` and read that error too. It is a different error from every other one on this page, and knowing it on sight saves a minute.

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

- [STRUCTS.md](../../STRUCTS.md) — the map: every struct lesson in reading order
- [What a struct is](../what_a_struct_is/README.md) — the three flavors, and why `mut` is on the binding
- [Ownership and moves](../ownership_and_moves/README.md) — what a move is, before it happens field by field
- [`unwrap_or_default`](../unwrap_or_default/README.md) — a derived `Default` is the type's zero, not your domain's
