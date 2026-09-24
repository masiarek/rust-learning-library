//! Two independent choices — `let` or `let mut` on the binding, `&T` or
//! `&mut T` as the reference — and what each of the four combinations lets
//! you do; then the nested forms that §4.6 of *Rust: The Practical Guide*
//! (Nouman Azam, Rheinwerk 2025) calls the fifth and sixth "types of
//! reference", and the reborrow `&*z`. The page is
//! 18_Ownership/references/mutable_binding_vs_mutable_reference/README.md.
//!
//!   rustc --edition 2024 pg_bind_lesson.rs -o /tmp/pgbl && /tmp/pgbl

// Block 3 keeps the book's `&mut &vec_2` spelling on purpose; clippy's
// needless_borrow fires on exactly those two lines, and the page says so.
#[allow(clippy::needless_borrow)]
fn main() {
    println!("1. Two choices, four combinations");
    let mut a = 1;
    let mut b = 2;

    let r = &a;
    println!("   let r = &a;                              read *r = {}", *r);

    let mut r = &a;
    let first = *r;
    r = &b;
    println!("   let mut r = &a; r = &b;                  re-pointed: *r was {first}, now {}", *r);

    let r = &mut a;
    *r += 10;
    println!("   let r = &mut a; *r += 10;                wrote through: a = {a}");

    let mut r = &mut a;
    *r += 100;
    r = &mut b;
    *r += 100;
    println!("   let mut r = &mut a; ...; r = &mut b;     both: a = {a}, b = {b}");

    println!();
    println!("2. Nesting: the same two choices, taken again at each level");
    let vec_1 = vec![1, 2, 3];
    let vec_2 = vec![4, 5, 6];

    let mut inner = &vec_1;
    let r = &mut inner;
    *r = &vec_2;
    println!("   &mut &T:     let r = &mut inner; *r = &vec_2;      inner now {inner:?}, r itself never re-pointed");

    let mut n = 1;
    let m = &mut n;
    let rr = &m;
    println!("   & &mut T:    let rr = &m;                          read **rr = {}; **rr = 5 is E0594", **rr);

    let s = 7;
    let s1 = &s;
    let s2 = &s1;
    println!("   &&T:         let s2 = &s1;                         read **s2 = {}", **s2);

    let mut p = 1;
    let mut q = 2;
    let mut mp = &mut p;
    let rm = &mut mp;
    **rm += 10;
    *rm = &mut q;
    **rm += 10;
    println!("   &mut &mut T: **rm += 10; *rm = &mut q; **rm += 10;  p = {p}, q = {q}");

    println!();
    println!("3. Listing 4.33: *reference = &mut &vec_2 is a coercion, and *reference = &vec_2 is its plain spelling");
    let mut inner = &vec_1;
    let reference = &mut inner;
    *reference = &mut &vec_2;
    println!("   after *reference = &mut &vec_2:  inner = {inner:?}");
    let plain: &Vec<i32> = &mut &vec_1;
    println!("   let plain: &Vec<i32> = &mut &vec_1; -> {plain:?}  (the same coercion, on its own)");
    let mut inner = &vec_1;
    let reference = &mut inner;
    *reference = &vec_2;
    println!("   after *reference = &vec_2:       inner = {inner:?}");

    println!();
    println!("4. \"Not possible with other types of references\": an immutable binding of a &mut writes too");
    let mut vec_3 = vec![1, 2, 3];
    let r = &mut vec_3;
    r.push(10);
    *r = vec![9];
    println!("   let r = &mut vec_3; r.push(10); *r = vec![9];  -> vec_3 = {vec_3:?}");

    println!();
    println!("5. Listing 4.35: &*z is a reborrow, and let y: &i32 = z; is the same one");
    let mut x = 45;
    let z = &mut x;
    let y = &*z;
    let y2: &i32 = z;
    println!("   y = {y}, y2 = {y2}: two shared reborrows of *z, and z is frozen while they are used");
    *z += 1;
    println!("   after their last use, *z += 1 compiles: x = {x}");
}
