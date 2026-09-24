pub struct Customer {
    id: u64,
    name: String,
    email: String,
}

impl Customer {
    fn new(id: u64, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}
