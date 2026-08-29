fn main() {
    let mut s = String::from("hello world");
    s.replace_range(0..5, "goodbye");
    println!("{s:?}");

    // The replacement can be shorter, longer, or empty.
    let mut t = String::from("a-b-c");
    t.replace_range(1..2, "");
    println!("{t:?}");

    // Several edits, back to front, so the offsets stay valid.
    let text = "one,two,three";
    let mut owned = String::from(text);
    for (i, m) in text.rmatch_indices(',') {
        owned.replace_range(i..i + m.len(), " and ");
    }
    println!("{owned:?}");

    // Front to back with stale offsets: the same loop, wrong answer.
    let mut broken = String::from(text);
    for (i, m) in text.match_indices(',') {
        if i + m.len() <= broken.len() && broken.is_char_boundary(i) {
            broken.replace_range(i..i + m.len(), " and ");
        }
    }
    println!("{broken:?}");
}
