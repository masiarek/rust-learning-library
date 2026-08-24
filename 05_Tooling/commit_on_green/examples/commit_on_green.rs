//! The savepoint state machine: two states, and a commit on exactly one edge.
//!
//! The whole tool is this. What makes it useful is not the states but WHICH
//! transition saves: only Failing -> Passing. Passing -> Passing does nothing,
//! which is why you do not get a commit every time you save a file.
//!
//!   rustc --edition 2024 commit_on_green.rs -o /tmp/cog && /tmp/cog

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Passing,
    Failing,
}

struct Watcher {
    state: State,
    commits: u32,
}

impl Watcher {
    /// Start pessimistic: nothing is known to work yet.
    fn new() -> Self {
        Watcher { state: State::Failing, commits: 0 }
    }

    /// One run of the test command. Returns what happened, for the log.
    fn observe(&mut self, tests_pass: bool) -> &'static str {
        let action = match (self.state, tests_pass) {
            // The one edge that saves.
            (State::Failing, true) => {
                self.commits += 1;
                "SAVEPOINT REACHED  <- commit"
            }
            // Already green. Nothing new is known, so nothing is saved.
            (State::Passing, true) => "still passing     (no commit)",
            // Just broke it. This is the moment the last savepoint starts earning.
            (State::Passing, false) => "Error!",
            (State::Failing, false) => "still failing",
        };
        self.state = if tests_pass { State::Passing } else { State::Failing };
        action
    }
}

fn main() {
    // A plausible half hour: get it green, break it, flail, fix it, tidy up.
    let session = [
        ("write the first test and make it pass", true),
        ("refactor, still green", true),
        ("start the risky change", false),
        ("try something", false),
        ("try something else", false),
        ("revert to the last savepoint and retry", true),
        ("tidy up", true),
    ];

    let mut w = Watcher::new();
    println!("{:<42} {:<10} {}", "what you did", "tests", "savepoint");
    println!("{}", "-".repeat(78));
    for (what, pass) in session {
        let verdict = if pass { "pass" } else { "FAIL" };
        println!("{:<42} {:<10} {}", what, verdict, w.observe(pass));
    }

    println!("\n{} commits from {} edits.", w.commits, session.len());
    println!("Note what did NOT commit: the second green run, and the fourth and");
    println!("fifth failures. Only the EDGE from red to green saves, so a passing");
    println!("suite you keep re-running does not bury you in commits.");
    println!("\nBoth say \"SAVEPOINT REACHED!\", which is why the next step is squashing.");
    assert_eq!(w.commits, 2);
}
