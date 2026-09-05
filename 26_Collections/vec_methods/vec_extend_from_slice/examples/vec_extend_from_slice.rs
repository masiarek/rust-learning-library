fn main() {
    // Clones each element out of a slice you keep.
    let mut v = vec![1, 2];
    let more = [3, 4, 5];
    v.extend_from_slice(&more);
    println!("{v:?}  source still usable: {more:?}");

    // T: Clone is the requirement — not Copy. Strings work.
    let mut names = vec![String::from("Ada")];
    let rest = vec![String::from("Ben"), String::from("Cara")];
    names.extend_from_slice(&rest);
    println!("{names:?}  rest {rest:?}");

    // extend(&v) goes through Extend<&T>, which demands T: Copy — so it takes
    // numbers and refuses the Strings above:
    //     names.extend(&rest);   // error[E0271]: expected `String`, found `&String`
    // extend_from_slice asks only for Clone, which is why the method exists.
    let nums = vec![1, 2, 3];
    let mut copied: Vec<i32> = Vec::new();
    copied.extend(&nums);
    println!("extend(&v) needs Copy: {copied:?}");

    // The element types must match exactly. A Vec<String> will not take a
    // &[&str], because cloning a &str gives a &str, not a String:
    //     owned.extend_from_slice(words);   // error[E0308]: expected `&[String]`
    // Convert on the way in with extend instead.
    let words: &[&str] = &["apple", "banana", "cherry"];
    let mut owned: Vec<String> = Vec::new();
    owned.extend(words.iter().map(|&s| s.to_string()));
    println!("{owned:?} from {words:?}");

    // A &Vec<T> coerces to &[T], so this is how you concatenate two vectors
    // you both want to keep.
    let a = vec![1, 2];
    let b = vec![3, 4];
    let mut joined = Vec::with_capacity(a.len() + b.len());
    joined.extend_from_slice(&a);
    joined.extend_from_slice(&b);
    println!("{joined:?} from {a:?} and {b:?}");

    // It reserves once for the whole slice, which extend() from an iterator
    // can only do when the iterator knows its own length.
    let mut v: Vec<u8> = Vec::new();
    v.extend_from_slice(&[0; 100]);
    println!("100 bytes in one go: len {} cap {}", v.len(), v.capacity());

    // For bytes and &str there is a shorthand worth knowing.
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"hello ");
    buf.extend_from_slice("world".as_bytes());
    println!("{}", String::from_utf8(buf).unwrap());
}
