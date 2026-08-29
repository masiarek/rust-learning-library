fn main() {
    let padded = "  \t hello world \n ";
    println!("{:?}", padded.trim());
    println!("{:?} unchanged", padded);

    // Only the ends; the interior is left alone.
    println!("{:?}", "  a  b  ".trim());

    // Borrowed, not allocated: the result is a window on the same bytes.
    let owner = String::from("  hi  ");
    let view: &str = owner.trim();
    println!("{:?} from a {}-byte owner", view, owner.len());

    // Unicode whitespace counts.
    println!("{:?}", "\u{00A0}hi\u{2003}".trim());

    // The input-handling reason it exists.
    for field in ["", "   ", " x "] {
        println!("{field:<5?} empty={:<5} trimmed empty={}",
                 field.is_empty(), field.trim().is_empty());
    }
}
