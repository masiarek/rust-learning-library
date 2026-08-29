fn main() {
    let file = "notes.tar.gz";

    println!("{}", file.ends_with(".gz"));
    println!("{}", file.ends_with('z'));
    println!("{}", file.ends_with(char::is_alphabetic));

    // Test and remove in one step.
    println!("{:?}", file.strip_suffix(".gz"));

    // Peeling two extensions.
    let mut name = file;
    while let Some(rest) = name.strip_suffix(".gz").or_else(|| name.strip_suffix(".tar")) {
        name = rest;
    }
    println!("stem = {name}");

    // A byte comparison, so case matters and a bare extension passes.
    println!("{} {} {}",
             "README.RS".ends_with(".rs"),
             "README.RS".to_lowercase().ends_with(".rs"),
             ".rs".ends_with(".rs"));
}
