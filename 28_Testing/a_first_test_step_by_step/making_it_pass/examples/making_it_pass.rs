//! The loader that makes the first test pass, one stage at a time, and the
//! `split('\n')` version that fails it.
//! The page is 28_Testing/a_first_test_step_by_step/making_it_pass/README.md.
//!
//!   rustc --edition 2024 making_it_pass.rs -o /tmp/t && /tmp/t

use std::any::type_name_of_val;
use std::panic;

fn parse_rows(csv: String) -> Vec<Vec<f32>> {
    csv.lines()
        .map(|line| {
            line.split(',')
                .map(|field| field.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let csv = String::from(
        "1,1,1\n\
         1,-1,1\n\
         -1,1,1\n\
         -1,-1,-1\n",
    );

    println!("1. The test's three assertions, checked here without the harness");
    let rows = parse_rows(csv.clone());
    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0][2], 1.0);
    assert_eq!(rows[3], [-1.0, -1.0, -1.0]);
    println!("   all three hold: {rows:?}");

    println!();
    println!("2. What each stage hands to the next");
    let first = csv.lines().next().unwrap();
    println!("   csv.lines()          is a {}", type_name_of_val(&csv.lines()));
    println!("   first line           = {first:?}");
    println!("   line.split(',')      is a {}", type_name_of_val(&first.split(',')));
    println!("   its items            = {:?}", first.split(',').collect::<Vec<&str>>());
    println!("   \"-1\".parse::<f32>()  = {:?}   <- integer text parses as a float", "-1".parse::<f32>());
    println!("   inner collect()      -> Vec<f32>, outer collect() -> Vec<Vec<f32>>, both from the return type");

    println!();
    println!("3. Why trim(), and why lines() rather than split('\\n')");
    println!("   \" 1\".parse::<f32>()   = {:?}", " 1".parse::<f32>());
    println!("   lines()        gives {:?}", "1,1\n-1,1\n".lines().collect::<Vec<_>>());
    println!("   split('\\n')    gives {:?}", "1,1\n-1,1\n".split('\n').collect::<Vec<_>>());
    println!("   \"\".parse::<f32>()    = {:?}", "".parse::<f32>());

    panic::set_hook(Box::new(|_| {}));
    let with_split = panic::catch_unwind(|| {
        csv.split('\n')
            .map(|line| {
                line.split(',')
                    .map(|field| field.trim().parse::<f32>().unwrap())
                    .collect::<Vec<f32>>()
            })
            .collect::<Vec<Vec<f32>>>()
    });
    let _ = panic::take_hook();
    let message = match with_split {
        Ok(_) => "no panic".to_string(),
        Err(payload) => payload.downcast_ref::<String>().cloned().unwrap_or_default(),
    };
    println!("   the loader rewritten with split('\\n') panics: {message}");
}
