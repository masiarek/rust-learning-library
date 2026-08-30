//! A channel moves values between threads, and ownership goes with them.
//!
//!   rustc --edition 2024 channels.rs -o /tmp/chan && /tmp/chan

use std::sync::mpsc;
use std::thread;

fn main() {
    println!("1. One sender, one receiver, one value");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(String::from("counted")).unwrap();
    });
    let got: String = rx.recv().unwrap();
    println!("   rx.recv() = {got:?}");
    println!("   `send` takes the value BY VALUE. The String was built on the");
    println!("   spawned thread and is owned by main now — one move, no copy, and");
    println!("   the sending thread cannot touch it again because it no longer has");
    println!("   it. That is the channel's whole safety argument.");

    println!();
    println!("2. Many senders: clone the Sender, not the Receiver");
    let (tx, rx) = mpsc::channel();
    for name in ["Ada", "Ben", "Cara"] {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send((name, name.len() as u32)).unwrap();
        });
    }
    drop(tx); // <- the original, or the loop below never ends
    let mut rows: Vec<(&str, u32)> = rx.iter().collect();
    rows.sort();
    println!("   {rows:?}");
    println!("   mpsc is Multi-Producer, Single-Consumer: Sender is Clone and");
    println!("   Receiver is not. Note the sort — the arrival ORDER is whatever the");
    println!("   scheduler decides, so anything comparing output has to impose one.");

    println!();
    println!("3. The hang, and the one line that prevents it");
    println!("   `for x in rx` ends when every Sender has been dropped. The clones");
    println!("   inside the threads drop when those threads finish; the ORIGINAL tx");
    println!("   in main does not, unless you drop it. Forget that `drop(tx)` and");
    println!("   the loop above waits forever, on a program that looks finished.");
    println!("   It is the single most common mpsc bug, and it has no error message");
    println!("   at all — just a process that never exits.");

    println!();
    println!("4. `recv` tells you the difference between empty and finished");
    let (tx, rx) = mpsc::channel::<u32>();
    tx.send(5).unwrap();
    drop(tx);
    println!("   first  recv() = {:?}", rx.recv());
    println!("   second recv() = {:?}   <- RecvError: every sender is gone", rx.recv());
    println!("   `recv` BLOCKS until a value arrives or the channel closes;");
    println!("   `try_recv` returns immediately with Empty or Disconnected. Those");
    println!("   are two different questions and the types keep them apart.");

    println!();
    println!("5. And the send can fail too");
    let (tx, rx) = mpsc::channel::<u32>();
    drop(rx);
    println!("   send() with no receiver = {:?}", tx.send(9).is_err());
    println!("   The value comes back inside the SendError, so nothing is lost —");
    println!("   which is the only sensible design for a function that took");
    println!("   ownership and could not deliver.");

    println!();
    println!("6. Which tool for which problem");
    println!("   channel        hand a value OVER: a pipeline, a work queue, a");
    println!("                  result collected from a fan-out");
    println!("   Arc<Mutex<T>>  share ONE value: a counter, a cache, a registry");
    println!("   The question is whether the data has an owner at each moment or");
    println!("   is genuinely shared. Reaching for a Mutex when a channel would do");
    println!("   is how a program acquires a lock it then has to reason about.");
}
