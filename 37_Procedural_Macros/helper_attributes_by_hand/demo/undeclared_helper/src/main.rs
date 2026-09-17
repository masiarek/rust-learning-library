#[derive(Debug)] // std's derive declares no helper attributes
struct User {
    id: u64,
    #[debug(skip)]
    password: String,
}

fn main() {
    let user = User {
        id: 7,
        password: "hunter2".to_owned(),
    };
    println!("{user:?}");
}
