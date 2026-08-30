# Anki cards: Rust Vec.  Every `code` block is compiled with rustc 1.98.0
# --edition 2024 and run; `expect` must match stdout exactly.

SITE = "https://masiarek.github.io/rust-learning-library/"
DECK = "Rust::Vec"

CARDS = [

dict(id="vec_three_numbers",
 front="What is a <code>Vec&lt;T&gt;</code>, physically? How big is it on the stack?",
 back="<b>Three numbers on the stack &mdash; pointer, length, capacity &mdash; and one allocation on the heap. "
      "24 bytes on a 64-bit machine, WHATEVER <code>T</code> is.</b>"
      "<br><br>A <code>Vec&lt;[u8; 999]&gt;</code> is also 24 bytes. The elements are not in the <code>Vec</code>; "
      "the <code>Vec</code> is a receipt for them."
      "<br><br>• <b>len</b> &mdash; how many are initialised (what you almost always want)"
      "<br>• <b>capacity</b> &mdash; how many fit before the next allocation (matters exactly once: when you are about to fill it)",
 code='''fn main() {
    println!("{}", size_of::<Vec<u8>>());
    println!("{}", size_of::<Vec<[u8; 999]>>());
    let v = vec![5, 3, 0];
    println!("{v:?} len {} cap {}", v.len(), v.capacity());
}''',
 expect="24\n24\n[5, 3, 0] len 3 cap 3",
 code_on="back",
 bridge="<b>ABAP:</b> an internal table &mdash; but ABAP hides the header. Here the three numbers are the type.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec core memory"),

dict(id="vec_make",
 front="Five ways to make a <code>Vec</code>. Name them and when each is right.",
 back="<b><code>Vec::new()</code> · <code>vec![a, b, c]</code> · <code>vec![0; n]</code> · "
      "<code>Vec::with_capacity(n)</code> · <code>.collect()</code></b>"
      "<br><br>• <code>Vec::new()</code> &mdash; empty, <b>allocates nothing</b> until the first push"
      "<br>• <code>vec![0; n]</code> &mdash; n copies; needs <code>T: Clone</code>"
      "<br>• <code>with_capacity(n)</code> &mdash; when you know the size: one allocation instead of several, nothing copied"
      "<br>• <code>collect()</code> &mdash; when the values come from an iterator; it calls "
      "<code>with_capacity</code> for you if the iterator knows its own length (a range does, a <code>filter</code> does not)",
 code='''fn main() {
    let a: Vec<i32> = Vec::new();
    let b = vec![5, 3, 0];
    let c = vec![0u8; 4];
    let d: Vec<u32> = (1..=5).collect();
    println!("{} {b:?} {c:?} {d:?}", a.capacity());
}''',
 expect="0 [5, 3, 0] [0, 0, 0, 0] [1, 2, 3, 4, 5]",
 code_on="back",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec"),

dict(id="vec_growth",
 front="What does this print?",
 code='''fn main() {
    let mut v: Vec<u32> = Vec::new();
    for n in 1..=9 {
        v.push(n);
        print!("{} ", v.capacity());
    }
    println!();
}''',
 code_on="front",
 expect="4 4 4 4 8 8 8 8 16 ",
 back="<b>Growth is amortised doubling: 0 &rarr; 4 &rarr; 8 &rarr; 16.</b> Nine pushes cause three reallocations, "
      "and each one <b>copies everything already stored</b> into the new buffer."
      "<br><br>Doubling is what makes pushing <i>n</i> items cost O(<i>n</i>) in total instead of O(<i>n</i>&sup2;). "
      "The exact sequence is this std's choice, not a promise in the language &mdash; so "
      "<b>never assert on a capacity in a test</b>."
      "<br><br><code>Vec::with_capacity(9)</code> gives capacity 9 and one allocation.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec memory performance"),

dict(id="vec_pop",
 front="<code>v.pop()</code> &mdash; what comes back?",
 back="<b><code>Option&lt;T&gt;</code> &mdash; the owned last element, or <code>None</code> if empty.</b>"
      "<br><br>It removes from the <b>end</b>, which is why it is O(1). Capacity is unchanged."
      "<br><br>Contrast <code>v.last()</code> &rarr; <code>Option&lt;&amp;T&gt;</code>: looks, does not take.",
 code='''fn main() {
    let mut v = vec![1, 2, 3];
    println!("{:?} {:?}", v.pop(), v.last());
    v.clear();
    println!("{:?} cap {}", v.pop(), v.capacity());
}''',
 expect="Some(3) Some(2)\nNone cap 3",
 code_on="back",
 bridge="<b>Python:</b> <code>lst.pop()</code> on an empty list <i>raises</i> <code>IndexError</code>. Rust returns "
        "<code>None</code>, so the empty case cannot be forgotten.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec option"),

dict(id="vec_index_vs_get",
 front="<code>v[10]</code> vs <code>v.get(10)</code> on a 3-element Vec &mdash; what does each do?",
 back="<b><code>v[10]</code> PANICS. <code>v.get(10)</code> returns <code>None</code>.</b>"
      "<br><br><code>get</code> hands back <code>Option&lt;&amp;T&gt;</code>, so the missing case is in the type and the "
      "compiler makes you handle it. Indexing is for when being out of range is a bug you want to hear about loudly."
      "<br><br><code>get_mut</code> is the mutable twin. Both have a slice-taking form: "
      "<code>v.get(1..3)</code> &rarr; <code>Option&lt;&amp;[T]&gt;</code>.",
 code='''fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v[0]);
    println!("{:?}", v.get(10));
    println!("{:?}", v.get(1..3));
    match v.get(10) {
        Some(x) => println!("got {x}"),
        None    => println!("out of range"),
    }
}''',
 expect="1\nNone\nSome([2, 3])\nout of range",
 code_on="back",
 bridge="<b>Python:</b> <code>lst[10]</code> raises; <code>dict.get</code> is the closest cousin to <code>v.get</code>.",
 link=("arrays_and_slices", SITE+"26_Collections/arrays_and_slices/index.html"),
 tags="rust vec option gotcha"),

dict(id="vec_index_type",
 front="Does <code>v[v.len() - 1]</code> work on an empty Vec? And can you write <code>v[-1]</code>?",
 back="<b>Neither. There is no negative indexing, and <code>0usize - 1</code> panics on overflow before indexing "
      "even happens.</b>"
      "<br><br>Vec indices are <code>usize</code> &mdash; unsigned. <code>v[-1]</code> will not compile; "
      "<code>v[v.len() - 1]</code> compiles and then panics with "
      "<code>attempt to subtract with overflow</code> on an empty Vec (in debug; it wraps to a huge number in release)."
      "<br><br><b>Use <code>v.last()</code></b> &rarr; <code>Option&lt;&amp;T&gt;</code>. That is the whole reason it exists.",
 code='''fn main() {
    let v: Vec<i32> = vec![];
    println!("{:?}", v.last());
    let w = vec![1, 2, 3];
    println!("{:?} {:?}", w.last(), w.first());
}''',
 expect="None\nSome(3) Some(1)",
 code_on="back",
 bridge="<b>Python:</b> <code>lst[-1]</code> is idiomatic and safe-ish. In Rust the idiom is <code>last()</code>, "
        "and it cannot panic.",
 link=("arrays_and_slices", SITE+"26_Collections/arrays_and_slices/index.html"),
 tags="rust vec gotcha"),

dict(id="vec_three_iters",
 front="<code>iter()</code>, <code>iter_mut()</code>, <code>into_iter()</code> &mdash; what does each yield, and what happens to the Vec?",
 back="<b><code>iter()</code> &rarr; <code>&amp;T</code> (Vec survives) · <code>iter_mut()</code> &rarr; <code>&amp;mut T</code> "
      "(Vec survives, elements change) · <code>into_iter()</code> &rarr; <code>T</code> (Vec is CONSUMED)</b>"
      "<br><br>The <code>for</code> loop picks for you:<br>"
      "• <code>for x in &amp;v</code> &rarr; <code>iter()</code><br>"
      "• <code>for x in &amp;mut v</code> &rarr; <code>iter_mut()</code><br>"
      "• <code>for x in v</code> &rarr; <code>into_iter()</code> &mdash; <b><code>v</code> is gone after this loop</b>"
      "<br><br>With <code>iter_mut</code> you must dereference to assign: <code>*x *= 2</code>.",
 code='''fn main() {
    let mut v = vec![1, 2, 3];
    let total: i32 = v.iter().sum();
    for x in v.iter_mut() { *x *= 10; }
    println!("{total} {v:?}");
    let owned: Vec<String> = vec!["a".to_string()].into_iter().collect();
    println!("{owned:?}");
}''',
 expect='6 [10, 20, 30]\n["a"]',
 code_on="back",
 bridge="<b>ABAP:</b> <code>LOOP AT itab INTO wa</code> copies each row (like <code>iter()</code> then clone); "
        "<code>LOOP AT itab ASSIGNING &lt;fs&gt;</code> is exactly <code>iter_mut()</code> &mdash; a field-symbol IS a "
        "<code>&amp;mut</code>.",
 link=("iterators", SITE+"24_Iterators/index.html"),
 tags="rust vec iterators ownership"),

dict(id="vec_for_moves",
 front="Does this compile?",
 code='''fn main() {
    let v = vec![1, 2, 3];
    for x in v {
        println!("{x}");
    }
    println!("{}", v.len());
}''',
 code_on="front",
 fails="E0382",
 back="<b>No &mdash; <code>E0382</code>: borrow of moved value <code>v</code>.</b>"
      "<br><br><code>for x in v</code> calls <code>into_iter()</code>, which <b>consumes</b> the Vec. After the loop it "
      "does not exist."
      "<br><br>Fix: <code>for x in &amp;v</code> &mdash; one character, and it is the one you want 90% of the time. "
      "The compiler's own suggestion says so."
      "<br><br>This is the single most common beginner move-error in Rust.",
 bridge="<b>Python / ABAP:</b> looping never destroys the collection, so nothing in either language prepares you for this.",
 link=("ownership", SITE+"18_Ownership/index.html"),
 tags="rust vec ownership compile-error gotcha"),

dict(id="vec_borrow_while_push",
 front="Does this compile?",
 code='''fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("{first}");
}''',
 code_on="front",
 fails="E0502",
 back="<b>No &mdash; <code>E0502</code>: cannot borrow <code>v</code> as mutable because it is also borrowed as immutable.</b>"
      "<br><br>This is not pedantry. <code>push</code> may reallocate and copy the elements to a new address &mdash; "
      "<code>first</code> would then point at freed memory. In C++ this compiles and is the classic "
      "iterator-invalidation bug; Rust makes it a compile error."
      "<br><br>Fixes: copy the value out (<code>let first = v[0];</code>), or move the <code>push</code> before the borrow, "
      "or end the borrow by using <code>first</code> first.",
 bridge="<b>Python:</b> mutating a list while iterating it silently skips elements. Same class of bug, caught here at "
        "compile time.",
 link=("iterator_invalidation", SITE+"31_C_and_Cpp/iterator_invalidation/index.html"),
 tags="rust vec borrow-checker compile-error"),

dict(id="vec_move_out",
 front="Does this compile? <code>v: Vec&lt;String&gt;</code>",
 code='''fn main() {
    let v = vec![String::from("a"), String::from("b")];
    let s: String = v[0];
    println!("{s}");
}''',
 code_on="front",
 fails="E0507",
 back="<b>No &mdash; <code>E0507</code>: cannot move out of index of <code>Vec&lt;String&gt;</code>.</b>"
      "<br><br>Indexing gives you a <i>place</i>, not ownership. Moving the <code>String</code> out would leave a hole "
      "in the Vec, and <code>Vec</code> has no way to represent a hole."
      "<br><br>Four ways out, cheapest first:"
      "<br>• <code>&amp;v[0]</code> &mdash; just borrow it"
      "<br>• <code>v[0].clone()</code> &mdash; pay for a copy"
      "<br>• <code>v.remove(0)</code> / <code>v.swap_remove(0)</code> &mdash; take it and close the hole"
      "<br>• <code>v.into_iter().next()</code> &mdash; consume the whole Vec"
      "<br><br>Note it works fine for <code>Vec&lt;i32&gt;</code>: <code>i32</code> is <code>Copy</code>, so nothing moves.",
 link=("ownership", SITE+"18_Ownership/index.html"),
 tags="rust vec ownership compile-error gotcha"),

dict(id="vec_remove_vs_swap",
 front="Remove element at index 1. <code>remove</code> vs <code>swap_remove</code> &mdash; what is the trade?",
 back="<b><code>remove(i)</code> keeps order, O(n) &mdash; shifts everything after it down.<br>"
      "<code>swap_remove(i)</code> is O(1) &mdash; moves the LAST element into the hole, order destroyed.</b>"
      "<br><br>Both return the removed value (not an <code>Option</code>) and both panic if <code>i</code> is out of range."
      "<br><br>Removing many? Neither &mdash; use <code>retain</code>, which is one pass.",
 code='''fn main() {
    let mut a = vec!['a', 'b', 'c', 'd'];
    println!("{:?} {a:?}", a.remove(1));

    let mut b = vec!['a', 'b', 'c', 'd'];
    println!("{:?} {b:?}", b.swap_remove(1));
}''',
 expect="'b' ['a', 'c', 'd']\n'b' ['a', 'd', 'c']",
 code_on="back",
 bridge="<b>ABAP:</b> <code>DELETE itab INDEX 2</code> is <code>remove</code> &mdash; and has the same O(n) shift.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec performance"),

dict(id="vec_retain",
 front="Delete every element that fails a test, <b>in place</b>, in one pass. Which method &mdash; and mind the polarity.",
 back="<b><code>v.retain(|x| ...)</code> &mdash; it KEEPS the elements where the closure is true.</b>"
      "<br><br>The polarity is the trap: it is named for what survives, not what goes. One pass, O(n), no allocation, "
      "order preserved."
      "<br><br><code>retain_mut</code> lets the closure also modify as it decides. "
      "<code>extract_if</code> gives you the removed ones back as an iterator.",
 code='''fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|n| n % 2 == 0);
    println!("{v:?}");

    let mut names = vec!["ada".to_string(), "".to_string(), "bob".to_string()];
    names.retain(|s| !s.is_empty());
    println!("{names:?}");
}''',
 expect='[2, 4, 6]\n["ada", "bob"]',
 code_on="back",
 bridge="<b>ABAP:</b> <code>DELETE itab WHERE ...</code> &mdash; same one-pass operation, but stated as what to REMOVE. "
        "Rust states what to KEEP. Invert the condition."
        "<br><b>Python:</b> <code>lst[:] = [x for x in lst if ...]</code> &mdash; and note the <code>[:]</code>, "
        "without which you rebind instead of mutating.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec gotcha"),

dict(id="vec_dedup",
 front="What does this print?",
 code='''fn main() {
    let mut v = vec![1, 2, 1, 2, 1];
    v.dedup();
    println!("{v:?}");

    let mut w = vec![1, 2, 1, 2, 1];
    w.sort();
    w.dedup();
    println!("{w:?}");
}''',
 code_on="front",
 expect="[1, 2, 1, 2, 1]\n[1, 2]",
 back="<b><code>dedup()</code> only removes <b>CONSECUTIVE</b> duplicates.</b> Unsorted, it does almost nothing."
      "<br><br><code>sort()</code> then <code>dedup()</code> is the unique-elements idiom, O(n log n)."
      "<br><br>Variants: <code>dedup_by_key(|x| ...)</code>, <code>dedup_by(|a, b| ...)</code>."
      "<br><br>Order matters and you cannot sort? Then it is a <code>HashSet</code> job, not a <code>Vec</code> one.",
 bridge="<b>ABAP:</b> <code>DELETE ADJACENT DUPLICATES FROM itab</code> &mdash; <i>identical</i> trap, and the same fix: "
        "<code>SORT</code> first. If you have ever been bitten in ABAP, you already know this card."
        "<br><b>Python:</b> <code>set(lst)</code> is the usual move, but it loses order; "
        "<code>list(dict.fromkeys(lst))</code> keeps it.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec gotcha abap"),

dict(id="vec_sort_family",
 front="Sort a Vec: by natural order, by a key, and by a custom rule. And what about <code>Vec&lt;f64&gt;</code>?",
 back="<b><code>v.sort()</code> · <code>v.sort_by_key(|x| ...)</code> · <code>v.sort_by(|a, b| ...)</code></b>"
      "<br><br><code>sort()</code> is stable and needs <code>T: Ord</code>. <code>sort_unstable()</code> is faster and "
      "does not preserve the order of equals &mdash; the right default when elements have no identity beyond their value."
      "<br><br><b>Floats do not implement <code>Ord</code></b> (because of <code>NaN</code>), so <code>v.sort()</code> "
      "will not compile on <code>Vec&lt;f64&gt;</code>. Use <code>v.sort_by(f64::total_cmp)</code>."
      "<br><br>Descending: <code>sort_by(|a, b| b.cmp(a))</code>, or sort then <code>reverse()</code>.",
 code='''fn main() {
    let mut v = vec![3, 1, 2];
    v.sort();
    println!("{v:?}");

    let mut words = vec!["ccc", "a", "bb"];
    words.sort_by_key(|s| s.len());
    println!("{words:?}");

    let mut f = vec![2.5, 0.5, 1.5];
    f.sort_by(f64::total_cmp);
    println!("{f:?}");

    v.sort_by(|a, b| b.cmp(a));
    println!("{v:?}");
}''',
 expect='[1, 2, 3]\n["a", "bb", "ccc"]\n[0.5, 1.5, 2.5]\n[3, 2, 1]',
 code_on="back",
 bridge="<b>Python:</b> <code>lst.sort(key=len)</code> &rarr; <code>sort_by_key(|s| s.len())</code>. Same idea, and Rust's "
        "is also stable.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec sorting"),

dict(id="vec_binary_search",
 front="<code>v.binary_search(&amp;3)</code> &mdash; what does it return, and what is the precondition?",
 back="<b><code>Result&lt;usize, usize&gt;</code> &mdash; <code>Ok(i)</code> where it is, <code>Err(i)</code> where it "
      "<i>would</i> go. The Vec MUST already be sorted.</b>"
      "<br><br>On an unsorted Vec it returns a meaningless answer with no error &mdash; a silent wrong result, not a panic."
      "<br><br>The <code>Err(i)</code> is not a failure so much as an insertion point: "
      "<code>v.insert(i, x)</code> keeps the Vec sorted.",
 code='''fn main() {
    let v = vec![10, 20, 30, 40];
    println!("{:?}", v.binary_search(&30));
    println!("{:?}", v.binary_search(&35));

    let mut w = vec![10, 20, 40];
    let at = w.binary_search(&30).unwrap_or_else(|i| i);
    w.insert(at, 30);
    println!("{w:?}");
}''',
 expect="Ok(2)\nErr(3)\n[10, 20, 30, 40]",
 code_on="back",
 bridge="<b>ABAP:</b> <code>READ TABLE itab ... BINARY SEARCH</code> &mdash; same precondition, same silent-wrong-answer "
        "failure mode if the table is not sorted by that key."
        "<br><b>Python:</b> <code>bisect.bisect_left</code> gives you the <code>Err(i)</code> half.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec result abap gotcha"),

dict(id="vec_deref_slice",
 front="You are writing a function that reads a list of numbers. Should the parameter be <code>&amp;Vec&lt;i32&gt;</code> or <code>&amp;[i32]</code>?",
 back="<b><code>&amp;[i32]</code>. Always, for a read-only parameter.</b>"
      "<br><br><code>Vec&lt;T&gt;</code> derefs to <code>[T]</code>, so a <code>&amp;Vec&lt;i32&gt;</code> coerces at the "
      "call site for free &mdash; existing callers do not change. And <code>&amp;[i32]</code> <i>additionally</i> accepts "
      "arrays, sub-slices (<code>&amp;v[1..3]</code>), and anything else that derefs to a slice."
      "<br><br>Same reasoning as <code>&amp;str</code> over <code>&amp;String</code>. It is the same rule."
      "<br><br>That deref is also why <code>first</code>, <code>last</code>, <code>contains</code>, <code>sort</code>, "
      "<code>windows</code>, <code>iter</code> are not on <code>Vec</code> at all &mdash; they are slice methods you get free.",
 code='''fn total(xs: &[i32]) -> i32 { xs.iter().sum() }

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{}", total(&v));
    println!("{}", total(&v[1..3]));
    println!("{}", total(&[10, 20]));
}''',
 expect="10\n5\n30",
 code_on="back",
 link=("arrays_and_slices", SITE+"26_Collections/arrays_and_slices/index.html"),
 tags="rust vec api-design"),

dict(id="vec_collect",
 front="<code>.collect()</code> needs to know what to build. Two ways to tell it &mdash; write both.",
 back="<b>Annotate the binding: <code>let v: Vec&lt;i32&gt; = it.collect();</code><br>"
      "or turbofish the call: <code>it.collect::&lt;Vec&lt;i32&gt;&gt;()</code></b>"
      "<br><br>Without one of them: <code>E0282, type annotations needed</code>. <code>collect</code> is generic over its "
      "<i>return</i> type, which inference cannot guess."
      "<br><br><code>Vec&lt;_&gt;</code> is usually enough &mdash; the element type is inferable, only the container is not."
      "<br><br>It builds more than Vecs: <code>String</code>, <code>HashMap</code>, <code>HashSet</code>, and &mdash; the "
      "good trick &mdash; <code>Result&lt;Vec&lt;T&gt;, E&gt;</code> from an iterator of Results, which short-circuits on "
      "the first error.",
 code='''fn main() {
    let v: Vec<u32> = (1..=3).collect();
    let w = (1..=3).map(|n| n * n).collect::<Vec<_>>();
    println!("{v:?} {w:?}");

    let nums: Result<Vec<i32>, _> = ["1", "2", "x"].iter().map(|s| s.parse::<i32>()).collect();
    println!("{}", nums.is_err());
}''',
 expect="[1, 2, 3] [1, 4, 9]\ntrue",
 code_on="back",
 link=("collect_into_a_vec", SITE+"24_Iterators/collect_into_a_vec/index.html"),
 tags="rust vec iterators"),

dict(id="vec_sum_type",
 front="Why does <code>v.iter().sum()</code> sometimes fail to compile, and how do you fix it?",
 back="<b><code>sum</code> is generic over its output type. Annotate it: "
      "<code>let t: i32 = v.iter().sum();</code> or <code>v.iter().sum::&lt;i32&gt;()</code>.</b>"
      "<br><br>Same shape of problem as <code>collect</code>. It works unannotated only when the surrounding code "
      "already pins the type."
      "<br><br>The same applies to <code>product()</code>. Not to <code>max()</code>/<code>min()</code>, which return "
      "<code>Option&lt;&amp;T&gt;</code> with the element type already known.",
 code='''fn main() {
    let v = vec![1, 2, 3];
    let t: i32 = v.iter().sum();
    println!("{t} {}", v.iter().sum::<i32>());
    println!("{:?} {:?}", v.iter().max(), v.iter().min());
    println!("{}", v.iter().product::<i32>());
}''',
 expect="6 6\nSome(3) Some(1)\n6",
 code_on="back",
 link=("iterators", SITE+"24_Iterators/index.html"),
 tags="rust vec iterators"),

dict(id="vec_enumerate_position",
 front="You need the <b>index</b> of an element while looping, and separately the index of the first match. Which two methods?",
 back="<b><code>.iter().enumerate()</code> &rarr; <code>(usize, &amp;T)</code><br>"
      "<code>.iter().position(|x| ...)</code> &rarr; <code>Option&lt;usize&gt;</code></b>"
      "<br><br><code>position</code> short-circuits on the first hit. <code>rposition</code> searches from the end. "
      "<code>find</code> is the sibling that returns the <i>element</i> instead of the index."
      "<br><br>Note <code>position</code>'s closure takes <code>&amp;&amp;T</code> from <code>iter()</code> &mdash; hence the "
      "<code>|&amp;x|</code> pattern, or compare with <code>*x</code>.",
 code='''fn main() {
    let v = vec!["a", "b", "c"];
    for (i, s) in v.iter().enumerate() {
        print!("{i}:{s} ");
    }
    println!();
    println!("{:?}", v.iter().position(|&s| s == "b"));
    println!("{:?}", v.iter().position(|&s| s == "z"));
    println!("{:?}", v.iter().find(|s| s.starts_with('c')));
}''',
 expect='0:a 1:b 2:c \nSome(1)\nNone\nSome("c")',
 code_on="back",
 bridge="<b>Python:</b> <code>enumerate(lst)</code> is identical; <code>lst.index(x)</code> raises when missing, where "
        "<code>position</code> returns <code>None</code>.",
 link=("iterators", SITE+"24_Iterators/index.html"),
 tags="rust vec iterators option"),

dict(id="vec_contains",
 front="Test whether a Vec contains a value. Write it &mdash; and watch the argument.",
 back="<b><code>v.contains(&amp;x)</code> &mdash; it takes a REFERENCE.</b>"
      "<br><br><code>v.contains(x)</code> will not compile: the signature is <code>contains(&amp;self, x: &amp;T)</code>."
      "<br><br>Careful on <code>Vec&lt;String&gt;</code>: <code>v.contains(&amp;\"a\")</code> fails, because "
      "<code>&amp;&amp;str</code> is not <code>&amp;String</code>. Use "
      "<code>v.iter().any(|s| s == \"a\")</code>, which compares through deref."
      "<br><br>And note it is <b>O(n)</b>. If you do this in a loop, you want a <code>HashSet</code>.",
 code='''fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v.contains(&2));

    let names = vec![String::from("ada")];
    println!("{}", names.iter().any(|s| s == "ada"));
}''',
 expect="true\ntrue",
 code_on="back",
 bridge="<b>ABAP:</b> <code>line_exists( itab[ key = x ] )</code>. <b>Python:</b> <code>x in lst</code> &mdash; also O(n), "
        "and also the signal to reach for a set.",
 link=("the_hashset", SITE+"26_Collections/the_hashset/index.html"),
 tags="rust vec gotcha"),

dict(id="vec_extend_append",
 front="Add all of <code>b</code>'s elements to <code>a</code>. Three ways &mdash; and what happens to <code>b</code> in each?",
 back="<b><code>a.extend(b)</code> &mdash; consumes <code>b</code><br>"
      "<code>a.append(&amp;mut b)</code> &mdash; <b>EMPTIES</b> <code>b</code>, which stays alive<br>"
      "<code>a.extend_from_slice(&amp;b)</code> &mdash; clones; <code>b</code> untouched</b>"
      "<br><br><code>append</code> is the surprising one: <code>b</code> is still there afterwards, but with length 0. "
      "It is the fastest, because elements are moved in bulk with no per-item work."
      "<br><br><code>extend</code> takes any <code>IntoIterator</code>, so <code>a.extend(1..=3)</code> works too.",
 code='''fn main() {
    let mut a = vec![1, 2];
    let mut b = vec![3, 4];
    a.append(&mut b);
    println!("{a:?} {b:?}");

    let c = vec![5, 6];
    a.extend_from_slice(&c);
    a.extend(7..=8);
    println!("{a:?} {c:?}");
}''',
 expect="[1, 2, 3, 4] []\n[1, 2, 3, 4, 5, 6, 7, 8] [5, 6]",
 code_on="back",
 bridge="<b>Python:</b> <code>a.extend(b)</code> leaves <code>b</code> alone &mdash; that is Rust's "
        "<code>extend_from_slice</code>, not its <code>append</code>.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec gotcha"),

dict(id="vec_drain",
 front="Take a range of elements OUT of a Vec and use them, leaving the Vec shorter. Which method?",
 back="<b><code>v.drain(1..3)</code> &mdash; yields the removed elements as an iterator and removes them from the Vec.</b>"
      "<br><br><code>v.drain(..)</code> empties it while handing you everything &mdash; the way to reuse an allocation "
      "without cloning."
      "<br><br>The removal happens even if you drop the iterator without reading it. "
      "<code>v.clear()</code> is <code>drain(..)</code> with the values thrown away."
      "<br><br>Compare: <code>split_off(n)</code> gives you the tail as a NEW Vec.",
 code='''fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let taken: Vec<i32> = v.drain(1..3).collect();
    println!("{taken:?} {v:?}");

    let mut w = vec![1, 2, 3, 4];
    let tail = w.split_off(2);
    println!("{w:?} {tail:?}");
}''',
 expect="[2, 3] [1, 4, 5]\n[1, 2] [3, 4]",
 code_on="back",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec"),

dict(id="vec_clear_capacity",
 front="After <code>v.clear()</code>, what are <code>len</code> and <code>capacity</code>?",
 back="<b><code>len</code> is 0. <code>capacity</code> is UNCHANGED &mdash; the buffer is kept.</b>"
      "<br><br>That is a feature: clearing and refilling in a loop reuses one allocation. Same for "
      "<code>truncate(n)</code>, which only drops the tail."
      "<br><br>Actually give the memory back with <code>shrink_to_fit()</code>, or drop the Vec. "
      "<code>v = Vec::new()</code> also does it &mdash; the old buffer is freed with the old value.",
 code='''fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.truncate(2);
    println!("{v:?} len {} cap {}", v.len(), v.capacity());
    v.clear();
    println!("len {} cap {}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("cap {}", v.capacity());
}''',
 expect="[1, 2] len 2 cap 5\nlen 0 cap 5\ncap 0",
 code_on="back",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec memory"),

dict(id="vec_2d",
 front="Make a 2-row &times; 3-column grid of zeros, then set row 1 column 2 to 9.",
 back="<b><code>let mut g = vec![vec![0; 3]; 2];</code> then <code>g[1][2] = 9;</code></b>"
      "<br><br>The inner <code>vec!</code> is cloned per row, so each row is its own allocation &mdash; "
      "<i>n</i>+1 allocations for <i>n</i> rows, and the rows are scattered in memory."
      "<br><br>For anything performance-sensitive, use <b>one flat Vec</b> and index arithmetic: "
      "<code>g[row * cols + col]</code>. One allocation, cache-friendly, and the shape lives in two "
      "<code>usize</code>s you keep beside it."
      "<br><br>Watch out: <code>vec![vec![0; 3]; 2]</code> clones (fine), but with <code>Rc</code>/<code>RefCell</code> "
      "inside, cloning shares &mdash; a classic accidental-aliasing bug.",
 code='''fn main() {
    let mut g = vec![vec![0; 3]; 2];
    g[1][2] = 9;
    println!("{g:?}");

    let (rows, cols) = (2, 3);
    let mut flat = vec![0; rows * cols];
    flat[1 * cols + 2] = 9;
    println!("{flat:?}");
}''',
 expect="[[0, 0, 0], [0, 0, 9]]\n[0, 0, 0, 0, 0, 9]",
 code_on="back",
 link=("vec_of_vecs", SITE+"26_Collections/vec_of_vecs/index.html"),
 tags="rust vec"),

dict(id="vec_windows_chunks",
 front="Walk a Vec in overlapping pairs, and separately in fixed-size batches. Which two methods?",
 back="<b><code>v.windows(2)</code> &mdash; OVERLAPPING, sliding by one<br>"
      "<code>v.chunks(2)</code> &mdash; DISJOINT batches, last one may be short</b>"
      "<br><br><code>windows</code> is the tool for \"compare each element to the next\" &mdash; differences, "
      "is-sorted checks, run detection. It yields nothing at all if the slice is shorter than the window."
      "<br><br><code>chunks_exact(n)</code> drops the short tail (and is faster); "
      "<code>chunks_mut</code> lets you write.",
 code='''fn main() {
    let v = vec![1, 2, 4, 8, 9];
    println!("{:?}", v.windows(2).collect::<Vec<_>>());
    println!("{:?}", v.chunks(2).collect::<Vec<_>>());
    let deltas: Vec<i32> = v.windows(2).map(|w| w[1] - w[0]).collect();
    println!("{deltas:?}");
    println!("{}", v.windows(2).all(|w| w[0] <= w[1]));
}''',
 expect="[[1, 2], [2, 4], [4, 8], [8, 9]]\n[[1, 2], [4, 8], [9]]\n[1, 2, 4, 1]\ntrue",
 code_on="back",
 link=("arrays_and_slices", SITE+"26_Collections/arrays_and_slices/index.html"),
 tags="rust vec iterators"),

dict(id="vec_map_vs_mut",
 front="Double every element. Two idioms &mdash; when do you want each?",
 back="<b>New Vec: <code>let d: Vec&lt;i32&gt; = v.iter().map(|x| x * 2).collect();</code><br>"
      "In place: <code>for x in &amp;mut v { *x *= 2; }</code></b>"
      "<br><br><code>map</code>+<code>collect</code> allocates a second Vec and can change the element type. "
      "<code>iter_mut</code> allocates nothing and cannot."
      "<br><br>Reach for <code>map</code> by default &mdash; it composes with <code>filter</code>, "
      "<code>enumerate</code>, <code>take</code>, and it reads as a pipeline. Reach for <code>iter_mut</code> when the "
      "Vec is large and the type is unchanged."
      "<br><br>Don't forget the <code>*</code>: <code>x</code> is a <code>&amp;mut i32</code>, not an <code>i32</code>.",
 code='''fn main() {
    let v = vec![1, 2, 3];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    let labels: Vec<String> = v.iter().map(|x| format!("#{x}")).collect();
    println!("{doubled:?} {labels:?}");

    let mut w = vec![1, 2, 3];
    for x in &mut w { *x *= 2; }
    println!("{w:?}");

    let evens: Vec<i32> = (1..=10).filter(|n| n % 2 == 0).collect();
    println!("{evens:?}");
}''',
 expect='[2, 4, 6] ["#1", "#2", "#3"]\n[2, 4, 6]\n[2, 4, 6, 8, 10]',
 code_on="back",
 bridge="<b>Python:</b> <code>[x * 2 for x in v]</code> is the <code>map</code>+<code>collect</code> line; Python has no "
        "clean in-place equivalent.",
 link=("iterators", SITE+"24_Iterators/index.html"),
 tags="rust vec iterators"),

dict(id="vec_join",
 front="Turn <code>Vec&lt;String&gt;</code> into one comma-separated <code>String</code>. And <code>Vec&lt;Vec&lt;i32&gt;&gt;</code> into a flat <code>Vec&lt;i32&gt;</code>?",
 back="<b><code>v.join(\", \")</code> for the strings.<br>"
      "<code>v.concat()</code> or <code>v.into_iter().flatten().collect()</code> for the nested Vec.</b>"
      "<br><br>Both live on the <b>slice</b>, which is why they work on a <code>Vec</code> at all. "
      "<code>concat()</code> is <code>join</code> with no separator."
      "<br><br>For non-string elements, go through <code>map</code> first: "
      "<code>v.iter().map(|n| n.to_string()).collect::&lt;Vec&lt;_&gt;&gt;().join(\", \")</code>.",
 code='''fn main() {
    let names = vec![String::from("ada"), String::from("bob")];
    println!("{}", names.join(", "));

    let nested = vec![vec![1, 2], vec![3]];
    println!("{:?}", nested.concat());

    let nums = vec![1, 2, 3];
    println!("{}", nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("-"));
}''',
 expect="ada, bob\n[1, 2, 3]\n1-2-3",
 code_on="back",
 bridge="<b>ABAP:</b> <code>CONCATENATE LINES OF itab INTO lv SEPARATED BY ', '</code>."
        "<br><b>Python:</b> <code>\", \".join(names)</code> &mdash; separator first, the other way round from Rust.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec strings"),

dict(id="vec_reverse",
 front="<code>v.reverse()</code> vs <code>v.iter().rev()</code> &mdash; what is the difference?",
 back="<b><code>reverse()</code> mutates the Vec in place and returns <code>()</code>. "
      "<code>rev()</code> walks it backwards and changes nothing.</b>"
      "<br><br><code>let r = v.reverse();</code> is the classic slip: <code>r</code> is <code>()</code>. Any Rust method "
      "named as an imperative verb (<code>sort</code>, <code>reverse</code>, <code>push</code>, <code>clear</code>, "
      "<code>truncate</code>) mutates and returns nothing."
      "<br><br><code>rev()</code> needs a <code>DoubleEndedIterator</code> &mdash; fine for slices and Vecs, "
      "not for a <code>HashMap</code>.",
 code='''fn main() {
    let mut v = vec![1, 2, 3];
    v.reverse();
    println!("{v:?}");

    let w = vec![1, 2, 3];
    println!("{:?} {w:?}", w.iter().rev().collect::<Vec<_>>());
}''',
 expect="[3, 2, 1]\n[3, 2, 1] [1, 2, 3]",
 code_on="back",
 bridge="<b>Python:</b> exactly the same split &mdash; <code>lst.reverse()</code> returns <code>None</code> and mutates; "
        "<code>reversed(lst)</code> and <code>lst[::-1]</code> do not.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec gotcha"),

dict(id="vec_insert_swap",
 front="<code>insert(i, x)</code> and <code>swap(i, j)</code> &mdash; what do they cost?",
 back="<b><code>insert(i, x)</code> is O(n) &mdash; it shifts everything from <code>i</code> rightwards. "
      "<code>swap(i, j)</code> is O(1).</b>"
      "<br><br><code>insert(0, x)</code> &mdash; prepending &mdash; copies the whole Vec every time. "
      "In a loop that is O(n&sup2;). If you need to prepend often you want a <code>VecDeque</code>, "
      "which has <code>push_front</code>."
      "<br><br>Both panic if the index is out of range (<code>insert</code> allows <code>i == len</code>, "
      "which is a push).",
 code='''fn main() {
    let mut v = vec!['a', 'c'];
    v.insert(1, 'b');
    v.insert(3, 'd');
    println!("{v:?}");
    v.swap(0, 3);
    println!("{v:?}");
}''',
 expect="['a', 'b', 'c', 'd']\n['d', 'b', 'c', 'a']",
 code_on="back",
 bridge="<b>ABAP:</b> <code>INSERT wa INTO itab INDEX i</code> &mdash; same shift, same cost.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec performance"),

dict(id="vec_any_all",
 front="Ask \"does any element match?\" and \"do all of them?\" &mdash; and what do these do on an EMPTY Vec?",
 back="<b><code>v.iter().any(|x| ...)</code> and <code>v.iter().all(|x| ...)</code> &mdash; both return <code>bool</code> "
      "and both short-circuit.</b>"
      "<br><br>On an empty Vec: <code>any</code> is <b>false</b>, <code>all</code> is <b>true</b>. That is the "
      "mathematical convention (vacuous truth), it is almost never what a first-time reader expects, and it is a real "
      "source of bugs in validation code &mdash; \"all rows are valid\" is true when there are no rows.",
 code='''fn main() {
    let v = vec![2, 4, 6];
    println!("{} {}", v.iter().any(|n| n % 2 == 1), v.iter().all(|n| n % 2 == 0));

    let empty: Vec<i32> = vec![];
    println!("{} {}", empty.iter().any(|n| *n > 0), empty.iter().all(|n| *n > 0));
}''',
 expect="false true\nfalse true",
 code_on="back",
 bridge="<b>Python:</b> <code>any([])</code> is <code>False</code>, <code>all([])</code> is <code>True</code> &mdash; "
        "identical convention, identical trap.",
 link=("iterators", SITE+"24_Iterators/index.html"),
 tags="rust vec iterators gotcha"),

dict(id="vec_len_empty",
 front="<code>len()</code>, <code>is_empty()</code>, and iterating a Vec while you need its length &mdash; any traps?",
 back="<b><code>v.len()</code> is O(1) (it is one of the three stored numbers). <code>v.is_empty()</code> is the "
      "idiomatic emptiness test.</b>"
      "<br><br>The trap is the loop: <code>for i in 0..v.len()</code> reads the length ONCE, before the loop body ever "
      "runs. If the body pushes, the loop will not see the new elements; if it removes, the index will go out of range "
      "and panic."
      "<br><br>That is a good thing &mdash; and the reason to write <code>for x in &amp;v</code>, which the borrow "
      "checker will not let you invalidate at all.",
 code='''fn main() {
    let v = vec![1, 2, 3];
    println!("{} {}", v.len(), v.is_empty());
    let e: Vec<i32> = Vec::new();
    println!("{} {}", e.len(), e.is_empty());
}''',
 expect="3 false\n0 true",
 code_on="back",
 bridge="<b>ABAP:</b> <code>lines( itab )</code> and <code>itab IS INITIAL</code>.",
 link=("the_vec", SITE+"26_Collections/the_vec/index.html"),
 tags="rust vec"),

dict(id="vec_when_not_vec",
 front="When is <code>Vec</code> the wrong choice? Name the three alternatives and their trigger.",
 back="<b>• repeated <code>contains</code> / membership &rarr; <code>HashSet</code> (O(1) instead of O(n))<br>"
      "• lookup by key &rarr; <code>HashMap</code><br>"
      "• pushing or popping at the FRONT &rarr; <code>VecDeque</code> (<code>push_front</code> is O(1))</b>"
      "<br><br>Also: a fixed, compile-time-known size &rarr; an array <code>[T; N]</code>, which lives on the stack "
      "with no allocation at all."
      "<br><br>And the one that is <i>not</i> a reason to leave: needing a sorted list. "
      "<code>Vec</code> + <code>sort</code> + <code>binary_search</code> beats a tree for most real sizes, because it is "
      "one contiguous block and the cache loves it.",
 code='''use std::collections::{HashSet, VecDeque};

fn main() {
    let seen: HashSet<i32> = vec![1, 2, 2, 3].into_iter().collect();
    println!("{} {}", seen.len(), seen.contains(&2));

    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(2);
    q.push_front(1);
    println!("{:?} {:?}", q.pop_front(), q);
}''',
 expect="3 true\nSome(1) [2]",
 code_on="back",
 link=("collections", SITE+"26_Collections/index.html"),
 tags="rust vec collections"),
]
