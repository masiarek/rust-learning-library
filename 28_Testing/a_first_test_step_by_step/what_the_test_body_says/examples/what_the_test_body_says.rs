//! Every expression in the first test's body, run outside the harness.
//! The page is 28_Testing/a_first_test_step_by_step/what_the_test_body_says/README.md.
//!
//!   rustc --edition 2024 what_the_test_body_says.rs -o /tmp/t && /tmp/t

use std::any::{type_name, type_name_of_val};

/// Names the one type both arguments share, so a literal's inferred type
/// can be printed from the context it is used in.
fn shared_type<T>(_: &T, _: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("1. A \\ at the end of a line removes the newline and the next line's indent");
    let continued = String::from(
        "1,1,1\n\
         1,-1,1\n",
    );
    let plain = String::from(
        "1,1,1
         1,-1,1",
    );
    println!("   with \\:    {continued:?}");
    println!("   without:  {plain:?}");

    println!();
    println!("2. Vec<Vec<f32>>: a Vec of rows, each row a Vec of columns");
    let rows: Vec<Vec<f32>> = vec![
        vec![1.0, 1.0, 1.0],
        vec![1.0, -1.0, 1.0],
        vec![-1.0, 1.0, 1.0],
        vec![-1.0, -1.0, -1.0],
    ];
    println!("   rows.len()  = {}, rows[1].len() = {}", rows.len(), rows[1].len());
    println!("   rows[1]     = {:?}   <- one row, a Vec<f32>", rows[1]);
    println!("   rows[1][1]  = {:?}               <- row 1, column 1, an f32", rows[1][1]);
    println!("   rows.get(9) = {:?}               <- rows[9] would panic instead", rows.get(9));

    println!();
    println!("3. assert_eq! compares a Vec with an array: Vec<T> implements PartialEq<[U; N]>");
    println!("   rows[3] == [-1.0, -1.0, -1.0]  is {}", rows[3] == [-1.0, -1.0, -1.0]);
    println!("   rows[3] == [-1.0, -1.0]        is {}   <- lengths differ", rows[3] == [-1.0, -1.0]);
    assert_eq!(rows[3], [-1.0, -1.0, -1.0]);
    println!("   assert_eq!(rows[3], [-1.0, -1.0, -1.0]) passed");

    println!();
    println!("4. The literal 1.0 takes its type from what it is compared with");
    println!("   1.0 on its own:          {}", type_name_of_val(&1.0));
    println!("   1.0 beside rows[0][2]:   {}", shared_type(&rows[0][2], &1.0));
    println!("   rows[0][2] == 1.0 is {}: 1.0 and -1.0 are exact in binary", rows[0][2] == 1.0);
    println!("   0.1_f32 == 0.1_f64 as f32 is {}, 0.1_f32 as f64 == 0.1 is {}", 0.1_f32 == 0.1_f64 as f32, 0.1_f32 as f64 == 0.1);
}
