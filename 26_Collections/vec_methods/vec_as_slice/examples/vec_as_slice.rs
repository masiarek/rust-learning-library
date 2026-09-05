fn main() {
    let v = vec![1, 2, 3];
    let s: &[i32] = v.as_slice();
    println!("{s:?} len {}", s.len());

    // Free: a slice is the Vec's pointer and length, with the capacity left
    // behind. No copying, no allocation.
    println!("same buffer: {}", v.as_slice().as_ptr() == v.as_ptr());

    // You rarely have to write it, because Deref<Target = [T]> inserts it.
    // These three lines call the same slice method.
    println!("{:?} {:?} {:?}", v.first(), v.as_slice().first(), (*v).first());

    // Where you DO write it: to name the type at a coercion site the compiler
    // will not guess, and to hand a &[T] to a function.
    fn total(xs: &[i32]) -> i32 { xs.iter().sum() }
    println!("{} {} {}", total(&v), total(v.as_slice()), total(&v[..]));

    // A `let` is a coercion site too, and the ANNOTATION is what does the work:
    // a bare `let u = &v;` gives you a &Vec<i32>, not a slice.
    let u: &[i32] = &v;
    let w: &[_] = &v;                 // the element type can be inferred
    let plain = &v;                   // ...but this one is still a &Vec<i32>
    println!("{} {} {}", u.len(), w.len(), plain.capacity());

    // "Slices are read-only objects" is a sentence std's own Vec page uses,
    // and it is true of &[T] rather than of slices. &mut [T] is a slice too:
    // what a slice cannot change is the LENGTH.
    let mut m = vec![3, 1, 2];
    m.as_mut_slice().sort();
    m[..].reverse();
    println!("{m:?} — sorted and reversed through a slice, without touching the Vec");

    // Which is the argument for taking &[T] rather than &Vec<T> in a signature:
    // the same function then accepts an array and a slice too.
    let arr = [4, 5, 6];
    println!("{} {}", total(&arr), total(&arr[1..]));

    // Comparing a Vec with an array works through the same coercion.
    println!("{} {}", v.as_slice() == [1, 2, 3], v == vec![1, 2, 3]);

    // An empty Vec gives an empty slice, never a null one.
    let e: Vec<u8> = Vec::new();
    println!("empty slice len {} is_empty {}", e.as_slice().len(), e.as_slice().is_empty());
}
