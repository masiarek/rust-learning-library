#![allow(deprecated)]

fn main() {
    let s = "value///";
    println!("{:?}", s.trim_right_matches('/'));
    println!("{:?}", s.trim_end_matches('/'));
    println!("identical: {}", s.trim_right_matches('/') == s.trim_end_matches('/'));
}
