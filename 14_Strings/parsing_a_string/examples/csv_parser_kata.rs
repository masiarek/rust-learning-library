//! Kata solution: a CSV reader with quotes — a comma inside quotes, a doubled
//! quote for a literal one, empty fields kept, and the newline inside a quoted
//! field that a line-by-line reader cuts in two.
//!
//!   rustc --edition 2024 csv_parser_kata.rs -o /tmp/cvk && /tmp/cvk

#[derive(Clone, Copy, PartialEq)]
enum State {
    Start,
    Quoted,
    AfterQuote,
}

/// One pass over the whole input, character by character.
///
/// Outside quotes a comma ends a field and a newline ends a record, and an
/// unquoted field is trimmed. A field that starts with `"` is quoted: inside
/// it `""` stands for one literal quote, and commas and newlines are ordinary
/// text. Only whitespace may follow the closing quote.
fn parse(input: &str) -> Result<Vec<Vec<String>>, String> {
    let mut records = Vec::new();
    let mut record = Vec::new();
    let mut field = String::new();
    let mut state = State::Start;
    let mut opened_at = 0;
    let mut chars = input.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        match state {
            State::Quoted => match c {
                '"' if matches!(chars.peek(), Some((_, '"'))) => {
                    field.push('"');
                    chars.next();
                }
                '"' => state = State::AfterQuote,
                _ => field.push(c),
            },
            _ if c == ',' || c == '\n' => {
                record.push(finish(&mut field, state));
                state = State::Start;
                if c == '\n' {
                    records.push(std::mem::take(&mut record));
                }
            }
            // CRLF: skip the carriage return and let the newline end the record.
            _ if c == '\r' && matches!(chars.peek(), Some((_, '\n'))) => {}
            State::Start if c == '"' && field.trim().is_empty() => {
                field.clear();
                state = State::Quoted;
                opened_at = i;
            }
            State::AfterQuote if c.is_whitespace() => {}
            State::AfterQuote => return Err(format!("text after a closing quote at byte {i}")),
            State::Start => field.push(c),
        }
    }
    if state == State::Quoted {
        return Err(format!("unterminated quote opened at byte {opened_at}"));
    }
    if !field.is_empty() || !record.is_empty() || state == State::AfterQuote {
        record.push(finish(&mut field, state));
        records.push(record);
    }
    Ok(records)
}

/// A quoted field is taken exactly as written; an unquoted one is trimmed.
fn finish(field: &mut String, state: State) -> String {
    let text = std::mem::take(field);
    if state == State::AfterQuote { text } else { text.trim().to_string() }
}

fn main() {
    println!("1. Quotes: a comma inside them, and a doubled quote for a literal one");
    for record in parse("id,name,said\n1,\"Doe, Jane\",\"She said \"\"hi\"\"\"\n").unwrap() {
        println!("   {record:?}");
    }

    println!();
    println!("2. An empty field is data");
    for record in parse("a,,c\n,,\n").unwrap() {
        println!("   {record:?}");
    }
    println!("   Three fields on both lines. The tokenizer kata on this page drops the");
    println!("   empty run between two delimiters on purpose; a CSV reader cannot, or");
    println!("   every column after a blank one shifts one place to the left.");

    println!();
    println!("3. Trim outside the quotes, keep what is inside them");
    println!("   {:?}", parse("  a  , \" b \" ,c").unwrap()[0]);

    println!();
    println!("4. The newline inside the quotes");
    let text = "name,note\nAda,\"first line\nsecond line\"\nBen,short\n";
    let records = parse(text).unwrap();
    println!("   the whole input: {} records from {} lines", records.len(), text.lines().count());
    for record in &records {
        println!("   {record:?}");
    }
    println!("   and one line at a time instead:");
    for line in text.lines() {
        println!("   {:<20} -> {:?}", format!("{line:?}"), parse(line));
    }
    println!("   Splitting on lines first is the bug. It cuts the record at the quoted");
    println!("   newline, and neither half means anything on its own.");

    println!();
    println!("5. Two failures, and where each one is");
    for bad in ["a,\"b", "\"a\"x,b"] {
        println!("   {:<10} -> {:?}", format!("{bad:?}"), parse(bad));
    }

    println!();
    println!("6. Windows line endings");
    println!("   {:?}", parse("a,b\r\nc,d\r\n").unwrap());
}
