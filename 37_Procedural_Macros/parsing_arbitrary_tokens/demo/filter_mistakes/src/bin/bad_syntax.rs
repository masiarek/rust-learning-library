use filter_macro::filter;

struct Order {
    status: &'static str,
    total: u32,
}

fn main() {
    let orders = [Order { status: "open", total: 1250 }];
    let _ = orders.iter().filter(filter!(status != "cancelled"));
    let _ = orders.iter().filter(filter!(status == "open"));
    let _ = orders.iter().filter(filter!(status = "open" total > 0));
    let _ = orders.iter().filter(filter!(status in ["open" "paid"]));
}
