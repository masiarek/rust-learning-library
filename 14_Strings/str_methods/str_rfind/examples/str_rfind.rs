fn main() {
    let path = "/usr/local/share/doc";

    println!("{:?} {:?}", path.find('/'), path.rfind('/'));

    // The last segment, two ways.
    if let Some(i) = path.rfind('/') {
        println!("{:?}", &path[i + 1..]);
    }
    println!("{:?}", path.rsplit_once('/'));

    // Extensions: rfind takes the last dot, find would take the first.
    let file = "archive.tar.gz";
    println!("{:?} {:?}",
             file.find('.').map(|i| &file[i + 1..]),
             file.rfind('.').map(|i| &file[i + 1..]));

    // Offsets always count from the front.
    println!("{:?}", "a.b.c".rfind('.'));
    println!("{:?}", "abc".rfind('z'));
}
