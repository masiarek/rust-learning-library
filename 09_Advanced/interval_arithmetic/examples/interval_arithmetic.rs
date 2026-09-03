//! Interval arithmetic: not "what is the exact answer", but "did my rounding decide it".
//!
//!   rustc --edition 2024 interval_arithmetic.rs -o /tmp/ia && /tmp/ia

use std::cmp::Ordering;

fn rule(title: &str) {
    println!("\n=== {title} ===");
}

// ------------------------------------------------------------ exact rationals
// Only here to be the answer key. The whole point of the page is the verdict
// you can reach WITHOUT one.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Ratio {
    n: i128,
    d: i128,
}

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.max(1)
}

impl Ratio {
    fn new(n: i128, d: i128) -> Self {
        let g = gcd(n, d);
        Ratio { n: n / g, d: d / g }
    }
    fn add(self, o: Ratio) -> Self {
        Ratio::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }
    fn scale(self, k: i128) -> Self {
        Ratio::new(self.n * k, self.d)
    }
    fn cmp(self, o: Ratio) -> Ordering {
        (self.n * o.d).cmp(&(o.n * self.d))
    }
    fn text(self) -> String {
        if self.d == 1 { format!("{}", self.n) } else { format!("{}/{}", self.n, self.d) }
    }
}

// ------------------------------------------------------- one project, three ways
/// Round to nearest, half up. What a fixed-point count actually stores.
fn rounded(v: Ratio, scale: i128) -> i128 {
    (v.n * scale * 2 + v.d) / (v.d * 2)
}

/// The bracket the exact value is GUARANTEED to lie in, in units of 1/scale.
/// floor <= exact*scale <= ceil, so summing the ends brackets the sum.
fn bracket(v: Ratio, scale: i128) -> (i128, i128) {
    let num = v.n * scale;
    let lo = num.div_euclid(v.d);
    let hi = if num.rem_euclid(v.d) == 0 { lo } else { lo + 1 };
    (lo, hi)
}

#[derive(Clone, Copy)]
struct Total {
    exact: Ratio,
    lo: i128,
    hi: i128,
    naive: i128,
}

/// One project's column, counted at `scale`, exactly and in bounds, in one pass.
fn tally(scores: &[i128], weights: &[Ratio], scale: i128) -> Total {
    let mut t = Total { exact: Ratio::new(0, 1), lo: 0, hi: 0, naive: 0 };
    for (&s, &w) in scores.iter().zip(weights) {
        let term = w.scale(s);
        t.exact = t.exact.add(term);
        let (lo, hi) = bracket(term, scale);
        t.lo += lo;
        t.hi += hi;
        t.naive += rounded(term, scale);
    }
    t
}

#[derive(PartialEq, Debug)]
enum Verdict {
    Decided(&'static str),
    Undecided,
}

/// The only claim interval arithmetic makes. Note what it CANNOT return: a wrong leader.
fn verdict(a: &Total, b: &Total, an: &'static str, bn: &'static str) -> Verdict {
    if a.lo > b.hi {
        Verdict::Decided(an)
    } else if b.lo > a.hi {
        Verdict::Decided(bn)
    } else {
        Verdict::Undecided
    }
}

fn naive_leader(a: &Total, b: &Total, an: &'static str, bn: &'static str) -> &'static str {
    match a.naive.cmp(&b.naive) {
        Ordering::Greater => an,
        Ordering::Less => bn,
        Ordering::Equal => "tie",
    }
}

fn exact_leader(a: &Total, b: &Total, an: &'static str, bn: &'static str) -> &'static str {
    match a.exact.cmp(b.exact) {
        Ordering::Greater => an,
        Ordering::Less => bn,
        Ordering::Equal => "tie",
    }
}

const SCALES: [i128; 5] = [10, 100, 1_000, 10_000, 100_000];

