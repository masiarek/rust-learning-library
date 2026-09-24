//! Exercise 3, solved: one `use` line brings `Season` and `is_holiday` into
//! `main`'s scope. Every season is checked, so no variant is dead.
//!
//!   rustc --edition 2024 pg6_03_use_seasons.rs -o /tmp/pg603 && /tmp/pg603

mod seasons {
    #[derive(Debug)]
    pub enum Season {
        Spring,
        Summer,
        Autumn,
        Winter,
    }

    pub fn is_holiday(season: &Season) -> bool {
        match season {
            Season::Summer => true,
            _ => false,
        }
    }
}

use seasons::{is_holiday, Season};

fn main() {
    let current_season = Season::Autumn;
    if is_holiday(&current_season) {
        println!("It's a holiday season! Time for a vacation!");
    } else {
        println!("Regular work season. Keep hustling!"); // this one
    }

    // The same call with no `use` at all: a full path works everywhere.
    println!("{}", seasons::is_holiday(&seasons::Season::Summer)); // true

    for season in [Season::Spring, Season::Summer, Season::Autumn, Season::Winter] {
        println!("{season:?}: holiday = {}", is_holiday(&season));
    }
}
