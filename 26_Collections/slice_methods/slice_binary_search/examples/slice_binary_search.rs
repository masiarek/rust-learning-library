fn main() {
    let sorted = [1, 3, 5, 7, 9];
    println!("{:?}", sorted.binary_search(&5));
    println!("{:?}  <- absent; 4 would go at index 2", sorted.binary_search(&4));
    println!("{:?}  <- absent; past the end", sorted.binary_search(&100));

    // Err carries the insertion point, so this keeps a Vec sorted.
    let mut v = vec![1, 3, 5, 7, 9];
    for x in [4, 0, 10] {
        let at = v.binary_search(&x).unwrap_or_else(|i| i);
        v.insert(at, x);
    }
    println!("{v:?}");

    // On an unsorted slice the answer is unspecified: not wrong, not an error.
    let unsorted = [9, 1, 5, 3, 7];
    println!("{:?}  <- 5 IS in there, at index 2", unsorted.binary_search(&5));

    // Duplicates: any one of the matching indices may come back.
    let dups = [1, 2, 2, 2, 3];
    println!("{:?}", dups.binary_search(&2));
    println!("first 2 at {}", dups.partition_point(|&x| x < 2));
    println!("past the last 2 at {}", dups.partition_point(|&x| x <= 2));
}
