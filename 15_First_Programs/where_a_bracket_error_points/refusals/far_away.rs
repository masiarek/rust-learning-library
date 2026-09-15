fn count_long_words(text: &str) -> usize {
    let words: Vec<&str> = text.split_whitespace().collect();
    let long = words.iter().filter(|w| w.len() > 4).count());
    let short = words.len() - long;
    println!("{long} long, {short} short");
    if long > short {
        println!("mostly long words");
    } else {
        println!("mostly short words");
    }
    long
}

fn main() {
    let n = count_long_words("the quick brown fox jumps");
    println!("{n}");
}
