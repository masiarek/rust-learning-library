struct Cart {
    items: u32,
}

fn main() {
    let cart = Cart { items: 3 };
    println!("{cart} has {} items", cart.items);
}
