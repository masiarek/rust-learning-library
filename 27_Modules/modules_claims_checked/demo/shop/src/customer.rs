pub struct Customer {
    id: u64,
    name: String,
    email: String,
}

impl Customer {
    pub fn new(id: u64, name: String, email: String) -> Self {
        Self { id, name, email }
    }

    pub fn label(&self) -> String {
        format!("{} <{}>, customer #{}", self.name, self.email, self.id)
    }
}
