// The three ways to answer E0282, and the case where nothing is needed at all.

#[derive(Debug)]
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

fn main() {
    // let ambiguous = Container { value: None };   // error[E0282]: type annotations needed

    // 1. Annotate the binding.
    let annotated: Container<Option<String>> = Container { value: None };

    // 2. Turbofish: the same information, written at the call.
    let turbofished = Container::<Option<String>>::new(None);

    // 3. Name the type on the value itself.
    let on_the_value = Container::new(None::<String>);

    // One Vec holds one type, so this line only compiles because all three agree.
    let all = vec![annotated, turbofished, on_the_value];
    println!("three spellings, one type, {} values", all.len());
    for c in &all {
        println!("  {:?}", c.value);
    }
    println!();

    // Inference is not left-to-right: a later line can settle an earlier one.
    let mut names = Vec::new();          // T is unknown here...
    names.push(String::from("Ada"));     // ...and decided here
    println!("names {names:?}");

    // The type need not come from a value at all — a later *use* will do.
    let tally = Container::new(Vec::new());
    let total: u32 = tally.value.iter().sum();   // this line is what makes it Vec<u32>
    println!("a Vec<u32> decided one line below where it was built, summing to {total}");
    println!();

    // Same shape, no struct in sight: two spellings of one call.
    let annotated_parse: i32 = "42".parse().unwrap();
    let turbofished_parse = "42".parse::<i32>().unwrap();
    let letters: String = ['R', 'u', 's', 't'].iter().collect();
    println!("{annotated_parse} {turbofished_parse} {letters}");
}
