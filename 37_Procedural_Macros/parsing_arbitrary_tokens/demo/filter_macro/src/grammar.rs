//! The language `filter!` accepts, as a struct per rule and a `Parse` impl
//! that reads it:
//!
//! ```text
//! filter    = condition ("and" condition)*
//! condition = field "in" "[" value ("," value)* "]"
//!           | field op value
//! op        = "=" | "<>" | "<=" | ">=" | "<" | ">"
//! ```

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Lit, Token, bracketed};

mod kw {
    // `and` is an ordinary identifier to Rust; this makes it a token type.
    syn::custom_keyword!(and);
}

// `<>` is not a Rust operator, so there is no `Token![<>]` to borrow.
syn::custom_punctuation!(LtGt, <>);

pub struct Filter {
    pub conditions: Punctuated<Condition, kw::and>,
}

pub enum Condition {
    Compare { field: Ident, op: Op, value: Lit },
    In { field: Ident, values: Punctuated<Lit, Token![,]> },
}

pub enum Op {
    Eq,
    NotEq,
    Le,
    Ge,
    Lt,
    Gt,
}

impl Parse for Filter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let conditions = Punctuated::parse_separated_nonempty(input)?;
        Ok(Filter { conditions })
    }
}

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let field: Ident = input.parse()?;
        if input.peek(Token![==]) {
            let token: Token![==] = input.parse()?;
            return Err(syn::Error::new_spanned(token, "`filter!` compares with `=`, not `==`"));
        }
        // Every `peek` that fails is remembered, and `lookahead.error()` lists
        // them. Two-character operators go first: `Token![<]` also matches `<=`.
        let lookahead = input.lookahead1();
        let op = if lookahead.peek(Token![in]) {
            input.parse::<Token![in]>()?;
            let content;
            bracketed!(content in input);
            let values = Punctuated::parse_terminated(&content)?;
            return Ok(Condition::In { field, values });
        } else if lookahead.peek(LtGt) {
            input.parse::<LtGt>().map(|_| Op::NotEq)?
        } else if lookahead.peek(Token![<=]) {
            input.parse::<Token![<=]>().map(|_| Op::Le)?
        } else if lookahead.peek(Token![>=]) {
            input.parse::<Token![>=]>().map(|_| Op::Ge)?
        } else if lookahead.peek(Token![<]) {
            input.parse::<Token![<]>().map(|_| Op::Lt)?
        } else if lookahead.peek(Token![>]) {
            input.parse::<Token![>]>().map(|_| Op::Gt)?
        } else if lookahead.peek(Token![=]) {
            input.parse::<Token![=]>().map(|_| Op::Eq)?
        } else {
            return Err(lookahead.error());
        };
        let value = input.parse()?;
        Ok(Condition::Compare { field, op, value })
    }
}
