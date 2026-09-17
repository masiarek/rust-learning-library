//! Two calls to `println!`, and nothing else, so that the expansion fits on
//! the page.

fn main() {
    let name = "Ada";
    let items = 3;
    println!("{name} has {items} items in the cart"); // Ada has 3 items in the cart
    println!("{} bought {:?}", name, ["tea", "milk"]); // Ada bought ["tea", "milk"]
}
