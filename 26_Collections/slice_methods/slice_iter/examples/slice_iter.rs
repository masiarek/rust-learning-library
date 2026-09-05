fn main() {
    let v = vec![1, 2, 3];
    for x in v.iter() {
        print!("{x} ");
    }
    println!("<- same as `for x in &v`");

    let total: i32 = v.iter().sum();
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("{total} {doubled:?}");

    // Items are &T, so filter's closure sees &&i32: destructure or deref.
    let big = v.iter().filter(|&&x| x > 1).count();
    println!("{big}");

    // The searches contains() cannot do.
    println!("{:?} {:?}", v.iter().position(|&x| x == 2), v.iter().find(|&&x| x > 5));

    // It borrows: v is intact afterwards. into_iter() would have consumed it.
    println!("{v:?}");

    // Double-ended and exact-size.
    println!("{:?} {}", v.iter().rev().collect::<Vec<_>>(), v.iter().len());
}
