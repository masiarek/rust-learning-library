//! Kata solution: match on a tuple and the compiler writes out the whole table.
//!
//!   rustc --edition 2024 an_enum_as_a_state_machine_kata.rs -o /tmp/esm && /tmp/esm

#[derive(Debug, Clone, Copy, PartialEq)]
enum State { Idle, Loading, Ready, Failed }

#[derive(Debug, Clone, Copy)]
enum Event { Start, Loaded, Error, Reset }

fn next(state: State, event: Event) -> State {
    use Event::*;
    use State::*;
    match (state, event) {
        (Idle, Start) => Loading,
        (Loading, Loaded) => Ready,
        (Loading, Error) => Failed,
        (Failed, Reset) => Idle,
        (Ready, Reset) => Idle,

        // Every remaining cell, written out rather than swept up by `_`. The
        // compiler required them, which is the point: it enumerated four
        // states times four events and would not build until all sixteen were
        // accounted for.
        (Idle, Loaded) | (Idle, Error) | (Idle, Reset) => state,
        (Loading, Start) | (Loading, Reset) => state,
        (Ready, Start) | (Ready, Loaded) | (Ready, Error) => state,
        (Failed, Start) | (Failed, Loaded) | (Failed, Error) => state,
    }
}

fn main() {
    println!("THE TRANSITION TABLE, RUN");
    let mut s = State::Idle;
    for e in [Event::Start, Event::Loaded, Event::Reset, Event::Start, Event::Error] {
        let before = s;
        s = next(s, e);
        println!("  {:<8} + {:<7} -> {:?}", format!("{before:?}"), format!("{e:?}"), s);
    }
    println!();

    println!("THE WHOLE GRID");
    use Event::*;
    use State::*;
    print!("  {:<9}", "");
    for e in [Start, Loaded, Error, Reset] { print!("{:<9}", format!("{e:?}")); }
    println!();
    for st in [Idle, Loading, Ready, Failed] {
        print!("  {:<9}", format!("{st:?}"));
        for e in [Start, Loaded, Error, Reset] {
            print!("{:<9}", format!("{:?}", next(st, e)));
        }
        println!();
    }
    println!();

    println!("WHAT THE COMPILER DID FOR YOU");
    println!("  Matching on the TUPLE (State, Event) makes the match's subject");
    println!("  the cross product, so exhaustiveness checking covers all 4 x 4");
    println!("  = 16 cells. Delete any one arm and E0004 names the exact pair");
    println!("  you left out -- `(Ready, Error)` and so on.");
    println!();
    println!("  Add a fifth state and every incomplete match in the program");
    println!("  fails to build, with the missing cells listed. That is a design");
    println!("  review the compiler performs for free, every time.");
    println!();

    println!("THE ARM THAT THROWS IT AWAY");
    println!("  `_ => state` at the bottom would compile today and would also");
    println!("  silently absorb every cell you add later. On a state machine");
    println!("  that is exactly the code you do NOT want to write: the value of");
    println!("  the enum is that forgetting is a build error.");
    println!();
    println!("  Writing the ignored cells out as `(Idle, Loaded) | ...` keeps");
    println!("  that guarantee AND documents that they were considered -- which");
    println!("  a reader cannot tell from a wildcard.");

    assert_eq!(next(State::Idle, Event::Start), State::Loading);
    assert_eq!(next(State::Ready, Event::Error), State::Ready);
}
