//! Kata solution — the hole the compiler could not see.
//!
//! In the lesson's design B, `cast` consumes the token, so no token is ever
//! spent twice. Ada still voted twice, because she signed in twice and was
//! handed a *second* token. Move semantics govern one value's lifetime; they
//! have nothing to say about how many values a constructor hands out.
//!
//! The fix is in `sign_in`, and it is bookkeeping rather than types: the roll
//! holds one unspent entitlement per voter, and signing in takes it. The last
//! two lines of output are the price of doing it this way.

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

mod moved_token {
    use super::{Ballot, VoterId};

    /// The register of who may still vote. One entry per voter, removed on
    /// sign-in — so the roll shrinks as the election runs.
    pub struct Roll {
        unspent: Vec<VoterId>,
    }

    impl Roll {
        pub fn new(ids: &[VoterId]) -> Roll {
            Roll { unspent: ids.to_vec() }
        }

        pub fn remaining(&self) -> usize {
            self.unspent.len()
        }
    }

    pub struct Eligible {
        id: VoterId,
    }

    /// `&mut Roll`, not `&[VoterId]`. Signing in now *changes* the roll, and
    /// the `?` on `position` is the whole of the second-token guard.
    pub fn sign_in(id: VoterId, roll: &mut Roll, password_ok: bool) -> Option<Eligible> {
        if !password_ok {
            return None;
        }
        let pos = roll.unspent.iter().position(|&v| v == id)?;
        roll.unspent.swap_remove(pos);
        Some(Eligible { id })
    }

    #[derive(Debug)]
    pub struct Receipt {
        pub serial: usize,
    }

    impl Eligible {
        pub fn id(&self) -> VoterId {
            self.id
        }

        pub fn cast(self, ballot: Ballot, ballot_box: &mut Vec<Ballot>) -> Receipt {
            ballot_box.push(ballot);
            Receipt { serial: ballot_box.len() }
        }
    }
}

fn main() {
    use moved_token::{Roll, sign_in};

    let mut roll = Roll::new(&[VoterId(1), VoterId(2)]);
    let mut ballot_box: Vec<Ballot> = Vec::new();

    println!("Roll opens with {} unspent entitlements.\n", roll.remaining());

    // Ada votes.
    if let Some(token) = sign_in(VoterId(1), &mut roll, true) {
        println!("  {:?} signed in", token.id());
        let receipt = token.cast(Ballot([5, 2, 0]), &mut ballot_box);
        println!("      ballot accepted, receipt serial {}", receipt.serial);
    }

    // Ada tries again with the right password. This is the line the lesson's
    // design B let through.
    match sign_in(VoterId(1), &mut roll, true) {
        Some(_) => println!("  VoterId(1) signed in a second time"),
        None => println!("  VoterId(1) refused  (entitlement already spent)"),
    }

    // Ben signs in, then his connection dies before he submits.
    if let Some(token) = sign_in(VoterId(2), &mut roll, true) {
        println!("  {:?} signed in", token.id());
        drop(token); // the browser closed; no ballot was ever cast
        println!("      token dropped without casting");
    }

    // Ben comes back.
    match sign_in(VoterId(2), &mut roll, true) {
        Some(_) => println!("  VoterId(2) signed in again"),
        None => println!("  VoterId(2) refused  (entitlement already spent)"),
    }

    let points: u32 = ballot_box.iter().map(Ballot::total).sum();
    println!(
        "\n  -> ballots in the box: {} (2 eligible voters, {points} points cast)",
        ballot_box.len()
    );
    println!("  -> entitlements left on the roll: {}", roll.remaining());
    println!("  -> nobody voted twice, and Ben did not vote at all");
}
