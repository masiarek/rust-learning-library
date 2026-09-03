//! Reborrowing: why `&mut` acts like `Copy` when you pass it, and moves when you bind it.
//!
//!   rustc --edition 2024 reborrowing.rs -o /tmp/reborrowing && /tmp/reborrowing

fn bump(n: &mut i32) {
    *n += 1;
}

/// A struct holding a `&mut`. Every `&mut self` method reborrows the field.
struct Counter<'a> {
    n: &'a mut i32,
}

impl Counter<'_> {
    fn bump(&mut self) {
        *self.n += 1; // `self.n` is reborrowed for the length of this call
    }
}

fn main() {
    println!("1. Passing a `&mut` to a function does not move it");
    let mut total = 0;
    let r = &mut total;
    bump(r);
    bump(r);
    bump(r);
    println!("   bump(r) three times       -> *r = {}", *r);
    println!("   each call passed `&mut *r`, inserted for you — `r` itself stayed put");

    println!();
    println!("2. A bare `let` moves it — unless the line says what type it wants");
    let annotated: &mut i32 = r; // an expected type is present, so this REBORROWS
    bump(annotated);
    println!("   let a: &mut i32 = r       -> *a = {}", *annotated);
    let inner = &mut *r; // the explicit spelling, which needs no annotation
    bump(inner);
    println!("   let inner = &mut *r       -> *inner = {}", *inner);
    bump(r); // `inner`'s last use was the line above, so the reborrow has ended
    println!("   ...and r works again      -> *r = {}", *r);

    println!();
    println!("3. A shared reborrow: one `&mut` hands out as many `&` as you like");
    let a = &*r;
    let b = &*r;
    println!("   &*r twice                 -> {a} and {b}");

    println!();
    println!("4. A method call reborrows its receiver, which is the only reason a loop works");
    let mut seats = vec![1, 2, 3];
    let sr = &mut seats;
    for x in 4..=6 {
        sr.push(x); // `(&mut *sr).push(x)` — a move would end the loop after one turn
    }
    println!("   sr.push() three times     -> {sr:?}");

    println!();
    println!("5. `&mut self` is the same insertion, one level in");
    let mut score = 10;
    let mut c = Counter { n: &mut score };
    c.bump();
    c.bump();
    println!("   c.bump() twice            -> score = {}", *c.n);

    println!();
    println!("6. The trap: `Option<&mut T>` matched by value moves the reference OUT");
    let mut n = 5;
    let mut slot: Option<&mut i32> = Some(&mut n);
    if let Some(x) = slot.as_deref_mut() {
        *x += 1;
    }
    if let Some(x) = slot.as_deref_mut() {
        *x += 1;
    }
    println!("   slot.as_deref_mut() twice -> {slot:?}");
    println!("   `if let Some(x) = slot` compiles once, then E0382 on the second line");
}
