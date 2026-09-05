fn main() {
    let mut v = vec![1, 2, 3];
    for x in v.iter_mut() {
        *x *= 10;
    }
    println!("{v:?}");

    // `for x in &mut v` is the same call.
    for x in &mut v {
        *x += 1;
    }
    println!("{v:?}");

    // The items are &mut T: every write goes through a *.
    v.iter_mut().for_each(|x| *x = -*x);
    println!("{v:?}");

    // Index and element together, without indexing the borrowed Vec.
    for (i, x) in v.iter_mut().enumerate() {
        *x = i as i32 * 100;
    }
    println!("{v:?}");

    // Strings edit in place too: the &mut String is the item.
    let mut words = vec![String::from("a"), String::from("b")];
    for w in &mut words {
        w.push('!');
    }
    println!("{words:?}");
}
