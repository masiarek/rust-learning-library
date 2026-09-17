use retry::retry;

#[retry(times = 3, delay = 100)]
fn unknown_key() -> Result<(), String> {
    Ok(())
}

#[retry(times = 3, delay_ms = 0.5)]
fn wrong_type() -> Result<(), String> {
    Ok(())
}

#[retry(times = 0)]
fn zero_times() -> Result<(), String> {
    Ok(())
}

#[retry(delay_ms = 100)]
fn missing_times() -> Result<(), String> {
    Ok(())
}

#[retry(times = "3")]
fn quoted_number() -> Result<(), String> {
    Ok(())
}

fn main() {}
