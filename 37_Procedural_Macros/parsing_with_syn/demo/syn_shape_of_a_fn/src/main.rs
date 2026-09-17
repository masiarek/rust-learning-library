use syn_shapes::shape_of;

fn main() {
    println!("{}", shape_of!(fn point() {})); // a function is not a DeriveInput
}