fn report(
    title: &str,
    weights: &[Ratio],
    (an, a_scores): (&'static str, &[i128]),
    (bn, b_scores): (&'static str, &[i128]),
) {
    let a0 = tally(a_scores, weights, 1);
    let b0 = tally(b_scores, weights, 1);
    println!("\n  {title}");
    println!("    exact: {an} {}  vs  {bn} {}   -> {}",
             a0.exact.text(), b0.exact.text(), exact_leader(&a0, &b0, an, bn));
    println!("    {:>8}  {:>22}  {:>26}", "scale", "rounded count says", "intervals say");
    for scale in SCALES {
        let a = tally(a_scores, weights, scale);
        let b = tally(b_scores, weights, scale);
        let v = match verdict(&a, &b, an, bn) {
            Verdict::Decided(w) => format!("DECIDED: {w}"),
            Verdict::Undecided => "undecided".to_string(),
        };
        println!("    {:>8}  {:>7} {:>7}-{:<7} [{:>6},{:>6}] [{:>6},{:>6}]  {}",
                 scale, naive_leader(&a, &b, an, bn), a.naive, b.naive, a.lo, a.hi, b.lo, b.hi, v);
    }
}

fn main() {
    // The cluster's canonical round-2 dataset: after round 1 went to Wren, a
    // reviewer weighs 5/(5 + what it gave Wren).
    let weights: Vec<Ratio> = [5, 4, 3, 2, 1, 0].iter().map(|&s| Ratio::new(5, 5 + s)).collect();

    rule("the count is the same; only the question changes");
    println!("  six reviewers, weights 5/10 5/9 5/8 5/7 5/6 5/5");
    println!("  a rounded count answers 'who won'. it always answers.");
    println!("  intervals answer 'is that answer mine, or the rounding's'.");

    rule("dataset 1 -- the one that is exactly tied");
    report("Alpha vs Bravo (the canonical round 2)", &weights,
           ("Alpha", &[0, 0, 1, 1, 1, 5]), ("Bravo", &[0, 3, 1, 1, 5, 0]));
    println!("\n    The exact answer is a TIE, so no scale can honestly name a leader.");
    println!("    The rounded count names Bravo anyway -- at every scale, and it does");
    println!("    not waver as the scale refines. That is the part worth sitting with:");
    println!("    this is not noise that averages out, it is the half-up rule leaning");
    println!("    the same way every time. Refining the scale buys more digits of a");
    println!("    wrong answer. The intervals overlap at every scale, and so decline.");

    rule("dataset 2 -- a real margin, and a small one");
    report("Petra vs Quinn (margin 1/504)", &weights,
           ("Petra", &[1, 3, 4, 0, 1, 2]), ("Quinn", &[0, 2, 1, 2, 4, 1]));
    println!("\n    Here there IS a leader, and the intervals find it -- but only once");
    println!("    the scale is fine enough to separate 1/504 from the slack.");

    rule("the coarsest scale that PROVES it");
    let pa: &[i128] = &[1, 3, 4, 0, 1, 2];
    let qa: &[i128] = &[0, 2, 1, 2, 4, 1];
    let mut first_decided = None;
    let mut naive_wrong_below = 0;
    let mut undecided_above = 0;
    let mut highest_undecided = 0;
    for scale in 1..=20_000i128 {
        let a = tally(pa, &weights, scale);
        let b = tally(qa, &weights, scale);
        let decided = verdict(&a, &b, "Petra", "Quinn") != Verdict::Undecided;
        if decided && first_decided.is_none() {
            first_decided = Some(scale);
        }
        match first_decided {
            None => {
                if naive_leader(&a, &b, "Petra", "Quinn") != exact_leader(&a, &b, "Petra", "Quinn") {
                    naive_wrong_below += 1;
                }
            }
            Some(_) if !decided => {
                undecided_above += 1;
                highest_undecided = scale;
            }
            _ => {}
        }
    }
    let first = first_decided.unwrap();
    println!("  intervals first PROVE it at scale : {first}");
    println!("  below that, the rounded count got the answer wrong at {naive_wrong_below}");
    println!("  of the {} coarser scales -- right most of the time, which is", first - 1);
    println!("  exactly what makes it untrustworthy: you cannot tell which run");
    println!("  you are looking at without the answer you were trying to compute.");
    println!();
    println!("  And refining is NOT monotone: {undecided_above} scales above {first} still");
    println!("  come back undecided, the highest at {highest_undecided}. Each term's bracket");
    println!("  depends on whether that scale divides that denominator, so a finer");
    println!("  scale can widen the total slack. Search for a scale that decides;");
    println!("  do not assume the next power of ten will.");

    rule("the property that makes it usable: the error is one-sided");
    let mut decided = 0;
    let mut agreed = 0;
    let mut conservative = 0;
    for scale in 1..=20_000i128 {
        for (an, asc, bn, bsc) in [
            ("Alpha", &[0, 0, 1, 1, 1, 5][..], "Bravo", &[0, 3, 1, 1, 5, 0][..]),
            ("Petra", &[1, 3, 4, 0, 1, 2][..], "Quinn", &[0, 2, 1, 2, 4, 1][..]),
        ] {
            let a = tally(asc, &weights, scale);
            let b = tally(bsc, &weights, scale);
            match verdict(&a, &b, an, bn) {
                Verdict::Decided(w) => {
                    decided += 1;
                    if w == exact_leader(&a, &b, an, bn) {
                        agreed += 1;
                    }
                }
                Verdict::Undecided => {
                    if exact_leader(&a, &b, an, bn) != "tie" {
                        conservative += 1;
                    }
                }
            }
        }
    }
    println!("  40,000 (dataset, scale) pairs checked against the exact answer");
    println!("    said DECIDED            : {decided}");
    println!("    ...and was right        : {agreed}   <- every time. never a wrong leader.");
    println!("    said undecided anyway   : {conservative}   <- exact answer existed; bounds too loose");
    println!("  SOUND but not COMPLETE: 'decided' is a proof, 'undecided' is an admission");
    println!("  that this scale cannot tell you -- never a claim that nobody can.");
}
