use summary_derive::Summary;

/// Unconventional, since constants are usually upper case, and legal.
#[allow(non_upper_case_globals)]
const text: &str = "untitled";

#[derive(Summary)]
struct Comment {
    author: String,
}

fn main() {
    let comment = Comment {
        author: "Ada".to_string(),
    };
    println!("{} {}", text, comment.summary());
}
