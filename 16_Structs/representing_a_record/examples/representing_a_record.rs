//! What *is* a record, in memory?
//!
//! One survey response, three questions, the ratings 5 / 3 / 0. That single
//! fact can be stored at least six ways in Rust, and the choice decides which
//! bugs are possible — not just how much memory it costs.

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

/// Shape 4: one named field per question. Impossible to misalign.
#[derive(Debug, Clone, Copy)]
struct NamedRow {
    speed: Stars,
    price: Stars,
    support: Stars,
}

/// Shape 6: the whole survey as one flat matrix, row-major by response.
struct Survey {
    questions: Vec<String>,
    cells: Vec<u8>,
}

impl Survey {
    fn new(questions: &[&str], rows: &[[u8; 3]]) -> Survey {
        Survey {
            questions: questions.iter().map(|q| q.to_string()).collect(),
            cells: rows.iter().flatten().copied().collect(),
        }
    }

    fn rating(&self, response: usize, question: usize) -> u8 {
        self.cells[response * self.questions.len() + question]
    }

    fn totals(&self) -> Vec<u32> {
        let n = self.questions.len();
        (0..n)
            .map(|q| (0..self.cells.len() / n).map(|r| self.rating(r, q) as u32).sum())
            .collect()
    }
}

/// Shape 7: identical responses compressed into one weighted row.
struct Group {
    count: u32,
    ratings: [u8; 3],
}

fn rule(title: &str) {
    println!("\n──── {title}");
}

/// Totals for positional rows, given a header that says which column is which.
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

fn top(totals: &[(String, u32)]) -> String {
    let best = totals.iter().max_by_key(|(_, t)| *t).unwrap();
    format!("{} ({} points)", best.0, best.1)
}

