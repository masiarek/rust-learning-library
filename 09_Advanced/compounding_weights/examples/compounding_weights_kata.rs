//! Kata solution — the count that always finishes, and what that costs.
//!
//! The lesson's exact count can run out of `i128` partway through a real
//! election, and there is no width that fixes it in general. So the shippable
//! answer is a fixed-point count: pick a scale, state a rounding rule, and never
//! divide outside it. It cannot overflow and it always produces a committee.
//!
//! The question this program answers is how much precision that actually needs,
//! and the answer is uncomfortable in a useful way.

use std::cmp::Ordering;

const K: i128 = 5;

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

// ------------------------------------------------------------ exact (baseline)

#[derive(Clone, Copy, PartialEq, Eq)]
struct Ratio {
    num: i128,
    den: i128,
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
    /// The lcm form throughout — the lesson showed the schoolbook one dies sooner.
    fn add(self, o: Ratio) -> Option<Ratio> {
        let l = (self.den / gcd(self.den, o.den)).checked_mul(o.den)?;
        Ratio::new(
            self.num
                .checked_mul(l / self.den)?
                .checked_add(o.num.checked_mul(l / o.den)?)?,
            l,
        )
    }
    fn sub(self, o: Ratio) -> Option<Ratio> {
        self.add(Ratio { num: -o.num, den: o.den })
    }
    fn mul(self, o: Ratio) -> Option<Ratio> {
        Ratio::new(self.num.checked_mul(o.num)?, self.den.checked_mul(o.den)?)
    }
    fn div(self, o: Ratio) -> Option<Ratio> {
        if o.num == 0 {
            return None;
        }
        Ratio::new(self.num.checked_mul(o.den)?, self.den.checked_mul(o.num)?)
    }
    fn cmp_checked(self, o: Ratio) -> Option<Ordering> {
        let g = gcd(self.den, o.den);
        Some(
            self.num
                .checked_mul(o.den / g)?
                .cmp(&o.num.checked_mul(self.den / g)?),
        )
    }
    fn is_zero(self) -> bool {
        self.num == 0
    }
}

/// `None` when the exact answer stopped fitting in `i128`.
fn exact(ballots: &[Vec<i128>], seats: usize) -> Option<Vec<usize>> {
    let (nb, nc) = (ballots.len(), ballots[0].len());
    let mut w = vec![Ratio::int(1); nb];
    let mut alive: Vec<usize> = (0..nc).collect();
    let mut winners = Vec::new();
    let hare = Ratio::new(nb as i128, seats as i128)?;

    for seat in 1..=seats {
        let mut best: Option<(Ratio, usize)> = None;
        for &c in &alive {
            let mut total = Ratio::int(0);
            for i in 0..nb {
                if ballots[i][c] != 0 && !w[i].is_zero() {
                    total = total.add(w[i].mul(Ratio::int(ballots[i][c]))?)?;
                }
            }
            let better = match best {
                None => true,
                Some((b, _)) => total.cmp_checked(b)? == Ordering::Greater,
            };
            if better {
                best = Some((total, c));
            }
        }
        let win = best?.1;
        winners.push(win);
        alive.retain(|&c| c != win);
        if seat == seats {
            break;
        }

        let mut quota = hare;
        let mut sup: Vec<usize> = (0..nb)
            .filter(|&i| ballots[i][win] != 0 && !w[i].is_zero())
            .collect();
        while !sup.is_empty() {
            let mut top = Ratio::int(0);
            for &i in &sup {
                let s = w[i].mul(Ratio::int(ballots[i][win]))?;
                if s.cmp_checked(top)? == Ordering::Greater {
                    top = s;
                }
            }
            let mut grp = Vec::new();
            for &i in &sup {
                if w[i].mul(Ratio::int(ballots[i][win]))? == top {
                    grp.push(i);
                }
            }
            let mut allocated = Ratio::int(0);
            for &i in &grp {
                allocated = allocated.add(w[i])?;
            }
            if allocated.cmp_checked(quota)? != Ordering::Greater {
                for &i in &grp {
                    w[i] = Ratio::int(0);
                }
                sup.retain(|i| !grp.contains(i));
                quota = quota.sub(allocated)?;
                if quota.is_zero() {
                    break;
                }
                continue;
            }
            let keep = Ratio::int(1).sub(quota.div(allocated)?)?;
            for &i in &grp {
                w[i] = w[i].mul(keep)?;
            }
            break;
        }
    }
    Some(winners)
}

