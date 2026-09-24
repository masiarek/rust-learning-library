// Exercise 3.5: the Pythagorean triple with a + b + c = 1000. The book's search
// tries every c; the second search computes c and leaves both loops with a label.
// Each search also counts how many candidates it looked at.
fn search_every_c(total: i32) -> (Option<(i32, i32, i32)>, u64) {
    let mut candidates = 0;
    let mut found = None;
    let mut keep_going = true;
    for a in 1..=total {
        for b in a + 1..total {
            for c in b + 1..total {
                candidates += 1;
                if a * a + b * b == c * c && a + b + c == total {
                    found = Some((a, b, c));
                    keep_going = false;
                    break;
                }
            }
            if !keep_going {
                break;
            }
        }
        if !keep_going {
            break;
        }
    }
    (found, candidates)
}

fn search_with_c_computed(total: i32) -> (Option<(i32, i32, i32)>, u64) {
    let mut candidates = 0;
    let mut found = None;
    'search: for a in 1..total {
        for b in a + 1..total {
            let c = total - a - b;
            if c <= b {
                break; // b has grown past c: no larger b can work for this a
            }
            candidates += 1;
            if a * a + b * b == c * c {
                found = Some((a, b, c));
                break 'search;
            }
        }
    }
    (found, candidates)
}

fn main() {
    let (every_c, looked_at) = search_every_c(1000);
    println!("trying every c: {every_c:?} after {looked_at} candidates"); // trying every c: Some((200, 375, 425)) after 80778750 candidates
    let (computed, looked_at) = search_with_c_computed(1000);
    println!("computing c:    {computed:?} after {looked_at} candidates"); // computing c:    Some((200, 375, 425)) after 69676 candidates

    let (a, b, c) = computed.expect("the triple exists");
    println!("a < b < c: {}", a < b && b < c); // a < b < c: true
    println!("a² + b² = {} and c² = {}", a * a + b * b, c * c); // a² + b² = 180625 and c² = 180625
    println!("a + b + c = {}", a + b + c); // a + b + c = 1000
    println!("a · b · c = {}", a * b * c); // a · b · c = 31875000
}
