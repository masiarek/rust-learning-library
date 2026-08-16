//! Where a debug build's time goes, and how far each optimization can reach into it.
//!
//! Nothing here is timed by this program. A wall clock is exactly the input the
//! library's answer keys cannot hold — it differs on every machine, so a recorded
//! one would be a number nobody could reproduce. The profile below is instead
//! written down as *data*: five numbers off one `cargo build --timings` run, in
//! plain sight, where you can disagree with them. The program does the arithmetic
//! on top, and the arithmetic is the part that generalizes. Put your own five
//! numbers in and the conclusions move with them.

/// One phase of a clean debug build, and the seconds it took.
#[derive(Clone, Copy)]
struct Phase {
    name: &'static str,
    seconds: f64,
}

/// A build-configuration change, and how much of each phase survives it.
struct Tweak {
    name: &'static str,
    nightly: bool,
    /// `(phase, the fraction of that phase's time still left afterwards)`.
    /// A phase this tweak cannot reach is simply not listed.
    effect: &'static [(&'static str, f64)],
}

impl Tweak {
    /// The multiplier for `phase` — 1.0, untouched, when this tweak cannot reach it.
    fn survival(&self, phase: &str) -> f64 {
        self.effect
            .iter()
            .find(|(name, _)| *name == phase)
            .map(|&(_, fraction)| fraction)
            .unwrap_or(1.0)
    }
}

/// One project, one machine, one clean `cargo build`. Replace with your own.
const BASELINE: &[Phase] = &[
    Phase { name: "parse & macro expansion", seconds: 4.0 },
    Phase { name: "type & borrow check", seconds: 22.0 },
    Phase { name: "codegen (LLVM)", seconds: 38.0 },
    Phase { name: "debug info", seconds: 14.0 },
    Phase { name: "link", seconds: 12.0 },
];

/// The three knobs, in the order the lesson applies them.
const TWEAKS: &[Tweak] = &[
    Tweak {
        name: "debug = \"line-tables-only\"",
        nightly: false,
        effect: &[("debug info", 0.15), ("link", 0.60)],
    },
    Tweak {
        name: "-Z threads=8",
        nightly: true,
        effect: &[("type & borrow check", 0.60)],
    },
    Tweak {
        name: "codegen-backend = cranelift",
        nightly: true,
        effect: &[("codegen (LLVM)", 0.55)],
    },
];

fn total(profile: &[f64]) -> f64 {
    profile.iter().sum()
}

fn main() {
    let baseline: Vec<f64> = BASELINE.iter().map(|p| p.seconds).collect();
    let start = total(&baseline);
    println!("Clean debug build, as measured:            {start:>6.1} s\n");

    // Apply the tweaks one at a time, carrying the profile forward.
    println!("{:<31} {:>8} {:>8}  {}", "after adding", "total", "saved", "needs");
    println!("{}", "-".repeat(61));

    let mut current = baseline.clone();
    for tweak in TWEAKS {
        current = current
            .iter()
            .zip(BASELINE)
            .map(|(seconds, phase)| seconds * tweak.survival(phase.name))
            .collect();
        let now = total(&current);
        let saved = 100.0 * (start - now) / start;
        let channel = if tweak.nightly { "nightly" } else { "stable" };
        println!("{:<31} {now:>6.1} s {saved:>7.1}%  {channel}", tweak.name);
    }

    let end = total(&current);
    println!("\n{:<26} {:>8} {:>8} {:>9}", "phase", "before", "after", "removed");
    println!("{}", "-".repeat(61));
    for (phase, after) in BASELINE.iter().zip(&current) {
        let removed = phase.seconds - after;
        println!(
            "{:<26} {:>6.1} s {:>6.1} s {removed:>7.1} s",
            phase.name, phase.seconds, after
        );
    }

    // What no knob reached is what sets the floor.
    let untouched: f64 = BASELINE
        .iter()
        .zip(&current)
        .filter(|(phase, after)| (phase.seconds - *after).abs() < f64::EPSILON)
        .map(|(phase, _)| phase.seconds)
        .sum();
    let (slowest, slowest_seconds) = BASELINE
        .iter()
        .zip(&current)
        .max_by(|a, b| a.1.partial_cmp(b.1).expect("no NaN in a build profile"))
        .map(|(phase, after)| (phase.name, *after))
        .expect("the profile is not empty");

    println!(
        "\n{:.1} s -> {:.1} s, a {:.0}% cut.",
        start,
        end,
        100.0 * (start - end) / start
    );
    println!(
        "The floor: {untouched:.1} s no knob reached, now {:.0}% of the build.",
        100.0 * untouched / end
    );
    println!("The next thing worth attacking is {slowest} at {slowest_seconds:.1} s.");
}
