# Anki cards: Rust ownership, borrowing, lifetimes.
# Verified by verify.py -- rustc 1.98.0, edition 2024.

SITE = "https://masiarek.github.io/rust-learning-library/"
DECK = "Rust::Ownership"

CARDS = [

dict(id="own_what_is_a_move",
 front="What actually HAPPENS to the data when a value is moved?",
 back="<b>Nothing. The bytes stay exactly where they are.</b>"
      "<br><br>A move is not a copy and not a free &mdash; it is a <b>transfer of responsibility</b>. What changes is "
      "<i>who will free the value, and therefore when</i>. The old name is marked unusable at compile time; no "
      "instruction runs."
      "<br><br>For a <code>String</code>, the three-word header (pointer, len, capacity) is copied to the new stack "
      "slot. The heap text is never touched, never relocated, never duplicated."
      "<br><br>So \"moves are expensive\" is backwards: a move is at most a few words, and is usually optimised away.",
 code='''fn main() {
    let a = String::from("hello world");
    let heap_before = a.as_ptr();
    let b = a;                      // move
    println!("{}", heap_before == b.as_ptr());
    println!("{b}");
}''',
 expect="true\nhello world",
 code_on="back",
 bridge="<b>Python:</b> nothing corresponds &mdash; every binding is a new reference to the same object and the GC "
        "decides when it dies. In Rust the compiler decides, statically, and a move is how it knows.",
 link=("ownership_and_moves", SITE+"18_Ownership/ownership_and_moves/index.html"),
 tags="rust ownership core"),

dict(id="own_move_is_absence",
 front="Why does the compiler explain a move error by naming a trait you never wrote?",
 code='''fn take(s: String) -> usize { s.len() }

fn main() {
    let s = String::from("hi");
    take(s);
    println!("{s}");
}''',
 code_on="front",
 fails="E0382",
 back="<b><code>E0382</code>: <i>value used after move</i> &mdash; and the note says <i>\"which does not implement the "
      "<code>Copy</code> trait\"</i>.</b>"
      "<br><br>Moving is what <b>every</b> type does by default, so there is nothing to implement to make a type move. "
      "<code>Copy</code> is the <b>opt-out that stops it</b>. That is why the compiler explains a move as an "
      "<b>absence</b> &mdash; it is telling you which escape hatch you did not have."
      "<br><br>There is no <code>Move</code> trait to go looking for.",
 link=("no_move_trait", SITE+"18_Ownership/no_move_trait/index.html"),
 tags="rust ownership compile-error core"),

dict(id="own_which_are_copy",
 front="Which types are <code>Copy</code>? State the rule, not a list.",
 back="<b>A type is <code>Copy</code> if it is entirely made of bits that can be duplicated with no bookkeeping &mdash; "
      "and every one of its fields is also <code>Copy</code>.</b>"
      "<br><br>Yes: all the integers and floats, <code>bool</code>, <code>char</code>, <code>&amp;T</code> (a shared "
      "reference), arrays and tuples of <code>Copy</code> things."
      "<br><br>No: <code>String</code>, <code>Vec</code>, <code>Box</code>, and <b><code>&amp;mut T</code></b> &mdash; "
      "copying a unique reference would make it not unique, which is the whole point of it."
      "<br><br>Your own struct is <code>Copy</code> only if you write <code>#[derive(Copy, Clone)]</code>, and only if "
      "every field allows it. <code>Copy</code> implies <code>Clone</code>, never the reverse.",
 code='''#[derive(Copy, Clone, Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 1, y: 2 };
    let q = p;                    // copy, not move
    println!("{p:?} {q:?}");

    let n = 5;
    let m = n;
    println!("{n} {m}");
}''',
 expect="Point { x: 1, y: 2 } Point { x: 1, y: 2 }\n5 5",
 code_on="back",
 link=("copy_vs_clone", SITE+"16_Structs/copy_vs_clone/index.html"),
 tags="rust ownership core"),

dict(id="own_no_heap_keyword",
 front="Which keyword puts a value on the heap in Rust?",
 back="<b>There isn't one. <code>let</code> ALWAYS makes a stack slot.</b>"
      "<br><br>Whether that slot <i>is</i> the data or a <i>pointer to</i> the data is a property of the <b>type</b>, "
      "not of how you declared it. <code>let n = 5</code> puts four bytes on the stack; "
      "<code>let s = String::from(\"hi\")</code> puts a three-word header on the stack that points at heap text."
      "<br><br>That is what prices every move, copy and clone you will ever write &mdash; and it is why "
      "<code>size_of</code> is a constant per type and never depends on the contents."
      "<br><br><code>Box::new(x)</code> is the explicit heap allocation, and it is a <i>function call</i>, not syntax.",
 code='''fn main() {
    println!("{}", size_of::<i32>());
    println!("{}", size_of::<String>());
    println!("{}", size_of::<Box<[u8; 4096]>>());
    println!("{}", size_of::<[u8; 4096]>());
}''',
 expect="4\n24\n8\n4096",
 code_on="back",
 bridge="<b>Python:</b> everything is on the heap and every name is a pointer &mdash; which is why Python needs no "
        "such distinction and gives you no way to make one.",
 link=("stack_and_heap", SITE+"18_Ownership/stack_and_heap/index.html"),
 tags="rust ownership memory core"),

dict(id="own_address_shows",
 front="You print <code>{:p}</code> for a <code>&amp;String</code>. Whose address is it &mdash; the header or the text?",
 back="<b>The three-word header on the STACK. Never the text on the heap.</b>"
      "<br><br>So after a move the printed number changes &mdash; without a single byte of the text relocating. "
      "Printing addresses to \"see\" a move therefore shows you the opposite of what people expect it to."
      "<br><br><code>s.as_ptr()</code> is the one that gives the heap address, and <b>that</b> is the number that "
      "stays the same across a move.",
 code='''fn main() {
    let a = String::from("hello world");
    let heap = a.as_ptr();
    let stack_a = &a as *const String;
    let b = a;
    let stack_b = &b as *const String;
    println!("heap same:  {}", heap == b.as_ptr());
    println!("stack same: {}", stack_a == stack_b);
}''',
 expect="heap same:  true\nstack same: false",
 code_on="back",
 link=("what_an_address_shows", SITE+"18_Ownership/what_an_address_shows/index.html"),
 tags="rust ownership memory"),

dict(id="own_borrow_rule",
 front="State the borrowing rule in one line.",
 back="<b>Many readers, or one writer. Never both at once.</b>"
      "<br><br>Any number of <code>&amp;T</code>, OR exactly one <code>&amp;mut T</code>, for any given value at any "
      "given moment."
      "<br><br>It is not a rule about <i>lines of code</i> &mdash; it is about <b>where the compiler thinks each borrow "
      "ended</b>, which is the part that actually decides whether your code compiles."
      "<br><br>The payoff is bigger than memory safety: it is what makes data races impossible, and what lets the "
      "compiler reason about aliasing hard enough to optimise well.",
 code='''fn main() {
    let mut v = vec![1, 2, 3];
    let a = &v;
    let b = &v;                 // many readers: fine
    println!("{} {}", a.len(), b.len());

    let m = &mut v;             // the reads above have ended, so this is fine
    m.push(4);
    println!("{v:?}");
}''',
 expect="3 3\n[1, 2, 3, 4]",
 code_on="back",
 link=("borrowing", SITE+"18_Ownership/borrowing/index.html"),
 tags="rust ownership borrow-checker core"),

dict(id="own_nll",
 front="When does a borrow END &mdash; at the closing brace, or somewhere else?",
 back="<b>At its LAST USE, not at the end of the scope.</b> (Non-lexical lifetimes, since Rust 2018.)"
      "<br><br>This is the single most useful thing to know about the borrow checker, because it turns a large class "
      "of \"obviously fine\" code from an error into a non-event &mdash; and because pre-2018 advice on the internet "
      "still says otherwise."
      "<br><br>So the fix for a borrow error is often just <b>to move the last use earlier</b>, or to introduce a "
      "<code>{ }</code> block that ends the borrow explicitly. You rarely need to clone.",
 code='''fn main() {
    let mut s = String::from("hi");
    let r = &s;
    println!("{r}");        // last use of r -- the borrow ends HERE
    s.push_str(" there");   // so this is fine
    println!("{s}");
}''',
 expect="hi\nhi there",
 code_on="back",
 link=("borrowing", SITE+"18_Ownership/borrowing/index.html"),
 tags="rust ownership borrow-checker core"),

dict(id="own_two_mut",
 front="Does this compile?",
 code='''fn main() {
    let mut v = vec![1, 2, 3];
    let a = &mut v;
    let b = &mut v;
    a.push(4);
    b.push(5);
}''',
 code_on="front",
 fails="E0499",
 back="<b>No &mdash; <code>E0499</code>: cannot borrow <code>v</code> as mutable more than once at a time.</b>"
      "<br><br>Note WHY it fails here but not always: <code>a</code> is still used after <code>b</code> is created, so "
      "its borrow is still live. Delete the <code>a.push(4)</code> line and it compiles &mdash; the first borrow would "
      "have ended before the second began."
      "<br><br>That is the practical shape of almost every <code>E0499</code>: not \"two mutable borrows exist\" but "
      "\"two mutable borrows OVERLAP\".",
 link=("borrowing", SITE+"18_Ownership/borrowing/index.html"),
 tags="rust ownership borrow-checker compile-error"),

dict(id="own_mutate_while_iterating",
 front="Does this compile?",
 code='''fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {
        if *x == 2 { v.push(99); }
    }
    println!("{v:?}");
}''',
 code_on="front",
 fails="E0502",
 back="<b>No &mdash; <code>E0502</code>: cannot borrow <code>v</code> as mutable because it is also borrowed as "
      "immutable.</b>"
      "<br><br><code>for x in &amp;v</code> holds a shared borrow for the whole loop. Pushing could reallocate and move "
      "every element, invalidating the loop's own cursor."
      "<br><br>Fixes: collect what you want to add and push it <b>after</b> the loop; or loop over indices "
      "<code>0..v.len()</code>; or use <code>retain</code> / <code>drain</code> if you are removing.",
 bridge="<b>Python:</b> the same code runs and silently misbehaves &mdash; mutating a list while iterating skips "
        "elements. <b>C++:</b> it is undefined behaviour. Rust makes it a compile error.",
 link=("borrowing", SITE+"18_Ownership/borrowing/index.html"),
 tags="rust ownership borrow-checker compile-error gotcha"),

dict(id="own_return_local_ref",
 front="Does this compile?",
 code='''fn make<'a>() -> &'a str {
    let s = String::from("hi");
    &s
}

fn main() { println!("{}", make()); }''',
 code_on="front",
 fails="E0515",
 back="<b>No &mdash; <code>E0515</code>: cannot return reference to local variable <code>s</code>.</b>"
      "<br><br>The <code>&lt;'a&gt;</code> does not save it, and this is exactly why: <b>a lifetime annotation does not "
      "make anything live longer.</b> <code>s</code> is dropped at the closing brace regardless of what you named."
      "<br><br>Fixes: return the <code>String</code> itself (give up ownership), or return "
      "<code>&amp;'static str</code> if the text is a literal, or take a reference <i>in</i> and return one derived "
      "from it.",
 link=("lifetime_annotations", SITE+"18_Ownership/lifetime_annotations/index.html"),
 tags="rust ownership lifetimes compile-error"),

dict(id="own_lifetime_meaning",
 front="What does <code>&lt;'a&gt;</code> actually DO?",
 back="<b>Nothing at runtime. It NAMES a relationship between lifetimes that already exist, so the compiler can refuse "
      "the arrangement where one outlives the other.</b>"
      "<br><br>It is a constraint you are stating, not a capability you are granting. <code>fn longest&lt;'a&gt;(a: "
      "&amp;'a str, b: &amp;'a str) -&gt; &amp;'a str</code> says \"the result may not outlive either input\" &mdash; it "
      "does not extend either one."
      "<br><br>Lifetimes are erased entirely before code generation. There is no lifetime in the binary.",
 code='''fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let x = String::from("hello");
    let out;
    {
        let y = String::from("hi");
        out = longest(&x, &y).len();   // use it while BOTH are alive
    }
    println!("{out}");
}''',
 expect="5",
 code_on="back",
 link=("lifetime_annotations", SITE+"18_Ownership/lifetime_annotations/index.html"),
 tags="rust ownership lifetimes"),

dict(id="own_elision",
 front="Why do most functions taking <code>&amp;str</code> need no lifetime annotation at all?",
 back="<b>Lifetime elision &mdash; three rules the compiler applies before asking you.</b>"
      "<br><br>1. Each elided <b>input</b> reference gets its own fresh lifetime."
      "<br>2. If there is <b>exactly one</b> input lifetime, it is given to every elided output."
      "<br>3. If one of the inputs is <code>&amp;self</code> or <code>&amp;mut self</code>, <b>its</b> lifetime is given "
      "to every elided output."
      "<br><br>Rule 3 is why methods almost never need annotations. You are forced to write them when there are two "
      "input references and an output reference &mdash; the compiler will not guess which one the result came from."
      "<br><br>So a lifetime error is usually not \"I need to learn lifetimes\" but \"the elision rules ran out\".",
 code='''fn first_word(s: &str) -> &str {          // rule 2: one input, so output borrows it
    s.split_whitespace().next().unwrap_or("")
}

struct Doc { text: String }
impl Doc {
    fn head(&self) -> &str { &self.text[..2] }   // rule 3: borrows self
}

fn main() {
    println!("{}", first_word("hello there"));
    println!("{}", Doc { text: "abcd".into() }.head());
}''',
 expect="hello\nab",
 code_on="back",
 link=("how_to_learn_lifetimes", SITE+"18_Ownership/how_to_learn_lifetimes/index.html"),
 tags="rust ownership lifetimes"),

dict(id="own_clone_to_write",
 front="\"Just <code>.clone()</code> it\" gets you past most borrow errors. Name the case where it compiles, runs, and is WRONG.",
 code='''fn add(v: &mut Vec<i32>, n: i32) { v.push(n); }
fn add_cloned(mut v: Vec<i32>, n: i32) { v.push(n); }

fn main() {
    let mut a = vec![5, 3];
    add(&mut a, 4);
    println!("{a:?}");

    let b = vec![5, 3];
    add_cloned(b.clone(), 4);
    println!("{b:?}");
}''',
 code_on="front",
 expect="[5, 3, 4]\n[5, 3]",
 back="<b>Cloning to escape a MUTATION error compiles, runs, and silently does nothing.</b>"
      "<br><br>The push landed &mdash; on the clone, which was then dropped. No error, no warning, no panic. "
      "The compiler cannot help here, because mutating a copy is a perfectly legal thing to want."
      "<br><br>So the advice <i>\"copy and clone everything, obey the compiler\"</i> is good &mdash; it is the fastest "
      "way past the wall &mdash; but with one amendment: <b>it works for READ errors and betrays you on WRITE "
      "errors.</b> If you cloned to fix a borrow error and the value should have changed, check that it did.",
 link=("how_to_learn_lifetimes", SITE+"18_Ownership/how_to_learn_lifetimes/index.html"),
 tags="rust ownership gotcha core"),

dict(id="own_clone_two_meanings",
 front="<code>Vec::clone</code> and <code>Rc::clone</code> are the same method name. What is the difference?",
 back="<b>Opposite meanings. <code>Vec::clone</code> duplicates the DATA. <code>Rc::clone</code> duplicates a pointer "
      "and bumps a COUNT &mdash; and never touches the data.</b>"
      "<br><br><code>Rc::clone</code> is the cheapest <code>.clone()</code> in Rust and the most commonly misread one: "
      "people avoid it thinking it is expensive, and people call it thinking they got an independent copy. Neither."
      "<br><br>House style: write <code>Rc::clone(&amp;a)</code> rather than <code>a.clone()</code>, precisely so the "
      "reader can see at the call site which of the two things is happening.",
 code='''use std::rc::Rc;

fn main() {
    let v = vec![1, 2, 3];
    let w = v.clone();
    println!("{}", v.as_ptr() == w.as_ptr());

    let a = Rc::new(vec![1, 2, 3]);
    println!("{}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("{} {}", Rc::strong_count(&a), a.as_ptr() == b.as_ptr());
}''',
 expect="false\n1\n2 true",
 code_on="back",
 link=("reference_counting", SITE+"18_Ownership/reference_counting/index.html"),
 tags="rust ownership gotcha"),

dict(id="own_rc_vs_arc",
 front="<code>Rc</code> vs <code>Arc</code> &mdash; and why is the difference not a performance note?",
 back="<b><code>Arc</code> is <code>Rc</code> with an ATOMIC counter &mdash; and that is the reason one of them "
      "compiles across a thread boundary and the other does not.</b>"
      "<br><br><code>Rc</code> is not <code>Send</code>, so passing one to another thread is a compile error, not a "
      "race you have to find. Use <code>Rc</code> single-threaded (it is faster), <code>Arc</code> when a value "
      "genuinely crosses threads."
      "<br><br>Both give <b>shared, immutable</b> access. To mutate you need an interior-mutability cell inside: "
      "<code>Rc&lt;RefCell&lt;T&gt;&gt;</code> single-threaded, <code>Arc&lt;Mutex&lt;T&gt;&gt;</code> across threads. "
      "Those two pairings are worth memorising as pairings.",
 code='''use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || { *c.lock().unwrap() += 1; }));
    }
    for h in handles { h.join().unwrap(); }
    println!("{}", counter.lock().unwrap());
}''',
 expect="4",
 code_on="back",
 link=("sharing_across_threads", SITE+"18_Ownership/sharing_across_threads/index.html"),
 tags="rust ownership concurrency"),

dict(id="own_refcell",
 front="<code>RefCell</code> moves the borrow check from compile time to where?",
 back="<b>Runtime &mdash; and a violation is a PANIC, not a compile error.</b>"
      "<br><br><code>borrow()</code> and <code>borrow_mut()</code> keep the same rule (many readers or one writer) with "
      "a counter, and <code>borrow_mut()</code> while a borrow is live panics with "
      "<code>already borrowed: BorrowMutError</code>."
      "<br><br>So it is a deliberate trade, not a loophole: you take on a runtime failure to express a shape the "
      "static checker cannot verify &mdash; typically a graph, an observer list, or a cache."
      "<br><br><code>try_borrow_mut()</code> returns a <code>Result</code> if you would rather not panic.",
 code='''use std::cell::RefCell;

fn main() {
    let c = RefCell::new(vec![1, 2]);
    c.borrow_mut().push(3);
    println!("{:?}", c.borrow());

    let _live = c.borrow();
    println!("{}", c.try_borrow_mut().is_err());
}''',
 expect="[1, 2, 3]\ntrue",
 code_on="back",
 link=("reference_counting", SITE+"18_Ownership/reference_counting/index.html"),
 tags="rust ownership"),

dict(id="own_shadow_vs_mut",
 front="<code>let x = 5; let x = x + 1;</code> vs <code>let mut x = 5; x = x + 1;</code> &mdash; what is physically different?",
 back="<b><code>mut</code> writes into the SAME place. A shadow builds a SECOND place and moves the name onto it.</b>"
      "<br><br>So a shadow may change the type; a <code>mut</code> assignment may not. And the way to prove the two "
      "are different is not to print addresses but to <b>take a reference</b>: the borrow checker accepts the shadow "
      "and rejects the <code>mut</code> spelling of the same four lines, because the shadow never wrote through the "
      "place the reference points at."
      "<br><br>Use a shadow when the new binding is <b>the same concept in a new form</b> (the parsed version of a "
      "string). Use a second name when it is a different thing. Never shadow something holding a resource.",
 code='''fn main() {
    let spaces = "   ";
    let spaces = spaces.len();      // shadow: &str becomes usize
    println!("{spaces}");

    let s = String::from("hi");
    let r = &s;
    let s = s.len();                // shadow -- r is still valid
    println!("{r} {s}");
}''',
 expect="3\nhi 2",
 code_on="back",
 link=("a_name_is_not_a_place", SITE+"18_Ownership/a_name_is_not_a_place/index.html"),
 tags="rust ownership shadowing"),

dict(id="own_shadow_no_drop",
 front="You shadow a <code>String</code>. Is the old one dropped at that point?",
 back="<b>No. Shadowing takes away a NAME, not a VALUE.</b>"
      "<br><br>The shadowed <code>String</code> is still alive, still owned, still borrowable &mdash; which is why a "
      "reference taken <i>before</i> the shadow keeps working <i>after</i> it. It is dropped at the end of the scope, "
      "in the normal way, along with everything else."
      "<br><br>Consequence worth knowing: shadowing a <code>MutexGuard</code> or a file handle does <b>not</b> release "
      "it early. Use <code>drop(x)</code> or a <code>{ }</code> block for that."
      "<br><br>And: <b>rustc has no lint for a shadowed variable.</b> What gets mistaken for protection is the type "
      "error a wrong shadow trips on its way past &mdash; so when the shadow's type matches what it hides, there is "
      "nothing between you and a wrong answer.",
 code='''fn main() {
    let s = String::from("original");
    let r = &s;
    let s = String::from("shadow");
    println!("{r} / {s}");
}''',
 expect="original / shadow",
 code_on="back",
 link=("shadowing_does_not_drop", SITE+"18_Ownership/shadowing_does_not_drop/index.html"),
 tags="rust ownership shadowing gotcha"),

dict(id="own_drop_order",
 front="Three values declared in one scope. In what order do they drop?",
 back="<b>Reverse declaration order &mdash; last declared, first dropped.</b>"
      "<br><br>It has to be that way: a later value may borrow an earlier one, so the earlier one must outlive it. "
      "Struct <i>fields</i>, by contrast, drop in <b>declaration</b> order."
      "<br><br>This matters whenever drop has an effect you can observe: a lock released, a file flushed, a "
      "transaction committed, a log line written."
      "<br><br><code>drop(x)</code> ends one early. You cannot call <code>x.drop()</code> &mdash; that would leave the "
      "compiler planning to drop it a second time, so it is a compile error and the free function is the only door.",
 code='''struct Noisy(&'static str);
impl Drop for Noisy {
    fn drop(&mut self) { println!("drop {}", self.0); }
}

fn main() {
    let _a = Noisy("a");
    let _b = Noisy("b");
    {
        let _inner = Noisy("inner");
        println!("-- leaving block --");
    }
    println!("-- leaving main --");
}''',
 expect="-- leaving block --\ndrop inner\n-- leaving main --\ndrop b\ndrop a",
 code_on="back",
 link=("scope_is_about_names", SITE+"18_Ownership/scope_is_about_names/index.html"),
 tags="rust ownership drop"),

dict(id="own_scope_three_questions",
 front="\"It goes out of scope\" answers three different questions. What are they, and do they answer at the same moment?",
 back="<b>No &mdash; three different moments.</b>"
      "<br><br>1. <b>When can I still write this name?</b> &rarr; until the closing brace (or until a shadow takes it)."
      "<br>2. <b>When is the value freed?</b> &rarr; at the closing brace, in reverse declaration order."
      "<br>3. <b>When does a borrow stop mattering?</b> &rarr; at its <b>last use</b>, which is usually much earlier."
      "<br><br>That is why one phrase explains a drop, a compile error and an unlocked critical section differently &mdash; "
      "and why \"just add a scope\" sometimes fixes a borrow error and sometimes does nothing. If the problem is (3), "
      "moving the last use is enough; if it is (2), you need the block.",
 link=("scope_is_about_names", SITE+"18_Ownership/scope_is_about_names/index.html"),
 tags="rust ownership core"),

dict(id="own_partial_move",
 front="Does this compile?",
 code='''struct User { name: String, age: u32 }

fn main() {
    let u = User { name: String::from("ada"), age: 36 };
    let name = u.name;
    println!("{name} {}", u.age);
    let u2 = u;
}''',
 code_on="front",
 fails="E0382",
 back="<b>No &mdash; <code>E0382</code>: use of partially moved value <code>u</code>.</b>"
      "<br><br>But note what DOES work: after <code>let name = u.name;</code> the line "
      "<code>println!(\"{name} {}\", u.age)</code> is fine. The struct is <b>partially moved</b> &mdash; the "
      "<code>String</code> field is gone, the <code>Copy</code> field is still readable."
      "<br><br>What you cannot do is use <code>u</code> <b>as a whole</b> again: move it, pass it by value, or "
      "<code>{:?}</code> it."
      "<br><br>Fixes: <code>u.name.clone()</code>, or borrow <code>&amp;u.name</code>, or destructure the whole struct "
      "at once with <code>let User { name, age } = u;</code>.",
 link=("ownership_and_moves", SITE+"18_Ownership/ownership_and_moves/index.html"),
 tags="rust ownership compile-error"),

dict(id="own_disjoint_fields",
 front="Can you hold <code>&amp;mut</code> to two different FIELDS of the same struct at once?",
 back="<b>Yes &mdash; the borrow checker tracks fields separately. But not through a METHOD.</b>"
      "<br><br><code>&amp;mut p.a</code> and <code>&amp;mut p.b</code> are disjoint borrows and both are fine. "
      "<code>p.get_a_mut()</code> and <code>p.get_b_mut()</code> are not, because each takes <code>&amp;mut self</code> "
      "&mdash; borrowing the <b>whole</b> struct."
      "<br><br>This is the most common reason people conclude the borrow checker is being unreasonable: the direct "
      "field access works and the accessor does not. The usual fixes are to inline the field access, to split the "
      "struct, or to return both at once from one method.",
 code='''struct Pair { a: Vec<i32>, b: Vec<i32> }

fn main() {
    let mut p = Pair { a: vec![1], b: vec![2] };
    let x = &mut p.a;
    let y = &mut p.b;      // disjoint field -- fine
    x.push(9);
    y.push(9);
    println!("{:?} {:?}", p.a, p.b);
}''',
 expect="[1, 9] [2, 9]",
 code_on="back",
 link=("borrowing", SITE+"18_Ownership/borrowing/index.html"),
 tags="rust ownership borrow-checker"),

dict(id="own_move_closure",
 front="When do you need <code>move</code> on a closure?",
 back="<b>When the closure must OUTLIVE the scope that created it &mdash; a thread, a returned closure, anything "
      "stored.</b>"
      "<br><br>Without <code>move</code>, a closure captures by reference where it can, and that reference cannot "
      "outlive its scope. <code>move</code> forces every capture to be taken by value."
      "<br><br>Two things it does not mean:"
      "<br>• it does not make the closure <code>FnOnce</code> &mdash; that is decided by what the body <i>does</i> "
      "with the captures"
      "<br>• for a <code>Copy</code> type it copies rather than moves, so the original stays usable",
 code='''use std::thread;

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn main() {
    let add5 = make_adder(5);
    println!("{}", add5(10));

    let name = String::from("ada");
    let h = thread::spawn(move || format!("hi {name}"));
    println!("{}", h.join().unwrap());
}''',
 expect="15\nhi ada",
 code_on="back",
 link=("closures", SITE+"23_Closures/index.html"),
 tags="rust ownership closures"),

dict(id="own_param_by_value",
 front="What does <code>fn f(s: String)</code> tell the caller, versus <code>fn f(s: &amp;str)</code>?",
 back="<b>Taking by value is a claim that you NEED ownership &mdash; you will store it, consume it, or return it "
      "transformed.</b>"
      "<br><br>The parameter type is documentation the compiler enforces:"
      "<br>• <code>&amp;str</code> / <code>&amp;[T]</code> &mdash; I will read it and give it back"
      "<br>• <code>&amp;mut T</code> &mdash; I will change it in place"
      "<br>• <code>T</code> &mdash; I am keeping it; you do not have it any more"
      "<br><br>Default to borrowing. Taking <code>String</code> when you only read it forces every caller to clone, "
      "and that cost is invisible at the call site.",
 code='''fn read(s: &str) -> usize { s.len() }
fn grow(s: &mut String) { s.push('!'); }
fn keep(s: String) -> String { s }

fn main() {
    let mut s = String::from("hi");
    println!("{}", read(&s));
    grow(&mut s);
    println!("{s}");
    let owned = keep(s);        // s is gone from here on
    println!("{owned}");
}''',
 expect="2\nhi!\nhi!",
 code_on="back",
 link=("borrowing", SITE+"18_Ownership/borrowing/index.html"),
 tags="rust ownership api-design"),

dict(id="own_mem_take",
 front="You need to move a value OUT of a struct field you only have <code>&amp;mut</code> to. How?",
 back="<b><code>std::mem::take(&amp;mut field)</code> &mdash; takes the value and leaves <code>Default::default()</code> "
      "behind.</b>"
      "<br><br>The struct is never left with a hole, so the borrow checker is satisfied. Three in the family:"
      "<br>• <code>mem::take(&amp;mut x)</code> &mdash; leaves the default (needs <code>T: Default</code>)"
      "<br>• <code>mem::replace(&amp;mut x, val)</code> &mdash; leaves what you supply"
      "<br>• <code>mem::swap(&amp;mut a, &amp;mut b)</code> &mdash; exchanges two"
      "<br><br><code>Option::take()</code> is the same idea specialised: takes the <code>Some</code>, leaves "
      "<code>None</code>. It is how you write a linked list or a state machine that consumes its own state.",
 code='''use std::mem;

struct Buf { rows: Vec<i32> }

fn drain(b: &mut Buf) -> Vec<i32> { mem::take(&mut b.rows) }

fn main() {
    let mut b = Buf { rows: vec![1, 2, 3] };
    println!("{:?} {:?}", drain(&mut b), b.rows);

    let mut slot = Some(String::from("hi"));
    println!("{:?} {:?}", slot.take(), slot);
}''',
 expect='[1, 2, 3] []\nSome("hi") None',
 code_on="back",
 link=("ownership_and_moves", SITE+"18_Ownership/ownership_and_moves/index.html"),
 tags="rust ownership"),

dict(id="own_cow",
 front="One function must return either borrowed or owned text, deciding at runtime. What type?",
 back="<b><code>Cow&lt;'a, str&gt;</code> &mdash; an ordinary enum with a <code>Borrowed</code> arm and an "
      "<code>Owned</code> arm.</b>"
      "<br><br>It lets a function avoid deciding in advance, so the common case (nothing to change) allocates nothing "
      "and the rare case still works. <code>String::from_utf8_lossy</code> is the std example."
      "<br><br><code>to_mut()</code> is the moment the copy actually happens &mdash; that is the <i>write</i> that "
      "\"clone on write\" is named after. <code>into_owned()</code> forces it.",
 code='''use std::borrow::Cow;

fn clean(s: &str) -> Cow<'_, str> {
    if s.contains(' ') { Cow::Owned(s.replace(' ', "_")) } else { Cow::Borrowed(s) }
}

fn main() {
    let a = clean("already_clean");
    let b = clean("has spaces");
    println!("{}", matches!(a, Cow::Borrowed(_)));
    println!("{}", matches!(b, Cow::Owned(_)));
    println!("{a} {b}");
}''',
 expect="true\ntrue\nalready_clean has_spaces",
 code_on="back",
 link=("clone_on_write", SITE+"18_Ownership/clone_on_write/index.html"),
 tags="rust ownership performance"),

dict(id="own_box",
 front="Name the three jobs <code>Box&lt;T&gt;</code> actually does.",
 back="<b>1. Put a large value on the heap so moves stay cheap.<br>"
      "2. Give a recursive type a known size.<br>"
      "3. Hold a trait object: <code>Box&lt;dyn Trait&gt;</code>.</b>"
      "<br><br>Job 2 is the one that is a hard requirement rather than a choice: "
      "<code>enum List { Cons(i32, List), Nil }</code> has infinite size and will not compile; "
      "<code>Box&lt;List&gt;</code> is one pointer, so the size is known."
      "<br><br>Job 3 is how you get a heterogeneous collection &mdash; <code>Vec&lt;Box&lt;dyn Shape&gt;&gt;</code> &mdash; "
      "or return one of two different concrete types from one function."
      "<br><br><code>Box</code> is a single owner. Sharing needs <code>Rc</code>/<code>Arc</code>.",
 code='''#[derive(Debug)]
enum List { Cons(i32, Box<List>), Nil }

fn main() {
    use List::{Cons, Nil};
    let l = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{l:?}");

    let shapes: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
    ];
    println!("{:?}", shapes.iter().map(|f| f(10)).collect::<Vec<_>>());
}''',
 expect="Cons(1, Cons(2, Nil))\n[11, 20]",
 code_on="back",
 link=("stack_and_heap", SITE+"18_Ownership/stack_and_heap/index.html"),
 tags="rust ownership memory"),

dict(id="own_static",
 front="What does <code>&amp;'static T</code> mean &mdash; and what is the common misreading?",
 back="<b>It means the reference is valid for the whole program. The misreading is thinking it makes the value live "
      "that long.</b>"
      "<br><br>It is a <b>constraint</b>, like every lifetime. String literals are genuinely <code>&amp;'static str</code> "
      "because their bytes are in the binary."
      "<br><br>The trap is <code>T: 'static</code> as a bound, which is a <b>different and much weaker</b> claim: it "
      "means \"contains no non-static references\", which every OWNED type satisfies. A <code>String</code> is "
      "<code>'static</code> by that bound and is dropped whenever you like. That is why <code>thread::spawn</code> "
      "requires <code>'static</code> and yet accepts a moved <code>String</code> happily.",
 code='''fn needs_static<T: 'static>(_x: T) {}

fn main() {
    let lit: &'static str = "in the binary";
    println!("{lit}");
    needs_static(String::from("owned, and still 'static by the bound"));
    println!("ok");
}''',
 expect="in the binary\nok",
 code_on="back",
 link=("lifetime_annotations", SITE+"18_Ownership/lifetime_annotations/index.html"),
 tags="rust ownership lifetimes gotcha"),

dict(id="own_deref_coercion",
 front="Why does a function taking <code>&amp;str</code> accept a <code>&amp;String</code> with no conversion at the call site?",
 back="<b>Deref coercion &mdash; the compiler inserts <code>&amp;*s</code> for you when the types do not match and a "
      "<code>Deref</code> impl bridges them.</b>"
      "<br><br>The chain that matters in practice: <code>&amp;String</code> &rarr; <code>&amp;str</code>, "
      "<code>&amp;Vec&lt;T&gt;</code> &rarr; <code>&amp;[T]</code>, <code>&amp;Box&lt;T&gt;</code> &rarr; <code>&amp;T</code>."
      "<br><br>It is also why <code>String</code> appears to have hundreds of methods it does not define &mdash; "
      "<code>split</code>, <code>trim</code>, <code>find</code> all live on <code>str</code> and arrive through the "
      "same mechanism."
      "<br><br>It goes one way only: a <code>&amp;str</code> will not become a <code>&amp;String</code>.",
 code='''fn read_str(s: &str) -> usize { s.len() }
fn read_slice(v: &[i32]) -> usize { v.len() }

fn main() {
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let b = Box::new(7);
    println!("{} {} {}", read_str(&s), read_slice(&v), *b);
}''',
 expect="5 3 7",
 code_on="back",
 link=("borrowing", SITE+"18_Ownership/borrowing/index.html"),
 tags="rust ownership api-design"),

dict(id="own_slice_borrow",
 front="What is a slice, in ownership terms?",
 back="<b>A borrow of part of something else: a pointer and a length, owning nothing.</b>"
      "<br><br><code>&amp;v[1..3]</code> does not copy and does not allocate &mdash; it is a view, and it keeps the "
      "original borrowed for as long as it lives. That is why you cannot push to the Vec while a slice of it is alive."
      "<br><br>It is also why slices are the right parameter type: they accept a <code>Vec</code>, an array, another "
      "slice, or a sub-range of any of them, and they promise the caller you kept nothing."
      "<br><br><code>&amp;str</code> is exactly this for text &mdash; a string slice, no more and no less.",
 code='''fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let middle = &v[1..4];
    println!("{middle:?} {}", middle.len());
    println!("{:?}", &middle[1..]);
    let s = "hello world";
    println!("{}", &s[..5]);
}''',
 expect="[2, 3, 4] 3\n[3, 4]\nhello",
 code_on="back",
 link=("borrowing", SITE+"18_Ownership/borrowing/index.html"),
 tags="rust ownership"),

dict(id="own_first_moves",
 front="Your borrow error will not go away. Walk the checklist &mdash; in order.",
 back="<b>1. <b>Move the last use earlier.</b> Borrows end at last use, so many errors are just an ordering problem."
      "<br>2. <b>Copy the value out</b> rather than holding a reference: <code>let x = v[0];</code> for a "
      "<code>Copy</code> type."
      "<br>3. <b>Introduce a <code>{ }</code> block</b> to end a borrow before the next one starts."
      "<br>4. <b>Clone</b> &mdash; but only if you are reading. If you are WRITING, cloning compiles and silently does "
      "nothing."
      "<br>5. <b>Restructure</b>: split the struct, return both values from one method, collect-then-mutate."
      "<br>6. <b>Interior mutability</b> (<code>RefCell</code>/<code>Mutex</code>) &mdash; last, and deliberately.</b>"
      "<br><br>Reaching for 4 or 6 first is what makes Rust feel like a fight. Steps 1&ndash;3 are free and fix most of it.",
 link=("how_to_learn_lifetimes", SITE+"18_Ownership/how_to_learn_lifetimes/index.html"),
 tags="rust ownership borrow-checker core"),
]
