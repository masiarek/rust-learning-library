use summary_comments::{call_site, mixed_site};

fn main() {
    let (author, text) = ("Ada".to_string(), "Nice page".to_string());
    let before = call_site::Comment {
        author: author.clone(),
        text: text.clone(),
    };
    let after = mixed_site::Comment { author, text };
    println!("call_site:  {}", before.summary());
    println!("mixed_site: {}", after.summary());
}
