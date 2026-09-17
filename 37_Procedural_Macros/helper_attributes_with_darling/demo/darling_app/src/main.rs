use custom_debug::CustomDebug;

#[derive(CustomDebug)]
#[debug(rename = "Account", mask = "<hidden>")]
struct User {
    id: u64,
    #[debug(rename = "email")]
    email_address: String,
    /// A doc comment is an attribute too; the derive has to step over it.
    #[debug(redact)]
    password: String,
    #[debug(skip)]
    failed_logins: u32,
}

#[derive(CustomDebug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let user = User {
        id: 7,
        email_address: "ada@example.com".to_owned(),
        password: "hunter2".to_owned(),
        failed_logins: 3,
    };
    println!("{user:?}");
    println!("{user:#?}");
    println!(
        "hidden, not gone: password has {} bytes, failed_logins = {}",
        user.password.len(),
        user.failed_logins
    );
    println!("{:?}", Point { x: 1, y: 2 });
}
