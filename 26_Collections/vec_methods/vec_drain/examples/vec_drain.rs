fn main() {
    // Removes a range and hands you the removed elements as an iterator.
    let mut v = vec![1, 2, 3, 4, 5];
    let taken: Vec<i32> = v.drain(1..3).collect();
    println!("taken {taken:?}  left {v:?}");

    // A full-range drain empties the vector but keeps the buffer — unlike
    // into_iter(), which consumes the vector itself.
    let mut v = vec![1, 2, 3];
    let cap = v.capacity();
    let all: Vec<i32> = v.drain(..).collect();
    println!("all {all:?}  v {v:?}  buffer kept {}", v.capacity() == cap);

    // The elements are moved out, so non-Clone types work.
    let mut owners = vec![String::from("Ada"), String::from("Ben")];
    let first: Vec<String> = owners.drain(..1).collect();
    println!("{first:?} then {owners:?}");

    // Dropping the Drain without collecting still removes the range.
    let mut v = vec![1, 2, 3, 4];
    v.drain(1..3);
    println!("dropped the iterator, range still gone: {v:?}");

    // It is double-ended and lazy in the usual way.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let back: Vec<i32> = v.drain(..).rev().take(2).collect();
    println!("last two, back to front: {back:?}  v {v:?}");

    // Compare the three ways of emptying a Vec:
    //   drain(..)      elements out, vector and buffer stay
    //   clear()        elements dropped, buffer stays
    //   into_iter()    elements out, the vector is consumed
    let mut a = vec![1, 2];
    let out: Vec<i32> = a.drain(..).collect();
    println!("drain -> {out:?}, a is still usable: {a:?}");

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| { let mut v = vec![1, 2]; v.drain(0..9); });
    std::panic::set_hook(hook);
    println!("drain past the end panicked: {}", caught.is_err());
}
