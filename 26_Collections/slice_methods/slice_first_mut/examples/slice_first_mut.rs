fn main() {
    let mut nums = vec![1, 2, 3];
    if let Some(first) = nums.first_mut() {
        *first = 10;
    }
    println!("{nums:?}");

    // On an empty vector nums[0] = 10 would panic; this is a no-op.
    let mut empty: Vec<i32> = vec![];
    if let Some(first) = empty.first_mut() {
        *first = 10;
    }
    println!("{:?}", empty.first_mut());
    println!("{empty:?}");

    // The &mut is a whole-slice borrow while it lives.
    let mut words = vec![String::from("a"), String::from("b")];
    let first = words.first_mut().unwrap();
    first.push_str("da");
    // println!("{words:?}");   // error[E0502] here: `first` is still alive
    println!("{first}");
    println!("{words:?}");      // fine: `first` was last used above
}
