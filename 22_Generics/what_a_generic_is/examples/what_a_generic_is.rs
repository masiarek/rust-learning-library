// One definition, many types: a generic struct, a generic impl, a generic function.

#[derive(Debug)]
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

// One body, type-checked once, compiled once per type actually used.
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut best = items[0];
    for &item in items {
        if item > best {
            best = item;
        }
    }
    best
}

fn main() {
    // Nothing here names a type: the compiler reads T off the value.
    let text = Container::new("Thought is free.");
    let count = Container::new(4u8);
    let scores = Container { value: vec![5, 3, 0] };

    println!("text    {}", text.value);
    println!("count   {:?}", count.value);
    println!("scores  {:?}", scores.value);
    println!();

    // A Container<T> IS its T: no tag, no pointer, nothing added.
    println!("size_of::<u8>()                   {}", size_of::<u8>());
    println!("size_of::<Container<u8>>()        {}", size_of::<Container<u8>>());
    println!("size_of::<Container<i64>>()       {}", size_of::<Container<i64>>());
    println!("size_of::<Container<[u8; 16]>>()  {}", size_of::<Container<[u8; 16]>>());
    println!();

    // Three calls, three separate functions in the finished binary.
    println!("largest(&[3, 9, 4])         {}", largest(&[3, 9, 4]));
    println!("largest(&['a', 'q', 'f'])   {}", largest(&['a', 'q', 'f']));
    println!("largest(&[0.5, 0.25])       {}", largest(&[0.5, 0.25]));
    println!();

    // Container<u8> and Container<&str> are two different types.
    let batch = vec![Container::new(1u8), Container::new(2u8)];
    // batch.push(Container::new("three"));   // error[E0308]: mismatched types
    println!("a Vec<Container<u8>> holds {} of them, and nothing else", batch.len());
}
