//! Struct update syntax: `..base` fills the rest — by MOVING, one field at a time.
//!
//!   rustc --edition 2024 struct_update.rs -o /tmp/su && /tmp/su

#[derive(Debug, Default)]
struct User {
    active: bool,        // Copy
    sign_in_count: u64,  // Copy
    username: String,    // NOT Copy
    email: String,       // NOT Copy
}

fn main() {
    println!("1. What it saves you");
    let user1 = User {
        active: true,
        sign_in_count: 1,
        username: "someusername123".to_string(),
        email: "someone@example.com".to_string(),
    };
    // `..user1` says: every field I did not name, take from user1.
    let user2 = User { email: "another@example.com".to_string(), ..user1 };
    println!("   user2 = User {{ email: …, ..user1 }}");
    println!("   {user2:?}");
    println!("   `..base` must come LAST, and takes no trailing comma.");

    println!("\n2. It is an assignment, so it MOVES — and it moves per FIELD");
    println!("   user1.active        {}   <- bool is Copy, so it was copied", user1.active);
    println!("   user1.sign_in_count {}   <- u64 is Copy, so it was copied", user1.sign_in_count);
    println!("   user1.username      -- moved out, and now unusable:");
    println!("     error[E0382]: borrow of moved value: `user1.username`");
    println!("     note: move occurs because `user1.username` has type `String`,");
    println!("           which does not implement the `Copy` trait");
    println!("   user1.email         {}   <- NOT moved: user2 supplied its own", user1.email);
    println!("   user2.username      {}   <- this is user1's String, relocated", user2.username);
    println!("   So `user1` is not dead. It is PARTIALLY moved, field by field.");

    println!("\n3. Name every non-Copy field and the base survives intact");
    let base = User {
        active: true,
        sign_in_count: 7,
        username: "ada".to_string(),
        email: "ada@example.com".to_string(),
    };
    let clone_ish = User {
        username: "ben".to_string(),
        email: "ben@example.com".to_string(),
        ..base // only the two Copy fields come across
    };
    println!("   base is still whole:  {base:?}");
    println!("   and the new one:      {clone_ish:?}");

    println!("\n4. `..Default::default()` never strands anything");
    let partial = User { username: "cara".to_string(), ..Default::default() };
    println!("   {partial:?}");
    println!("   The base is a temporary nobody holds, so there is no binding");
    println!("   left half-moved. That is why config structs use this form.");

    println!("\n5. It is not a copy constructor");
    println!("   `..base` does not clone, and it does not call any of your code.");
    println!("   If you want `base` intact and a full duplicate, that is `.clone()`.");
}
