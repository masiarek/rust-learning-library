use describe_mistakes::DescribeNamedOnly;

#[derive(DescribeNamedOnly)]
struct Meters(f64);

fn main() {
    println!("{:?}", Meters(5.5).describe());
}
