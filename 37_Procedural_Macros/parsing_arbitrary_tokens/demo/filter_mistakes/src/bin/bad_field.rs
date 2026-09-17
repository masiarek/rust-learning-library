use filter_macro::filter;

struct Order {
    status: &'static str,
    total: u32,
}

fn main() {
    let orders = [Order { status: "open", total: 1250 }];
    let count = orders.iter().filter(filter!(totl >= 5000)).count();
    println!("{count} {}", orders[0].status);
}
