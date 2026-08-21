//! Allocated Score: the proportional count whose denominators will not stay put.
//!
//! The companion lesson scales the denominators away, which works because a
//! Reweighted Range Voting weight is `C/(C+S)` for an integer `S` — so every
//! denominator that can ever appear is known before the first ballot is read.
//!
//! Allocated Score sets the weight a different way:
//!
//!     weight <- weight * (1 - quota / allocated)
//!
//! where `allocated` is a sum of the CURRENT weights. This round's denominators
//! are therefore built out of last round's, and there is no set to enumerate.
//! This program shows what that costs.

use std::cmp::Ordering;

const K: i128 = 5; // maximum score

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

    /// Comparison cross-multiplies too, so it divides the shared factor out
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
    /// Every seat filled exactly. Carries the widest denominator it needed.
    Exact { winners: Vec<usize>, peak_bits: u32 },
    /// The exact answer to some step did not fit. Carries the seat it died on.
    Overflowed { seat: usize, peak_bits: u32 },
}

/// Allocated Score, following starvote's formulation of the STAR Voting spec:
/// elect the highest weighted score, then spend a Hare quota of ballot weight on
/// that winner, highest scorers first, reweighting the group that straddles the
/// quota line.
fn allocated_score(
    ballots: &[Vec<i128>],
    seats: usize,
    how: Add,
    trace: bool,
) -> Outcome {
    let (nb, nc) = (ballots.len(), ballots[0].len());
    let mut w = vec![Ratio::int(1); nb];
    let mut alive: Vec<usize> = (0..nc).collect();
    let mut winners = Vec::new();
    let mut peak = 1u32;

    let hare = match Ratio::new(nb as i128, seats as i128) {
        Some(q) => q,
        None => return Outcome::Overflowed { seat: 0, peak_bits: peak },
    };

    macro_rules! step {
        ($e:expr, $seat:expr) => {
            match $e {
                Some(v) => v,
                None => return Outcome::Overflowed { seat: $seat, peak_bits: peak },
            }
        };
    }

    for seat in 1..=seats {
        // --- scoring round: highest weighted total wins the seat
        let mut best: Option<(Ratio, usize)> = None;
        for &c in &alive {
            let mut total = Ratio::int(0);
            for i in 0..nb {
                if ballots[i][c] != 0 && !w[i].is_zero() {
                    let contribution = step!(w[i].mul(Ratio::int(ballots[i][c])), seat);
                    total = step!(total.add(contribution, how), seat);
                }
            }
            let better = match best {
                None => true,
                Some((b, _)) => step!(total.cmp_checked(b), seat) == Ordering::Greater,
            };
            if better {
                best = Some((total, c));
            }
        }
        let win = best.expect("at least one candidate remains").1;
        winners.push(win);
        alive.retain(|&c| c != win);
        if seat == seats {
            break;
        }

        // --- allocation round: spend one Hare quota of weight on the winner
        let mut quota = hare;
        let mut supporters: Vec<usize> = (0..nb)
            .filter(|&i| ballots[i][win] != 0 && !w[i].is_zero())
            .collect();

        while !supporters.is_empty() {
            // the group sharing the highest weighted score for this winner
            let mut top = Ratio::int(0);
            for &i in &supporters {
                let s = step!(w[i].mul(Ratio::int(ballots[i][win])), seat);
                if step!(s.cmp_checked(top), seat) == Ordering::Greater {
                    top = s;
                }
            }
            let mut group = Vec::new();
            for &i in &supporters {
                if step!(w[i].mul(Ratio::int(ballots[i][win])), seat) == top {
                    group.push(i);
                }
            }

            let mut allocated = Ratio::int(0);
            for &i in &group {
                allocated = step!(allocated.add(w[i], how), seat);
            }

            if step!(allocated.cmp_checked(quota), seat) != Ordering::Greater {
                // the whole group fits inside the quota: spend it outright
                for &i in &group {
                    w[i] = Ratio::int(0);
                }
                supporters.retain(|i| !group.contains(i));
                quota = step!(quota.sub(allocated, how), seat);
                if quota.is_zero() {
                    break;
                }
                continue;
            }

            // the group overfills the quota: keep everyone, at a reduced weight.
            // THIS is the line the companion lesson's trick cannot survive — the
            // new denominator is built from `allocated`, a sum of current weights.
            let spent = step!(quota.div(allocated), seat);
            let keep = step!(Ratio::int(1).sub(spent, how), seat);
            for &i in &group {
                w[i] = step!(w[i].mul(keep), seat);
            }
            break;
        }

        for x in &w {
            peak = peak.max(x.bits());
        }
        if trace {
            let widest = w.iter().max_by_key(|x| x.den).unwrap();
            println!(
                "     seat {seat} -> candidate {win}   widest weight {widest}   ({} bits)",
                widest.bits()
            );
        }
    }
    Outcome::Exact { winners, peak_bits: peak }
}

// ------------------------------------------------------------- deterministic data

/// A fixed sequence, not a random one. The answer key has to match on every
/// machine, so this is an ordinary linear congruential generator with the seed
/// written down — a hard-coded table of ballots, just too long to print.
struct Ballots(u64);

