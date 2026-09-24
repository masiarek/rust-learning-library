//! Kata solution: eight one-line edits to `let r = &a;`, each predicted, then
//! run here if it compiles. The four refusals are rustc transcripts on the
//! page; a program cannot print an error it did not get.
//!
//!   rustc --edition 2024 pg_bind_kata.rs -o /tmp/pgbk && /tmp/pgbk

fn main() {
    println!("edit  the line(s) after let a / let b                 verdict");
    println!("1     let r = &a; r = &b;                             E0384: cannot assign twice to immutable variable r");
    {
        let a = 1;
        let b = 2;
        let mut r = &a;
        let first = *r;
        r = &b;
        println!("2     let mut r = &a; r = &b;                         compiles: *r was {first}, now {}", *r);
    }
    println!("3     let r = &a; *r = 5;                             E0594: cannot assign to *r, which is behind a & reference");
    {
        let mut a = 1;
        let r = &mut a;
        *r = 5;
        println!("4     let r = &mut a; *r = 5;                         compiles: a = {a}");
    }
    println!("5     let r = &mut a; *r = 5; r = &mut b;             E0384: the binding is immutable, whatever it points at");
    {
        let mut a = 1;
        let mut b = 2;
        let mut r = &mut a;
        *r = 5;
        r = &mut b;
        *r = 7;
        println!("6     let mut r = &mut a; *r = 5; r = &mut b; *r = 7; compiles: a = {a}, b = {b}");
    }
    println!("7     let r = &mut a; let y = &*r; *r += 1; use y;    E0506: *r is borrowed by y until y's last use");
    {
        let a = 1;
        let b = 2;
        let r = &mut &a;
        *r = &b;
        println!("8     let r = &mut &a; *r = &b;                       compiles: **r reads {}", **r);
    }
}
