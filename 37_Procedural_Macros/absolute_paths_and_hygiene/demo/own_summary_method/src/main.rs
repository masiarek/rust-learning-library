use summary_derive::Summary;

#[derive(Summary)]
struct Comment {
    author: String,
    text: String,
}

impl Comment {
    /// The user's own `summary`, written before anyone derived one.
    fn summary(&self) -> String {
        format!("{}: {}", self.author, self.text)
    }
}

fn main() {
    let comment = Comment {
        author: "Ada".to_string(),
        text: "Nice page".to_string(),
    };
    println!("{}", comment.summary());
}
