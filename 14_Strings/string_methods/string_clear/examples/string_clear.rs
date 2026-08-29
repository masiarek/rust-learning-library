fn main() {
    let mut s = String::with_capacity(32);
    s.push_str("hello");
    println!("before len {} capacity {}", s.len(), s.capacity());
    s.clear();
    println!("after  len {} capacity {}", s.len(), s.capacity());

    // The reusable-buffer pattern: one allocation for the whole loop.
    let mut buf = String::new();
    for line in ["alpha", "beta", "gamma"] {
        buf.clear();
        buf.push_str(line);
        buf.push('!');
        println!("{buf:?} capacity {}", buf.capacity());
    }

    // Reassigning instead throws the buffer away.
    let mut fresh = String::with_capacity(32);
    fresh.push_str("x");
    fresh = String::new();
    println!("reassigned capacity {}", fresh.capacity());

    // Releasing the memory for real.
    let mut done = String::with_capacity(32);
    done.push_str("x");
    done.clear();
    done.shrink_to_fit();
    println!("cleared and shrunk: capacity {}", done.capacity());
}
