//! Macros that report what they were handed, one level below `to_string()`:
//! every token tree, its kind, its spacing or delimiter, and where in the
//! source it came from. The text each one reports goes back as a string
//! literal built as a token, so no quote or backslash in it needs escaping.

use std::fmt::Write;

use proc_macro::{Literal, TokenStream, TokenTree};

/// One line per token tree, with a group's contents indented under it.
#[proc_macro]
pub fn token_trees(input: TokenStream) -> TokenStream {
    let mut lines = String::new();
    walk(input, 0, &mut lines);
    string_literal(lines.trim_end())
}

/// The input's `to_string()`.
#[proc_macro]
pub fn text(input: TokenStream) -> TokenStream {
    string_literal(&input.to_string())
}

/// A derive's input's `to_string()`, as a constant beside the item.
#[proc_macro_derive(Text)]
pub fn derive_text(item: TokenStream) -> TokenStream {
    let mut out: TokenStream = "pub const DERIVE_INPUT: &str =".parse().unwrap();
    out.extend([string_literal(&item.to_string()), ";".parse().unwrap()]);
    out
}

/// An attribute's item's `to_string()`, as a constant beside the item.
#[proc_macro_attribute]
pub fn text_attr(_args: TokenStream, item: TokenStream) -> TokenStream {
    let text = item.to_string();
    let mut out = item;
    out.extend(["pub const ATTRIBUTE_INPUT: &str =".parse().unwrap(), string_literal(&text), ";".parse().unwrap()]);
    out
}

fn walk(stream: TokenStream, depth: usize, out: &mut String) {
    for tree in stream {
        let span = tree.span();
        let at = format!("{}:{}", span.line(), span.column());
        let indent = "    ".repeat(depth);
        let _ = match &tree {
            TokenTree::Ident(ident) => writeln!(out, "{at:>5}  {indent}Ident    {ident}"),
            TokenTree::Punct(punct) => {
                let (ch, spacing) = (punct.as_char(), punct.spacing());
                writeln!(out, "{at:>5}  {indent}Punct    {ch}  {spacing:?}")
            }
            TokenTree::Literal(literal) => writeln!(out, "{at:>5}  {indent}Literal  {literal}"),
            TokenTree::Group(group) => writeln!(out, "{at:>5}  {indent}Group    {:?}", group.delimiter()),
        };
        if let TokenTree::Group(group) = tree {
            walk(group.stream(), depth + 1, out);
        }
    }
}

fn string_literal(text: &str) -> TokenStream {
    TokenTree::Literal(Literal::string(text)).into()
}
