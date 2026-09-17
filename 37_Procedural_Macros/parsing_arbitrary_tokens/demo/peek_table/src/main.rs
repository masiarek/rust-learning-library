//! What `peek` answers for a few operators. `syn` works outside a macro too,
//! on a `proc_macro2::TokenStream` parsed from a string.

use syn::Token;
use syn::parse::{ParseStream, Parser};

/// Runs `peek` on `source` and returns its answer. The closure has to consume
/// every token, or `parse_str` reports the rest as unexpected.
fn peeks(source: &str, peek: fn(ParseStream) -> bool) -> bool {
    let parser = |input: ParseStream| {
        let answer = peek(input);
        input.parse::<proc_macro2::TokenStream>()?;
        Ok(answer)
    };
    parser.parse_str(source).unwrap()
}

fn main() {
    println!("source   Token![=]  Token![==]  Token![<]  Token![<=]");
    for source in ["=", "==", "=>", "<", "<=", "<>", "< ="] {
        println!(
            "{:<8} {:<10} {:<11} {:<10} {}",
            source,
            peeks(source, |input| input.peek(Token![=])),
            peeks(source, |input| input.peek(Token![==])),
            peeks(source, |input| input.peek(Token![<])),
            peeks(source, |input| input.peek(Token![<=])),
        );
    }
}
