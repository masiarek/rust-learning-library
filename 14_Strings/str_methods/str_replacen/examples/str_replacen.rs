fn main() {
    println!("{:?}", "aaa".replacen('a', "b", 2));
    println!("{:?}", "aaa".replacen('a', "b", 9));
    println!("{:?}", "aaa".replacen('a', "b", 0));

    // Only the first delimiter, leaving the value intact.
    println!("{:?}", "key=a=b".replacen('=', ": ", 1));
    println!("{:?}", "key=a=b".replace('=', ": "));

    // The count is always from the left; for the last n, edit from the back.
    let s = "a.b.c.d";
    let mut owned = String::from(s);
    for (i, m) in s.rmatch_indices('.').take(2) {
        owned.replace_range(i..i + m.len(), "/");
    }
    println!("{owned:?}");
}
