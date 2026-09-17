use variant_name_derive::VariantName;

#[derive(VariantName)]
enum Method {
    Get,
    Post,
    Custom(String),
}

fn main() {
    let method = Method::Custom("PURGE".to_string());
    println!("{}", method.name());
}
