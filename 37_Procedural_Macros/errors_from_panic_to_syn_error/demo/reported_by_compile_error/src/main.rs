use variant_name_derive::VariantNameCompileError;

#[derive(VariantNameCompileError)]
enum Method {
    Get,
    Post,
    Custom(String),
}

fn main() {
    let method = Method::Custom("PURGE".to_string());
    println!("{}", method.name());
}
