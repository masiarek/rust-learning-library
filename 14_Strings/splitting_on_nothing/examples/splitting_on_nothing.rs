//! `"abc".split("")` yields FIVE pieces, not three. `split` reports the gaps
//! between matches — n matches, n+1 pieces — and the empty pattern matches at
//! every char boundary, of which "abc" has four. There is no special case
//! anywhere in this; the arithmetic is the whole explanation.
//!
//! Run:  rustc --edition 2024 splitting_on_nothing.rs && ./splitting_on_nothing

fn main() {
    let s = "abc";

    println!("1. Three characters, five pieces");
    println!("   split(\"\")           {:?}", s.split("").collect::<Vec<&str>>());
    println!("   chars()             {:?}", s.chars().collect::<Vec<char>>());

    println!();
    println!("2. Where the five come from — the matches, at their byte offsets");
    println!("   match_indices(\"\")   {:?}", s.match_indices("").collect::<Vec<(usize, &str)>>());
    println!("   {} matches, {} pieces — the same n+1 as any other pattern",
             s.matches("").count(), s.split("").count());
    println!("   \"xaxbxcx\".split('x') {:?}   <- four matches too",
             "xaxbxcx".split('x').collect::<Vec<&str>>());

    println!();
    println!("3. Even the empty string has one position in it");
    println!("   \"\".split(',')       {:?}   0 matches, 1 piece", "".split(',').collect::<Vec<&str>>());
    println!("   \"\".split(\"\")        {:?}   1 match,  2 pieces", "".split("").collect::<Vec<&str>>());

    println!();
    println!("4. The positions are CHAR boundaries, not byte offsets");
    let u = "añb";
    println!("   \"añb\".len()         {} bytes, {} chars", u.len(), u.chars().count());
    println!("   char_indices()      {:?}", u.char_indices().collect::<Vec<(usize, char)>>());
    println!("   split(\"\")           {:?}", u.split("").collect::<Vec<&str>>());
    println!("   byte 2 is inside 'ñ': is_char_boundary(2) = {}, and no piece starts there",
             u.is_char_boundary(2));

    println!();
    println!("5. ...but a char boundary is not a LETTER boundary");
    let accented = "a\u{301}";                  // 'a' + combining acute, one á on screen
    println!("   {:?} renders as {}    -> {:?}",
             accented, accented, accented.split("").collect::<Vec<&str>>());
    let modified = "\u{1F44D}\u{1F3FD}";        // thumbs-up + skin-tone modifier
    println!("   {:?} renders as {}      -> {} pieces: the two ends, and the emoji cut in two",
             modified, modified, modified.split("").count());

    println!();
    println!("6. The rest of the family, all on the empty pattern");
    println!("   split_terminator    {:?}", s.split_terminator("").collect::<Vec<&str>>());
    println!("   split_inclusive     {:?}", s.split_inclusive("").collect::<Vec<&str>>());
    println!("   rsplit              {:?}", s.rsplit("").collect::<Vec<&str>>());
    println!("   splitn(3, \"\")       {:?}   <- 3 pieces, and only ONE of them a character",
             s.splitn(3, "").collect::<Vec<&str>>());
    println!("   split_once(\"\")      {:?}   <- never None, for any string",
             s.split_once(""));
    println!("   rsplit_once(\"\")     {:?}   <- reading order, where rsplitn(2) gives {:?}",
             s.rsplit_once(""), s.rsplitn(2, "").collect::<Vec<&str>>());

    println!();
    println!("7. Each character as a &str, which is what split(\"\") is usually reached for");
    println!("   filtered            {:?}", s.split("").filter(|p| !p.is_empty()).collect::<Vec<&str>>());
    let sliced: Vec<&str> = u.char_indices().map(|(i, c)| &u[i..i + c.len_utf8()]).collect();
    println!("   from char_indices   {:?}   <- same idea, and it never made an empty", sliced);
}
