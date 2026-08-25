//! What *is* a ballot, in memory?
//!
//! One voter, three candidates, the scores 5 / 3 / 0. That single fact can be
//! stored at least six ways in Rust, and the choice decides which bugs are
//! possible — not just how much memory it costs.

use std::collections::BTreeMap;

/// From the previous rung: six variants, no seventh expressible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Stars {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Stars {
    fn new(raw: u8) -> Option<Stars> {
        use Stars::*;
        Some(match raw {
            0 => Zero,
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            _ => return None,
        })
    }

    fn value(self) -> u32 {
        self as u32
    }
}

/// Shape 3: positional, but with the arity fixed by the type.
#[derive(Debug, Clone, Copy)]
struct Row(Stars, Stars, Stars);

/// Shape 4: one named field per candidate. Impossible to misalign.
#[derive(Debug, Clone, Copy)]
struct NamedBallot {
    ada: Stars,
    ben: Stars,
    cara: Stars,
}

/// Shape 6: the whole election as one flat matrix, row-major by voter.
struct Election {
    candidates: Vec<String>,
    cells: Vec<u8>,
}

impl Election {
    fn new(candidates: &[&str], rows: &[[u8; 3]]) -> Election {
        Election {
            candidates: candidates.iter().map(|c| c.to_string()).collect(),
            cells: rows.iter().flatten().copied().collect(),
        }
    }

    fn score(&self, voter: usize, cand: usize) -> u8 {
        self.cells[voter * self.candidates.len() + cand]
    }

    fn totals(&self) -> Vec<u32> {
        let n = self.candidates.len();
        (0..n)
            .map(|c| (0..self.cells.len() / n).map(|v| self.score(v, c) as u32).sum())
            .collect()
    }
}

/// Shape 7: identical ballots compressed into a weighted bloc.
struct Bloc {
    count: u32,
    scores: [u8; 3],
}

fn rule(title: &str) {
    println!("\n──── {title}");
}

/// Totals for positional rows, given a header that says which column is who.
fn totals_positional(header: &[&str; 3], rows: &[[u8; 3]]) -> Vec<(String, u32)> {
    (0..3)
        .map(|c| {
            (
                header[c].to_string(),
                rows.iter().map(|r| r[c] as u32).sum::<u32>(),
            )
        })
        .collect()
}

fn winner(totals: &[(String, u32)]) -> String {
    let best = totals.iter().max_by_key(|(_, t)| *t).unwrap();
    format!("{} ({} points)", best.0, best.1)
}

