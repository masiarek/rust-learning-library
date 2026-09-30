# Anki cards: Rust sets.  Every `code` block is compiled with rustc 1.98.0
# --edition 2024 and run; `expect` must match stdout exactly.

SITE = "https://masiarek.github.io/rust-learning-library/"
DECK = "Rust::Sets"
LESSON = ("set_operations", SITE + "26_Collections/set_operations/index.html")

CARDS = [

dict(id="set_method_vs_operator",
 front="<code>a.intersection(&amp;b)</code> vs <code>&amp;a &amp; &amp;b</code> &mdash; what does each return, and what does each cost?",
 back="<b>The method borrows both sets and returns a lazy iterator of <code>&amp;T</code>; the operator clones the members "
      "into a brand-new set.</b>"
      "<br><br>Nothing is allocated by the method until you <code>collect</code> &mdash; <code>.count()</code> or "
      "<code>.next()</code> never allocate at all. The operator needs <code>T: Clone</code>. Both leave <code>a</code> and "
      "<code>b</code> usable.",
 code='''use std::collections::BTreeSet;

fn main() {
    let a = BTreeSet::from([1, 2, 3, 4]);
    let b = BTreeSet::from([3, 4, 5]);
    let lazy: Vec<&i32> = a.intersection(&b).collect();
    let built: BTreeSet<i32> = &a & &b;
    println!("{lazy:?} {built:?} {}", a.intersection(&b).count());
}''',
 expect="[3, 4] {3, 4} 2",
 code_on="back",
 bridge="<b>Python:</b> <code>a.intersection(b)</code> and <code>a &amp; b</code> both build a new set eagerly. "
        "Rust's method is the lazy one.",
 link=LESSON,
 tags="rust sets iterators"),

dict(id="set_owned_bitor",
 front="Does this compile?",
 code='''use std::collections::HashSet;

fn main() {
    let a: HashSet<i32> = [1, 2].into_iter().collect();
    let b: HashSet<i32> = [2, 3].into_iter().collect();
    let c = a | b;
    println!("{}", c.len());
}''',
 code_on="front",
 fails="E0369",
 back="<b>No: E0369, <i>no implementation for <code>HashSet&lt;i32&gt; | HashSet&lt;i32&gt;</code></i>.</b>"
      "<br><br>std implements <code>BitOr</code>, <code>BitAnd</code>, <code>Sub</code> and <code>BitXor</code> for "
      "<code>&amp;HashSet</code> and <code>&amp;BTreeSet</code> only. Write <code>&amp;a | &amp;b</code>. "
      "Half-borrowed, <code>&amp;a | b</code>, is E0308. To pour one owned set into another, <code>a.extend(b)</code>.",
 bridge="<b>Python:</b> <code>a | b</code> works on plain sets and leaves both alone &mdash; which is exactly what "
        "Rust's <code>&amp;a | &amp;b</code> does.",
 link=LESSON,
 tags="rust sets compile-error"),

dict(id="set_f64",
 front="Does this compile?",
 code='''use std::collections::HashSet;

fn main() {
    let mut s: HashSet<f64> = HashSet::new();
    s.insert(1.0);
    println!("{}", s.len());
}''',
 code_on="front",
 fails="E0599",
 back="<b>No: E0599 on <code>insert</code> &mdash; <code>f64: Eq</code> and <code>f64: Hash</code> not satisfied.</b>"
      "<br><br>The type can be named and <code>new()</code> works; the first <code>insert</code> is refused. "
      "<code>NaN != NaN</code>, so equality is not an equivalence and no hash can agree with it. "
      "<code>BTreeSet&lt;f64&gt;</code> fails too (E0277, no <code>Ord</code>).",
 bridge="<b>Python:</b> floats are hashable, and <code>{1, 1.0, True}</code> is ONE member because the three are "
        "equal. In Rust they are three types and the question cannot arise.",
 link=LESSON,
 tags="rust sets traits compile-error"),

dict(id="set_insert_bool",
 front="What does this print?",
 code='''use std::collections::HashSet;

fn main() {
    let mut s: HashSet<&str> = HashSet::new();
    println!("{} {} {}", s.insert("a"), s.insert("a"), s.len());
    println!("{} {}", s.remove("a"), s.remove("a"));
}''',
 code_on="front",
 expect="true false 1\ntrue false",
 back="<b><code>insert</code> is <code>true</code> when the value was NOT there; <code>remove</code> is <code>true</code> "
      "when it was.</b>"
      "<br><br>So <code>if !seen.insert(x) { /* duplicate */ }</code> is one lookup, with no <code>contains</code> first.",
 bridge="<b>Python:</b> <code>s.add(x)</code> also ignores a repeat silently, but returns <code>None</code>. "
        "<b>ABAP:</b> <code>INSERT &hellip; INTO TABLE</code> sets <code>sy-subrc = 4</code> &mdash; the same bool.",
 link=LESSON,
 tags="rust sets"),

dict(id="set_subset_proper",
 front="<code>a.is_subset(&amp;a)</code> &mdash; true or false? And how do you ask for a <i>proper</i> subset?",
 back="<b>True: <code>is_subset</code> is &sube;, not &sub;.</b> The empty set is a subset of everything, too."
      "<br><br>There is no proper-subset method: write <code>x.is_subset(&amp;y) &amp;&amp; x != y</code>. "
      "<code>is_disjoint</code> is <i>the intersection is empty</i>, answered without building it.",
 code='''use std::collections::BTreeSet;

fn main() {
    let a = BTreeSet::from([1, 2, 3]);
    let small = BTreeSet::from([2, 3]);
    let empty: BTreeSet<i32> = BTreeSet::new();
    println!("{} {} {}", a.is_subset(&a), small.is_subset(&a) && small != a, empty.is_subset(&small));
    println!("{}", a.is_disjoint(&BTreeSet::from([9])));
}''',
 expect="true true true\ntrue",
 code_on="back",
 bridge="<b>Python:</b> <code>a &lt;= b</code> is <code>is_subset</code>; <code>a &lt; b</code> is the proper subset "
        "Rust has no operator for.",
 link=LESSON,
 tags="rust sets"),

dict(id="set_xor_chain",
 front="What does this print?",
 code='''use std::collections::BTreeSet;

fn main() {
    let x = BTreeSet::from([0, 1, 2, 3, 4]);
    let y = BTreeSet::from([2, 3, 4]);
    let z = BTreeSet::from([2, 5]);
    println!("{:?}", &(&x ^ &y) ^ &z);
}''',
 code_on="front",
 expect="{0, 1, 2, 5}",
 back="<b>A chain of <code>^</code> keeps the members that are in an ODD number of the sets.</b>"
      "<br><br><code>2</code> is in all three and survives; <code>3</code> and <code>4</code> are in two and cancel. "
      "It is XOR, member by member: <code>{1,2,3} ^ {3,4,5} ^ {5,6,7} ^ {7,8,1}</code> is <code>{2, 4, 6, 8}</code>.",
 link=LESSON,
 tags="rust sets math"),

dict(id="set_print_order",
 front="Why does this library never <code>println!(\"{:?}\", some_hash_set)</code>?",
 back="<b>The default hasher, <code>RandomState</code>, is seeded afresh for every process, so the same members print "
      "in a different order on each run.</b>"
      "<br><br>Print through a <code>BTreeSet</code> (or a sorted <code>Vec</code>). When the order matters to the "
      "program itself, choose <code>BTreeSet</code> in the first place: needs <code>Ord</code>, O(log n), ordered always.",
 code='''use std::collections::{BTreeSet, HashSet};

fn main() {
    let h: HashSet<i32> = [3, 1, 2].into_iter().collect();
    let b: BTreeSet<i32> = h.iter().copied().collect();
    println!("{b:?} {}", h == [2, 3, 1].into_iter().collect());
}''',
 expect="{1, 2, 3} true",
 code_on="back",
 bridge="<b>Python:</b> a set iterates the same way within one process for the same history, which tempts code to "
        "depend on it. Rust breaks that dependency on the first run.",
 link=LESSON,
 tags="rust sets hashing"),

dict(id="set_dedup_two_ways",
 front="Deduplicate <code>vec![3, 1, 1, 2, 3, 1]</code> keeping first-seen order &mdash; and what does plain "
       "<code>dedup()</code> leave?",
 back="<b>A <code>HashSet</code> <code>seen</code> as the filter: <code>seen.insert(n)</code> is true the first time only.</b>"
      "<br><br><code>sort()</code> + <code>dedup()</code> gives sorted order instead. <code>dedup()</code> alone "
      "removes ADJACENT repeats only.",
 code='''use std::collections::HashSet;

fn main() {
    let v = vec![3, 1, 1, 2, 3, 1];
    let mut seen = HashSet::new();
    let first: Vec<i32> = v.iter().copied().filter(|n| seen.insert(*n)).collect();
    let mut plain = v.clone();
    plain.dedup();
    println!("{first:?} {plain:?}");
}''',
 expect="[3, 1, 2] [3, 1, 2, 3, 1]",
 code_on="back",
 bridge="<b>ABAP:</b> <code>DELETE ADJACENT DUPLICATES</code> has the same trap, and the same fix: <code>SORT</code> first.",
 link=LESSON,
 tags="rust sets vec"),
]
