// One value, one owner. Whoever owns it at the end of a scope frees it, and
// a move hands that job over rather than copying it.

struct Ballot {
    voter: &'static str,
}

impl Drop for Ballot {
    fn drop(&mut self) {
        println!("freeing {}", self.voter);
    }
}

fn file(ballot: Ballot) {
    println!("filing {}", ballot.voter);
}                                               // the parameter owns it: freed here

fn main() {
    let ada = Ballot { voter: "Ada" };
    file(ada);                                  // ownership moves in

    println!("main is done");                   // nothing left in main to free

    // Using `ada` here is E0382 "borrow of moved value" — so there is no way
    // to reach a second free.
}
