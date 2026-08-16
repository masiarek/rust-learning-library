//! Kata: add a fourth knob — a faster linker — and credit it honestly.
//!
//! Same profile as the lesson, same three knobs, plus `linker = "lld"`, which
//! leaves 45% of link time. The question is what the linker swap is *worth*, and
//! the answer is that the question is badly posed: run the ladder with the swap
//! first and again with it last, and the same change earns a different number of
//! seconds while the final total does not move at all.

#[derive(Clone, Copy)]
struct Phase {
    name: &'static str,
    seconds: f64,
}

struct Tweak {
    name: &'static str,
    effect: &'static [(&'static str, f64)],
}

impl Tweak {
    fn survival(&self, phase: &str) -> f64 {
        self.effect
            .iter()
            .find(|(name, _)| *name == phase)
            .map(|&(_, fraction)| fraction)
            .unwrap_or(1.0)
    }
}

const BASELINE: &[Phase] = &[
    Phase { name: "parse & macro expansion", seconds: 4.0 },
    Phase { name: "type & borrow check", seconds: 22.0 },
    Phase { name: "codegen (LLVM)", seconds: 38.0 },
    Phase { name: "debug info", seconds: 14.0 },
    Phase { name: "link", seconds: 12.0 },
];

const DEBUGINFO: Tweak = Tweak {
    name: "debug = \"line-tables-only\"",
    effect: &[("debug info", 0.15), ("link", 0.60)],
};
const THREADS: Tweak = Tweak {
    name: "-Z threads=8",
    effect: &[("type & borrow check", 0.60)],
};
const CRANELIFT: Tweak = Tweak {
    name: "codegen-backend = cranelift",
    effect: &[("codegen (LLVM)", 0.55)],
};
const LLD: Tweak = Tweak {
    name: "linker = \"lld\"",
    effect: &[("link", 0.45)],
};

/// Walk one ordering, printing what each rung takes off the running total.
fn ladder(label: &str, order: &[&Tweak]) -> f64 {
    println!("{label}");
    let mut current: Vec<f64> = BASELINE.iter().map(|p| p.seconds).collect();
    for tweak in order {
        let before = current.iter().sum::<f64>();
        current = current
            .iter()
            .zip(BASELINE)
            .map(|(seconds, phase)| seconds * tweak.survival(phase.name))
            .collect();
        let after = current.iter().sum::<f64>();
        println!("  {:<31} -{:>5.2} s   ({after:>5.1} s left)", tweak.name, before - after);
    }
    current.iter().sum()
}

fn main() {
    let first = ladder("Linker swapped FIRST:", &[&LLD, &DEBUGINFO, &THREADS, &CRANELIFT]);
    println!();
    let last = ladder("Linker swapped LAST:", &[&DEBUGINFO, &THREADS, &CRANELIFT, &LLD]);

    println!("\nSame four changes, same final build: {first:.2} s and {last:.2} s.");
    println!(
        "But the linker swap is credited with 6.60 s in one order and 3.96 s in the other, \
         so \"lld saved us N seconds\" is a fact about the ORDER, not about lld."
    );
    let baseline: f64 = BASELINE.iter().map(|p| p.seconds).sum();
    println!("Overall: {baseline:.1} s -> {last:.1} s, a {:.0}% cut.", 100.0 * (baseline - last) / baseline);
}
