//! The proportional allocation whose denominators will not stay put.
//!
//! The companion lesson scales the denominators away, which works because a
//! state-recomputed weight is `C/(C+S)` for an integer `S` — so every
//! denominator that can ever appear is known before the first row is read.
//!
//! The compounding rule sets the weight a different way:
//!
//!     weight <- weight * (1 - quota / allocated)
//!
//! where `allocated` is a sum of the CURRENT weights. This round's denominators
//! are therefore built out of last round's, and there is no set to enumerate.
//! This program shows what that costs.

use std::cmp::Ordering;

const K: i128 = 5; // the rating cap

// ------------------------------------------------------- checked exact rational

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

/// An exact fraction over `i128`, where every operation returns `Option`.
///
/// `None` means "the exact answer to this step does not fit in `i128`". That is
/// the whole point of the type: the count is allowed to fail loudly, and is
/// never allowed to wrap and keep going.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Ratio {
    num: i128,
    den: i128,
}

/// Which addition to use. Both are exact; they differ only in how large an
/// intermediate they build on the way to the same answer.
#[derive(Clone, Copy, PartialEq)]
enum Add {
    /// `a/b + c/d = (ad + cb) / bd`. The schoolbook rule.
    CrossMultiply,
    /// Put both over `lcm(b, d)` first, which is never larger and usually much
    /// smaller than `bd`.
    LeastCommonMultiple,
}

impl Ratio {
    fn int(n: i128) -> Ratio {
        Ratio { num: n, den: 1 }
    }

    fn new(num: i128, den: i128) -> Option<Ratio> {
        if den == 0 {
            return None;
        }
        let g = gcd(num, den);
        let g = if g == 0 { 1 } else { g };
        let (mut num, mut den) = (num / g, den / g);
        if den < 0 {
            num = -num;
            den = -den;
        }
        Some(Ratio { num, den })
    }

    fn add(self, other: Ratio, how: Add) -> Option<Ratio> {
        match how {
            Add::CrossMultiply => Ratio::new(
                self.num
                    .checked_mul(other.den)?
                    .checked_add(other.num.checked_mul(self.den)?)?,
                self.den.checked_mul(other.den)?,
            ),
            Add::LeastCommonMultiple => {
                let l = (self.den / gcd(self.den, other.den)).checked_mul(other.den)?;
                Ratio::new(
                    self.num
                        .checked_mul(l / self.den)?
                        .checked_add(other.num.checked_mul(l / other.den)?)?,
                    l,
                )
            }
        }
    }

    fn sub(self, other: Ratio, how: Add) -> Option<Ratio> {
        self.add(
            Ratio { num: -other.num, den: other.den },
            how,
        )
    }

    fn mul(self, other: Ratio) -> Option<Ratio> {
        Ratio::new(
            self.num.checked_mul(other.num)?,
            self.den.checked_mul(other.den)?,
        )
    }

    fn div(self, other: Ratio) -> Option<Ratio> {
        if other.num == 0 {
            return None;
        }
        Ratio::new(
            self.num.checked_mul(other.den)?,
            self.den.checked_mul(other.num)?,
        )
    }

    /// Comparison cross-multiplies too, so it divides the squota_sized factor out
    /// first — `a/b` vs `c/d` becomes `a*(d/g)` vs `c*(b/g)`. Same answer, much
    /// smaller intermediate, and it matters: with the naive form the count dies
    /// in the comparison long before it dies in the arithmetic.
    fn cmp_checked(self, other: Ratio) -> Option<Ordering> {
        let g = gcd(self.den, other.den);
        Some(
            self.num
                .checked_mul(other.den / g)?
                .cmp(&other.num.checked_mul(self.den / g)?),
        )
    }

    fn is_zero(self) -> bool {
        self.num == 0
    }

    fn bits(self) -> u32 {
        self.den.unsigned_abs().checked_ilog2().unwrap_or(0) + 1
    }
}

impl std::fmt::Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

// -------------------------------------------------------------- the count itself

/// What happened to a count: it finished, or it ran out of `i128`.
enum Outcome {
    /// Every round settled exactly. Carries the widest denominator it needed.
    Exact { picked: Vec<usize>, peak_bits: u32 },
    /// The exact answer to some step did not fit. Carries the round it died on.
    Overflowed { round: usize, peak_bits: u32 },
}

