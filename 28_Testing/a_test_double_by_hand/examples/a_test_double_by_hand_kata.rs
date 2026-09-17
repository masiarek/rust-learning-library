//! Kata solution: the clock you cannot wait for.
//!
//!   rustc --edition 2024 a_test_double_by_hand_kata.rs -o /tmp/atdbhk && /tmp/atdbhk
//!   rustc --edition 2024 --test a_test_double_by_hand_kata.rs -o /tmp/atdbhkt && /tmp/atdbhkt

use std::time::{SystemTime, UNIX_EPOCH};

const WEEK: u64 = 7 * 24 * 60 * 60;

/// The one thing the coupon code needs to know about time.
trait Clock {
    fn now(&self) -> u64; // seconds since the Unix epoch
}

/// Production: asks the operating system.
struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("after 1970").as_secs()
    }
}

/// Tests: says whatever it was told to.
struct FixedClock(u64);

impl Clock for FixedClock {
    fn now(&self) -> u64 {
        self.0
    }
}

/// Valid for 7 days after issue. The boundary second itself is the last valid
/// one: "valid for 7 days" includes the 7-day mark, and this is where that
/// reading is decided.
fn is_valid(issued_at: u64, clock: &impl Clock) -> bool {
    clock.now().saturating_sub(issued_at) <= WEEK
}

const ISSUED: u64 = 1_788_000_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_one_second_before_the_week_is_up() {
        assert!(is_valid(ISSUED, &FixedClock(ISSUED + WEEK - 1)));
    }

    #[test]
    fn valid_on_the_last_second() {
        assert!(is_valid(ISSUED, &FixedClock(ISSUED + WEEK)));
    }

    #[test]
    fn expired_one_second_later() {
        assert!(!is_valid(ISSUED, &FixedClock(ISSUED + WEEK + 1)));
    }

    #[test]
    fn a_clock_behind_the_issue_time_does_not_underflow() {
        assert!(is_valid(ISSUED, &FixedClock(ISSUED - 60)));
    }
}

fn main() {
    println!("1. The boundary, tested without waiting a week");
    for (label, offset) in [("issued + 7 days - 1s", -1i64), ("issued + 7 days     ", 0), ("issued + 7 days + 1s", 1)] {
        let now = (ISSUED + WEEK).checked_add_signed(offset).expect("in range");
        println!("   {label}  is_valid = {}", is_valid(ISSUED, &FixedClock(now)));
    }

    println!();
    println!("2. A clock that runs behind the issue time");
    println!("   issued - 60s          is_valid = {}", is_valid(ISSUED, &FixedClock(ISSUED - 60)));
    println!("   now - issued would underflow a u64; saturating_sub makes it 0.");
    println!("   Only a fixed clock can put now before the issue time on purpose.");

    println!();
    println!("3. The real clock still works in production code");
    let issued_long_ago = 1_000_000_000; // September 2001
    println!("   issued September 2001, checked on SystemClock: is_valid = {}", is_valid(issued_long_ago, &SystemClock));
    println!("   SystemClock and FixedClock are both `impl Clock`; is_valid never knows which.");

    println!();
    println!("4. Why the test is where the boundary is decided");
    println!("   \"Valid for 7 days\" does not say whether the 7-day second counts.");
    println!("   The code picks one with <= or <. valid_on_the_last_second is the");
    println!("   line that says which, and it fails if someone changes their mind silently.");
}
