// The same Arc<Mutex<T>>, used correctly, and the account still goes overdrawn.
//
// One account holding 100. Two clerks, each told to withdraw 100 only if the
// money is there. Every access to the balance is under the lock, so there is no
// data race here and ThreadSanitizer has nothing to report — and both
// withdrawals go through anyway, because the lock is released between the
// question and the answer.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn withdraw_twice() -> i64 {
    let balance = Arc::new(Mutex::new(100i64));

    let clerks: Vec<_> = (0..2)
        .map(|_| {
            let balance = Arc::clone(&balance);
            thread::spawn(move || {
                // CHECK — take the lock, read, hand it straight back.
                let affordable = { *balance.lock().unwrap() >= 100 };

                thread::sleep(Duration::from_millis(50));   // holding nothing

                // ACT — take the lock again, on a fact that has since expired.
                if affordable {
                    *balance.lock().unwrap() -= 100;
                }
            })
        })
        .collect();

    for c in clerks {
        c.join().unwrap();
    }

    *balance.lock().unwrap()
}

fn main() {
    for run in 1..=3 {
        println!("run {run} -> {}", withdraw_twice());   // -100, every time
    }
}
