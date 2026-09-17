use retry::retry;

#[retry(times = 3)]
async fn in_async() -> Result<(), String> {
    Ok(())
}

#[retry(times = 3)]
fn no_result() {}

fn main() {}
