//! `counters!` again, joining the name with the unstable `${concat(..)}`.

macro_rules! counters {
    ($($name:ident),* $(,)?) => {
        pub struct Counters {
            $(pub $name: u64,)*
        }

        impl Counters {
            pub fn new() -> Self {
                Self { $($name: 0,)* }
            }

            $(pub fn ${concat(incr_, $name)}(&mut self) {
                self.$name += 1;
            })*
        }
    };
}

counters!(requests, errors);

fn main() {
    let mut counts = Counters::new();
    counts.incr_requests();
    println!("requests = {}", counts.requests);
}
