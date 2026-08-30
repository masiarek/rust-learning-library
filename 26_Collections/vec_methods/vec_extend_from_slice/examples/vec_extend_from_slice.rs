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
