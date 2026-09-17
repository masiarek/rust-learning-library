use by_name_derive::ByNameAsWritten;

#[derive(ByNameAsWritten)]
enum Method {
    Get,
    Post,
}

fn main() {
    let parsed: Method = "Post".parse().unwrap();
    println!("{} {}", Method::Get, parsed);
}