fn main() {
    rule("Step 1: One record, six shapes");

    let as_array: [u8; 3] = [5, 3, 0];
    let as_vec: Vec<u8> = vec![5, 3, 0];
    let as_row = Row(Stars::Five, Stars::Three, Stars::Zero);
    let as_named = NamedRow {
        speed: Stars::Five,
        price: Stars::Three,
        support: Stars::Zero,
    };
    let as_map: BTreeMap<&str, Stars> = BTreeMap::from([
        ("speed", Stars::Five),
        ("price", Stars::Three),
        ("support", Stars::Zero),
    ]);
    let as_pairs: Vec<(&str, Stars)> = vec![("speed", Stars::Five), ("price", Stars::Three)];

    println!("  [u8; 3]                 {as_array:?}");
    println!("  Vec<u8>                 {as_vec:?}");
    println!("  Row(Stars, Stars, ..)   {as_row:?}");
    println!("  NamedRow {{ .. }}         speed={:?} price={:?}", as_named.speed, as_named.price);
    println!("  BTreeMap<&str, Stars>   {as_map:?}");
    println!("  Vec<(&str, Stars)>      {as_pairs:?}  <- sparse: support simply absent");
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
    println!("    NamedRow                   {:>3} bytes", size_of::<NamedRow>());
    println!("    Vec<u8>                    {:>3} bytes  + heap for the data", size_of::<Vec<u8>>());
    println!("    BTreeMap<&str, Stars>      {:>3} bytes  + heap, + the keys", size_of::<BTreeMap<&str, Stars>>());
    println!("      A fixed array IS the data. A Vec is a 24-byte handle (pointer,");
    println!("      length, capacity) pointing at data somewhere else — which is");
    println!("      the price of not knowing the column count until runtime.");

    rule("Step 2: The bug positional storage cannot see");

    let rows: [[u8; 3]; 3] = [[5, 3, 0], [4, 5, 1], [0, 5, 4]];
    let header_a = ["speed", "price", "support"];
    let header_b = ["price", "speed", "support"]; // the first two columns swapped

    let ta = totals_positional(&header_a, &rows);
    let tb = totals_positional(&header_b, &rows);
    println!("  same numbers, header speed,price,support -> {ta:?}");
    println!("                        top             -> {}", top(&ta));
    println!("  same numbers, header price,speed,support -> {tb:?}");
    println!("                        top             -> {}", top(&tb));
    println!("      Nothing failed. No parse error, no panic, no warning — just a");
    println!("      different answer, because a column's meaning lives outside the");
    println!("      data, in a header the type system never saw.");

    let named = NamedRow { speed: Stars::Five, price: Stars::Three, support: Stars::Zero };
    println!("  NamedRow: speed={} price={} support={}", named.speed.value(), named.price.value(), named.support.value());
    println!("      Here the same mistake is unwriteable: you ask for `.speed`, not");
    println!("      for column 0. The cost is that the questions are baked into");
    println!("      the type at compile time — fine for a lesson, useless for a");
    println!("      form you load from a file. That tension is the real lesson.");

    rule("Step 3: Blank is not zero");

    let scored_zero: Vec<Option<Stars>> = vec![Some(Stars::Five), Some(Stars::Three), Some(Stars::Zero)];
    let left_blank: Vec<Option<Stars>> = vec![Some(Stars::Five), Some(Stars::Three), None];

    for (label, b) in [("scored 0", &scored_zero), ("left blank", &left_blank)] {
        let total: u32 = b.iter().flatten().map(|s| s.value()).sum();
        let marked = b.iter().flatten().count();
        println!("  {label:<10} {b:?} -> total {total}, {marked} of 3 questions answered");
    }
    println!("      Both total 8: a blank sums as zero. But they are different");
    println!("      answers, and only the Option remembers which one happened.");
    println!("  size_of::<Stars>()         = {}", size_of::<Stars>());
    println!("  size_of::<Option<Stars>>() = {}  <- remembering costs nothing", size_of::<Option<Stars>>());

    rule("Step 4: Order-free lookup, and the determinism trap");

    let record: BTreeMap<&str, Stars> = BTreeMap::from([
        ("support", Stars::Zero),
        ("speed", Stars::Five),
        ("price", Stars::Three),
    ]);
    println!("  inserted support, speed, price — iterating a BTreeMap gives:");
    for (name, s) in &record {
        println!("    {name:<8} {}", s.value());
    }
    println!("  record.get(\"price\")  -> {:?}", record.get("price").map(|s| s.value()));
    println!("  record.get(\"colour\") -> {:?}", record.get("colour"));
    println!("      A map drops the column problem entirely: the key IS the meaning.");
    println!("      This program deliberately does NOT print a HashMap's iteration");
    println!("      order, because it is randomised per run — the answer key could");
    println!("      not be recorded. A report that breaks ties by scanning a map");
    println!("      would inherit exactly that irreproducibility.");

    rule("Step 5: One allocation for the whole survey");

    let survey = Survey::new(&["speed", "price", "support"], &rows);
    println!("  cells (row-major, 3 responses x 3 questions): {:?}", survey.cells);
    println!("  rating(response 2, question 1) = {}", survey.rating(2, 1));
    for (name, total) in survey.questions.iter().zip(survey.totals()) {
        println!("    {name:<8} {total}");
    }
    println!("      One Vec for the whole survey instead of one per response: the");
    println!("      shape a real data frame uses. It trades the misalignment bug for");
    println!("      an index-arithmetic bug — `row * n + col`, written once, in one");
    println!("      method, which is the only reason the trade is worth making.");

    rule("Step 6: Identical responses, compressed");

    let groups = [
        Group { count: 42, ratings: [5, 4, 3] },
        Group { count: 26, ratings: [0, 5, 4] },
        Group { count: 7, ratings: [3, 0, 5] },
    ];
    let responses: u32 = groups.iter().map(|g| g.count).sum();
    let totals: Vec<u32> = (0..3)
        .map(|q| groups.iter().map(|g| g.count * g.ratings[q] as u32).sum())
        .collect();
    println!("  {responses} responses stored as {} rows", groups.len());
    for (name, total) in ["speed", "price", "support"].iter().zip(&totals) {
        println!("    {name:<8} {total}");
    }
    println!("      A weighted row is a COMPRESSION, not a different survey: the");
    println!("      count multiplies, it does not join the ratings. Storing 42 as a");
    println!("      fourth number in the same array would be the same category");
    println!("      error as the header in Step 2 — a value whose meaning depends");
    println!("      on which column it landed in.");
}
