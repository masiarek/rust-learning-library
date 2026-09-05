use std::cell::Cell;
use std::cmp::Reverse;

fn main() {
    let mut words = vec!["pear", "fig", "apple", "kiwi"];
    words.sort_by_key(|w| w.len());
    println!("{words:?}   <- stable: pear stays before kiwi");

    words.sort_by_key(|w| Reverse(w.len()));
    println!("{words:?}   <- Reverse for descending");

    // A tuple key sorts by two fields at once.
    let mut staff = vec![("ops", "Cara"), ("dev", "Ben"), ("ops", "Ada")];
    staff.sort_by_key(|&(dept, name)| (dept, name));
    println!("{staff:?}");

    // The key is recomputed at every comparison, not once per element.
    let calls = Cell::new(0);
    let mut nums = vec![8, 3, 5, 1, 9, 2, 7, 4, 6, 0];
    nums.sort_by_key(|n| {
        calls.set(calls.get() + 1);
        *n
    });
    println!("{nums:?}   key computed more times than there are elements: {}",
             calls.get() > nums.len());

    // The key cannot borrow from the element: `|p| &p.0` is refused
    // ("lifetime may not live long enough"). Compare in place instead.
    let mut pairs = vec![(String::from("b"), 2), (String::from("a"), 1)];
    // pairs.sort_by_key(|p| &p.0);
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{pairs:?}");
}
