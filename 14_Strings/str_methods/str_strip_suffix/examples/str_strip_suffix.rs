fn main() {
    println!("{:?}", "main.rs".strip_suffix(".rs"));
    println!("{:?}", "main.py".strip_suffix(".rs"));

    // Line endings: this reports whether the line was terminated.
    for raw in ["done\n", "done"] {
        println!("{raw:<7?} -> {:?}", raw.strip_suffix('\n'));
    }

    // Balanced unquoting: both ends, or nothing.
    for q in ["\"hi\"", "\"hi", "hi"] {
        let inner = q.strip_prefix('"').and_then(|s| s.strip_suffix('"'));
        println!("{q:<6?} -> {inner:?}");
    }

    // Once, not repeatedly.
    println!("{:?}", "a///".strip_suffix('/'));
    println!("{:?}", "a///".trim_end_matches('/'));
}
