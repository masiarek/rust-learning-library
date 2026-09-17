//! The working half of every pair on the page, called and printed. Each module
//! is one numbered section; the `Clone` half of each pair is the page's
//! `compile_fail` fence, which tools/check_fences.py proves still fails.
//!
//!   rustc --edition 2024 where_clone_will_not_do.rs -o /tmp/wcwnd && /tmp/wcwnd

use std::any::type_name_of_val;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

mod s1 {
    pub fn keep(name: &str) -> String {
        name.to_owned()
    }
}

mod s2 {
    pub fn keep(name: &str) -> String {
        (*name).to_owned()
    }
}

mod s3 {
    pub fn keep(scores: &[i32]) -> Vec<i32> {
        scores.to_owned()
    }
}

mod s4 {
    use std::path::{Path, PathBuf};

    pub fn keep(file: &Path) -> PathBuf {
        file.to_owned()
    }
}

mod s5 {
    pub fn refill(buf: &mut String, name: &str) {
        name.clone_into(buf);
    }
}

mod s6 {
    pub fn own_all<B: ToOwned + ?Sized>(items: &[&B]) -> Vec<B::Owned> {
        items.iter().map(|&item| item.to_owned()).collect()
    }
}

mod s7 {
    pub struct Keeper<B: ToOwned + ?Sized> {
        pub kept: B::Owned,
    }

    impl<B: ToOwned + ?Sized> Keeper<B> {
        pub fn new(borrowed: &B) -> Self {
            Keeper { kept: borrowed.to_owned() }
        }
    }
}

mod s8 {
    use std::borrow::Cow;

    pub fn underscores(label: &str) -> Cow<'_, str> {
        if label.contains(' ') {
            Cow::Owned(label.replace(' ', "_"))
        } else {
            Cow::Borrowed(label)
        }
    }
}

mod s9 {
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use std::hash::Hash;

    pub fn count<K, Q>(tally: &mut HashMap<K, u32>, word: &Q)
    where
        K: Borrow<Q> + Hash + Eq,
        Q: ToOwned<Owned = K> + Hash + Eq + ?Sized,
    {
        match tally.get_mut(word) {
            Some(n) => *n += 1,
            None => {
                tally.insert(word.to_owned(), 1);
            }
        }
    }
}

mod s10 {
    pub fn own_words(words: &[&str]) -> Vec<String> {
        words.iter().map(|&word| word.to_owned()).collect()
    }
}

fn sorted<K: Ord + Clone, V: Clone>(map: &HashMap<K, V>) -> Vec<(K, V)> {
    let mut pairs: Vec<(K, V)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs
}

fn main() {
    let kept = s1::keep("Ada");
    println!(" 1. keep(\"Ada\")                -> {kept:?}, {}", type_name_of_val(&kept));

    let kept = s2::keep("Ada");
    println!(" 2. (*name).to_owned()         -> {kept:?}, {}", type_name_of_val(&kept));

    let kept = s3::keep(&[3, 1, 2]);
    println!(" 3. keep(&[3, 1, 2])           -> {kept:?}, {}", type_name_of_val(&kept));

    let kept = s4::keep(Path::new("notes.txt"));
    println!(" 4. keep(Path::new(..))        -> {kept:?}, {}", type_name_of_val(&kept));

    let mut buf = String::with_capacity(32);
    let start = buf.as_ptr();
    s5::refill(&mut buf, "Grace");
    println!(" 5. refill(&mut buf, \"Grace\")  -> {buf:?}, same buffer: {}", buf.as_ptr() == start);

    let names = s6::own_all::<str>(&["Ada", "Grace"]);
    let rows = s6::own_all::<[i32]>(&[&[1, 2][..], &[3][..]]);
    println!(" 6. own_all::<str>             -> {names:?}, {}", type_name_of_val(&names));
    println!("    own_all::<[i32]>           -> {rows:?}, {}", type_name_of_val(&rows));

    let name = s7::Keeper::<str>::new("Ada");
    let bytes = s7::Keeper::<[u8]>::new(b"Ada");
    println!(" 7. Keeper::<str>::new         -> kept {:?}, {}", name.kept, type_name_of_val(&name.kept));
    println!("    Keeper::<[u8]>::new        -> kept {:?}, {}", bytes.kept, type_name_of_val(&bytes.kept));

    for label in ["first name", "surname"] {
        let variant = match s8::underscores(label) {
            Cow::Borrowed(s) => format!("Borrowed {s:?}"),
            Cow::Owned(s) => format!("Owned {s:?}"),
        };
        let call = format!("underscores({label:?})");
        println!(" 8. {call:<26} -> {variant}");
    }

    let mut words: HashMap<String, u32> = HashMap::new();
    for word in ["ada", "grace", "ada"] {
        s9::count(&mut words, word);
    }
    let mut chunks: HashMap<Vec<u8>, u32> = HashMap::new();
    for chunk in [&b"GET"[..], b"PUT", b"GET"] {
        s9::count(&mut chunks, chunk);
    }
    println!(" 9. count words, &str keys     -> {:?}", sorted(&words));
    println!("    count chunks, &[u8] keys   -> {:?}", sorted(&chunks));

    let owned = s10::own_words(&["Ada", "Grace"]);
    println!("10. own_words(&[\"Ada\", ..])    -> {owned:?}, {}", type_name_of_val(&owned));
}
