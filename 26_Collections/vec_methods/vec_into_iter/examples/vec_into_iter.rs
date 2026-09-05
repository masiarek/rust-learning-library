use std::slice;
use std::vec::IntoIter;

fn main() {
    // 1. Vec implements IntoIterator three times, and the receiver picks one.
    println!("1. Three impls, three item types");
    let mut v = vec![String::from("Ada"), String::from("Ben")];

    let by_ref: slice::Iter<'_, String> = (&v).into_iter();
    println!("   &Vec<T>     -> Item = &T       first {:?}", by_ref.clone().next());

    let by_mut: slice::IterMut<'_, String> = (&mut v).into_iter();
    for s in by_mut { s.push('!'); }
    println!("   &mut Vec<T> -> Item = &mut T   now {v:?}");

    let by_val: IntoIter<String> = v.clone().into_iter();
    println!("   Vec<T>      -> Item = T        collected {:?}", by_val.collect::<Vec<_>>());

    // 2. The associated types are the whole difference. Named out loud:
    println!("\n2. The named types");
    let borrowed = vec![1, 2, 3];
    let a: slice::Iter<'_, i32> = (&borrowed).into_iter();
    println!("   (&Vec<i32>).into_iter()      is slice::Iter<i32>,    count {}", a.count());
    let mut owned = vec![1, 2, 3];
    let b: slice::IterMut<'_, i32> = (&mut owned).into_iter();
    println!("   (&mut Vec<i32>).into_iter()  is slice::IterMut<i32>, count {}", b.count());
    let c: IntoIter<i32> = vec![1, 2, 3].into_iter();
    println!("   Vec<i32>.into_iter()         is vec::IntoIter<i32>,  count {}", c.count());

    // 3. A `for` loop is one of those three, chosen by what you wrote.
    println!("\n3. What `for` desugars to");
    let mut v = vec![1, 2, 3];
    let mut sum = 0;
    for n in &v { sum += *n; }                 // (&v).into_iter()
    for n in &mut v { *n *= 10; }              // (&mut v).into_iter()
    let owned: Vec<i32> = v.into_iter().collect();   // v.into_iter() — v is gone
    println!("   &v sum {sum}, &mut v then owned {owned:?}");

    // 4. into_iter() on a REFERENCE does not consume anything, despite the
    //    `self` in its signature: self IS the reference there.
    println!("\n4. `self` is the reference in two of the three");
    let v = vec![1, 2, 3];
    let n = (&v).into_iter().count();
    println!("   counted {n} through &v, and v is still here: {v:?}");

    // 5. iter() and iter_mut() are the unambiguous spellings. They are
    //    slice methods, reached through Deref — Vec does not define them.
    println!("\n5. iter() vs into_iter()");
    let v = vec![1, 2, 3];
    let same_type: bool = {
        let _a: slice::Iter<'_, i32> = v.iter();
        let _b: slice::Iter<'_, i32> = (&v).into_iter();
        true
    };
    println!("   v.iter() and (&v).into_iter() are the same type: {same_type}");
    println!("   iter() always yields &T; into_iter() depends on the receiver.");

    // 6. The pre-1.53 array surprise, and what it looks like today.
    println!("\n6. Arrays, before and after Rust 1.53");
    let arr = [1u8, 2, 3];
    let values: Vec<u8> = arr.into_iter().collect();     // Item = u8
    let refs: Vec<&u8> = arr.iter().collect();           // Item = &u8
    println!("   [u8; 3].into_iter() -> {values:?} (u8)");
    println!("   [u8; 3].iter()      -> {refs:?} (&u8)");
    println!("   Before 1.53 the first line yielded &u8: arrays had no by-value");
    println!("   impl, so method resolution fell through to the slice.");

    // 7. Why `[1, 2].iter().collect()` compiles but `[x, y].iter().collect()`
    //    does not: the literal array is promoted to a 'static constant, so the
    //    references outlive the statement. Nothing about Vec is involved.
    println!("\n7. Promotion, and why the same line can stop compiling");
    let from_literal: Vec<&i32> = [1, 2].iter().collect();
    let x = 1; let y = 2;
    let from_runtime: Vec<i32> = [x, y].into_iter().collect();   // note: into_iter
    println!("   [1, 2].iter().collect()          -> Vec<&i32>, len {}", from_literal.len());
    println!("   [x, y].into_iter().collect()     -> Vec<i32>,  len {}", from_runtime.len());
    println!("   [x, y].iter().collect() would be error[E0716] — the array is a");
    println!("   temporary. rustc offers two fixes: add `into_` (giving Vec<i32>),");
    println!("   or bind the array first (keeping the Vec<&i32> you asked for).");
    println!("   Debug prints both as {from_literal:?}, so the output cannot tell");
    println!("   you which is which — only the type can.");

    // 8. The error this all exists to explain.
    println!("\n8. Why the borrow checker cares");
    let names = vec![String::from("Ada")];
    let consumed: Vec<String> = names.into_iter().collect();
    // println!("{names:?}");   // error[E0382]: borrow of moved value: `names`
    println!("   after into_iter(), `names` is moved; only {consumed:?} remains");
}
