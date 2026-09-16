//! `Borrow`: an owned key lent out as its borrowed form — and the promise,
//! which nothing checks, that the two compare and hash alike.
//!
//!   rustc --edition 2024 borrow_trait.rs -o /tmp/bt && /tmp/bt

use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher, RandomState};
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// The std docs' own example of a key that must not implement `Borrow<str>`:
/// equality, ordering and hashing all ignore ASCII case, and `str`'s do not.
struct CaseInsensitive(String);

impl PartialEq for CaseInsensitive {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for CaseInsensitive {}

impl Hash for CaseInsensitive {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in self.0.as_bytes() {
            c.to_ascii_lowercase().hash(state);
        }
    }
}

impl Ord for CaseInsensitive {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.to_ascii_lowercase().cmp(&other.0.to_ascii_lowercase())
    }
}

impl PartialOrd for CaseInsensitive {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Compiles without a murmur, and breaks the contract.
impl Borrow<str> for CaseInsensitive {
    fn borrow(&self) -> &str {
        &self.0
    }
}

fn main() {
    println!("1. A String key, looked up with a &str");
    let mut seats: HashMap<String, u32> = HashMap::new();
    seats.insert(String::from("Ada"), 3);
    println!("   seats.get(\"Ada\")                = {:?}", seats.get("Ada"));
    println!("   seats.get(&String::from(\"Ada\")) = {:?}   (T: Borrow<T> covers the owned form too)", seats.get(&String::from("Ada")));
    println!("   seats.get(\"ada\")                = {:?}   (str equality, so case counts)", seats.get("ada"));

    println!();
    println!("2. Every owned/borrowed pair works the same way");
    let by_path: HashMap<PathBuf, u32> = HashMap::from([(PathBuf::from("notes.txt"), 7)]);
    let verbs: HashSet<Vec<u8>> = HashSet::from([b"GET".to_vec()]);
    let boxed: BTreeMap<Box<str>, u32> = BTreeMap::from([(Box::from("Ada"), 3)]);
    let by_ref: HashMap<&str, u32> = HashMap::from([("Ada", 3)]);
    println!("   HashMap<PathBuf, _>.get(Path::new(\"notes.txt\")) = {:?}", by_path.get(Path::new("notes.txt")));
    println!("   HashSet<Vec<u8>>.contains(b\"GET\".as_slice())    = {}", verbs.contains(b"GET".as_slice()));
    println!("   BTreeMap<Box<str>, _>.get(\"Ada\")                = {:?}", boxed.get("Ada"));
    println!("   HashMap<&str, _>.get(\"Ada\")                     = {:?}   (&T: Borrow<T>)", by_ref.get("Ada"));

    println!();
    println!("3. The promise: the borrowed form hashes like the owned one");
    let s = RandomState::new();
    println!("   hash(String \"Ada\") == hash(&str \"Ada\")   : {}", s.hash_one(String::from("Ada")) == s.hash_one("Ada"));
    println!("   hash(&str \"Ada\")   == hash(&[u8] b\"Ada\") : {}", s.hash_one("Ada") == s.hash_one(b"Ada".as_slice()));
    let name = String::from("Ada");
    let bytes: &[u8] = name.as_ref();
    println!("   so String is AsRef<[u8]> ({bytes:?}) but not Borrow<[u8]>");

    println!();
    println!("4. A key that breaks the promise compiles anyway");
    println!("   hash(CaseInsensitive(\"Ada\")) == hash(\"Ada\") : {}", s.hash_one(CaseInsensitive(String::from("Ada"))) == s.hash_one("Ada"));
    let mut stock: BTreeMap<CaseInsensitive, u32> = BTreeMap::new();
    for (i, fruit) in ["apple", "Banana", "cherry"].into_iter().enumerate() {
        stock.insert(CaseInsensitive(String::from(fruit)), i as u32 + 1);
    }
    println!("   inserted \"apple\", \"Banana\", \"cherry\"");
    println!("   stock.get(&CaseInsensitive(\"BANANA\")) = {:?}", stock.get(&CaseInsensitive(String::from("BANANA"))));
    println!("   stock.get(\"cherry\")                   = {:?}", stock.get("cherry"));
    println!("   stock.get(\"Banana\")                   = {:?}   <- the exact key inserted", stock.get("Banana"));
    println!("   str orders \"Banana\" before \"apple\"; the map stored it after, and the search stopped at the first key.");

    println!();
    println!("5. The trap: with Borrow in scope, .borrow() on an Rc<RefCell<_>>");
    let scores = Rc::new(RefCell::new(vec![1, 2, 3]));
    // let total: i32 = scores.borrow().iter().sum();   // E0282: the trait method is found first
    let total: i32 = (*scores).borrow().iter().sum();
    let again: i32 = RefCell::borrow(&scores).iter().sum();
    println!("   (*scores).borrow()        sums to {total}");
    println!("   RefCell::borrow(&scores)  sums to {again}");
}