/// The compounding rule, as published under the name *the compounding rule*: pick the
/// highest weighted total, then spend one quota of row weight on that pick,
/// highest raters first, reweighting the group that straddles the quota line.
/// The name is here as provenance for the arithmetic; nothing below depends on it.
fn compounding_allocation(
    rows: &[Vec<i128>],
    rounds: usize,
    how: Add,
    trace: bool,
) -> Outcome {
    let (nb, nc) = (rows.len(), rows[0].len());
    let mut w = vec![Ratio::int(1); nb];
    let mut alive: Vec<usize> = (0..nc).collect();
    let mut picked = Vec::new();
    let mut peak = 1u32;

    let quota_size = match Ratio::new(nb as i128, rounds as i128) {
        Some(q) => q,
        None => return Outcome::Overflowed { round: 0, peak_bits: peak },
    };

    macro_rules! step {
        ($e:expr, $round:expr) => {
            match $e {
                Some(v) => v,
                None => return Outcome::Overflowed { round: $round, peak_bits: peak },
            }
        };
    }

    for round in 1..=rounds {
        // --- selection round: highest weighted total wins the round
        let mut best: Option<(Ratio, usize)> = None;
        for &c in &alive {
            let mut total = Ratio::int(0);
            for i in 0..nb {
                if rows[i][c] != 0 && !w[i].is_zero() {
                    let contribution = step!(w[i].mul(Ratio::int(rows[i][c])), round);
                    total = step!(total.add(contribution, how), round);
                }
            }
            let better = match best {
                None => true,
                Some((b, _)) => step!(total.cmp_checked(b), round) == Ordering::Greater,
            };
            if better {
                best = Some((total, c));
            }
        }
        let win = best.expect("at least one project remains").1;
        picked.push(win);
        alive.retain(|&c| c != win);
        if round == rounds {
            break;
        }

        // --- allocation round: spend one quota of weight on the pick
        let mut quota = quota_size;
        let mut backers: Vec<usize> = (0..nb)
            .filter(|&i| rows[i][win] != 0 && !w[i].is_zero())
            .collect();

        while !backers.is_empty() {
            // the group sharing the highest weighted score for this pick
            let mut top = Ratio::int(0);
            for &i in &backers {
                let s = step!(w[i].mul(Ratio::int(rows[i][win])), round);
                if step!(s.cmp_checked(top), round) == Ordering::Greater {
                    top = s;
                }
            }
            let mut group = Vec::new();
            for &i in &backers {
                if step!(w[i].mul(Ratio::int(rows[i][win])), round) == top {
                    group.push(i);
                }
            }

            let mut allocated = Ratio::int(0);
            for &i in &group {
                allocated = step!(allocated.add(w[i], how), round);
            }

            if step!(allocated.cmp_checked(quota), round) != Ordering::Greater {
                // the whole group fits inside the quota: spend it outright
                for &i in &group {
                    w[i] = Ratio::int(0);
                }
                backers.retain(|i| !group.contains(i));
                quota = step!(quota.sub(allocated, how), round);
                if quota.is_zero() {
                    break;
                }
                continue;
            }

            // the group overfills the quota: keep everyone, at a reduced weight.
            // THIS is the line the companion lesson's trick cannot survive — the
            // new denominator is built from `allocated`, a sum of current weights.
            let spent = step!(quota.div(allocated), round);
            let keep = step!(Ratio::int(1).sub(spent, how), round);
            for &i in &group {
                w[i] = step!(w[i].mul(keep), round);
            }
            break;
        }

        for x in &w {
            peak = peak.max(x.bits());
        }
        if trace {
            let widest = w.iter().max_by_key(|x| x.den).unwrap();
            println!(
                "     round {round} -> project {win}   widest weight {widest}   ({} bits)",
                widest.bits()
            );
        }
    }
    Outcome::Exact { picked, peak_bits: peak }
}

// ------------------------------------------------------------- deterministic data

/// A fixed sequence, not a random one. The answer key has to match on every
/// machine, so this is an ordinary linear congruential generator with the seed
/// written down — a hard-coded table of rows, just too long to print.
struct Rng(u64);

impl Rng {
    fn next(&mut self, m: u64) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) % m
    }

    fn dataset(seed: u64, raters: usize, projects: usize) -> Vec<Vec<i128>> {
        let mut g = Rng(seed);
        (0..raters)
            .map(|_| (0..projects).map(|_| g.next(K as u64 + 1) as i128).collect())
            .collect()
    }
}

// ------------------------------------------------------------------------- main

