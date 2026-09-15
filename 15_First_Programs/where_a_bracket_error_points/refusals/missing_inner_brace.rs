struct Counter {
    n: u32,
}

impl Counter {
    fn get(&self) -> u32 {
        self.n

    fn bump(&mut self) {
        self.n += 1;
    }

    fn reset(&mut self) {
        self.n = 0;
    }
}

fn main() {
    let mut c = Counter { n: 0 };
    c.bump();
    c.reset();
    println!("{}", c.get());
}
