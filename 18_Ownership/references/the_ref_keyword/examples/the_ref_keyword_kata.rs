//! Kata solution: one `let` pattern that moves one field and borrows the other.
//!
//!   rustc --edition 2024 the_ref_keyword_kata.rs -o /tmp/the_ref_keyword_kata && /tmp/the_ref_keyword_kata

struct Upload {
    name: String,
    bytes: Vec<u8>,
}

/// Takes the buffer by value: whoever calls this gives it up.
fn store(bytes: Vec<u8>) -> usize {
    bytes.len()
}

fn main() {
    let upload = Upload { name: String::from("notes.txt"), bytes: vec![7, 8, 9] };

    // `bytes` binds by value (moves out), `name` binds by reference (borrows).
    let Upload { ref name, bytes } = upload;
    let stored = store(bytes);
    println!("stored {stored} bytes for {name}");

    // `name` never left the struct, so the field is still readable.
    println!("upload.name afterwards: {}", upload.name);

    // Not these two — each is E0382, because `bytes` moved out of `upload`:
    //   println!("{}", upload.bytes.len());   // borrow of moved value: `upload.bytes`
    //   let whole = upload;                   // use of partially moved value: `upload`

    // Why `&upload` cannot do this in one pattern:
    //   let Upload { name, bytes } = &upload;          // bytes: &Vec<u8>, so store(bytes) is E0308
    //   let &Upload { ref name, bytes } = &upload;     // E0507: cannot move out of a shared reference
    println!("one pattern, two binding modes: `ref` switches one name, `&upload` switches them all");
}
