//! `loop`: no condition, a value carried out by `break`, and every other way
//! control can leave it.
//!
//!   rustc --edition 2024 the_loop_keyword.rs -o /tmp/tlk && /tmp/tlk

use std::num::ParseIntError;

// The whole body is a `loop` that never `break`s, so the loop has type `!`
// and stands in for the `Option<usize>` the signature promises.
fn first_success(responses: &[u16]) -> Option<usize> {
    let mut i = 0;
    loop {
        if i == responses.len() {
            return None;
        }
        if responses[i] == 200 {
            return Some(i + 1);
        }
        i += 1;
    }
}

fn parse_all(lines: &[&str]) -> Result<Vec<u32>, ParseIntError> {
    let mut parsed = Vec::new();
    let mut i = 0;
    loop {
        if i == lines.len() {
            return Ok(parsed);
        }
        parsed.push(lines[i].parse::<u32>()?); // `?` leaves the loop and the function
        i += 1;
    }
}

fn main() {
    println!("1. Retry until success: break carries the answer out");
    let responses = [503, 503, 200, 503];
    let mut attempts = 0;
    let status = loop {
        let code = responses[attempts];
        attempts += 1;
        if code == 200 {
            break code;
        }
    };
    println!("   responses = {responses:?}");
    println!("   let status = loop {{ .. break code; }};   status = {status}, attempts = {attempts}");

    println!();
    println!("2. Read until a sentinel: the server shape, over a fixed table");
    let requests = ["GET /", "GET /about", "QUIT", "GET /never-read"];
    let mut handled: Vec<&str> = Vec::new();
    let mut next = 0;
    loop {
        let request = requests[next]; // accept
        next += 1;
        if request == "QUIT" {
            break;
        }
        handled.push(request); // process and respond
    }
    println!("   handled = {handled:?}");
    println!("   read {next} of {} requests; the one after QUIT is never touched", requests.len());

    println!();
    println!("3. The same two loops, once there is an iterator to hand");
    let position = responses.iter().position(|&code| code == 200).map(|i| i + 1);
    let before_quit: Vec<&str> = requests.iter().copied().take_while(|&r| r != "QUIT").collect();
    println!("   responses.iter().position(..)  = {position:?}   <- attempts, as an Option");
    println!("   requests.iter().take_while(..) = {before_quit:?}");
    println!("   Neither needs a mut, an index, or a break.");

    println!();
    println!("4. Every way out of a loop");
    println!("   break with a value       status = {status}");
    println!("   return from inside       first_success([503, 200]) = {:?}", first_success(&[503, 200]));
    println!("                            first_success([503, 503]) = {:?}", first_success(&[503, 503]));
    println!("   ? from inside            parse_all([\"7\", \"8\"])  = {:?}", parse_all(&["7", "8"]));
    println!("                            parse_all([\"7\", \"x\"])  = {:?}", parse_all(&["7", "x"]));

    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(|| {
        let mut passes = 0;
        loop {
            passes += 1;
            if passes == 3 {
                panic!("gave up after {passes} passes");
            }
        }
    });
    std::panic::set_hook(quiet);
    let message = outcome
        .err()
        .and_then(|payload| payload.downcast_ref::<String>().cloned())
        .unwrap_or_default();
    println!("   a panic                  {message}");

    let mut passes = 0;
    loop {
        passes += 1;
        if passes == 2 {
            println!("   std::process::exit(0)    on pass {passes}: this is the last line the program prints");
            std::process::exit(0);
        }
    }
}
