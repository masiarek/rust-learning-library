use custom_debug::CustomDebug;

#[debug(rename = "Account")] // above the derive that declares it
#[derive(CustomDebug)]
struct User {
    id: u64,
}

fn main() {
    println!("{:?}", User { id: 7 });
}
