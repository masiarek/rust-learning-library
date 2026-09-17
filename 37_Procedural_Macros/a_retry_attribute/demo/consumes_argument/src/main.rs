use retry::retry;

fn deliver(message: String) -> Result<(), String> {
    Err(message)
}

#[retry(times = 3)]
fn send(message: String) -> Result<(), String> {
    deliver(message)
}

fn main() {
    let _ = send(String::from("hello"));
}
