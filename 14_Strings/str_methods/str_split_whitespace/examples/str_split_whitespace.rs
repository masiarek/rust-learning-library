fn main() {
    let messy = "  the   quick \t brown \n fox  ";

    println!("{:?}", messy.split_whitespace().collect::<Vec<&str>>());
    println!("{:?}", messy.split(' ').collect::<Vec<&str>>());
    println!("{} vs {} pieces", messy.split_whitespace().count(), messy.split(' ').count());

    // Never yields an empty piece.
    println!("{:?}", "   ".split_whitespace().collect::<Vec<&str>>());
    println!("{:?}", "".split_whitespace().collect::<Vec<&str>>());

    // Unicode whitespace, not just the ASCII five.
    println!("{:?}", "a\u{00A0}b".split_whitespace().collect::<Vec<&str>>());

    // The trap: on delimited data it merges empty fields.
    let row = "alice,,42";
    let fields: Vec<&str> = row.split(',').collect();
    println!("split(',')        {:?}  -> {} fields", fields, fields.len());
    println!("whitespace route  {:?}  -> {} fields, and 42 is now column 2",
             row.replace(',', " ").split_whitespace().collect::<Vec<&str>>(),
             row.replace(',', " ").split_whitespace().count());

    println!("{} words", "the quick brown fox".split_whitespace().count());
}