fn main() {
    // === 1. the mechanism, on a real dataset ===============================
    //
    // BetterVoting's own AllocatedScore test, "Fractional surplus": twelve
    // rows, four projects, two rounds.
    let names = ["Alpha", "Bravo", "Charlie", "Delta"];
    let real: Vec<Vec<i128>> = vec![
        vec![5, 5, 1, 0], vec![5, 5, 1, 0], vec![5, 5, 1, 0], vec![5, 5, 1, 0],
        vec![5, 5, 1, 0], vec![5, 5, 1, 0], vec![5, 5, 1, 0], vec![5, 4, 4, 0],
        vec![0, 0, 0, 3], vec![0, 0, 4, 5], vec![0, 0, 4, 5], vec![0, 0, 4, 5],
    ];
    println!("1. Where the denominator comes from");
    println!("   Twelve rows, two rounds, so the quota is 12/2 = 6 rows.");
    println!("   Alpha wins round 1 with 8 backers at the top rating — 8 units of");
    println!("   weight to spend a 6-unit quota, so each keeps 1 - 6/8:\n");
    match compounding_allocation(&real, 2, Add::LeastCommonMultiple, true) {
        Outcome::Exact { picked, .. } => println!(
            "\n   picked: {}",
            picked.iter().map(|&i| names[i]).collect::<Vec<_>>().join(", ")
        ),
        Outcome::Overflowed { round, .. } => println!("   overflowed at round {round}"),
    }
    println!("\n   The 4 in that 1/4 was not chosen from any fixed set. It is the total");
    println!("   weight of whoever happened to tie at the top rating — so round 3's");
    println!("   denominators would be built from round 2's weights, and so on down.");
    println!("   There is no set of denominators to take an lcm over.");

    // === 2. how wide the answer gets is a property of the rows ===========
    println!("\n2. Eight datasets of exactly the same size");
    println!("   900 rows, 18 projects, 12 rounds. Only the votes differ.\n");
    let mut peaks = Vec::new();
    for seed in 1..=8u64 {
        let e = Rng::dataset(seed, 900, 18);
        if let Outcome::Exact { peak_bits, .. } =
            compounding_allocation(&e, 12, Add::LeastCommonMultiple, false)
        {
            peaks.push(peak_bits);
        }
    }
    println!("     widest denominator, in bits: {peaks:?}");
    println!(
        "     min {}, max {}, spread {} bits",
        peaks.iter().min().unwrap(),
        peaks.iter().max().unwrap(),
        peaks.iter().max().unwrap() - peaks.iter().min().unwrap(),
    );
    println!("   Same row count, same rounds, same projects. The arithmetic the");
    println!("   count needs is decided by how people voted, and by nothing you know");
    println!("   before they vote.");

    // === 3. the intermediate, not the answer, is what overflows =============
    println!("\n3. The same dataset, counted two ways");
    println!("   2000 rows, 30 projects, 20 rounds. Both additions are exact;");
    println!("   they differ only in how big a number they build on the way.\n");
    const ROUNDS_3: usize = 20;
    let e = Rng::dataset(2, 2000, 30);
    for (label, how) in [
        ("a/b + c/d = (ad+cb)/bd", Add::CrossMultiply),
        ("both over lcm(b, d)   ", Add::LeastCommonMultiple),
    ] {
        match compounding_allocation(&e, ROUNDS_3, how, false) {
            Outcome::Exact { peak_bits, .. } => println!(
                "     {label}  -> all {ROUNDS_3} rounds, widest denominator {peak_bits} bits"
            ),
            Outcome::Overflowed { round, peak_bits } => println!(
                "     {label}  -> OVERFLOWED at round {round} (widest so far {peak_bits} bits)"
            ),
        }
    }
    println!("\n   Read those two lines together. One counted the whole dataset; the");
    println!("   other gave up four rounds short. And the widest number the finishing");
    println!("   count ever had to REPRESENT was 68 bits — barely half of i128's 127.");
    println!("   The failing count did not run out of room for its answers. It ran");
    println!("   out building `(ad+cb)/bd`, a product of two denominators that it was");
    println!("   about to reduce away again.");

    // === 4. and where both give out =========================================
    println!("\n4. Far enough out, the answer itself stops fitting");
    println!("   2500 rows, 35 projects, 28 rounds:\n");
    const ROUNDS_4: usize = 28;
    let big = Rng::dataset(4, 2500, 35);
    for (label, how) in [
        ("cross-multiply", Add::CrossMultiply),
        ("lcm           ", Add::LeastCommonMultiple),
    ] {
        match compounding_allocation(&big, ROUNDS_4, how, false) {
            Outcome::Exact { peak_bits, .. } => {
                println!("     {label}  -> all {ROUNDS_4} rounds, widest denominator {peak_bits} bits")
            }
            Outcome::Overflowed { round, peak_bits } => println!(
                "     {label}  -> OVERFLOWED at round {round} (widest so far {peak_bits} bits)"
            ),
        }
    }
    println!("\n   The better addition bought eleven rounds, not safety. No fixed width");
    println!("   covers an unbounded quantity, and the width this count needs is not");
    println!("   knowable until it has read the rows — which is why it returns an");
    println!("   Outcome rather than a number.");
}