impl Ballots {
    fn next(&mut self, m: u64) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) % m
    }

    fn election(seed: u64, voters: usize, candidates: usize) -> Vec<Vec<i128>> {
        let mut g = Ballots(seed);
        (0..voters)
            .map(|_| (0..candidates).map(|_| g.next(K as u64 + 1) as i128).collect())
            .collect()
    }
}

// ------------------------------------------------------------------------- main

fn main() {
    // === 1. the mechanism, on a real election ===============================
    //
    // BetterVoting's own AllocatedScore test, "Fractional surplus": twelve
    // ballots, four candidates, two seats.
    let names = ["Allison", "Bill", "Carmen", "Doug"];
    let real: Vec<Vec<i128>> = vec![
        vec![5, 5, 1, 0], vec![5, 5, 1, 0], vec![5, 5, 1, 0], vec![5, 5, 1, 0],
        vec![5, 5, 1, 0], vec![5, 5, 1, 0], vec![5, 5, 1, 0], vec![5, 4, 4, 0],
        vec![0, 0, 0, 3], vec![0, 0, 4, 5], vec![0, 0, 4, 5], vec![0, 0, 4, 5],
    ];
    println!("1. Where the denominator comes from");
    println!("   Twelve ballots, two seats, so the Hare quota is 12/2 = 6 ballots.");
    println!("   Allison wins seat 1 with 8 supporters at the top score — 8 units of");
    println!("   weight to spend a 6-unit quota, so each keeps 1 - 6/8:\n");
    match allocated_score(&real, 2, Add::LeastCommonMultiple, true) {
        Outcome::Exact { winners, .. } => println!(
            "\n   winners: {}",
            winners.iter().map(|&i| names[i]).collect::<Vec<_>>().join(", ")
        ),
        Outcome::Overflowed { seat, .. } => println!("   overflowed at seat {seat}"),
    }
    println!("\n   The 4 in that 1/4 was not chosen from any fixed set. It is the total");
    println!("   weight of whoever happened to tie at the top score — so seat 3's");
    println!("   denominators would be built from seat 2's weights, and so on down.");
    println!("   There is no set of denominators to take an lcm over.");

    // === 2. how wide the answer gets is a property of the ballots ===========
    println!("\n2. Eight electorates of exactly the same size");
    println!("   900 ballots, 18 candidates, 12 seats. Only the votes differ.\n");
    let mut peaks = Vec::new();
    for seed in 1..=8u64 {
        let e = Ballots::election(seed, 900, 18);
        if let Outcome::Exact { peak_bits, .. } =
            allocated_score(&e, 12, Add::LeastCommonMultiple, false)
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
    println!("   Same ballot count, same seats, same candidates. The arithmetic the");
    println!("   count needs is decided by how people voted, and by nothing you know");
    println!("   before they vote.");

    // === 3. the intermediate, not the answer, is what overflows =============
    println!("\n3. The same election, counted two ways");
    println!("   2000 ballots, 30 candidates, 20 seats. Both additions are exact;");
    println!("   they differ only in how big a number they build on the way.\n");
    const SEATS_3: usize = 20;
    let e = Ballots::election(2, 2000, 30);
    for (label, how) in [
        ("a/b + c/d = (ad+cb)/bd", Add::CrossMultiply),
        ("both over lcm(b, d)   ", Add::LeastCommonMultiple),
    ] {
        match allocated_score(&e, SEATS_3, how, false) {
            Outcome::Exact { peak_bits, .. } => println!(
                "     {label}  -> all {SEATS_3} seats, widest denominator {peak_bits} bits"
            ),
            Outcome::Overflowed { seat, peak_bits } => println!(
                "     {label}  -> OVERFLOWED at seat {seat} (widest so far {peak_bits} bits)"
            ),
        }
    }
    println!("\n   Read those two lines together. One counted the whole election; the");
    println!("   other gave up four seats short. And the widest number the finishing");
    println!("   count ever had to REPRESENT was 68 bits — barely half of i128's 127.");
    println!("   The failing count did not run out of room for its answers. It ran");
    println!("   out building `(ad+cb)/bd`, a product of two denominators that it was");
    println!("   about to reduce away again.");

    // === 4. and where both give out =========================================
    println!("\n4. Far enough out, the answer itself stops fitting");
    println!("   2500 ballots, 35 candidates, 28 seats:\n");
    const SEATS_4: usize = 28;
    let big = Ballots::election(4, 2500, 35);
    for (label, how) in [
        ("cross-multiply", Add::CrossMultiply),
        ("lcm           ", Add::LeastCommonMultiple),
    ] {
        match allocated_score(&big, SEATS_4, how, false) {
            Outcome::Exact { peak_bits, .. } => {
                println!("     {label}  -> all {SEATS_4} seats, widest denominator {peak_bits} bits")
            }
            Outcome::Overflowed { seat, peak_bits } => println!(
                "     {label}  -> OVERFLOWED at seat {seat} (widest so far {peak_bits} bits)"
            ),
        }
    }
    println!("\n   The better addition bought eleven seats, not safety. No fixed width");
    println!("   covers an unbounded quantity, and the width this count needs is not");
    println!("   knowable until it has read the ballots — which is why it returns an");
    println!("   Outcome rather than a number.");
}
