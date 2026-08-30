fn main() {
    // Pops only if the last element passes the test. Nothing is removed
    // otherwise, and the element is left where it was.
    let mut v = vec![1, 2, 3, 4];
    println!("{:?} then {v:?}", v.pop_if(|n| *n % 2 == 0));
    println!("{:?} then {v:?}", v.pop_if(|n| *n % 2 == 0));

    // On an empty vector the predicate is never called.
    let mut calls = 0;
    let mut empty: Vec<i32> = Vec::new();
    let got = empty.pop_if(|_| { calls += 1; true });
    println!("empty: got {got:?}, predicate called {calls} times");

    // The predicate gets &mut, so it can change the element it decides to keep.
    let mut v = vec![String::from("ada")];
    let popped = v.pop_if(|s| { s.push('!'); s.len() > 99 });
    println!("kept and mutated: {popped:?} {v:?}");

    // Trimming a suffix without a loop-with-a-break.
    let mut trailing = vec![1, 2, 0, 0, 0];
    while trailing.pop_if(|n| *n == 0).is_some() {}
    println!("zeros trimmed: {trailing:?}");

    // The pre-1.86 spelling took two steps and a second bounds check.
    let mut v = vec![5, 6];
    if v.last().is_some_and(|n| *n > 5) { v.pop(); }
    println!("the old way: {v:?}");
}
