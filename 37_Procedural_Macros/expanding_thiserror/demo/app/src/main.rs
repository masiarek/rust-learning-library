use std::error::Error;
use std::io;

/// `?` converts the `io::Error` with whatever `From` impl `E` has.
fn read<E: From<io::Error>>() -> Result<String, E> {
    Err(io::Error::new(io::ErrorKind::NotFound, "config.toml"))?
}

/// An error's message, then each `source()` under it.
fn report(error: &dyn Error) {
    println!("  {error}");
    let mut source = error.source();
    while let Some(cause) = source {
        println!("    caused by: {cause}");
        source = cause.source();
    }
}

fn main() {
    let cause = || "80x".parse::<u16>().unwrap_err();

    println!("derived by thiserror");
    use config_error::ConfigError as Derived;
    report(&Derived::Empty);
    report(&read::<Derived>().unwrap_err());
    report(&Derived::BadPort { path: "config.toml".into(), cause: cause() });

    println!("written by hand");
    use by_hand::ConfigError as ByHand;
    report(&ByHand::Empty);
    report(&read::<ByHand>().unwrap_err());
    report(&ByHand::BadPort { path: "config.toml".into(), cause: cause() });
}
