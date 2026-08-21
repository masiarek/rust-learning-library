//! Kata solution — the scale that stopped covering the election.
//!
//! The lesson's scale was derived from `SEATS`. Deriving it costs an lcm over a
//! couple of dozen integers on every run, which looks like exactly the sort of
//! thing to compute once and paste in as a constant. Then the election grows a
//! seat, the constant does not, and the count keeps running.
//!
//! What makes this worth a kata is how it fails. It does not panic, it does not
//! produce an absurd number, and it does not change who won. It just quietly
//! stops being exact — which was the only reason to leave `f64` in the first
//! place.

const C: i128 = 5;

/// The election is seven seats now. It was five when the constant below was
/// computed.
const SEATS: i128 = 7;

/// Late in the count: six ballots, and the totals they have already spent on the
/// six candidates elected so far. Denominators are `C + S`.
const SPENT: [i128; 6] = [26, 27, 30, 12, 0, 5];

const ALMA: [i128; 6] = [4, 0, 3, 5, 2, 1];
const BRUNO: [i128; 6] = [1, 5, 2, 0, 4, 3];

/// The stale constant: `lcm(5..=30)`, correct for a five-seat election.
const STALE_SCALE: i128 = 2_329_089_562_800;

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

/// Derive the scale from the election, which is the fix.
fn scale_for(seats: i128) -> i128 {
    let mut l: i128 = 1;
    for den in C..=(C + seats * C) {
        l = l / gcd(l, den) * den;
    }
    l
}

/// The count with no precondition check — what the lesson's `assert!` was for.
fn count_unchecked(scores: &[i128; 6], l: i128) -> i128 {
    let mut total = 0;
    for (i, &score) in scores.iter().enumerate() {
        total += score * C * (l / (C + SPENT[i]));
    }
    total
}

/// The same count, refusing to run on a scale that cannot represent the answer.
fn count_checked(scores: &[i128; 6], l: i128) -> Result<i128, String> {
    let mut total = 0;
    for (i, &score) in scores.iter().enumerate() {
        let den = C + SPENT[i];
        if l % den != 0 {
            return Err(format!(
                "ballot {} has denominator {den}, which does not divide the scale {l}",
                i + 1
            ));
        }
        total += score * C * (l / den);
    }
    Ok(total)
}

/// Exact totals, as a reduced fraction, to compare both counts against.
fn count_exact(scores: &[i128; 6]) -> (i128, i128) {
    let (mut num, mut den) = (0i128, 1i128);
    for (i, &score) in scores.iter().enumerate() {
        let (n, d) = (score * C, C + SPENT[i]);
        let (nn, dd) = (num * d + n * den, den * d);
        let g = gcd(nn, dd);
        num = nn / g;
        den = dd / g;
    }
    (num, den)
}

fn main() {
    println!("A {SEATS}-seat count, still scaled by a constant computed for 5 seats.\n");

    // 1. The precondition, stated out loud.
    println!("1. Which denominators the stale scale can still represent");
    println!("     stale scale = {STALE_SCALE}  (= lcm 5..=30)");
    for (i, &s) in SPENT.iter().enumerate() {
        let den = C + s;
        println!(
            "     ballot {}  denominator {:>2}   {}",
            i + 1,
            den,
            if STALE_SCALE % den == 0 { "exact" } else { "TRUNCATES" },
        );
    }
    println!("     31 is a prime above 30, and 32 needs a fifth factor of two.");
    println!("     Neither is in lcm(5..=30), so neither divides it.");

    // 2. The failure, and how invisible it is.
    let good = scale_for(SEATS);
    let (stale_a, stale_b) = (
        count_unchecked(&ALMA, STALE_SCALE),
        count_unchecked(&BRUNO, STALE_SCALE),
    );
    let (good_a, good_b) = (
        count_unchecked(&ALMA, good),
        count_unchecked(&BRUNO, good),
    );
    let (ea, eb) = (count_exact(&ALMA), count_exact(&BRUNO));

    println!("\n2. What the stale scale actually does to the count");
    println!("     exact        Alma {}/{}   Bruno {}/{}", ea.0, ea.1, eb.0, eb.1);
    println!(
        "     stale scale  Alma {stale_a}   Bruno {stale_b}  -> {} wins",
        if stale_a > stale_b { "Alma" } else { "Bruno" }
    );
    println!(
        "     derived      Alma {good_a}   Bruno {good_b}  -> {} wins",
        if good_a > good_b { "Alma" } else { "Bruno" }
    );
    println!(
        "     stale total equals the exact value?  Alma {}   Bruno {}",
        stale_a * ea.1 == ea.0 * STALE_SCALE,
        stale_b * eb.1 == eb.0 * STALE_SCALE,
    );
    println!(
        "     derived total equals the exact value? Alma {}   Bruno {}",
        good_a * ea.1 == ea.0 * good,
        good_b * eb.1 == eb.0 * good,
    );
    println!("     -> the winner is the same either way. No test on the winner");
    println!("        would ever have caught this.");

    // 3. The check that does catch it.
    println!("\n3. The same count, checking its precondition first");
    match count_checked(&ALMA, STALE_SCALE) {
        Ok(v) => println!("     stale scale   -> Ok({v})"),
        Err(e) => println!("     stale scale   -> Err: {e}"),
    }
    match count_checked(&ALMA, good) {
        Ok(v) => println!("     derived scale -> Ok({v})"),
        Err(e) => println!("     derived scale -> Err: {e}"),
    }

    // 4. And the bill for fixing it.
    println!("\n4. What the correct scale costs");
    for seats in [5i128, 7] {
        let l = scale_for(seats);
        let per_ballot = C * C * (l / C);
        println!(
            "     {seats} seats: scale {l:>22}   ballots that fit i64: {:>7}",
            i64::MAX as i128 / per_ballot,
        );
    }
    println!("     Fixing the exactness bug cut the i64 ballot capacity from 792,015");
    println!("     to 345. In i128 the same election still has room for 6.4e21 ballots.");
}