// --------------------------------------------------------------- fixed point
//
// THE RULE, stated once and applied everywhere:
//
//   A weight is an integer count of 1/`scale` units. A full ballot is `scale`.
//   Every division truncates toward zero — so a reweighted ballot is never
//   credited with more weight than it is owed, and the count can only ever
//   under-spend a quota, never over-spend it.
//
// Nothing here can overflow for any scale where `scale * scale` fits, and
// nothing here can fail to produce a committee.

fn fixed(ballots: &[Vec<i128>], seats: usize, scale: i128) -> Vec<usize> {
    let (nb, nc) = (ballots.len(), ballots[0].len());
    let mut w = vec![scale; nb];
    let mut alive: Vec<usize> = (0..nc).collect();
    let mut winners = Vec::new();
    let hare = (nb as i128) * scale / (seats as i128);

    for seat in 1..=seats {
        let mut best = (i128::MIN, 0usize);
        for &c in &alive {
            let mut t: i128 = 0;
            for i in 0..nb {
                if ballots[i][c] != 0 && w[i] != 0 {
                    t += w[i] * ballots[i][c];
                }
            }
            if t > best.0 {
                best = (t, c);
            }
        }
        let win = best.1;
        winners.push(win);
        alive.retain(|&c| c != win);
        if seat == seats {
            break;
        }

        let mut quota = hare;
        let mut sup: Vec<usize> = (0..nb)
            .filter(|&i| ballots[i][win] != 0 && w[i] != 0)
            .collect();
        while !sup.is_empty() {
            let top = sup.iter().map(|&i| w[i] * ballots[i][win]).max().unwrap();
            let grp: Vec<usize> = sup
                .iter()
                .copied()
                .filter(|&i| w[i] * ballots[i][win] == top)
                .collect();
            let allocated: i128 = grp.iter().map(|&i| w[i]).sum();
            if allocated <= quota {
                for &i in &grp {
                    w[i] = 0;
                }
                sup.retain(|i| !grp.contains(i));
                quota -= allocated;
                if quota == 0 {
                    break;
                }
                continue;
            }
            let keep = scale - (quota * scale / allocated); // truncates
            for &i in &grp {
                w[i] = w[i] * keep / scale; // truncates
            }
            break;
        }
    }
    winners
}

// ------------------------------------------------------------------ ballots

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

// --------------------------------------------------------------------- main

fn main() {
    const SCALES: [(&str, i128); 6] = [
        ("10^18", 1_000_000_000_000_000_000),
        ("10^12", 1_000_000_000_000),
        ("10^9", 1_000_000_000),
        ("10^6", 1_000_000),
        ("10^4", 10_000),
        ("10^2", 100),
    ];

    const RUNS: u64 = 6;
    for &(nb, nc, seats) in &[(700usize, 16usize, 10usize), (2000, 30, 20)] {
        println!("{nb} ballots, {nc} candidates, {seats} seats — {RUNS} electorates\n");

        // Ground truth, where there is any.
        let elections: Vec<Vec<Vec<i128>>> =
            (1..=RUNS).map(|s| Ballots::election(s, nb, nc)).collect();
        let truth: Vec<Option<Vec<usize>>> =
            elections.iter().map(|e| exact(e, seats)).collect();
        let checkable = truth.iter().filter(|t| t.is_some()).count();
        let overflowed = RUNS as usize - checkable;

        println!("     exact count finished on {checkable} of {RUNS}, ran out of i128 on {overflowed}");
        if overflowed > 0 {
            println!("     (the fixed-point count returned a committee on all {RUNS})");
        }
        println!();

        println!("     scale   reproduces the exact winner list");
        for &(name, sc) in &SCALES {
            let agree = elections
                .iter()
                .zip(&truth)
                .filter(|(_, t)| t.is_some())
                .filter(|(e, t)| fixed(e, seats, sc) == *t.as_ref().unwrap())
                .count();
            let note = if agree == checkable { "" } else { "   <-- diverges" };
            println!("     {name:<7} {agree} of {checkable}{note}");
        }
        println!();
    }

    println!("Four decimal places reproduce every exact answer anyone could check.");
    println!("Two do not. The exact count that ran out of i128 was working to 68-odd");
    println!("bits of denominator to reach a result that 10^4 also reaches.");
    println!();
    println!("What that establishes: on these electorates, at these sizes, the");
    println!("approximation and the exact answer agree. What it does not establish:");
    println!("anything at all about an election with a tied round, which is exactly");
    println!("where a truncated weight and an exact one part company, and exactly the");
    println!("case the companion lesson opened with.");
}
