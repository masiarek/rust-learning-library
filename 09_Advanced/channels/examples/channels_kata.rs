//! Kata solution: a three-stage pipeline, and the drop that ends it.
//!
//!   rustc --edition 2024 channels_kata.rs -o /tmp/chk && /tmp/chk

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Raw ballot lines, two of which are not ballots.
const LINES: [&str; 7] = ["5,3,0", "4,4,1", "x,2,2", "0,5,2", "9,1,1", "3,3,3", "2,4,4"];

fn parse(line: &str) -> Option<[u32; 3]> {
    let mut out = [0u32; 3];
    let mut cells = line.split(',');
    for slot in &mut out {
        let n: u32 = cells.next()?.trim().parse().ok()?;
        if n > 5 {
            return None;
        }
        *slot = n;
    }
    if cells.next().is_some() { None } else { Some(out) }
}

fn main() {
    println!("1. Stage one: parse on a worker, collect in main");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for line in LINES {
            tx.send((line, parse(line))).unwrap();
        }
        // tx is dropped here, when the closure ends. That is what ends the
        // `for` loop below.
    });
    let mut seen: Vec<(&str, Option<[u32; 3]>)> = rx.iter().collect();
    seen.sort();
    for (line, parsed) in &seen {
        println!("   {line:8} -> {parsed:?}");
    }

    println!();
    println!("2. Two stages, wired together");
    let (raw_tx, raw_rx) = mpsc::channel::<&str>();
    let (good_tx, good_rx) = mpsc::channel::<[u32; 3]>();
    let reader = thread::spawn(move || {
        for line in LINES {
            raw_tx.send(line).unwrap();
        }
    });
    let parser = thread::spawn(move || {
        let mut rejected = 0;
        for line in raw_rx {
            match parse(line) {
                Some(b) => good_tx.send(b).unwrap(),
                None => rejected += 1,
            }
        }
        rejected
    });
    let totals = good_rx.iter().fold([0u32; 3], |mut acc, b| {
        for i in 0..3 {
            acc[i] += b[i];
        }
        acc
    });
    reader.join().unwrap();
    let rejected = parser.join().unwrap();
    println!("   totals   = {totals:?}");
    println!("   rejected = {rejected}");
    println!("   Each stage owns its Sender and drops it by finishing, which closes");
    println!("   the next stage's loop in turn. The pipeline shuts down from the");
    println!("   FRONT, one stage at a time, with nothing to co-ordinate.");

    println!();
    println!("3. The hang, demonstrated without hanging");
    let (tx, rx) = mpsc::channel::<u32>();
    let keep = tx.clone();
    thread::spawn(move || {
        tx.send(1).unwrap();
    })
    .join()
    .unwrap();
    println!("   one value sent, the worker's Sender dropped, and a clone kept:");
    println!("   recv()               = {:?}", rx.recv());
    println!("   recv_timeout(50ms)   = {:?}", rx.recv_timeout(Duration::from_millis(50)));
    println!("   That Timeout is what `for x in rx` would have waited on FOREVER,");
    println!("   because one Sender is still alive in this scope. Drop it:");
    drop(keep);
    println!("   after drop(keep), recv() = {:?}", rx.recv());
    println!("   Disconnected is a different answer from Empty, and only the second");
    println!("   one is a reason to stop.");

    println!();
    println!("4. The rule, in one line");
    println!("   A receiver's loop ends when the LAST Sender drops. So a Sender you");
    println!("   keep \"just in case\" is a Sender that never lets the program exit,");
    println!("   and the symptom is a process that finishes its work and hangs.");
    println!("   Either drop it explicitly, or never bind it — pass clones and let");
    println!("   the original go out of scope with the block that made it.");

    println!();
    println!("5. Bounded channels, and what the bound is for");
    let (tx, rx) = mpsc::sync_channel::<u32>(2);
    println!("   sync_channel(2): try_send #1 {:?}", tx.try_send(1).is_ok());
    println!("                    try_send #2 {:?}", tx.try_send(2).is_ok());
    println!("                    try_send #3 {:?}   <- Full", tx.try_send(3).is_ok());
    println!("   drained: {:?} {:?}", rx.recv().unwrap(), rx.recv().unwrap());
    println!("   An unbounded channel with a fast producer and a slow consumer is a");
    println!("   memory leak with good manners. A bound turns that into");
    println!("   backpressure: `send` blocks, and the producer slows to the");
    println!("   consumer's speed instead of buffering the difference.");
}
