use describe_derive::Describe;

#[derive(Describe)]
union IntOrFloat {
    i: u32,
    f: f32,
}

fn main() {
    let value = IntOrFloat { f: 1.0 };
    println!("{}", unsafe { value.i });
}
