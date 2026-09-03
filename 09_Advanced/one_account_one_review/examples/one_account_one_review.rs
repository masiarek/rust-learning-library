//! One account, one review — the rule as a pair of booleans, then as a value.
//!
//! Design A keeps "signed up" and "has posted" beside the account and trusts each
//! caller to read them. Design B hands the account a token that posting consumes,
//! so a second review from the same token is a compile error rather than a
//! review comment. Both designs run against the same four sign-in attempts, and
//! both end up with a review too many — for reasons that are not equally easy
//! to find.

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

fn report(design: &str, posted: &[Review], eligible: usize) {
    let post = posted.len();
    let scores: u32 = posted.iter().map(Review::total).sum();
    println!(
        "  -> {post} reviews in the box from {eligible} eligible accounts, {scores} points post"
    );
    if post > eligible {
        println!("  -> {design}: MORE REVIEWS THAN ACCOUNTS");
    }
    println!();
}

// ------------------------------------------------------------------ Design A

mod boolean_guard {
    use super::{Review, AccountId};

    pub struct Account {
        pub id: AccountId,
        pub signed_up: bool,
        pub has_posted: bool,
    }

    /// Nothing in this signature obliges the caller to have checked anything.
    /// `signed_up` and `has_posted` are two facts sitting next to the review,
    /// and reading them before calling is a convention, not a requirement.
    pub fn post(account: &mut Account, review: Review, posted: &mut Vec<Review>) {
        account.has_posted = true;
        posted.push(review);
    }
}

fn design_a() {
    use boolean_guard::{Account, post};

    println!("Design A — the rule lives in whoever calls `post`:");

    let mut ada = Account { id: AccountId(1), signed_up: true, has_posted: false };
    let mut ben = Account { id: AccountId(2), signed_up: true, has_posted: false };
    let mut cara = Account { id: AccountId(3), signed_up: false, has_posted: false };

    let mut posted: Vec<Review> = Vec::new();

    // The endpoint someone wrote first. It reads both facts.
    for account in [&mut ada, &mut ben] {
        if account.signed_up && !account.has_posted {
            println!("  {:?} accepted     (checked endpoint)", account.id);
            post(account, Review([5, 2, 0]), &mut posted);
        }
    }

    // "Let me fix my review" — added later, by someone else. It remembered one
    // of the two checks. It compiles exactly as well as the endpoint above.
    for account in [&mut ada, &mut cara] {
        if account.signed_up {
            println!("  {:?} accepted     (resubmit endpoint)", account.id);
            post(account, Review([0, 0, 5]), &mut posted);
        } else {
            println!("  {:?} refused      (not signed up)", account.id);
        }
    }

    report("design A", &posted, 2);
}

// ------------------------------------------------------------------ Design B

mod moved_token {
    use super::{Review, AccountId};

    /// Proof that this account is on the roll and got their password right.
    ///
    /// The field is private and this module offers exactly one constructor, so
    /// nobody outside can build an `Eligible` without going through the check.
    /// That, and not the name, is the whole of the guarantee.
    pub struct Eligible {
        id: AccountId,
    }

    /// The one door in. Returning `Option` means the failure has to be handled;
    /// there is no `Eligible` to be had on the `None` path.
    pub fn sign_in(id: AccountId, roll: &[AccountId], password_ok: bool) -> Option<Eligible> {
        if password_ok && roll.contains(&id) {
            Some(Eligible { id })
        } else {
            None
        }
    }

    /// What the account walks away with. Note what is *not* in it: no `AccountId`.
    #[derive(Debug)]
    pub struct Receipt {
        pub serial: usize,
    }

    impl Eligible {
        pub fn id(&self) -> AccountId {
            self.id
        }

        /// `self`, not `&self`. Posting a review consumes the right to post one.
        pub fn post(self, review: Review, posted: &mut Vec<Review>) -> Receipt {
            posted.push(review);
            Receipt { serial: posted.len() }
            // `self` is dropped here, and the account id inside it with it. The
            // review went into the box; the identity did not follow it.
        }
    }
}

fn design_b() {
    use moved_token::sign_in;

    println!("Design B — the right to post is a value, and posting spends it:");

    let roll = [AccountId(1), AccountId(2)];
    let mut posted: Vec<Review> = Vec::new();

    let attempts = [
        (AccountId(1), true),  // Ada, correct password
        (AccountId(2), true),  // Ben, correct password
        (AccountId(3), true),  // Cara, never signed up
        (AccountId(1), false), // Ada again, wrong password
        (AccountId(1), true),  // Ada again, correct password
    ];

    for (id, password_ok) in attempts {
        match sign_in(id, &roll, password_ok) {
            Some(token) => {
                println!("  {:?} signed in", token.id());
                let receipt = token.post(Review([5, 2, 0]), &mut posted);
                println!("      review accepted, receipt serial {}", receipt.serial);

                // A second review from the same token does not compile:
                //     token.post(Review([0, 0, 5]), &mut posted);
                //     ^^^^^ value used here after move
            }
            None => println!("  {id:?} refused at sign-in"),
        }
    }

    report("design B", &posted, 2);
}

fn main() {
    design_a();
    design_b();
    println!("Both boxes hold three reviews. Design A can be fixed at every call");
    println!("site that forgot a check; design B has exactly one function to fix.");
}
