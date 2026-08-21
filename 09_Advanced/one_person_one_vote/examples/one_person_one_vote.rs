//! One person, one vote — the rule as a pair of booleans, then as a value.
//!
//! Design A keeps "signed up" and "has voted" beside the voter and trusts each
//! caller to read them. Design B hands the voter a token that casting consumes,
//! so a second ballot from the same token is a compile error rather than a
//! review comment. Both designs run against the same four sign-in attempts, and
//! both end up with a ballot too many — for reasons that are not equally easy
//! to find.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VoterId(u32);

/// A 0-5 score for each of three candidates.
#[derive(Debug, Clone, Copy)]
struct Ballot([u8; 3]);

impl Ballot {
    fn total(&self) -> u32 {
        self.0.iter().map(|&s| s as u32).sum()
    }
}

fn report(design: &str, ballot_box: &[Ballot], eligible: usize) {
    let cast = ballot_box.len();
    let scores: u32 = ballot_box.iter().map(Ballot::total).sum();
    println!(
        "  -> {cast} ballots in the box from {eligible} eligible voters, {scores} points cast"
    );
    if cast > eligible {
        println!("  -> {design}: MORE BALLOTS THAN VOTERS");
    }
    println!();
}

// ------------------------------------------------------------------ Design A

mod boolean_guard {
    use super::{Ballot, VoterId};

    pub struct Voter {
        pub id: VoterId,
        pub signed_up: bool,
        pub has_voted: bool,
    }

    /// Nothing in this signature obliges the caller to have checked anything.
    /// `signed_up` and `has_voted` are two facts sitting next to the ballot,
    /// and reading them before calling is a convention, not a requirement.
    pub fn cast(voter: &mut Voter, ballot: Ballot, ballot_box: &mut Vec<Ballot>) {
        voter.has_voted = true;
        ballot_box.push(ballot);
    }
}

fn design_a() {
    use boolean_guard::{Voter, cast};

    println!("Design A — the rule lives in whoever calls `cast`:");

    let mut ada = Voter { id: VoterId(1), signed_up: true, has_voted: false };
    let mut ben = Voter { id: VoterId(2), signed_up: true, has_voted: false };
    let mut cara = Voter { id: VoterId(3), signed_up: false, has_voted: false };

    let mut ballot_box: Vec<Ballot> = Vec::new();

    // The endpoint someone wrote first. It reads both facts.
    for voter in [&mut ada, &mut ben] {
        if voter.signed_up && !voter.has_voted {
            println!("  {:?} accepted     (checked endpoint)", voter.id);
            cast(voter, Ballot([5, 2, 0]), &mut ballot_box);
        }
    }

    // "Let me fix my ballot" — added later, by someone else. It remembered one
    // of the two checks. It compiles exactly as well as the endpoint above.
    for voter in [&mut ada, &mut cara] {
        if voter.signed_up {
            println!("  {:?} accepted     (resubmit endpoint)", voter.id);
            cast(voter, Ballot([0, 0, 5]), &mut ballot_box);
        } else {
            println!("  {:?} refused      (not signed up)", voter.id);
        }
    }

    report("design A", &ballot_box, 2);
}

// ------------------------------------------------------------------ Design B

mod moved_token {
    use super::{Ballot, VoterId};

    /// Proof that this voter is on the roll and got their password right.
    ///
    /// The field is private and this module offers exactly one constructor, so
    /// nobody outside can build an `Eligible` without going through the check.
    /// That, and not the name, is the whole of the guarantee.
    pub struct Eligible {
        id: VoterId,
    }

    /// The one door in. Returning `Option` means the failure has to be handled;
    /// there is no `Eligible` to be had on the `None` path.
    pub fn sign_in(id: VoterId, roll: &[VoterId], password_ok: bool) -> Option<Eligible> {
        if password_ok && roll.contains(&id) {
            Some(Eligible { id })
        } else {
            None
        }
    }

    /// What the voter walks away with. Note what is *not* in it: no `VoterId`.
    #[derive(Debug)]
    pub struct Receipt {
        pub serial: usize,
    }

    impl Eligible {
        pub fn id(&self) -> VoterId {
            self.id
        }

        /// `self`, not `&self`. Casting a ballot consumes the right to cast one.
        pub fn cast(self, ballot: Ballot, ballot_box: &mut Vec<Ballot>) -> Receipt {
            ballot_box.push(ballot);
            Receipt { serial: ballot_box.len() }
            // `self` is dropped here, and the voter id inside it with it. The
            // ballot went into the box; the identity did not follow it.
        }
    }
}

fn design_b() {
    use moved_token::sign_in;

    println!("Design B — the right to vote is a value, and casting spends it:");

    let roll = [VoterId(1), VoterId(2)];
    let mut ballot_box: Vec<Ballot> = Vec::new();

    let attempts = [
        (VoterId(1), true),  // Ada, correct password
        (VoterId(2), true),  // Ben, correct password
        (VoterId(3), true),  // Cara, never signed up
        (VoterId(1), false), // Ada again, wrong password
        (VoterId(1), true),  // Ada again, correct password
    ];

    for (id, password_ok) in attempts {
        match sign_in(id, &roll, password_ok) {
            Some(token) => {
                println!("  {:?} signed in", token.id());
                let receipt = token.cast(Ballot([5, 2, 0]), &mut ballot_box);
                println!("      ballot accepted, receipt serial {}", receipt.serial);

                // A second ballot from the same token does not compile:
                //     token.cast(Ballot([0, 0, 5]), &mut ballot_box);
                //     ^^^^^ value used here after move
            }
            None => println!("  {id:?} refused at sign-in"),
        }
    }

    report("design B", &ballot_box, 2);
}

fn main() {
    design_a();
    design_b();
    println!("Both boxes hold three ballots. Design A can be fixed at every call");
    println!("site that forgot a check; design B has exactly one function to fix.");
}
