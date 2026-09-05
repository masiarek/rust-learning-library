fn main() {
    let rows = vec![vec![1, 2], vec![3], vec![]];
    let flat: Vec<i32> = rows.concat();
    println!("{flat:?}");

    // Strings: a slice of &str or of String becomes one String.
    let parts = ["ab", "cd", "ef"];
    let s: String = parts.concat();
    println!("{s}");
    let owned = vec![String::from("x"), String::from("y")];
    println!("{}", owned.concat());

    // It clones: the rows are still here.
    println!("{rows:?}");

    // Arrays of arrays flatten the same way.
    let grid = [[1, 2], [3, 4]];
    println!("{:?}", grid.concat());

    // The iterator spelling of the same thing.
    let via_iter: Vec<i32> = rows.iter().flatten().copied().collect();
    println!("{via_iter:?}");
}
