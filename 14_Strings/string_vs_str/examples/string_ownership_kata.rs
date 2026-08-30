//! Kata solution: the five moves — pivot, lifetime, own, dangle, and Cow.
//!
//!   rustc --edition 2024 string_ownership_kata.rs -o /tmp/sok && /tmp/sok

use std::borrow::Cow;

// 1. The conversion pivot ----------------------------------------------------
//
// Two functions, opposite directions, and only one of them allocates.

/// `&str` in, `String` out: the only way out of a borrow is a copy of the bytes.
fn to_owned_text(s: &str) -> String {
    s.to_string()
}

/// `&String` in, `&str` out: no allocation, no copy — a view of bytes that
/// already exist. `s.as_str()`, `&s[..]` and a bare `s` (deref coercion at the
/// call site) are all the same thing; `as_str` is the one that says so.
fn to_borrowed_text(s: &String) -> &str {
    s.as_str()
}

// 2. A struct that borrows needs a lifetime ----------------------------------
//
// Written without one, it does not compile:
//
//   struct User {
//       username: &str,
//   }
//
//   error[E0106]: missing lifetime specifier
//    --> e0106.rs:2:15
//     |
//   2 |     username: &str,
//     |               ^ expected named lifetime parameter
//     |
//   help: consider introducing a named lifetime parameter
//     |
//   1 ~ struct User<'a> {
//   2 ~     username: &'a str,
//     |
//
// The compiler is not asking for decoration. A field holding a reference means
// the struct is only valid while the borrowed text is, and `'a` is where you
// write that down: `UserRef<'a>` may not outlive the string it points into.

struct UserRef<'a> {
    username: &'a str,
    ballots: u32,
}

impl<'a> UserRef<'a> {
    /// The elided lifetime on the return is `&self`'s, not `'a` — either works
    /// here, and `&self`'s is the more conservative of the two.
    fn name(&self) -> &str {
        self.username
    }
}

// 3. The same struct, owning its text ----------------------------------------

#[derive(Debug)]
struct User {
    username: String,
    ballots: u32,
}

// 4. The dangling reference --------------------------------------------------
//
// The function that cannot exist:
//
//   fn label() -> &'static str {
//       let s = String::from("Ada Lovelace");
//       &s
//   }
//
//   error[E0515]: cannot return reference to local variable `s`
//    --> e0515.rs:3:5
//     |
//   3 |     &s
//     |     ^^ returns a reference to data owned by the current function
//
// `s` is dropped at the closing brace, so the reference would point at freed
// memory. The fix is not a longer lifetime — no lifetime can outlive the drop.
// It is to change the return type and hand over the buffer itself.

fn label() -> String {
    let s = String::from("Ada Lovelace");
    s
}

// 5. Cow: borrow until somebody writes ---------------------------------------

/// Lowercase only when there is something to lowercase. The return type is one
/// type with two shapes, so the caller writes one line either way.
fn normalise(s: &str) -> Cow<'_, str> {
    if s.chars().any(char::is_uppercase) {
        Cow::Owned(s.to_lowercase())
    } else {
        Cow::Borrowed(s)
    }
}

fn which(c: &Cow<'_, str>) -> &'static str {
    match c {
        Cow::Borrowed(_) => "Borrowed — no allocation",
        Cow::Owned(_) => "Owned    — allocated",
    }
}

fn main() {
    println!("1. The conversion pivot");
    let literal = "score then automatic runoff";
    let owned = String::from("equal support is allowed");
    println!("   to_owned_text(&str)      -> String  {:?}", to_owned_text(literal));
    println!("   to_borrowed_text(&String)-> &str    {:?}", to_borrowed_text(&owned));
    println!("   one direction copies the bytes, the other only points at them.");

    println!("\n2. A borrowed field needs a lifetime");
    let name = String::from("ada");
    let borrower = UserRef { username: &name, ballots: 3 };
    println!("   UserRef {{ username: {:?}, ballots: {} }}", borrower.name(), borrower.ballots);
    println!("   `borrower` may not outlive `name` — that is all <'a> says.");

    println!("\n3. The owning version moves");
    let u = User { username: String::from("ada"), ballots: 3 };
    let moved = u;
    // println!("{u:?}");   // error[E0382]: borrow of moved value: `u`
    println!("   after `let moved = u;`  {moved:?}");
    println!("   read through the new name: {} cast {} ballots", moved.username, moved.ballots);
    println!("   `u` is gone: one String field is enough to make the whole struct move.");

    println!("\n4. The reference that cannot leave");
    println!("   label() -> String       {:?}   <- E0515 if it returned &str", label());

    println!("\n5. Cow pays only when it has to");
    for s in ["already lowercase", "Mixed Case Here"] {
        let c = normalise(s);
        println!("   {:<19} -> {:<19} {}", s, format!("{c:?}"), which(&c));
    }
    println!("   Both arms are one type, so the caller never branches: {}", normalise("Ada").len());
}