fn main() {
    rule("Step 1: One ballot, six shapes");

    let as_array: [u8; 3] = [5, 3, 0];
    let as_vec: Vec<u8> = vec![5, 3, 0];
    let as_row = Row(Stars::Five, Stars::Three, Stars::Zero);
    let as_named = NamedBallot {
        ada: Stars::Five,
        ben: Stars::Three,
        cara: Stars::Zero,
    };
    let as_map: BTreeMap<&str, Stars> = BTreeMap::from([
        ("Ada", Stars::Five),
        ("Ben", Stars::Three),
        ("Cara", Stars::Zero),
    ]);
    let as_pairs: Vec<(&str, Stars)> = vec![("Ada", Stars::Five), ("Ben", Stars::Three)];

    println!("  [u8; 3]                 {as_array:?}");
    println!("  Vec<u8>                 {as_vec:?}");
    println!("  Row(Stars, Stars, ..)   {as_row:?}");
    println!("  NamedBallot {{ .. }}      ada={:?} ben={:?}", as_named.ada, as_named.ben);
    println!("  BTreeMap<&str, Stars>   {as_map:?}");
    println!("  Vec<(&str, Stars)>      {as_pairs:?}  <- sparse: Cara simply absent");
    println!(
        "    (Row is positional too: .0 = {}, .1 = {}, .2 = {})",
        as_row.0.value(),
        as_row.1.value(),
        as_row.2.value()
    );
    println!(
        "    (every shape above still goes through one door: Stars::new(4) -> {:?}, Stars::new(6) -> {:?})",
        Stars::new(4),
        Stars::new(6)
    );

    println!("\n  What each one costs (size_of, the handle only):");
    println!("    [u8; 3]                    {:>3} bytes  (all of it, no heap)", size_of::<[u8; 3]>());
    println!("    [Stars; 3]                 {:>3} bytes", size_of::<[Stars; 3]>());
    println!("    Row                        {:>3} bytes", size_of::<Row>());
    println!("    NamedBallot                {:>3} bytes", size_of::<NamedBallot>());
    println!("    Vec<u8>                    {:>3} bytes  + heap for the data", size_of::<Vec<u8>>());
    println!("    BTreeMap<&str, Stars>      {:>3} bytes  + heap, + the keys", size_of::<BTreeMap<&str, Stars>>());
    println!("      A fixed array IS the data. A Vec is a 24-byte handle (pointer,");
    println!("      length, capacity) pointing at data somewhere else — which is");
    println!("      the price of not knowing the candidate count until runtime.");

    rule("Step 2: The bug positional storage cannot see");

    let rows: [[u8; 3]; 3] = [[5, 3, 0], [4, 5, 1], [0, 5, 4]];
    let header_a = ["Ada", "Ben", "Cara"];
    let header_b = ["Ben", "Ada", "Cara"]; // the first two columns swapped

    let ta = totals_positional(&header_a, &rows);
    let tb = totals_positional(&header_b, &rows);
    println!("  same numbers, header Ada,Ben,Cara -> {ta:?}");
    println!("                        winner       -> {}", winner(&ta));
    println!("  same numbers, header Ben,Ada,Cara -> {tb:?}");
    println!("                        winner       -> {}", winner(&tb));
    println!("      Nothing failed. No parse error, no panic, no warning — just a");
    println!("      different winner, because a column's meaning lives outside the");
    println!("      data, in a header the type system never saw.");

    let named = NamedBallot { ada: Stars::Five, ben: Stars::Three, cara: Stars::Zero };
    println!("  NamedBallot: ada={} ben={} cara={}", named.ada.value(), named.ben.value(), named.cara.value());
    println!("      Here the same mistake is unwriteable: you ask for `.ada`, not");
    println!("      for column 0. The cost is that the candidates are baked into");
    println!("      the type at compile time — fine for a lesson, useless for an");
    println!("      election you load from a file. That tension is the real lesson.");

    rule("Step 3: Blank is not zero");

    let scored_zero: Vec<Option<Stars>> = vec![Some(Stars::Five), Some(Stars::Three), Some(Stars::Zero)];
    let left_blank: Vec<Option<Stars>> = vec![Some(Stars::Five), Some(Stars::Three), None];

    for (label, b) in [("scored 0", &scored_zero), ("left blank", &left_blank)] {
        let total: u32 = b.iter().flatten().map(|s| s.value()).sum();
        let marked = b.iter().flatten().count();
        println!("  {label:<10} {b:?} -> total {total}, {marked} of 3 candidates marked");
    }
    println!("      Both total 8: a blank tabulates as zero. But they are different");
    println!("      voter intentions, and only the Option remembers which happened.");
    println!("  size_of::<Stars>()         = {}", size_of::<Stars>());
    println!("  size_of::<Option<Stars>>() = {}  <- remembering costs nothing", size_of::<Option<Stars>>());

    rule("Step 4: Order-free lookup, and the determinism trap");

    let ballot: BTreeMap<&str, Stars> = BTreeMap::from([
        ("Cara", Stars::Zero),
        ("Ada", Stars::Five),
        ("Ben", Stars::Three),
    ]);
    println!("  inserted Cara, Ada, Ben — iterating a BTreeMap gives:");
    for (name, s) in &ballot {
        println!("    {name:<5} {}", s.value());
    }
    println!("  ballot.get(\"Ben\") -> {:?}", ballot.get("Ben").map(|s| s.value()));
    println!("  ballot.get(\"Dan\") -> {:?}", ballot.get("Dan"));
    println!("      A map drops the column problem entirely: the key IS the meaning.");
    println!("      This program deliberately does NOT print a HashMap's iteration");
    println!("      order, because it is randomised per run — the answer key could");
    println!("      not be recorded. An election that breaks ties by scanning a map");
    println!("      would inherit exactly that irreproducibility.");

    rule("Step 5: One allocation for the whole election");

    let election = Election::new(&["Ada", "Ben", "Cara"], &rows);
    println!("  cells (row-major, 3 voters x 3 candidates): {:?}", election.cells);
    println!("  score(voter 2, candidate 1) = {}", election.score(2, 1));
    for (name, total) in election.candidates.iter().zip(election.totals()) {
        println!("    {name:<5} {total}");
    }
    println!("      One Vec for the entire election instead of one per ballot: the");
    println!("      shape a real engine uses. It trades the misalignment bug for an");
    println!("      index-arithmetic bug — `voter * n + cand`, written once, in one");
    println!("      method, which is the only reason the trade is worth making.");

    rule("Step 6: Identical ballots, compressed");

    let blocs = [
        Bloc { count: 42, scores: [5, 4, 3] },
        Bloc { count: 26, scores: [0, 5, 4] },
        Bloc { count: 7, scores: [3, 0, 5] },
    ];
    let voters: u32 = blocs.iter().map(|b| b.count).sum();
    let totals: Vec<u32> = (0..3)
        .map(|c| blocs.iter().map(|b| b.count * b.scores[c] as u32).sum())
        .collect();
    println!("  {voters} voters stored as {} rows", blocs.len());
    for (name, total) in ["Ada", "Ben", "Cara"].iter().zip(&totals) {
        println!("    {name:<5} {total}");
    }
    println!("      A weighted row is a COMPRESSION, not a different election: the");
    println!("      count multiplies, it does not join the scores. Storing 42 as a");
    println!("      fourth number in the same array would be the same category");
    println!("      error as the header in Step 2 — a value whose meaning depends");
    println!("      on which column it landed in.");
}
