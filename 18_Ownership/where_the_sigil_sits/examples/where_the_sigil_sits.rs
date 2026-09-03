// The same two characters, `&` and `*`, do a different job in each of the
// three positions a Rust program has: a type, an expression, and a pattern.

#[derive(Debug, Clone)]
struct S(i32);

#[derive(Debug, Clone, Copy)]
struct C(i32);

fn main() {
    println!("──── One line, two different `&`");
    let a = S(1);
    let r: &S = &a;
    println!("  let r: &S = &a;");
    println!("  the first `&` named a TYPE, the second took an ADDRESS");
    println!("  a = {:?}, *r = {:?}", a, *r);
    println!("  and field access needs no `*` at all: r.0 = {}", r.0);

    println!();
    println!("──── In a pattern, `&` REMOVES a reference");
    let c = C(5);
    let rc: &C = &c;
    let &copied = rc;
    let total: i32 = [C(1), C(2), C(4)].iter().map(|&C(n)| n).sum();
    println!("  let &copied = rc;       copied = {:?}   (a C, not a &C)", copied);
    println!("  .map(|&C(n)| n).sum()   total  = {}", total);

    println!();
    println!("──── `*r` on the right READS the place; on the left it WRITES it");
    let mut b = S(1);
    let w: &mut S = &mut b;
    let before = w.clone();
    *w = S(2);
    println!("  before the write:  {:?}", before);
    println!("  b afterwards:      {:?}   (b itself changed)", b);

    println!();
    println!("──── Reading a place OUT is a copy, or a move");
    let mut d = C(9);
    let rd = &mut d;
    let taken = *rd;
    println!("  C is Copy:      let taken = *rd;  -> {:?}, d still {:?}", taken, d);
    let mut e = S(7);
    let re = &mut e;
    let old = std::mem::replace(re, S(8));
    println!("  S is not Copy:  mem::replace(..)  -> {:?}, e now  {:?}", old, e);

    println!();
    println!("──── The `*` in `*const S` is not a dereference");
    let p: *const S = &a;
    println!("  `*const S` is the NAME of a type; nothing was dereferenced.");
    println!("  the one `*` that does dereference: unsafe {{ &*p }} = {:?}", unsafe { &*p });
}
