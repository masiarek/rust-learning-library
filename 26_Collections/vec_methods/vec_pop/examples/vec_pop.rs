fn main() {
    let mut v = vec![1, 2, 3];
    println!("{:?} {:?} {:?} {:?}", v.pop(), v.pop(), v.pop(), v.pop());
    println!("empty now: {v:?}");

    // Option is the whole point: an empty Vec has nothing to give, and the
    // type makes you say what happens then.
    let mut stack = vec!["a", "b"];
    while let Some(top) = stack.pop() { println!("popped {top}"); }

    // pop is O(1) and does not shrink the buffer.
    let mut v = vec![1u8; 100];
    let cap = v.capacity();
    for _ in 0..100 { v.pop(); }
    println!("len {} cap unchanged {}", v.len(), v.capacity() == cap);

    // Vec + push/pop is Rust's stack. There is no separate Stack type.
    let mut stack: Vec<char> = Vec::new();
    let mut balanced = true;
    for c in "([{}])".chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => balanced &= stack.pop() == Some('('),
            ']' => balanced &= stack.pop() == Some('['),
            '}' => balanced &= stack.pop() == Some('{'),
            _ => {}
        }
    }
    println!("balanced: {} leftovers: {:?}", balanced && stack.is_empty(), stack);

    // The value is moved out, not cloned — pop works on non-Clone types.
    let mut owners = vec![String::from("only copy")];
    let taken = owners.pop().unwrap();
    println!("{taken:?} and the vec is {owners:?}");
}
