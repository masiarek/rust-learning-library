//! Kata solution: the audit that has to know when to stop.
//!
//! An audit certifies a rounded count when the intervals prove it, and escalates
//! when they do not. The obvious escalation is "refine the scale and try again",
//! and on one of these five races that loop never returns -- because no scale can
//! separate an exact tie, and the loop has no way to say so.
//!
//!   rustc --edition 2024 interval_arithmetic_kata.rs -o /tmp/iak && /tmp/iak

use std::cmp::Ordering;

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
    fn sub(self, o: Ratio) -> Self {
        Ratio::new(self.n * o.d - o.n * self.d, self.d * o.d)
    }
    fn scale(self, k: i128) -> Self {
        Ratio::new(self.n * k, self.d)
    }
    fn cmp(self, o: Ratio) -> Ordering {
        (self.n * o.d).cmp(&(o.n * self.d))
    }
    fn text(self) -> String {
        if self.n == 0 { "0".to_string() }
        else if self.d == 1 { format!("{}", self.n) }
        else { format!("{}/{}", self.n, self.d) }
    }
}

fn bracket(v: Ratio, scale: i128) -> (i128, i128) {
    let num = v.n * scale;
    let lo = num.div_euclid(v.d);
    let hi = if num.rem_euclid(v.d) == 0 { lo } else { lo + 1 };
    (lo, hi)
}

fn bounds(scores: &[i128], weights: &[Ratio], scale: i128) -> (i128, i128) {
    let (mut lo, mut hi) = (0, 0);
    for (&s, &w) in scores.iter().zip(weights) {
        let (l, h) = bracket(w.scale(s), scale);
        lo += l;
        hi += h;
    }
    (lo, hi)
}

fn exact(scores: &[i128], weights: &[Ratio]) -> Ratio {
    scores.iter().zip(weights).fold(Ratio::new(0, 1), |acc, (&s, &w)| acc.add(w.scale(s)))
}

/// What an audit is allowed to conclude. Three outcomes, not two -- the third is
/// the whole fix, and the reason the naive loop cannot be repaired by looping harder.
#[derive(Debug, PartialEq)]
enum Audit {
    Certified(&'static str, i128), // winner, and the scale that proved it
    HandCount,                     // no scale in budget separated them
}

struct Race {
    name: &'static str,
    a: (&'static str, &'static [i128]),
    b: (&'static str, &'static [i128]),
}

const BUDGET: [i128; 8] = [10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000];

fn audit(r: &Race, weights: &[Ratio]) -> Audit {
    for scale in BUDGET {
        let (alo, ahi) = bounds(r.a.1, weights, scale);
        let (blo, bhi) = bounds(r.b.1, weights, scale);
        if alo > bhi {
            return Audit::Certified(r.a.0, scale);
        }
        if blo > ahi {
            return Audit::Certified(r.b.0, scale);
        }
    }
    Audit::HandCount
}

fn exact_winner(r: &Race, weights: &[Ratio]) -> &'static str {
    match exact(r.a.1, weights).cmp(exact(r.b.1, weights)) {
        Ordering::Greater => r.a.0,
        Ordering::Less => r.b.0,
        Ordering::Equal => "tie",
    }
}

fn main() {
    let weights: Vec<Ratio> = [5, 4, 3, 2, 1, 0].iter().map(|&s| Ratio::new(5, 5 + s)).collect();

    let races = [
        Race { name: "Ward 1", a: ("Alma", &[0, 0, 1, 1, 1, 5]), b: ("Bruno", &[0, 3, 1, 1, 5, 0]) },
        Race { name: "Ward 2", a: ("Petra", &[1, 3, 4, 0, 1, 2]), b: ("Quinn", &[0, 2, 1, 2, 4, 1]) },
        Race { name: "Ward 3", a: ("Rosa", &[5, 4, 5, 4, 5, 4]), b: ("Sami", &[1, 1, 0, 1, 0, 1]) },
        Race { name: "Ward 4", a: ("Tariq", &[3, 3, 2, 2, 4, 1]), b: ("Vera", &[2, 4, 3, 1, 3, 2]) },
        Race { name: "Ward 5", a: ("Wes", &[0, 1, 2, 3, 4, 5]), b: ("Yuki", &[5, 4, 3, 2, 1, 0]) },
    ];

    println!("=== the slate, and what is actually true ===");
    println!("  {:<8} {:<14} {:>12} {:>12}  {:>10}", "race", "contest", "exact A", "exact B", "margin");
    for r in &races {
        let (ea, eb) = (exact(r.a.1, &weights), exact(r.b.1, &weights));
        let margin = if ea.cmp(eb) == Ordering::Less { eb.sub(ea) } else { ea.sub(eb) };
        println!("  {:<8} {:<14} {:>12} {:>12}  {:>10}",
                 r.name, format!("{} v {}", r.a.0, r.b.0), ea.text(), eb.text(), margin.text());
    }

    println!("\n=== the naive escalation: `while undecided {{ scale *= 10 }}` ===");
    println!("  Run it on Ward 1 with a counter instead of a prayer:");
    let r = &races[0];
    let mut scale: i128 = 10;
    let mut spins = 0;
    while spins < 30 {
        let (alo, ahi) = bounds(r.a.1, &weights, scale);
        let (blo, bhi) = bounds(r.b.1, &weights, scale);
        if alo > bhi || blo > ahi {
            break;
        }
        spins += 1;
        if let Some(next) = scale.checked_mul(10) { scale = next } else { break }
    }
    println!("    spun {spins} times, reached scale {scale}, still undecided");
    println!("    it is not slow -- it does not terminate. The exact answer is a");
    println!("    tie, and no scale separates a tie. A loop whose only two states");
    println!("    are 'decided' and 'try harder' cannot represent that.");

    println!("\n=== the fix is a third outcome, not a bigger budget ===");
    println!("  {:<8} {:<14} {:>10}  {:<24} {}", "race", "contest", "true", "audit says", "check");
    let mut certified = 0;
    let mut wrong = 0;
    let mut escalated = 0;
    for r in &races {
        let truth = exact_winner(r, &weights);
        let result = audit(r, &weights);
        let shown = match result {
            Audit::Certified(w, s) => format!("certified {w} at 1/{s}"),
            Audit::HandCount => "escalate: hand count".to_string(),
        };
        let check = match result {
            Audit::Certified(w, _) => {
                certified += 1;
                if w == truth { "ok" } else { wrong += 1; "WRONG WINNER" }
            }
            Audit::HandCount => {
                escalated += 1;
                if truth == "tie" { "correct to refuse" } else { "conservative" }
            }
        };
        println!("  {:<8} {:<14} {:>10}  {:<24} {}", r.name, format!("{} v {}", r.a.0, r.b.0), truth, shown, check);
    }

    println!("\n=== what the audit is allowed to claim ===");
    println!("  certified          : {certified}");
    println!("  certified wrongly  : {wrong}   <- the number that must stay 0");
    println!("  escalated          : {escalated}");
    println!("  An escalation is never a failure of the method. It is the method");
    println!("  declining to certify something it cannot prove -- which on Ward 1");
    println!("  is not conservatism at all, but the only correct answer there is.");
    println!("  Cost is one-sided too: the worst an audit can do is unnecessary work.");
}
