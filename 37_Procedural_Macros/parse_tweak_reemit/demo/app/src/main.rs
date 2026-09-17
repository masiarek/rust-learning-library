use std::pin::pin;
use std::task::{Context, Poll, Waker};

use trace_app::{double, largest, port};

fn main() {
    let ok = port("localhost:8080");
    println!("= {ok:?}\n");
    let early = port("localhost");
    println!("= {early:?}\n");
    let question_mark = port("localhost:http");
    println!("= {question_mark:?}\n");
    let max = largest(&[3, 7, 5]);
    println!("= {max:?}\n");

    let future = double(21);
    println!("double(21) called, nothing printed yet");
    // `double` never waits, so one poll with a waker that does nothing finishes it.
    let mut context = Context::from_waker(Waker::noop());
    if let Poll::Ready(n) = pin!(future).poll(&mut context) {
        println!("= {n}");
    }
}
