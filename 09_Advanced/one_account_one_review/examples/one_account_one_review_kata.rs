//! Kata solution — the hole the compiler could not see.
//!
//! In the lesson's design B, `post` consumes the token, so no token is ever
//! spent twice. Ada still posted twice, because she signed in twice and was
//! handed a *second* token. Move semantics govern one value's lifetime; they
//! have nothing to say about how many values a constructor hands out.
//!
//! The fix is in `sign_in`, and it is bookkeeping rather than types: the roll
//! holds one unspent entitlement per account, and signing in takes it. The last
//! two lines of output are the price of doing it this way.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AccountId(u32);

/// A 0-5 score for each of three products.
#[derive(Debug, Clone, Copy)]
struct Review([u8; 3]);

impl Review {
    fn total(&self) -> u32 {
        self.0.iter().map(|&s| s as u32).sum()
    }
}

mod moved_token {
    use super::{Review, AccountId};

    /// The register of who may still post. One entry per account, removed on
    /// sign-in — so the roll shrinks as the batch runs.
    pub struct Roll {
        unspent: Vec<AccountId>,
    }

    impl Roll {
        pub fn new(ids: &[AccountId]) -> Roll {
            Roll { unspent: ids.to_vec() }
        }

        pub fn remaining(&self) -> usize {
            self.unspent.len()
        }
    }

    pub struct Eligible {
        id: AccountId,
    }

    /// `&mut Roll`, not `&[AccountId]`. Signing in now *changes* the roll, and
    /// the `?` on `position` is the whole of the second-token guard.
    pub fn sign_in(id: AccountId, roll: &mut Roll, password_ok: bool) -> Option<Eligible> {
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
        pub fn id(&self) -> AccountId {
            self.id
        }

        pub fn post(self, review: Review, posted: &mut Vec<Review>) -> Receipt {
            posted.push(review);
            Receipt { serial: posted.len() }
        }
    }
}

fn main() {
    use moved_token::{Roll, sign_in};

    let mut roll = Roll::new(&[AccountId(1), AccountId(2)]);
    let mut posted: Vec<Review> = Vec::new();

    println!("Roll opens with {} unspent entitlements.\n", roll.remaining());

    // Ada posts.
    if let Some(token) = sign_in(AccountId(1), &mut roll, true) {
        println!("  {:?} signed in", token.id());
        let receipt = token.post(Review([5, 2, 0]), &mut posted);
        println!("      review accepted, receipt serial {}", receipt.serial);
    }

    // Ada tries again with the right password. This is the line the lesson's
    // design B let through.
    match sign_in(AccountId(1), &mut roll, true) {
        Some(_) => println!("  AccountId(1) signed in a second time"),
        None => println!("  AccountId(1) refused  (entitlement already spent)"),
    }

    // Ben signs in, then his connection dies before he submits.
    if let Some(token) = sign_in(AccountId(2), &mut roll, true) {
        println!("  {:?} signed in", token.id());
        drop(token); // the browser closed; no review was ever post
        println!("      token dropped without posting");
    }

    // Ben comes back.
    match sign_in(AccountId(2), &mut roll, true) {
        Some(_) => println!("  AccountId(2) signed in again"),
        None => println!("  AccountId(2) refused  (entitlement already spent)"),
    }

    let points: u32 = posted.iter().map(Review::total).sum();
    println!(
        "\n  -> reviews in the box: {} (2 eligible accounts, {points} points post)",
        posted.len()
    );
    println!("  -> entitlements left on the roll: {}", roll.remaining());
    println!("  -> nobody posted twice, and Ben did not post at all");
}
