fn main() {
    println!("{:?}", "--verbose".strip_prefix("--"));
    println!("{:?}", "plain".strip_prefix("--"));

    // Once, not repeatedly.
    println!("{:?}", "../../src".strip_prefix("../"));
    println!("{:?}", "../../src".trim_start_matches("../"));

    // No byte arithmetic, so a multi-byte prefix is safe.
    println!("{:?}", "→next".strip_prefix('→'));

    // Flag parsing: long form, then short.
    for arg in ["--verbose", "-v", "file.txt"] {
        let parsed = arg.strip_prefix("--").map(|n| format!("long {n}"))
            .or_else(|| arg.strip_prefix('-').map(|n| format!("short {n}")))
            .unwrap_or_else(|| format!("positional {arg}"));
        println!("{arg:<11} {parsed}");
    }
}
