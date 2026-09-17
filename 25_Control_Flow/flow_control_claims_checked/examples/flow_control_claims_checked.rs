//! Claims about Rust's flow control from a book chapter (Rust in Action, §2.4),
//! each one run. The claims the compiler refuses are on the page, not here.
//!
//!   rustc --edition 2024 flow_control_claims_checked.rs -o /tmp/fcc && /tmp/fcc

use std::collections::HashMap;

struct Cleanup(&'static str);

impl Drop for Cleanup {
    fn drop(&mut self) {
        println!("   drop: {}", self.0);
    }
}

fn claim(number: u32, text: &str) {
    println!("\n{number}. {text}");
}

/// Sets up two resources and leaves early when the second step fails.
/// Nothing jumps to a cleanup label: both values are dropped on the way out.
fn open_both(second_fails: bool) -> Result<(), &'static str> {
    let _config = Cleanup("config");
    if second_fails {
        return Err("log could not be opened");
    }
    let _log = Cleanup("log");
    println!("   both open");
    Ok(())
}

/// A `loop` that is left by `return`, not by `break`.
fn first_negative(values: &[i32]) -> Option<usize> {
    let mut i = 0;
    loop {
        if i == values.len() {
            return None;
        }
        if values[i] < 0 {
            return Some(i);
        }
        i += 1;
    }
}

fn main() {
    claim(1, "after `for item in container`, the container's lifetime has ended");
    let array = [1, 2, 3];
    for item in array {
        let _ = item;
    }
    println!("   an array of i32 is Copy, so it is still here: {array:?}");
    let mut v = vec![1, 2, 3];
    for item in v {
        let _ = item;
    }
    v = vec![9];
    println!("   a Vec was moved, but the name v takes a new value: {v:?}");

    claim(2, "`for item in &collection` is `for item in collection.iter()`");
    let v = vec![10, 20, 30];
    let by_ref: Vec<&i32> = (&v).into_iter().collect();
    let by_iter: Vec<&i32> = v.iter().collect();
    println!("   Vec:     IntoIterator for &Vec gives {by_ref:?}, iter() gives {by_iter:?}");
    let ages = HashMap::from([("ada", 36)]);
    let by_ref: Vec<(&&str, &i32)> = (&ages).into_iter().collect();
    let by_iter: Vec<(&&str, &i32)> = ages.iter().collect();
    println!("   HashMap: IntoIterator for &HashMap gives {by_ref:?}, iter() gives {by_iter:?}");
    println!("   same items, because each type's impl calls iter(); &String has no such impl");

    claim(3, "`n..m` is exclusive and `n..=m` is inclusive");
    let mut exclusive = 0;
    for _ in 0..10 {
        exclusive += 1;
    }
    let mut inclusive = 0;
    for _ in 0..=10 {
        inclusive += 1;
    }
    println!("   0..10 ran {exclusive} times, 0..=10 ran {inclusive} times");

    claim(5, "a `for` over the collection guarantees it is not changed during the loop");
    let mut v = vec![1, 2, 3];
    let mut passes = 0;
    for i in 0..v.len() {
        passes += 1;
        if v[i] == 2 {
            v.push(4);
        }
    }
    println!("   the index loop compiled, ran {passes} passes, and never saw the pushed 4: {v:?}");

    claim(6, "`continue` operates as you would expect");
    let mut i = 0;
    let mut passes = 0;
    while i < 3 {
        passes += 1;
        if passes == 10 {
            break; // a guard, so this example ends
        }
        if i == 1 {
            continue; // skips the i += 1 below, every time
        }
        i += 1;
    }
    println!("   in a while loop, a continue before i += 1: i is still {i} after {passes} passes");

    claim(9, "`loop` runs until a `break`, or until the program is stopped from outside");
    println!("   first_negative([4, -1, 7]) = {:?}  (left by return)", first_negative(&[4, -1, 7]));
    println!("   first_negative([4, 1, 7])  = {:?}  (left by return)", first_negative(&[4, 1, 7]));

    claim(10, "`for (x, y) in (0..).zip(0..)` with `break` when x + y > 100");
    for (x, y) in (0..).zip(0..) {
        if x + y > 100 {
            println!("   stopped at x = {x}, y = {y}");
            break;
        }
    }

    claim(11, "`break 'outer` leaves all three loops once x + y + z > 1000");
    let mut visited_x = Vec::new();
    let mut stop = (0, 0, 0);
    'outer: for x in 0.. {
        visited_x.push(x);
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    stop = (x, y, z);
                    break 'outer;
                }
            }
        }
    }
    println!("   stopped at (x, y, z) = {stop:?}; x took the values {visited_x:?}");

    claim(12, "labels work with `continue` too");
    let mut rows = Vec::new();
    'row: for row in 1..=3 {
        for col in 1..=3 {
            if col > row {
                continue 'row;
            }
        }
        rows.push(row);
    }
    println!("   only row 3 finished its inner loop: {rows:?}");

    claim(13, "Rust does not include the `goto` keyword");
    let goto = "not reserved";
    println!("   `let goto = ...` compiles: goto is {goto}");

    claim(14, "use loop labels for the jump-to-cleanup pattern");
    println!("   second step fails:");
    let result = open_both(true);
    println!("   -> {result:?}");
    println!("   both steps succeed:");
    let result = open_both(false);
    println!("   -> {result:?}");

    claim(16, "the `else` block matches anything that has not already been matched");
    let item = 132;
    let checked = std::cell::RefCell::new(Vec::new());
    let is = |n: i32| {
        checked.borrow_mut().push(n);
        item == n
    };
    let label = if is(42) {
        "the answer"
    } else if is(132) {
        "one-three-two"
    } else if is(7) {
        "seven"
    } else {
        "anything else"
    };
    println!("   {label}; conditions evaluated: {:?}", checked.borrow());

    claim(17, "assigning with `=` is a statement, so it has no value");
    let mut total = 1;
    let before = total;
    let assigned = total = 5;
    println!("   `let assigned = total = 5;` compiles: assigned is {assigned:?}, total went {before} -> {total}");

    claim(18, "the `break` keyword also returns a value");
    let n = loop {
        break 123;
    };
    println!("   let n = loop {{ break 123; }};  n is {n}");

    claim(20, "Listing 2.8: `42 | 132` matches both, `_` matches the rest");
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };
        if result == "hit!" {
            println!("   {item}: {result}");
        }
    }
}
