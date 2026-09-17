//! `counters!` as a `macro_rules!` macro. The struct and its fields expand
//! fine; the method names do not.

macro_rules! counters {
    ($($name:ident),* $(,)?) => {
        pub struct Counters {
            $(pub $name: u64,)*
        }

        impl Counters {
            pub fn new() -> Self {
                Self { $($name: 0,)* }
            }

            $(pub fn incr_$name(&mut self) {
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
