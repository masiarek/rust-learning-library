counters_macro::counters!(requests, errors);

fn main() {
    let mut counts = Counters::new();
    counts.incr_requests();
    counts.incr_requests();
    counts.incr_errors();
    println!("requests = {}, errors = {}", counts.requests, counts.errors); // requests = 2, errors = 1
}
