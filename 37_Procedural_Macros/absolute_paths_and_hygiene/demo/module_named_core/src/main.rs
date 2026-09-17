use by_name_derive::ByNameCore;

/// The application's core logic, in a module named for it.
mod core {
    pub fn is_safe(method: &super::Method) -> bool {
        matches!(method, super::Method::Get)
    }
}

#[derive(ByNameCore)]
enum Method {
    Get,
    Post,
}

fn main() {
    println!("{} {}", Method::Post, core::is_safe(&Method::Post));
}
