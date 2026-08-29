fn main() {
    for name in ["a.b.c", "noext", ".hidden", "trailing."] {
        println!("{name:<10?} -> {:?}", name.rsplit_once('.'));
    }

    // Reading order, unlike rsplitn.
    let path = "/usr/local/bin/rustc";
    println!("{:?}", path.rsplit_once('/'));
    println!("{:?}", path.rsplitn(2, '/').collect::<Vec<&str>>());

    // host:port, where the host may itself contain colons.
    for addr in ["example.com:8080", "::1:9000"] {
        println!("{addr:<18?} -> {:?}", addr.rsplit_once(':'));
    }
}
