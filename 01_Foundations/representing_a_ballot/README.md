# What is a ballot, in memory?

**Level:** 201 · working knowledge

**One line:** One voter's scores can be stored at least six ways in Rust, and the choice does not merely trade memory for speed — it decides which bugs are *possible to write*.

Ada 5, Ben 3, Cara 0. That is the entire content of a STAR ballot, and it is the smallest interesting data-modelling question there is: small enough to hold in your head, big enough that every major Rust container has an honest claim on it.

---

## Before the types: what is a ballot *mathematically*?

Worth doing first, because it explains why the three big ballot families need three different Rust shapes rather than three flavours of the same one.

| Ballot family | The mathematical object | The natural Rust type |
|---|---|---|
| **Score / STAR** | a **function** from candidate to a value in 0..=5 | `[Score; N]` — one slot per candidate, always |
| **Approval** | a **subset** of the candidates | a bitset (`u64`), or `HashSet<CandidateId>` |
| **Ranked** | an **ordering** of the candidates — a permutation, and usually a *partial* one | `Vec<CandidateId>` — the order *is* the data |

A score ballot has a slot per candidate whether or not the voter used it. A ranked ballot has no slots at all: its content is a sequence, and a voter who ranks two of five candidates has given you a sequence of length two, not a length-five thing with holes. Rank *ties* ("I rank these two equal") push it further, to `Vec<Vec<CandidateId>>` — a sequence of tiers. That is why a codebase cannot quietly reuse one ballot type for both families, and why the sibling [star-voting-library ↗](https://github.com/masiarek/star-voting-library) has one parser for `5,3,0` and a different one for `A>C>B`.

Everything below is about the score ballot, the simplest of the three.

## Six shapes for the same three numbers

| Shape | Rust | Makes easy | Makes possible to get wrong | Reach for it when |
|---|---|---|---|---|
| Fixed array | `[Score; 3]` | Everything; it *is* the data, no heap, `Copy` | Column meaning lives elsewhere | The candidate count is known when you compile |
| Growable list | `Vec<Score>` | A count known only at runtime | Same column problem, plus a length mismatch | The real world — you load elections from files |
| Tuple struct | `Row(Score, Score, Score)` | Fixing the *arity* in the type | `.0` / `.1` are still positions | You want an array with a name |
| Named struct | `Ballot { ada, ben, cara }` | Naming; misalignment becomes unwriteable | Nothing much — and that is the point | The candidates are fixed and few |
| Map | `BTreeMap<CandidateId, Score>` | Lookup by who, sparse ballots, no order at all | Iteration order (see below) | Candidates vary per ballot |
| Flat matrix | one `Vec<u8>`, row-major | The whole election in one allocation | Index arithmetic | You are writing the actual engine |

The sizes are worth seeing (they are printed by the program below): `[u8; 3]`, `[Stars; 3]`, the tuple struct and the named struct are **all 3 bytes** — identical layouts, differing only in what you are allowed to say about them. A `Vec<u8>` is **24 bytes before any data exists**, because it is a handle: pointer, length, capacity. That 24 bytes is precisely the price of not knowing the candidate count until the program runs, and it is usually worth paying.

## The bug positional storage cannot see

Three ballots, `[[5,3,0], [4,5,1], [0,5,4]]`, and a header naming the columns:

```text
header Ada,Ben,Cara -> Ada 9, Ben 13, Cara 5   winner: Ben
header Ben,Ada,Cara -> Ben 9, Ada 13, Cara 5   winner: Ada
```

Same numbers, different winner, and **nothing failed** — no parse error, no panic, not even a warning. The meaning of column 0 was never in the data; it was in a header the type system never saw. Every positional format has this hole, which is why CSV imports go wrong everywhere and always have.

The named struct closes it by construction: you write `.ada`, not `[0]`, and swapping the two is not a mistake you can make. The price is severe, though — the candidates are baked into the *type*, so a program that loads an election from a file cannot use it. **That tension is the actual content of this lesson.** Naming is safest and least flexible; positions are flexible and unsafe; the real engine buys back safety by making the position→candidate mapping a *value it carries* rather than a convention it remembers, which is what the flat matrix below does and what [rung 3](../../ROADMAP.md) is about.

## Blank is not zero

A voter who writes `0` and a voter who leaves the box empty both contribute nothing to the total. They did not do the same thing.

```rust
let scored_zero: Vec<Option<Stars>> = vec![Some(Five), Some(Three), Some(Zero)];
let left_blank:  Vec<Option<Stars>> = vec![Some(Five), Some(Three), None];
```

Both total 8. Only the second remembers that Cara was never considered. `Vec<Option<Stars>>` is the type that can tell you *"2 of 3 candidates marked"*, which is a real quantity in real elections — and because `Stars` has six variants and 250 spare bit patterns, `Option<Stars>` is **still one byte**. You are asking for a distinction the machine gives away free, and it is the same [niche optimization](../option_as_collection/README.md) as before, now paying for something you actually wanted.

Whether to keep it is a modelling decision, not a performance one. The tabulation does not care. The audit does.

## Maps, and the determinism trap

A map deletes the column problem outright: the key *is* the meaning, ballots may be sparse, and no header can drift out of step.

But this program deliberately does **not** print a `HashMap`'s iteration order, and the reason is the point. `HashMap` randomises its hash seed per process, so the order changes run to run — the recorded answer key this library is built on could not exist. Now transplant that into an election: any procedure that resolves a tie by *scanning* a map inherits exactly that irreproducibility, and a count nobody can reproduce is not a count. `BTreeMap` iterates in key order, always, which is why it is the default choice here even though it is asymptotically slower.

Rust makes you notice this. A language whose dictionaries happen to preserve insertion order lets you build the same dependency without ever finding out you have one.

## One allocation for the whole election

```rust
struct Election {
    candidates: Vec<String>,   // position -> who
    cells: Vec<u8>,            // row-major: voter * n + candidate
}
```

This is the shape a real engine converges on. All the ballots live in one contiguous block, which is fast for the same reason a spreadsheet is fast, and the candidate names are held **once**, beside the data, instead of being repeated per ballot or implied by a header somewhere else. The misalignment bug is traded for an index-arithmetic bug — `voter * n + cand` — and the trade is only worth it because that expression is written once, inside one method, where it can be tested. Written at nine call sites it would be worse than the header.

A weighted bloc (`42 × 5,4,3`) is the last shape, and it is a **compression** rather than a different election: the count multiplies the row, it does not join it. Storing 42 as a fourth number in the same array would be the same category error as the header — a value whose meaning depends on which column it landed in.

## If you are coming from another language

- **Python** — you have written all six: the list, the dict, the `namedtuple`/dataclass, the list-of-dicts, and the DataFrame, which *is* the flat matrix with the column names carried alongside. What changes in Rust is that the choice is visible in every signature. `def total(ballots)` accepts all six shapes and discovers the mismatch at runtime; `fn total(ballots: &[Score])` accepts one, and the compiler checks every caller.
- **ABAP** — an internal table of a structure is the named-struct shape, and a `TYPE STANDARD TABLE OF i` is the positional one. The header-drift bug in Step 2 is the classic CSV-into-ITAB defect: field order is a convention held between an upload routine and a `MOVE-CORRESPONDING`, and nothing complains when it slips. Rust's contribution is not the check; it is that the shape is part of the type, so the two ends cannot disagree silently.

## IOUs from this rung

Debts taken deliberately, each one a later lesson:

- Ballots are **hard-coded**, not read from a file — no parsing, no `Result`. → rung 7.
- Candidates are `&str` and `String`, chosen carelessly and cloned freely. → rung 3 (identity) and rung 8 (ownership).
- `winner()` calls `.unwrap()` on `max_by_key`, which panics on an empty election, and says nothing about a tie. → rungs 5 and 6.
- Three candidates everywhere, because `[u8; 3]` is a compile-time size. Const generics (`[Score; N]`) would generalise it.

---

## Practice

**The line you forgot.** Store one ballot two ways — parallel `Vec`s of candidates and scores, and a `Vec` of one-row-per-candidate structs — with a lookup for each. Then remove a candidate from the parallel version and forget the second `remove`.

Run the lookup afterwards. It does not panic and it does not return `None`: it returns another candidate's score, as a plausible number. Then make the same mistake in the row shape and find that there was no second line to forget. Finish by writing down what the row shape costs, because it does cost something.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:representing_a_ballot_kata -->
*[`representing_a_ballot_kata.rs`](examples/representing_a_ballot_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: two ways to store one ballot, and the bug each one allows.
//!
//!   rustc --edition 2024 representing_a_ballot_kata.rs -o /tmp/rbk && /tmp/rbk

/// Shape 1: two parallel Vecs. Nothing ties a name to a score.
struct Parallel {
    candidates: Vec<&'static str>,
    scores: Vec<u8>,
}

impl Parallel {
    fn score_for(&self, who: &str) -> Option<u8> {
        let i = self.candidates.iter().position(|c| *c == who)?;
        self.scores.get(i).copied()
    }
}

/// Shape 2: one row per candidate. The pairing is the type.
#[derive(Debug)]
struct Entry {
    candidate: &'static str,
    score: u8,
}

fn score_for(ballot: &[Entry], who: &str) -> Option<u8> {
    ballot.iter().find(|e| e.candidate == who).map(|e| e.score)
}

fn main() {
    let mut p = Parallel {
        candidates: vec!["Ada", "Ben", "Cara"],
        scores: vec![5, 3, 0],
    };
    let rows = vec![
        Entry { candidate: "Ada", score: 5 },
        Entry { candidate: "Ben", score: 3 },
        Entry { candidate: "Cara", score: 0 },
    ];

    println!("Both hold the same ballot:");
    println!("  parallel: Ben -> {:?}", p.score_for("Ben"));
    println!("  rows:     Ben -> {:?}", score_for(&rows, "Ben"));

    println!("\nNow a candidate withdraws, and one line gets forgotten:");
    p.candidates.remove(1); // and the matching scores.remove(1) never happens
    println!("  parallel: candidates {:?} scores {:?}", p.candidates, p.scores);
    println!("  parallel: Cara -> {:?}   <- WRONG, that is Ben's 3", p.score_for("Cara"));
    println!("      The two Vecs disagree and nothing noticed. This code compiles,");
    println!("      runs, and reports a plausible number.");

    println!("\nThe same mistake in the row shape:");
    let mut rows = rows;
    rows.remove(1);
    println!("  rows: {:?}", rows.iter().map(|e| (e.candidate, e.score)).collect::<Vec<_>>());
    println!("  rows: Cara -> {:?}   <- still right", score_for(&rows, "Cara"));
    println!("      There was no second line to forget. The desync is not a bug");
    println!("      you avoided by being careful; it is a bug you cannot write.");

    println!("\nWhat the row shape costs, honestly:");
    println!("  lookup is a scan, not an index — fine for 5 candidates, wrong for");
    println!("  50,000 ballots in a hot loop, which is where the flat matrix and");
    println!("  the index-newtype shapes start to earn their extra ceremony.");
}
```
<!-- /source -->

<!-- output:representing_a_ballot_kata -->
*Verified output of [`representing_a_ballot_kata.rs`](examples/representing_a_ballot_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Both hold the same ballot:
  parallel: Ben -> Some(3)
  rows:     Ben -> Some(3)

Now a candidate withdraws, and one line gets forgotten:
  parallel: candidates ["Ada", "Cara"] scores [5, 3, 0]
  parallel: Cara -> Some(3)   <- WRONG, that is Ben's 3
      The two Vecs disagree and nothing noticed. This code compiles,
      runs, and reports a plausible number.

The same mistake in the row shape:
  rows: [("Ada", 5), ("Cara", 0)]
  rows: Cara -> Some(0)   <- still right
      There was no second line to forget. The desync is not a bug
      you avoided by being careful; it is a bug you cannot write.

What the row shape costs, honestly:
  lookup is a scan, not an index — fine for 5 candidates, wrong for
  50,000 ballots in a hot loop, which is where the flat matrix and
  the index-newtype shapes start to earn their extra ceremony.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:representing_a_ballot -->
*Verified output of [`representing_a_ballot.rs`](examples/representing_a_ballot.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: One ballot, six shapes
  [u8; 3]                 [5, 3, 0]
  Vec<u8>                 [5, 3, 0]
  Row(Stars, Stars, ..)   Row(Five, Three, Zero)
  NamedBallot { .. }      ada=Five ben=Three
  BTreeMap<&str, Stars>   {"Ada": Five, "Ben": Three, "Cara": Zero}
  Vec<(&str, Stars)>      [("Ada", Five), ("Ben", Three)]  <- sparse: Cara simply absent
    (Row is positional too: .0 = 5, .1 = 3, .2 = 0)
    (every shape above still goes through one door: Stars::new(4) -> Some(Four), Stars::new(6) -> None)

  What each one costs (size_of, the handle only):
    [u8; 3]                      3 bytes  (all of it, no heap)
    [Stars; 3]                   3 bytes
    Row                          3 bytes
    NamedBallot                  3 bytes
    Vec<u8>                     24 bytes  + heap for the data
    BTreeMap<&str, Stars>       24 bytes  + heap, + the keys
      A fixed array IS the data. A Vec is a 24-byte handle (pointer,
      length, capacity) pointing at data somewhere else — which is
      the price of not knowing the candidate count until runtime.

──── Step 2: The bug positional storage cannot see
  same numbers, header Ada,Ben,Cara -> [("Ada", 9), ("Ben", 13), ("Cara", 5)]
                        winner       -> Ben (13 points)
  same numbers, header Ben,Ada,Cara -> [("Ben", 9), ("Ada", 13), ("Cara", 5)]
                        winner       -> Ada (13 points)
      Nothing failed. No parse error, no panic, no warning — just a
      different winner, because a column's meaning lives outside the
      data, in a header the type system never saw.
  NamedBallot: ada=5 ben=3 cara=0
      Here the same mistake is unwriteable: you ask for `.ada`, not
      for column 0. The cost is that the candidates are baked into
      the type at compile time — fine for a lesson, useless for an
      election you load from a file. That tension is the real lesson.

──── Step 3: Blank is not zero
  scored 0   [Some(Five), Some(Three), Some(Zero)] -> total 8, 3 of 3 candidates marked
  left blank [Some(Five), Some(Three), None] -> total 8, 2 of 3 candidates marked
      Both total 8: a blank tabulates as zero. But they are different
      voter intentions, and only the Option remembers which happened.
  size_of::<Stars>()         = 1
  size_of::<Option<Stars>>() = 1  <- remembering costs nothing

──── Step 4: Order-free lookup, and the determinism trap
  inserted Cara, Ada, Ben — iterating a BTreeMap gives:
    Ada   5
    Ben   3
    Cara  0
  ballot.get("Ben") -> Some(3)
  ballot.get("Dan") -> None
      A map drops the column problem entirely: the key IS the meaning.
      This program deliberately does NOT print a HashMap's iteration
      order, because it is randomised per run — the answer key could
      not be recorded. An election that breaks ties by scanning a map
      would inherit exactly that irreproducibility.

──── Step 5: One allocation for the whole election
  cells (row-major, 3 voters x 3 candidates): [5, 3, 0, 4, 5, 1, 0, 5, 4]
  score(voter 2, candidate 1) = 5
    Ada   9
    Ben   13
    Cara  5
      One Vec for the entire election instead of one per ballot: the
      shape a real engine uses. It trades the misalignment bug for an
      index-arithmetic bug — `voter * n + cand`, written once, in one
      method, which is the only reason the trade is worth making.

──── Step 6: Identical ballots, compressed
  75 voters stored as 3 rows
    Ada   231
    Ben   298
    Cara  265
      A weighted row is a COMPRESSION, not a different election: the
      count multiplies, it does not join the scores. Storing 42 as a
      fourth number in the same array would be the same category
      error as the header in Step 2 — a value whose meaning depends
      on which column it landed in.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/representing_a_ballot/examples/representing_a_ballot.rs -o /tmp/rb && /tmp/rb
```

## See also

- [A score is not a number](../newtype_score/README.md) — the `Stars` type this page stores six ways
- [`Option` fields](../option_fields/README.md) — the "required by default" instinct, and when `Option` in a struct is right
- [The long way round](../../ROADMAP.md) — the ladder these rungs climb
