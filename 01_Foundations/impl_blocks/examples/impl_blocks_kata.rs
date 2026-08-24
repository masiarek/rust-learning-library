//! Kata solution: pick the right receiver four times, then break two of them.
//!
//!   rustc --edition 2024 impl_blocks_kata.rs -o /tmp/ibk && /tmp/ibk

#[derive(Debug)]
struct Tally {
    contest: String,
    counts: Vec<u32>,
}

impl Tally {
    // 1. NO self. There is no Tally yet — this is the thing that makes one.
    fn new(contest: &str, candidates: usize) -> Self {
        Self { contest: contest.to_string(), counts: vec![0; candidates] }
    }

    // 2. &self. Asks a question and changes nothing. The caller keeps the tally,
    //    and two of these can run at once because shared borrows stack.
    fn leader(&self) -> Option<usize> {
        let best = *self.counts.iter().max()?;
        if best == 0 {
            return None;
        }
        self.counts.iter().position(|&c| c == best)
    }

    // 3. &mut self. Changes it, caller keeps it. Needs a `mut` binding to call.
    fn record(&mut self, candidate: usize) {
        self.counts[candidate] += 1;
    }

    // 4. self. Consumes it. Certifying ENDS the tally's life on purpose — you
    //    should not be able to record another vote into a certified result.
    fn certify(self) -> String {
        match self.leader() {
            Some(i) => format!("{}: candidate {} wins with {}", self.contest, i, self.counts[i]),
            None => format!("{}: no votes cast", self.contest),
        }
    }
}

fn main() {
    println!("Four operations, four different receivers:");
    println!("  new      no self     there is no value yet");
    println!("  leader   &self       asks, changes nothing");
    println!("  record   &mut self   changes it, you keep it");
    println!("  certify  self        ends it — that is the point\n");

    let mut t = Tally::new("Mayor", 3);
    println!("  fresh:  leader() = {:?}   (None, not Some(0) — nobody has voted)", t.leader());

    t.record(2);
    t.record(0);
    t.record(2);
    println!("  after 3 votes: counts {:?}, leader {:?}", t.counts, t.leader());

    println!("\nBreak 1 — call a &mut self method through a non-mut binding:");
    println!("    let t = Tally::new(..);  t.record(0);");
    println!("    error[E0596]: cannot borrow `t` as mutable, as it is not declared as mutable");
    println!("  The method signature is what demands it. `mut` on the BINDING is the answer.");

    println!("\nBreak 2 — use the value after a method that took `self`:");
    println!("    let receipt = t.certify();  t.record(1);");
    println!("    error[E0382]: borrow of moved value: `t`");
    println!("  Not a restriction to work around — it is the guarantee `self` buys:");
    println!("  a certified tally cannot be voted into, because it no longer exists.");

    println!("\n  {}", t.certify()); // t is consumed here, deliberately last
}
