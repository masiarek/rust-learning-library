//! What `syn` makes of a function: the fields of `syn::ItemFn`, each printed
//! back as tokens.

use quote::ToTokens;
use syn::ItemFn;

const SOURCE: &str = r#"
/// The port in `host:port`.
#[inline]
pub async fn port<T: Copy>(address: &str) -> Result<u16, String> where T: Default {
    todo!()
}
"#;

fn show(field: &str, value: &dyn ToTokens) {
    println!("{field:<15} {}", value.to_token_stream());
}

fn main() {
    let function: ItemFn = syn::parse_str(SOURCE).unwrap();
    println!("{} attrs", function.attrs.len());
    for attr in &function.attrs {
        show("  attr", attr);
    }
    show("vis", &function.vis);
    println!("{:<15} {:?}", "modifiers", function.modifiers);
    show("sig", &function.sig);
    show("  asyncness", &function.sig.asyncness);
    println!("{:<15} {:?}", "  safety", function.sig.safety);
    show("  ident", &function.sig.ident);
    show("  generics", &function.sig.generics);
    show("  where_clause", &function.sig.generics.where_clause);
    show("  inputs", &function.sig.inputs);
    show("  output", &function.sig.output);
    show("block", &function.block);
}
