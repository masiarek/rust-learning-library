//! Two enums and one `match` on the pair of them: a transition table the
//! compiler audits for holes.

#[derive(Debug, Clone, Copy, PartialEq)]
enum State { Blank, Marked, Cast }

#[derive(Debug, Clone, Copy)]
enum Event { Mark, Submit, Void }

/// Every cell of the 3 x 3 table, written out. No `_` arm anywhere, so the
/// compiler knows whether the table is complete — and says so if it is not.
fn step(state: State, event: Event) -> State {
    use Event::*;
    use State::*;
    match (state, event) {
        (Blank, Mark) => Marked,
        (Blank, Submit) => Blank, // nothing on it yet; submitting is a no-op
        (Blank, Void) => Blank,

        (Marked, Mark) => Marked, // changing your mind is allowed
        (Marked, Submit) => Cast,
        (Marked, Void) => Blank,

        // Once cast, a ballot is final. Spelled out rather than `(Cast, _)`,
        // so that a new event has to be considered here too.
        (Cast, Mark) | (Cast, Submit) | (Cast, Void) => Cast,
    }
}

fn main() {
    let script = [
        Event::Submit, // too early
        Event::Mark,
        Event::Void, // changed my mind
        Event::Mark,
        Event::Submit, // now it counts
        Event::Void,   // too late
    ];

    let mut state = State::Blank;
    println!("{:<8} {:<8} {}", "from", "event", "to");
    println!("{}", "-".repeat(28));
    for event in script {
        let next = step(state, event);
        let note = if next == state { "no change" } else { "" };
        let line = format!("{:<8} {:<8} {:<8} {}", format!("{state:?}"), format!("{event:?}"), format!("{next:?}"), note);
        println!("{}", line.trim_end());
        state = next;
    }
    println!("\nfinal state: {state:?}");
}
