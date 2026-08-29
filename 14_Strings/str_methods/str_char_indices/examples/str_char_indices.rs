fn main() {
    let s = "héllo";

    for (i, c) in s.char_indices() {
        let n = c.len_utf8();
        println!("byte {i:>2}  {c:?}  ({n} byte{})", if n == 1 { "" } else { "s" });
    }

    // The two disagree the moment a character is wider than one byte.
    let idx: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
    let ord: Vec<usize> = s.chars().enumerate().map(|(i, _)| i).collect();
    println!("offsets  {idx:?}");
    println!("ordinals {ord:?}");

    // An offset from char_indices is always a legal slice endpoint.
    let third = s.char_indices().nth(2).unwrap().0;
    println!("from the third char: {:?}", &s[third..]);

    // Slicing one character out: start, plus its own width.
    let (i, c) = s.char_indices().nth(1).unwrap();
    println!("just that char: {:?}", &s[i..i + c.len_utf8()]);
}
