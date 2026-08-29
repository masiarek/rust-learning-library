#![allow(deprecated)]

fn main() {
    let s = "xxxvalue";
    println!("{:?}", s.trim_left_matches('x'));
    println!("{:?}", s.trim_start_matches('x'));
    println!("identical: {}", s.trim_left_matches('x') == s.trim_start_matches('x'));
}
