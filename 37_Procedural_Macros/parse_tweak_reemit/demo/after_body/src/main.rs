use trace::trace_after_body;

/// The same `port` as in `trace_app`, under the macro that prints `leave` after the body.
#[trace_after_body]
fn port(address: &str) -> Result<u16, String> {
    let Some((_, port)) = address.split_once(':') else {
        return Err(format!("no port in {address:?}"));
    };
    let port = port.parse::<u16>().map_err(|e| e.to_string())?;
    Ok(port)
}

fn main() {
    println!("= {:?}\n", port("localhost:8080"));
    println!("= {:?}\n", port("localhost"));
    println!("= {:?}", port("localhost:http"));
}
