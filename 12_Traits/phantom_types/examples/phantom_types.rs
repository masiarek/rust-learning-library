//! A type parameter the struct never stores — checked at compile time, free at run time.
//!
//!   rustc --edition 2024 phantom_types.rs -o /tmp/pt && /tmp/pt

use std::marker::PhantomData;
use std::mem::size_of;

// The tags. Uninhabited, so nobody can make one by accident: they exist to be
// *named*, never to be constructed. `enum Star {}` has no variants, so there
// is no expression anywhere that produces a `Star`.
enum Star {}
enum Approval {}

// One struct. `Method` appears nowhere in the data — `_method` is a zero-sized
// field whose only job is to make the parameter real to the type system.
struct Ballot<Method> {
    scores: Vec<u8>,
    _method: PhantomData<Method>,
}

// Shared behaviour: written once, for every method.
impl<Method> Ballot<Method> {
    fn new(scores: Vec<u8>) -> Self {
        Ballot { scores, _method: PhantomData }
    }

    fn total(&self) -> u32 {
        self.scores.iter().map(|&s| u32::from(s)).sum()
    }
}

// Concrete specialization: this block is `Ballot<Star>`, not `Ballot<Method>`.
// `max_score` exists on a STAR ballot and nowhere else.
impl Ballot<Star> {
    fn max_score(&self) -> u8 {
        5
    }

    fn method_name(&self) -> &str {
        "STAR"
    }
}

impl Ballot<Approval> {
    fn max_score(&self) -> u8 {
        1
    }

    fn method_name(&self) -> &str {
        "Approval"
    }
}

// A function that accepts ONE tag. An Approval ballot cannot reach it, and the
// refusal is a type error at the call site rather than a check in the body.
fn star_runoff_pair(ballot: &Ballot<Star>) -> (u8, u8) {
    let mut sorted = ballot.scores.clone();
    sorted.sort_unstable();
    sorted.reverse();
    (sorted[0], sorted[1])
}

// Typestate: the tag records where the value is in its life, and a transition
// CONSUMES the old value, so a stale handle cannot be used again.
struct Blank;
struct Marked;

struct Paper<State> {
    scores: Vec<u8>,
    _state: PhantomData<State>,
}

impl Paper<Blank> {
    fn issue() -> Self {
        Paper { scores: Vec::new(), _state: PhantomData }
    }

    fn mark(self, scores: Vec<u8>) -> Paper<Marked> {
        Paper { scores, _state: PhantomData }
    }
}

impl Paper<Marked> {
    fn cast(self) -> Vec<u8> {
        self.scores
    }
}

fn main() {
    let star: Ballot<Star> = Ballot::new(vec![5, 3, 0]);
    let approval: Ballot<Approval> = Ballot::new(vec![1, 0, 1]);

    println!("1. One struct, two types");
    println!("   Ballot<Star>     total {}   max {}", star.total(), star.max_score());
    println!("   Ballot<Approval> total {}   max {}", approval.total(), approval.max_score());
    println!("   `star_runoff_pair(&approval)` is E0308: expected `Ballot<Star>`,");
    println!("   found `Ballot<Approval>`. The two share every byte and no type.");

    println!();
    println!("2. The specialization is concrete, not generic");
    println!("   impl Ballot<Star> {{ .. }}     method_name = {}", star.method_name());
    println!("   impl Ballot<Approval> {{ .. }} method_name = {}", approval.method_name());
    println!("   Neither name is stored anywhere: `Ballot` has one field, a Vec.");
    println!("   runoff pair of the STAR ballot = {:?}", star_runoff_pair(&star));

    println!();
    println!("3. The tag costs nothing");
    println!("   size_of::<Vec<u8>>()             = {}", size_of::<Vec<u8>>());
    println!("   size_of::<Ballot<Star>>()        = {}", size_of::<Ballot<Star>>());
    println!("   size_of::<Ballot<Approval>>()    = {}", size_of::<Ballot<Approval>>());
    println!("   size_of::<PhantomData<Star>>()   = {}", size_of::<PhantomData<Star>>());

    println!();
    println!("4. What the phantom field CLAIMS — all three are zero-sized");
    println!("   PhantomData<Vec<u8>>          = {}   owns a Vec<u8>", size_of::<PhantomData<Vec<u8>>>());
    println!("   PhantomData<fn() -> Vec<u8>>  = {}   merely produces one", size_of::<PhantomData<fn() -> Vec<u8>>>());
    println!("   PhantomData<*const Vec<u8>>   = {}   only points at one", size_of::<PhantomData<*const Vec<u8>>>());
    println!("   Same size, three different promises about variance and drop.");

    println!();
    println!("5. Typestate: the tag moves with the value");
    let blank = Paper::<Blank>::issue();
    let marked = blank.mark(vec![5, 2, 0]);
    let cast = marked.cast();
    println!("   issue() -> mark() -> cast() = {cast:?}");
    println!("   `blank.mark(..)` took `self`, so `blank` is moved-from: using it");
    println!("   again is E0382. And `Paper<Blank>` has no `cast` at all — an");
    println!("   unmarked paper cannot reach the ballot box, by construction.");
}
