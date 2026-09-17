use variant_name_derive::VariantName;

#[derive(VariantName)]
enum Method {
    Get,
    Custom(String),
    Post,
    Numbered { code: u16 },
}

fn main() {
    let method = Method::Custom("PURGE".to_string());
    println!("{}", method.name());
}
