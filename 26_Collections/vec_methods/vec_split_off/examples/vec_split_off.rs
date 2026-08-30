fn main() {
    // Everything from `at` onwards leaves in a new Vec; self keeps the front.
    let mut v = vec![1, 2, 3, 4, 5];
    let tail = v.split_off(2);
    println!("head {v:?}  tail {tail:?}");

    // at == 0 moves everything out and leaves self empty.
    let mut v = vec![1, 2];
    let all = v.split_off(0);
    println!("all {all:?}  v {v:?}");

    // at == len() gives an empty tail and changes nothing.
    let mut v = vec![1, 2];
    let none = v.split_off(2);
    println!("v {v:?}  tail {none:?}");

    // The tail is a NEW allocation; self keeps its original buffer.
    let mut v: Vec<u8> = Vec::with_capacity(64);
    v.extend_from_slice(&[0; 64]);
    let tail = v.split_off(32);
    println!("head cap {} tail cap {}", v.capacity(), tail.capacity());

    // The elements are moved, so non-Clone works.
    let mut owners = vec![String::from("a"), String::from("b"), String::from("c")];
    let back = owners.split_off(1);
    println!("{owners:?} + {back:?}");

    // Chunking a vector into fixed-size pieces, front to back.
    let mut v: Vec<u8> = (1..=7).collect();
    let mut chunks = vec![];
    while v.len() > 3 { let rest = v.split_off(3); chunks.push(v); v = rest; }
    chunks.push(v);
    println!("{chunks:?}");

    // For read-only chunking, `chunks()` on the slice does this without
    // moving anything at all.
    let v: Vec<u8> = (1..=7).collect();
    println!("{:?}", v.chunks(3).collect::<Vec<_>>());

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| { let mut v = vec![1]; let _ = v.split_off(9); });
    std::panic::set_hook(hook);
    println!("split_off past len panicked: {}", caught.is_err());
}
