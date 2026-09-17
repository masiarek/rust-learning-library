//! Three functions a `#[trace]` must hand back unchanged apart from the body:
//! one with an early `return` and a `?`, one generic with a `where` clause, and
//! an `async fn`.

use trace::trace;

/// The port in `host:port`.
#[trace]
pub fn port(address: &str) -> Result<u16, String> {
    let Some((_, port)) = address.split_once(':') else {
        return Err(format!("no port in {address:?}"));
    };
    let port = port.parse::<u16>().map_err(|e| e.to_string())?;
    Ok(port)
}

#[trace]
pub fn largest<T>(items: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    let mut best = items.first()?;
    for item in items {
        if item > best {
            best = item;
        }
    }
    Some(best)
}

#[trace]
pub async fn double(n: u32) -> u32 {
    n * 2
}
