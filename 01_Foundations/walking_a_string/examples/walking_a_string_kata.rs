//! Kata solution: an empty field is data, and split_whitespace eats it.
//!
//!   rustc --edition 2024 walking_a_string_kata.rs -o /tmp/wsk && /tmp/wsk

/// A ballot row: three scores, and a missing score is a real, different thing.
fn parse_row(line: &str) -> Vec<Option<u8>> {
    line.split(',')
        .map(|f| f.trim())
        .map(|f| if f.is_empty() { None } else { f.parse().ok() })
        .collect()
}

fn main() {
    let rows = ["5,2,0", "5, ,0", "5,,0", ",,", "5, 2 , 0 "];

    println!("split(',') keeps every field, including the empty ones:");
    for r in rows {
        println!("   {:<10} -> {:?}", r, r.split(',').collect::<Vec<_>>());
    }

    println!("\nsplit_whitespace() does not — it cannot even see the commas:");
    for r in rows {
        println!("   {:<10} -> {:?}", r, r.split_whitespace().collect::<Vec<_>>());
    }

    println!("\nAnd on whitespace-separated data it still drops the gap:");
    let spaced = ["5 2 0", "5  0", "5 2 "];
    for r in spaced {
        println!("   {:<8} split(' ')          {:?}", r, r.split(' ').collect::<Vec<_>>());
        println!("   {:<8} split_whitespace()  {:?}", "", r.split_whitespace().collect::<Vec<_>>());
    }

    println!("\nParsed as ballots — None is an abstention, not a zero:");
    for r in rows {
        let parsed = parse_row(r);
        let counted = parsed.iter().filter(|s| s.is_some()).count();
        println!("   {:<10} -> {:?}   ({counted} of {} scored)", r, parsed, parsed.len());
    }

    println!("\nThe rule:");
    println!("   split(sep)          n separators -> n+1 fields, empties included.");
    println!("                       Use it when position matters: CSV, fixed columns.");
    println!("   split_whitespace()  runs of whitespace collapse, ends are trimmed.");
    println!("                       Use it for prose and hand-typed input.");
    println!("   Reaching for split_whitespace() on delimited data silently shortens");
    println!("   the row, so column 3 becomes column 2 and nothing errors.");
}
