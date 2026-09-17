use describe_mistakes::DescribeUsizeIndex;

#[derive(DescribeUsizeIndex)]
struct Meters(f64);

fn main() {
    println!("{:?}", Meters(5.5).describe());
}
