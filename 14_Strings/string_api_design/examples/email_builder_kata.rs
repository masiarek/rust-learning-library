//! Kata solution: a builder whose setters take `impl Into<String>` — and the
//! price of a `build` that borrows the builder so it can run twice.
//!
//!   rustc --edition 2024 email_builder_kata.rs -o /tmp/ebk && /tmp/ebk

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

/// Counts allocations and reallocations, as on the lesson page.
struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

fn counted<T>(f: impl FnOnce() -> T) -> (T, usize) {
    let before = ALLOCS.load(Relaxed);
    let out = f();
    (out, ALLOCS.load(Relaxed) - before)
}

struct Email {
    to: String,
    subject: String,
    body: String,
}

/// Starts as three empty `String`s, and an empty `String` owns no buffer.
#[derive(Default)]
struct EmailBuilder {
    to: String,
    subject: String,
    body: String,
}

impl EmailBuilder {
    fn new() -> Self {
        EmailBuilder::default()
    }

    // Each setter keeps its text, so each asks for anything that can become a
    // `String` — and hands the builder back, so the calls chain.
    fn to(mut self, to: impl Into<String>) -> Self {
        self.to = to.into();
        self
    }

    fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = subject.into();
        self
    }

    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    /// Consumes the builder, so its three `String`s move into the `Email`.
    fn build(self) -> Email {
        Email { to: self.to, subject: self.subject, body: self.body }
    }

    /// Borrows the builder so it can build again — and so must copy every field.
    fn build_again(&self) -> Email {
        Email { to: self.to.clone(), subject: self.subject.clone(), body: self.body.clone() }
    }
}

/// The same builder with `&str` setters, for comparison.
#[derive(Default)]
struct BorrowingBuilder {
    to: String,
    subject: String,
    body: String,
}

impl BorrowingBuilder {
    fn to(mut self, to: &str) -> Self {
        self.to = to.to_owned();
        self
    }

    fn subject(mut self, subject: &str) -> Self {
        self.subject = subject.to_owned();
        self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = body.to_owned();
        self
    }

    fn build(self) -> Email {
        Email { to: self.to, subject: self.subject, body: self.body }
    }
}

fn main() {
    println!("1. Literals: each field is copied once, because it has to be");
    let (_, n) = counted(EmailBuilder::new);
    println!("   EmailBuilder::new()        {n} allocations   (an empty String owns no buffer)");
    let (email, n) = counted(|| {
        EmailBuilder::new().to("ada@example.com").subject("Hello").body("See you at ten.").build()
    });
    println!("   three literal setters      {n} allocations");
    println!("   to {:?}, subject {:?}, body {:?}", email.to, email.subject, email.body);

    println!();
    println!("2. Strings the caller is finished with");
    let (to, subject, body) = (String::from("ben@example.com"), String::from("Minutes"), String::from("Attached."));
    let (_, n) = counted(move || EmailBuilder::new().to(to).subject(subject).body(body).build());
    println!("   three owned setters        {n} allocations   (each String moves in)");

    println!();
    println!("3. The same Strings, through &str setters");
    let (to, subject, body) = (String::from("ben@example.com"), String::from("Minutes"), String::from("Attached."));
    let (_, n) = counted(|| BorrowingBuilder::default().to(&to).subject(&subject).body(&body).build());
    println!("   three &str setters         {n} allocations   (Strings to give, copied anyway)");

    println!();
    println!("4. build(self) against build_again(&self)");
    let builder = EmailBuilder::new()
        .to(String::from("cara@example.com"))
        .subject(String::from("Reminder"))
        .body(String::from("Tomorrow at nine."));
    let (_, once) = counted(|| builder.build_again());
    let (_, ten) = counted(|| {
        for _ in 0..10 {
            std::hint::black_box(builder.build_again());
        }
    });
    let (_, consumed) = counted(move || builder.build());
    println!("   build_again(&self), once   {once} allocations   (every field cloned)");
    println!("   build_again(&self), ×10    {ten} allocations");
    println!("   build(self)                {consumed} allocations   (the fields move out)");
    println!("   A builder that can build twice pays one copy per field per build.");
    println!("   Consume it, unless the caller really does build more than once.");
}
