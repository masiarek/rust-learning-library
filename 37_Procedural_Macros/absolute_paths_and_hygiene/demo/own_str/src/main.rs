use by_name_derive::ByName;

/// This crate's own `str`. Unusual, and legal: `str` is a name, not a keyword.
#[allow(non_camel_case_types)]
struct str(String);

#[derive(ByName)]
enum Method {
    Get,
    Post,
}

fn main() {
    let custom = str("PURGE".to_string());
    println!("{} {} {}", Method::Get, Method::Post, custom.0);
    println!("{}", "Post".parse::<Method>().is_ok());
}
