fn main() {
    let file = "a.b.c";

    println!("{:?}", file.splitn(2, '.').collect::<Vec<&str>>());
    println!("{:?}", file.rsplitn(2, '.').collect::<Vec<&str>>());

    // Right-to-left order: last field first, remainder last.
    println!("{:?}", file.rsplitn(3, '.').collect::<Vec<&str>>());

    // Stem and extension, the readable way.
    println!("{:?}", file.rsplit_once('.'));

    // A path's parent and final segment.
    let path = "/usr/local/bin/rustc";
    let mut it = path.rsplitn(2, '/');
    let name = it.next();
    let parent = it.next();
    println!("{name:?} in {parent:?}");
}
