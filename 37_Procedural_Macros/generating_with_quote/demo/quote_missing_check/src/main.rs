use quote_validate::{Validate, ValidateFromString};

#[derive(Validate)]
struct Reading {
    celsius: f64,
    sensor: String, // no check_sensor below
}

impl Reading {
    fn check_celsius(_: &f64) -> Result<(), String> {
        Ok(())
    }
}

#[derive(ValidateFromString)]
struct Price {
    cents: u64,
    currency: String, // no check_currency below
}

impl Price {
    fn check_cents(_: &u64) -> Result<(), String> {
        Ok(())
    }
}

fn main() {}
