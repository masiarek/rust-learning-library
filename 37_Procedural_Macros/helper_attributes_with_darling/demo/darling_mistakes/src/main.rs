use custom_debug::CustomDebug;

#[derive(CustomDebug)]
#[debug(rename = "Account", rename = "User")] // the same key twice
pub struct User {
    #[debug(skipp)] // a typo
    pub id: u64,
    #[debug(rename = 5)] // a number where a string belongs
    pub password: String,
}

fn main() {}
