fn main() {
    let mut nums = vec![1, 2, 3];
    if let Some(last) = nums.last_mut() {
        *last *= 100;
    }
    println!("{nums:?}");

    // Append to the most recent group without pop-and-push.
    let mut groups: Vec<Vec<&str>> = vec![vec!["a"], vec!["b"]];
    groups.last_mut().unwrap().push("c");
    println!("{groups:?}");

    // last_mut never creates: on an empty Vec it is None, and push is the answer.
    let mut empty: Vec<Vec<&str>> = vec![];
    match empty.last_mut() {
        Some(group) => group.push("x"),
        None => empty.push(vec!["x"]),
    }
    println!("{empty:?}");
}
