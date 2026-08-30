// Removing while walking, without the walk and the removal overlapping.

fn main() {
    let mut scores = vec![5, 0, 0, 4, 3];
    scores.retain(|&s| s != 0);                 // one pass, no iterator to break
    println!("retain               -> {scores:?}");       // [5, 4, 3]

    // The C++ shape — decide first, then mutate, so the two borrows never meet.
    let mut ballots = vec!["Ada", "", "Ben", ""];
    let blank: Vec<usize> = ballots
        .iter()
        .enumerate()
        .filter(|(_, name)| name.is_empty())
        .map(|(i, _)| i)
        .collect();                             // the borrow of `ballots` ends here

    for i in blank.into_iter().rev() {          // back to front: no index shifts
        ballots.remove(i);
    }
    println!("collect, then remove -> {ballots:?}");      // ["Ada", "Ben"]

    // Calling `ballots.remove(i)` inside the `.iter()` chain is E0502.
}
