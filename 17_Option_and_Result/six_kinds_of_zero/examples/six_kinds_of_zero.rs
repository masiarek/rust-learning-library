//! Six kinds of zero — where `Option` runs out and you write the enum yourself.
//!
//! A survey form is collected on paper as well as online, and the importer
//! accepts five marker characters alongside the digits 0-5. Every marker sums
//! as zero. They are not the same thing.
//!
//!   rustc --edition 2024 six_kinds_of_zero.rs -o /tmp/skz && /tmp/skz

/// A rating somebody actually wrote — six variants, and no seventh to write.
/// Same type as the sibling `newtype_score` lesson, and an enum for the same
/// reason: it leaves 250 unused bit patterns, which the compiler spends below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stars {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Stars {
    fn new(n: u8) -> Option<Stars> {
        use Stars::*;
        Some(match n {
            0 => Zero,
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            _ => return None,
        })
    }
    fn value(self) -> u8 {
        self as u8
    }
}

/// One cell of one form: either a rating, or one of five reasons there isn't one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mark {
    Rated(Stars),  // '0'..='5'
    Blank,         // '-'  the box was left empty
    SectionSkipped,// '~'  the respondent skipped this whole section
    Declined,      // '&'  the respondent declined to answer this item
    Unreadable,    // '?'  smudged, or two bubbles filled in one row
    Corrected,     // '%'  unreadable, and a corrected copy was supplied
}

impl TryFrom<char> for Mark {
    type Error = char;
    fn try_from(c: char) -> Result<Mark, char> {
        Ok(match c {
            '0'..='5' => Mark::Rated(Stars::new(c as u8 - b'0').expect("0-5")),
            '-' => Mark::Blank,
            '~' => Mark::SectionSkipped,
            '&' => Mark::Declined,
            '?' => Mark::Unreadable,
            '%' => Mark::Corrected,
            other => return Err(other),
        })
    }
}

impl Mark {
    /// What the total adds. Five of the six collapse to zero — deliberately.
    fn sum_value(self) -> u8 {
        match self {
            Mark::Rated(s) => s.value(),
            Mark::Blank
            | Mark::SectionSkipped
            | Mark::Declined
            | Mark::Unreadable
            | Mark::Corrected => 0,
        }
    }

    /// The audit label. Note there is no `_` arm anywhere in this file.
    fn label(self) -> &'static str {
        match self {
            Mark::Rated(_) => "rated",
            Mark::Blank => "blank",
            Mark::SectionSkipped => "section skipped",
            Mark::Declined => "declined",
            Mark::Unreadable => "unreadable",
            Mark::Corrected => "unreadable + corrected",
        }
    }

    /// Only paper can produce these. A web form prevents them at input.
    fn paper_only(self) -> bool {
        match self {
            Mark::Rated(_) | Mark::Blank => false,
            Mark::SectionSkipped | Mark::Declined | Mark::Unreadable | Mark::Corrected => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Channel {
    Paper,
    Online,
}

fn parse_row(row: &str) -> Vec<Mark> {
    row.chars()
        .filter(|c| *c != ',')
        .map(|c| Mark::try_from(c).expect("known marker"))
        .collect()
}

fn banner(s: &str) {
    println!("\n──── {s}");
}

fn main() {
    banner("Step 1: The six kinds of zero");
    let vocabulary = ['0', '-', '~', '&', '?', '%'];
    println!("  char   variant                          sums as");
    for c in vocabulary {
        let m = Mark::try_from(c).unwrap();
        println!("   {c}     {:<32} {}", m.label(), m.sum_value());
    }
    println!("      Every row adds zero to the total. Six different things");
    println!("      happened, and a report that says only \"0\" has lost five of them.");

    banner("Step 2: Option can tell two of them apart, and that is all");
    let as_option: Vec<Option<Stars>> = vec![Stars::new(0), None, None, None, None, None];
    println!("  Option<Stars>  {as_option:?}");
    println!("      Some(Stars(0)) is the person who wrote a zero. The five Nones");
    println!("      are five different people, and Option has one shape for all");
    println!("      of them. `Option` is not \"the maybe type\" — it is a sum type");
    println!("      with exactly two variants, and you have six cases.");

    banner("Step 3: Two tempting wrong turns");
    let sentinel: Vec<u8> = vec![0, 255, 254, 253, 252, 251];
    println!("  sentinel u8    {sentinel:?}");
    println!("      Works until somebody sums the column. 255 is a NUMBER; the");
    println!("      compiler will happily add it to a total, and 0..=5 no longer");
    println!("      describes the type. You have widened the thing you narrowed.");
    let ratings = vec![0u8, 0, 0, 0, 0, 0];
    let is_marker = vec![false, true, true, true, true, true];
    println!("  parallel flags ratings {ratings:?} is_marker {is_marker:?}");
    println!("      Two Vecs that must be edited together — the desync bug from");
    println!("      the record lesson, reintroduced. And it still cannot say WHICH");
    println!("      marker, so a third parallel Vec is coming.");

    banner("Step 4: The enum, and a projection that is lossy on purpose");
    let row = parse_row("5,-,3,~,?,%");
    println!("  row  5,-,3,~,?,%");
    for (i, m) in row.iter().enumerate() {
        println!(
            "    cell {i}  {:<24} -> sums {}   paper-only: {}",
            m.label(),
            m.sum_value(),
            m.paper_only()
        );
    }
    let total: u32 = row.iter().map(|m| m.sum_value() as u32).sum();
    println!("  total = {total}");
    println!("      sum_value() throws five distinctions away, and that is");
    println!("      correct — the arithmetic genuinely does not care. The type");
    println!("      keeps what the projection discards, so the audit still can.");

    banner("Step 5: The report the type makes possible");
    let forms = [
        (Channel::Paper, "5,-,3,~"),
        (Channel::Paper, "4,?,5,%"),
        (Channel::Paper, "0,5,&,-"),
        (Channel::Online, "5,0,3,0"),
        (Channel::Online, "4,-,5,-"),
        (Channel::Online, "0,5,0,-"),
    ];
    for channel in [Channel::Paper, Channel::Online] {
        let marks: Vec<Mark> = forms
            .iter()
            .filter(|(c, _)| *c == channel)
            .flat_map(|(_, row)| parse_row(row))
            .collect();
        let non_ratings = marks.iter().filter(|m| !matches!(m, Mark::Rated(_))).count();
        let paper_only = marks.iter().filter(|m| m.paper_only()).count();
        println!(
            "  {channel:?}: {} cells, {non_ratings} non-ratings, {paper_only} of them paper-only",
            marks.len()
        );
    }
    println!("      Pooled, that reads \"paper respondents are sloppier.\" Split, it reads");
    println!("      \"only paper respondents CAN produce four of the six marks.\" Same");
    println!("      numbers, opposite conclusion — which is why the channel has to");
    println!("      be in the type too, not in a comment.");

    banner("Step 6: What it costs");
    println!("  size_of::<Stars>()          = {}", size_of::<Stars>());
    println!("  size_of::<Mark>()           = {}", size_of::<Mark>());
    println!("  size_of::<Option<Mark>>()   = {}", size_of::<Option<Mark>>());
    println!("      Six rating values plus five markers is eleven states, and a byte");
    println!("      holds 256. Option<Mark> is still one byte, because the compiler");
    println!("      parks None in a bit pattern Mark never uses. Remembering all six");
    println!("      distinctions costs exactly nothing over storing the digit.");
}
