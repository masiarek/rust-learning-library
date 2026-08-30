fn main() {
    // append MOVES the elements across and leaves the source empty but alive.
    let mut a = vec![1, 2, 3];
    let mut b = vec![4, 5];
    a.append(&mut b);
    println!("a {a:?}  b {b:?}  b.capacity() {}", b.capacity() > 0);

    // That is the difference from extend_from_slice, which clones. Here the
    // elements are not cloneable at all and it still works.
    struct NotClone(u8);
    let mut xs = vec![NotClone(1)];
    let mut ys = vec![NotClone(2), NotClone(3)];
    xs.append(&mut ys);
    let tags: Vec<u8> = xs.iter().map(|n| n.0).collect();
    println!("moved {tags:?} — non-Clone values — source now len {}", ys.len());

    // It takes &mut, so both vectors have to be reachable and distinct.
    // v.append(&mut v) does not compile: two mutable borrows of one value.
    let mut v = vec!["a"];
    let mut w = vec!["b"];
    v.append(&mut w);
    println!("{v:?}");

    // The source keeps its allocation, so it is cheap to refill.
    let mut src = vec![1u8, 2, 3, 4];
    let cap_before = src.capacity();
    let mut dst = vec![];
    dst.append(&mut src);
    println!("source kept its buffer: {}", src.capacity() == cap_before);
    src.push(9);
    println!("refilled without allocating: {src:?}");

    // Three vectors into one, in order.
    let mut all = vec![];
    for mut part in [vec![1, 2], vec![3], vec![4, 5]] {
        all.append(&mut part);
    }
    println!("{all:?}");
}
