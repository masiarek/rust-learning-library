use variant_name_derive::VariantName;

#[derive(VariantName)]
enum Method {
    Get,
    Post,
    Delete,
}

fn main() {
    for method in [Method::Get, Method::Post, Method::Delete] {
        println!("{}", method.name());
    }
}
