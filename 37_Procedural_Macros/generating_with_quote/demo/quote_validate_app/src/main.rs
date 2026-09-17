use quote_validate::Validate;

#[derive(Validate)]
struct Reading {
    celsius: f64,
    sensor: String,
}

impl Reading {
    fn check_celsius(celsius: &f64) -> Result<(), String> {
        if *celsius < -273.15 { Err("below absolute zero".to_string()) } else { Ok(()) }
    }

    fn check_sensor(sensor: &str) -> Result<(), String> {
        if sensor.is_empty() { Err("no sensor name".to_string()) } else { Ok(()) }
    }
}

fn main() {
    let good = Reading { celsius: 21.5, sensor: "kitchen".to_string() };
    let cold = Reading { celsius: -300.0, sensor: "freezer".to_string() };
    println!("{:?}", good.validate());
    println!("{:?}", cold.validate());
}
