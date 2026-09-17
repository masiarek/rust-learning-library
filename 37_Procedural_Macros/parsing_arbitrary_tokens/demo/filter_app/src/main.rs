use filter_macro::filter;

struct Order {
    id: u32,
    status: &'static str,
    total: u32, // in cents
}

const ORDERS: [Order; 5] = [
    Order { id: 1, status: "open", total: 1250 },
    Order { id: 2, status: "paid", total: 8000 },
    Order { id: 3, status: "cancelled", total: 9900 },
    Order { id: 4, status: "open", total: 5000 },
    Order { id: 5, status: "paid", total: 1999 },
];

fn main() {
    let ids = |matched: Vec<&Order>| matched.iter().map(|order| order.id).collect::<Vec<_>>();

    let open: Vec<&Order> = ORDERS.iter().filter(filter!(status = "open")).collect();
    let large: Vec<&Order> = ORDERS
        .iter()
        .filter(filter!(status <> "cancelled" and total >= 5000))
        .collect();
    let small: Vec<&Order> = ORDERS
        .iter()
        .filter(filter!(status in ["open", "paid"] and total < 2000))
        .collect();

    println!("{:<24}{:?}", "open", ids(open)); // open                    [1, 4]
    println!("{:<24}{:?}", "not cancelled, >= 5000", ids(large)); // not cancelled, >= 5000  [2, 4]
    println!("{:<24}{:?}", "open or paid, < 2000", ids(small)); // open or paid, < 2000    [1, 5]
}
