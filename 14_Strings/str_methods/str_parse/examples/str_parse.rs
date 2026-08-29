use std::str::FromStr;

#[derive(Debug)]
struct Rgb(u8, u8, u8);

impl FromStr for Rgb {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').map(str::trim).collect();
        match parts.as_slice() {
            [r, g, b] => Ok(Rgb(
                r.parse().map_err(|_| format!("bad red: {r}"))?,
                g.parse().map_err(|_| format!("bad green: {g}"))?,
                b.parse().map_err(|_| format!("bad blue: {b}"))?,
            )),
            _ => Err(format!("want 3 parts, got {}", parts.len())),
        }
    }
}

fn main() {
    // Two spellings of the same call; the type is chosen by inference.
    println!("{:?}", "42".parse::<i32>());
    let n: i32 = "42".parse().unwrap();
    println!("{n}");

    // It does not trim.
    println!("{:?}", " 42".parse::<i32>().is_err());
    println!("{:?}", " 42".trim().parse::<i32>());

    // Per-type grammars.
    println!("{:?}", "+7".parse::<i32>());
    println!("{:?}", "3.5".parse::<f64>());
    println!("{:?}", "true".parse::<bool>());
    println!("{:?}", "True".parse::<bool>().is_err());
    println!("{:?}", "300".parse::<u8>().is_err());

    // Your own type.
    let colour = "12, 34,56".parse::<Rgb>();
    println!("{colour:?}");
    if let Ok(Rgb(r, g, b)) = colour {
        println!("red {r}, green {g}, blue {b}");
    }
    println!("{:?}", "12,34".parse::<Rgb>());
    println!("{:?}", "12,34,300".parse::<Rgb>());
}
