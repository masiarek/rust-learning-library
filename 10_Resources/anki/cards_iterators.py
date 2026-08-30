# Anki cards: Rust iterators.  Verified by verify.py -- rustc 1.98.0, edition 2024.

SITE = "https://masiarek.github.io/rust-learning-library/"
DECK = "Rust::Iterators"

CARDS = [

dict(id="it_lazy",
 front="What does this print, and in what ORDER?",
 code='''fn main() {
    let v = vec![1, 2, 3];
    let doubled = v.iter().map(|x| { println!("seen {x}"); x * 2 });
    println!("nothing yet");
    let out: Vec<i32> = doubled.collect();
    println!("{out:?}");
}''',
 code_on="front",
 expect="nothing yet\nseen 1\nseen 2\nseen 3\n[2, 4, 6]",
 back="<b>An adapter such as <code>map</code> or <code>filter</code> computes NOTHING. It builds a value describing "
      "the work. A <b>consumer</b> is what runs it.</b>"
      "<br><br>Nothing inside the closure happens until <code>collect</code> asks &mdash; which is why "
      "<code>nothing yet</code> prints first."
      "<br><br>Consumers: <code>collect</code>, <code>sum</code>, <code>count</code>, <code>fold</code>, <code>for_each</code>, "
      "<code>find</code>, <code>any</code>, <code>all</code>, <code>max</code>, and a <code>for</code> loop."
      "<br><br>An unconsumed chain is a value you built and dropped &mdash; <code>Iterator</code> is "
      "<code>#[must_use]</code>, so rustc warns.",
 bridge="<b>Python:</b> the same &mdash; <code>map()</code> and generators are lazy in Python 3; "
        "<code>list()</code> is the consumer.<br><b>JavaScript:</b> NOT the same &mdash; "
        "<code>Array.prototype.map</code> is eager and allocates immediately.",
 link=("iterators_are_lazy", SITE+"24_Iterators/iterators_are_lazy/index.html"),
 tags="rust iterators core"),

dict(id="it_adapter_or_consumer",
 front="How do you tell an <b>adapter</b> from a <b>consumer</b> by looking at it?",
 back="<b>An adapter returns something that still implements <code>Iterator</code>. A consumer returns an answer.</b>"
      "<br><br>You never have to memorise the list &mdash; read the return type. "
      "<code>map</code> &rarr; <code>Map&lt;...&gt;</code> (an iterator). <code>sum</code> &rarr; <code>i32</code> (an answer)."
      "<br><br>The practical consequence: <b>a chain of adapters costs nothing until a consumer is attached</b>, "
      "and adapters compose without allocating. <code>collect</code> is the only common consumer that allocates.",
 code='''fn main() {
    let v = vec![1, 2, 3, 4];
    // adapters: still lazy, no work done
    let chain = v.iter().map(|x| x * 2).filter(|x| x > &4);
    // one consumer runs the whole chain, once, allocating once
    println!("{:?}", chain.collect::<Vec<_>>());
    println!("{}", v.iter().sum::<i32>());
    println!("{}", v.iter().count());
}''',
 expect="[6, 8]\n10\n4",
 code_on="back",
 link=("iterators_are_lazy", SITE+"24_Iterators/iterators_are_lazy/index.html"),
 tags="rust iterators core"),

dict(id="it_next_mut",
 front="Why does driving an iterator by hand need <code>let mut</code>?",
 back="<b>Because <code>next(&amp;mut self)</code> CONSUMES an item &mdash; it changes the iterator's own state.</b>"
      "<br><br><code>fn next(&amp;mut self) -&gt; Option&lt;Self::Item&gt;</code>. An iterator is a cursor, not a view: "
      "asking for the next item permanently advances it. <code>None</code> means exhausted."
      "<br><br>This is also why an iterator cannot be read twice &mdash; and why <code>collect</code> exists.",
 code='''fn main() {
    let v = vec![1, 2];
    let mut it = v.iter();
    println!("{:?} {:?} {:?}", it.next(), it.next(), it.next());
}''',
 expect="Some(1) Some(2) None",
 code_on="back",
 link=("implementing_iterator", SITE+"24_Iterators/implementing_iterator/index.html"),
 tags="rust iterators core"),

dict(id="it_for_desugar",
 front="What does <code>for x in thing { ... }</code> desugar to?",
 back="<b><code>IntoIterator::into_iter(thing)</code>, then <code>while let Some(x) = it.next()</code>.</b>"
      "<br><br>So <code>for</code> works on anything implementing <code>IntoIterator</code> &mdash; and the <code>&amp;</code> "
      "you write picks the implementation:"
      "<br>• <code>for x in &amp;v</code> &rarr; <code>iter()</code>, x is <code>&amp;T</code>"
      "<br>• <code>for x in &amp;mut v</code> &rarr; <code>iter_mut()</code>, x is <code>&amp;mut T</code>"
      "<br>• <code>for x in v</code> &rarr; <code>into_iter()</code>, x is <code>T</code>, <b>v is consumed</b>"
      "<br><br>There is no separate <code>.iter()</code> call in the desugaring &mdash; that is why "
      "<code>for x in v.iter()</code> and <code>for x in &amp;v</code> are the same loop.",
 code='''fn main() {
    let v = vec![1, 2, 3];
    let mut it = (&v).into_iter();
    while let Some(x) = it.next() { print!("{x} "); }
    println!();
    for x in &v { print!("{x} "); }
    println!();
}''',
 expect="1 2 3 \n1 2 3 ",
 code_on="back",
 link=("iter_iter_mut_into_iter", SITE+"24_Iterators/iter_iter_mut_into_iter/index.html"),
 tags="rust iterators core"),

dict(id="it_collect_asks_type",
 front="<code>collect()</code> can build a Vec, a String, a HashMap, a HashSet, or a Result. How does one method do all that?",
 back="<b>It has no behaviour of its own. It asks the type you named to build ITSELF from your iterator &mdash; "
      "<code>FromIterator</code>.</b>"
      "<br><br><code>fn collect&lt;B: FromIterator&lt;Self::Item&gt;&gt;(self) -&gt; B</code>. The work lives in "
      "<code>B</code>'s impl, not in <code>collect</code>. That is also why the target type must be stated: it IS the "
      "choice of algorithm."
      "<br><br>A pair-yielding iterator collects into a <code>HashMap</code>; a <code>char</code>-yielding one into a "
      "<code>String</code>; and your own type can implement <code>FromIterator</code> and join in.",
 code='''use std::collections::{HashMap, HashSet};

fn main() {
    let v: Vec<i32>          = (1..=3).collect();
    let s: String            = "abc".chars().rev().collect();
    let set: HashSet<i32>    = vec![1, 1, 2].into_iter().collect();
    let m: HashMap<&str, i32> = vec![("a", 1), ("b", 2)].into_iter().collect();
    println!("{v:?} {s} {} {}", set.len(), m["a"]);
}''',
 expect="[1, 2, 3] cba 2 1",
 code_on="back",
 link=("collect_and_fromiterator", SITE+"24_Iterators/collect_and_fromiterator/index.html"),
 tags="rust iterators core"),

dict(id="it_collect_result",
 front="You have <code>Vec&lt;&amp;str&gt;</code> and want <code>Vec&lt;i32&gt;</code>, failing loudly if any row is not a number. Write it.",
 back="<b><code>let ns: Result&lt;Vec&lt;i32&gt;, _&gt; = rows.iter().map(|s| s.parse::&lt;i32&gt;()).collect();</code></b>"
      "<br><br><b>An iterator of <code>Result</code>s collects into ONE <code>Result</code> of a Vec</b> &mdash; and it "
      "short-circuits, stopping at the first <code>Err</code> rather than parsing the rest. Same trick for "
      "<code>Option</code>."
      "<br><br>Note where the type annotation goes: <code>Result&lt;Vec&lt;_&gt;, _&gt;</code>, not "
      "<code>Vec&lt;Result&lt;_, _&gt;&gt;</code> &mdash; that other spelling is also legal and gives you every row's "
      "outcome separately. The two are one character apart and mean completely different things.",
 code='''fn main() {
    let rows = ["1", "2", "3"];
    let all: Result<Vec<i32>, _> = rows.iter().map(|s| s.parse::<i32>()).collect();
    println!("{all:?}");

    let bad = ["1", "x", "3"];
    let some: Result<Vec<i32>, _> = bad.iter().map(|s| s.parse::<i32>()).collect();
    println!("{}", some.is_err());

    let each: Vec<Result<i32, _>> = bad.iter().map(|s| s.parse::<i32>()).collect();
    println!("{}", each.iter().filter(|r| r.is_ok()).count());
}''',
 expect="Ok([1, 2, 3])\ntrue\n2",
 code_on="back",
 link=("collect_and_fromiterator", SITE+"24_Iterators/collect_and_fromiterator/index.html"),
 tags="rust iterators result"),

dict(id="it_double_ref",
 front="Why does <code>v.iter().filter(|x| x &gt; 2)</code> not compile, when <code>v</code> is a <code>Vec&lt;i32&gt;</code>?",
 back="<b>Because <code>x</code> is a <code>&amp;&amp;i32</code>. <code>iter()</code> yields <code>&amp;i32</code>, and "
      "<code>filter</code> hands its closure a reference to that.</b>"
      "<br><br>Three fixes, all common:"
      "<br>• <code>.filter(|&amp;&amp;x| x &gt; 2)</code> &mdash; destructure both layers"
      "<br>• <code>.filter(|x| **x &gt; 2)</code> &mdash; dereference twice"
      "<br>• <code>.copied()</code> before it, so the stream is <code>i32</code> and the closure sees <code>&amp;i32</code>"
      "<br><br><code>filter</code> is the odd one out: <code>map</code> takes the item <i>by value</i>, so "
      "<code>.map(|x| x * 2)</code> on <code>&amp;i32</code> just works. Only the predicate adapters "
      "(<code>filter</code>, <code>take_while</code>, <code>skip_while</code>, <code>position</code>) add the extra "
      "<code>&amp;</code>, because they must not consume what they are judging.",
 code='''fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{:?}", v.iter().filter(|&&x| x > 2).collect::<Vec<_>>());
    println!("{:?}", v.iter().filter(|x| **x > 2).collect::<Vec<_>>());
    println!("{:?}", v.iter().copied().filter(|x| *x > 2).collect::<Vec<_>>());
    println!("{:?}", v.iter().map(|x| x * 2).collect::<Vec<_>>());
}''',
 expect="[3, 4]\n[3, 4]\n[3, 4]\n[2, 4, 6, 8]",
 code_on="back",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators gotcha"),

dict(id="it_filter_vs_take_while",
 front="What does this print?",
 code='''fn main() {
    let scores = [5, 3, 0, 4, 2, 1];
    println!("{:?}", scores.iter().filter(|s| **s < 5).collect::<Vec<_>>());
    println!("{:?}", scores.iter().take_while(|s| **s < 5).collect::<Vec<_>>());
}''',
 code_on="front",
 expect="[3, 0, 4, 2, 1]\n[]",
 back="<b><code>filter</code> tests EVERY item. <code>take_while</code> stops at the FIRST failure and never looks again.</b>"
      "<br><br>The leading <code>5</code> ends <code>take_while</code> immediately, so the result is empty."
      "<br><br>On <b>sorted</b> data that early stop is exactly what you want and much cheaper. On <b>unsorted</b> data "
      "it silently returns a prefix that looks like a plausible answer &mdash; the worst kind of bug, because nothing "
      "is wrong-looking."
      "<br><br><code>skip_while</code> is the mirror: it drops the leading run and then keeps everything, failures included.",
 bridge="<b>Python:</b> identical trap &mdash; <code>itertools.takewhile</code> vs <code>filter</code>, including the "
        "sorted/unsorted distinction.",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators gotcha"),

dict(id="it_flat_map_errors",
 front="What does this print? <code>rows = [\"1\", \"x\", \"3\"]</code>",
 code='''fn main() {
    let rows = ["1", "x", "3"];
    let parsed: Vec<i32> = rows.iter().flat_map(|s| s.parse::<i32>()).collect();
    println!("{parsed:?} from {} rows", rows.len());
}''',
 code_on="front",
 expect="[1, 3] from 3 rows",
 back="<b><code>flat_map</code> over a <code>Result</code> DELETES your errors.</b>"
      "<br><br>A <code>Result</code> is an iterator of length 0 or 1, so this compiles, reads well, and silently drops "
      "the failed row. Three rows in, two out &mdash; with the same type and shape as the success case, and nothing "
      "anywhere saying a row vanished."
      "<br><br>Legitimate when <i>\"skip what does not parse\"</i> is genuinely the requirement. A silent data-loss bug "
      "when it is not."
      "<br><br>When the failure matters: <code>collect()</code> into a <code>Result&lt;Vec&lt;_&gt;, _&gt;</code> and let "
      "the first error stop the chain. Same for <code>filter_map</code> with <code>.ok()</code>.",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators gotcha result"),

dict(id="it_filter_map",
 front="Keep only the items a fallible conversion succeeds on, and unwrap them in one step. Which adapter?",
 back="<b><code>filter_map(|x| ...)</code> &mdash; the closure returns <code>Option&lt;U&gt;</code>; "
      "<code>None</code> drops the item, <code>Some(u)</code> keeps <code>u</code>.</b>"
      "<br><br>It replaces <code>.filter(...).map(...)</code> where the test and the conversion are the same operation "
      "&mdash; and it is the honest spelling of the <code>flat_map</code>-over-<code>Result</code> trap, because "
      "<code>.ok()</code> makes the discard visible at the call site."
      "<br><br><code>map_while</code> is the <code>take_while</code>-flavoured cousin: it stops at the first "
      "<code>None</code> rather than skipping it.",
 code='''fn main() {
    let rows = ["1", "x", "3"];
    let kept: Vec<i32> = rows.iter().filter_map(|s| s.parse().ok()).collect();
    println!("{kept:?}");

    let stopped: Vec<i32> = rows.iter().map_while(|s| s.parse().ok()).collect();
    println!("{stopped:?}");
}''',
 expect="[1, 3]\n[1]",
 code_on="back",
 bridge="<b>Python:</b> no direct equivalent &mdash; you write a comprehension with an <code>if</code>, or "
        "<code>filter(None, map(f, xs))</code>.",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators option"),

dict(id="it_enumerate_position",
 front="What does this print? (where does <code>enumerate</code> sit relative to <code>filter</code>?)",
 code='''fn main() {
    let v = ["a", "b", "c"];
    let after: Vec<_> = v.iter().filter(|s| **s != "a").enumerate().collect();
    println!("{after:?}");
    let before: Vec<_> = v.iter().enumerate().filter(|(_, s)| **s != "a").collect();
    println!("{before:?}");
}''',
 code_on="front",
 expect='[(0, "b"), (1, "c")]\n[(1, "b"), (2, "c")]',
 back="<b><code>enumerate</code> counts the items IT sees &mdash; so after a <code>filter</code> the index is a position "
      "in the filtered stream, not in the original.</b>"
      "<br><br>Put <code>enumerate</code> <b>first</b> when you want the index in the source data (line numbers in a "
      "file, row numbers in a table), and destructure the tuple in the later closures."
      "<br><br>This is the single most common off-by-index bug in Rust iterator chains, and it never fails to compile.",
 bridge="<b>Python:</b> exactly the same &mdash; <code>enumerate(x for x in xs if ...)</code> vs "
        "<code>((i, x) for i, x in enumerate(xs) if ...)</code>.",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators gotcha"),

dict(id="it_zip",
 front="<code>a.iter().zip(b.iter())</code> where <code>a</code> has 3 items and <code>b</code> has 5. How many pairs, and how do you undo it?",
 back="<b>Three. <code>zip</code> stops when the SHORTER one runs out &mdash; silently, no error.</b>"
      "<br><br><code>unzip()</code> is the inverse: an iterator of pairs into two collections."
      "<br><br>Because it stops short, <code>zip</code> composes beautifully with an infinite iterator: "
      "<code>(1..).zip(names.iter())</code> is a numbered list with no bounds arithmetic."
      "<br><br>If the lengths <i>should</i> match, check first &mdash; nothing here will tell you they did not.",
 code='''fn main() {
    let a = [1, 2, 3];
    let b = ["x", "y", "z", "w", "v"];
    let pairs: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("{pairs:?}");

    let (nums, names): (Vec<i32>, Vec<&str>) = pairs.into_iter().map(|(n, s)| (*n, *s)).unzip();
    println!("{nums:?} {names:?}");

    let numbered: Vec<_> = (1..).zip(["a", "b"]).collect();
    println!("{numbered:?}");
}''',
 expect='[(1, "x"), (2, "y"), (3, "z")]\n[1, 2, 3] ["x", "y", "z"]\n[(1, "a"), (2, "b")]',
 code_on="back",
 bridge="<b>Python:</b> <code>zip</code> also stops at the shortest; <code>itertools.zip_longest</code> is the other "
        "behaviour, and Rust has no std equivalent.",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators"),

dict(id="it_rev_trait",
 front="<code>.rev()</code> and <code>.len()</code> are not on every iterator. Why not?",
 back="<b>They are two extra traits, each a PROMISE about the sequence that <code>next</code> alone cannot make: "
      "<code>DoubleEndedIterator</code> and <code>ExactSizeIterator</code>.</b>"
      "<br><br><code>rev()</code> needs a <code>next_back()</code> &mdash; the sequence must be walkable from both ends. "
      "A slice can. A <code>HashMap</code> iterator cannot, and a lazily-read file cannot."
      "<br><br><code>len()</code> needs the count to be known without walking. <code>filter</code> destroys that "
      "promise, which is why <code>v.iter().filter(..).len()</code> is <code>E0599</code> while "
      "<code>v.iter().filter(..).count()</code> works &mdash; <code>count()</code> walks and is O(n)."
      "<br><br>Checked at compile time, so you cannot get this wrong at runtime.",
 code='''fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v.iter().rev().collect::<Vec<_>>());
    println!("{}", v.iter().len());
    println!("{}", v.iter().filter(|x| **x > 1).count());
    println!("{:?}", v.iter().rev().next());
}''',
 expect="[3, 2, 1]\n3\n2\nSome(3)",
 code_on="back",
 link=("double_ended_and_exact_size", SITE+"24_Iterators/double_ended_and_exact_size/index.html"),
 tags="rust iterators traits"),

dict(id="it_windows_not_adapter",
 front="<code>v.iter().windows(2)</code> &mdash; what happens?",
 back="<b><code>E0599</code>: no method named <code>windows</code>. <code>windows</code> and <code>chunks</code> are "
      "SLICE methods, not iterator adapters.</b>"
      "<br><br>The reason is structural: both need to look at several items at once, and an iterator that has handed "
      "you an item cannot go back for it. Slices can, because the data is all still there."
      "<br><br>Fix: keep the slice (<code>v.windows(2)</code>), or <code>collect()</code> first."
      "<br><br><code>dedup</code> is the same story on <code>Vec</code>, and <code>sort</code> too &mdash; sorting needs "
      "the whole sequence at once.",
 code='''fn main() {
    let v = vec![1, 2, 4];
    println!("{:?}", v.windows(2).collect::<Vec<_>>());
    let collected: Vec<i32> = (1..=3).collect();
    println!("{:?}", collected.windows(2).collect::<Vec<_>>());
}''',
 expect="[[1, 2], [2, 4]]\n[[1, 2], [2, 3]]",
 code_on="back",
 link=("no_method_named", SITE+"12_Traits/no_method_named/index.html"),
 tags="rust iterators gotcha compile-error"),

dict(id="it_fold",
 front="When do you reach for <code>fold</code> instead of <code>sum</code>, <code>count</code> or <code>collect</code>?",
 back="<b>When the answer is not the same type as the items.</b>"
      "<br><br><code>fold(init, |acc, item| ...)</code> carries an accumulator through the sequence and hands it back. "
      "<code>sum</code>, <code>count</code>, <code>all</code> and <code>collect</code> are each <code>fold</code> with a "
      "particular accumulator &mdash; which is worth knowing because it means there is nothing exotic left to learn "
      "when a consumer does not exist for your case."
      "<br><br><code>try_fold</code> is the short-circuiting version for a fallible step.",
 code='''fn main() {
    let words = ["ada", "bo", "c"];
    let total_len = words.iter().fold(0, |acc, w| acc + w.len());
    println!("{total_len}");

    let joined = words.iter().fold(String::new(), |mut acc, w| {
        if !acc.is_empty() { acc.push('-'); }
        acc.push_str(w);
        acc
    });
    println!("{joined}");
}''',
 expect="6\nada-bo-c",
 code_on="back",
 bridge="<b>Python:</b> <code>functools.reduce(f, xs, init)</code>. "
        "<b>JavaScript:</b> <code>Array.prototype.reduce</code>.",
 link=("fold_and_reduce", SITE+"24_Iterators/fold_and_reduce/index.html"),
 tags="rust iterators"),

dict(id="it_reduce",
 front="<code>fold</code> vs <code>reduce</code> &mdash; what is the difference, and what does <code>reduce</code> return?",
 back="<b><code>reduce</code> has no initial value: it uses the FIRST item as the seed &mdash; so it returns "
      "<code>Option&lt;T&gt;</code>, because an empty iterator has no seed.</b>"
      "<br><br>That also forces the accumulator to be the item type, which is exactly the case <code>fold</code> exists "
      "to escape."
      "<br><br>Rule of thumb: an empty input has a sensible answer &rarr; <code>fold</code> with that answer as the "
      "seed. An empty input is meaningless (\"the longest word of nothing\") &rarr; <code>reduce</code>, and the "
      "<code>Option</code> is telling you something true.",
 code='''fn main() {
    let v = vec![3, 1, 2];
    println!("{:?}", v.iter().copied().reduce(|a, b| a.max(b)));
    println!("{}", v.iter().fold(0, |a, b| a + b));

    let empty: Vec<i32> = vec![];
    println!("{:?}", empty.iter().copied().reduce(|a, b| a + b));
    println!("{}", empty.iter().fold(0, |a, b| a + b));
}''',
 expect="Some(3)\n6\nNone\n0",
 code_on="back",
 link=("fold_and_reduce", SITE+"24_Iterators/fold_and_reduce/index.html"),
 tags="rust iterators option"),

dict(id="it_scan",
 front="You want a running total &mdash; every intermediate value, not just the final one. Which adapter?",
 back="<b><code>scan(init, |state, item| ...)</code>. <code>fold</code> gives the final accumulator; "
      "<code>scan</code> gives every step.</b>"
      "<br><br>Two details do a lot of work:"
      "<br>• the state is <code>&amp;mut</code> &mdash; you mutate <i>through</i> it rather than returning it, unlike "
      "<code>fold</code>"
      "<br>• the closure returns an <code>Option</code>, and returning <code>None</code> <b>ends the iterator</b> &mdash; "
      "which makes <code>scan</code> the way to write a <code>take_while</code> whose decision depends on everything "
      "seen so far",
 code='''fn main() {
    let scores = [5, 3, 0, 4, 2, 1];
    let running: Vec<i32> = scores.iter()
        .scan(0, |total, s| { *total += s; Some(*total) })
        .collect();
    println!("{running:?}");

    let until_10: Vec<i32> = scores.iter()
        .scan(0, |t, s| { *t += s; if *t > 10 { None } else { Some(*t) } })
        .collect();
    println!("{until_10:?}");
}''',
 expect="[5, 8, 8, 12, 14, 15]\n[5, 8, 8]",
 code_on="back",
 bridge="<b>Python:</b> <code>itertools.accumulate</code>.",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators"),

dict(id="it_peekable",
 front="Look at the next item WITHOUT consuming it. Which adapter &mdash; and what is the catch?",
 back="<b><code>.peekable()</code>, then <code>it.peek()</code>. The catch: <code>peek</code> takes "
      "<code>&amp;mut self</code>, so the iterator must be <code>let mut</code>.</b>"
      "<br><br>It looks like a read-only convenience, but <code>peek</code> has to pull the item and hold it &mdash; "
      "that IS a mutation. And a <code>peek</code> whose borrow is still live where you also call <code>next</code> is "
      "a borrow error, not a runtime problem."
      "<br><br><code>peek</code> returns <code>Option&lt;&amp;Item&gt;</code>, and repeated peeks return the same item. "
      "But you cannot write two of them in one <code>println!</code> &mdash; that is two <code>&amp;mut</code> "
      "borrows at once, <code>E0499</code>. Verified the hard way while building this deck.",
 code='''fn main() {
    let v = vec![1, 2, 3];
    let mut it = v.iter().peekable();
    println!("{:?}", it.peek());
    println!("{:?}", it.peek());   // same item -- peek does not advance
    println!("{:?}", it.next());   // now it does
    println!("{:?}", it.peek());
    // println!("{:?} {:?}", it.peek(), it.peek());  <- E0499, two &mut at once
}''',
 expect="Some(1)\nSome(1)\nSome(1)\nSome(2)",
 code_on="back",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators borrow-checker"),

dict(id="it_take_infinite",
 front="Is <code>(1..)</code> legal? What stops it running forever?",
 back="<b>Yes &mdash; <code>(1..)</code> is a <code>RangeFrom</code>, an infinite iterator. Laziness is what makes it "
      "safe: nothing runs until a consumer asks, and <code>take(n)</code> stops asking.</b>"
      "<br><br>The danger is only in consumers that need the END: <code>collect</code>, <code>count</code>, "
      "<code>sum</code>, <code>last</code>, <code>max</code> on an infinite iterator hang forever. "
      "<code>find</code>, <code>any</code>, <code>position</code>, <code>take</code>, <code>zip</code> are all fine "
      "because they can stop early."
      "<br><br><code>std::iter::repeat(x)</code>, <code>cycle()</code> and <code>successors</code> are the other "
      "infinite generators.",
 code='''fn main() {
    let squares: Vec<i32> = (1..).map(|n| n * n).take(5).collect();
    println!("{squares:?}");
    println!("{:?}", (1..).find(|n| n % 7 == 0 && n % 5 == 0));
    println!("{:?}", ["a", "b"].iter().cycle().take(5).collect::<Vec<_>>());
}''',
 expect='[1, 4, 9, 16, 25]\nSome(35)\n["a", "b", "a", "b", "a"]',
 code_on="back",
 link=("iterators_are_lazy", SITE+"24_Iterators/iterators_are_lazy/index.html"),
 tags="rust iterators"),

dict(id="it_cloned_copied",
 front="<code>.cloned()</code> and <code>.copied()</code> &mdash; what do they do, and which should you prefer?",
 back="<b>Both turn a stream of <code>&amp;T</code> into a stream of <code>T</code>. <code>copied()</code> requires "
      "<code>T: Copy</code>; <code>cloned()</code> only <code>T: Clone</code>.</b>"
      "<br><br><b>Prefer <code>copied()</code> when it compiles.</b> It is the same machine code, but it cannot "
      "silently become expensive &mdash; if someone later changes the element type to a <code>String</code>, "
      "<code>copied()</code> stops compiling while <code>cloned()</code> quietly starts allocating per item."
      "<br><br>Both are just <code>.map(|x| *x)</code> / <code>.map(|x| x.clone())</code> with a name that says why.",
 code='''fn main() {
    let v = vec![1, 2, 3];
    let owned: Vec<i32> = v.iter().copied().collect();
    println!("{}", owned.iter().sum::<i32>());

    let names = vec![String::from("ada")];
    let cloned: Vec<String> = names.iter().cloned().collect();
    println!("{cloned:?} {names:?}");
}''',
 expect='6\n["ada"] ["ada"]',
 code_on="back",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators"),

dict(id="it_min_max",
 front="<code>max()</code>, <code>max_by_key()</code>, <code>max_by()</code> &mdash; when each? And what about floats?",
 back="<b><code>max()</code> needs <code>Ord</code>. <code>max_by_key(|x| ...)</code> compares a derived key. "
      "<code>max_by(|a, b| ...)</code> takes a comparator. All return <code>Option</code>.</b>"
      "<br><br><b>Floats have no <code>Ord</code></b> (because of <code>NaN</code>), so <code>max()</code> will not "
      "compile on <code>f64</code>. Use <code>max_by(|a, b| a.total_cmp(b))</code>."
      "<br><br>Tie-break trap: <code>max</code> returns the <b>LAST</b> maximum, <code>min</code> the <b>first</b>. "
      "That asymmetry is documented and deliberate, and it bites when the items carry identity.",
 code='''fn main() {
    let words = ["bb", "a", "cc"];
    println!("{:?}", words.iter().max());
    println!("{:?}", words.iter().max_by_key(|w| w.len()));
    println!("{:?}", words.iter().min_by_key(|w| w.len()));

    let f = [2.5f64, 0.5, 1.5];
    println!("{:?}", f.iter().copied().max_by(|a, b| a.total_cmp(b)));
}''',
 expect='Some("cc")\nSome("cc")\nSome("a")\nSome(2.5)',
 code_on="back",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators option gotcha"),

dict(id="it_find_family",
 front="<code>find</code>, <code>position</code>, <code>find_map</code>, <code>any</code> &mdash; what does each give back?",
 back="<b><code>find</code> &rarr; <code>Option&lt;Item&gt;</code> (the thing) · <code>position</code> &rarr; "
      "<code>Option&lt;usize&gt;</code> (where) · <code>find_map</code> &rarr; <code>Option&lt;U&gt;</code> (the "
      "converted thing) · <code>any</code> &rarr; <code>bool</code></b>"
      "<br><br>All four short-circuit at the first hit, and all four take <code>&amp;mut self</code> &mdash; so the "
      "iterator is left <b>positioned after the match</b> and can be read on."
      "<br><br><code>find_map</code> is the one people re-implement by hand: it is "
      "<code>filter_map(..).next()</code>, for \"the first row that converts successfully\".",
 code='''fn main() {
    let v = ["a", "12", "b", "34"];
    println!("{:?}", v.iter().find(|s| s.len() == 2));
    println!("{:?}", v.iter().position(|s| s.len() == 2));
    println!("{:?}", v.iter().find_map(|s| s.parse::<i32>().ok()));
    println!("{}", v.iter().any(|s| *s == "b"));

    let mut it = v.iter();
    it.find(|s| **s == "b");
    println!("{:?}", it.next());
}''',
 expect='Some("12")\nSome(1)\nSome(12)\ntrue\nSome("34")',
 code_on="back",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators option"),

dict(id="it_partition",
 front="Split a sequence into two collections by a predicate, in one pass. Which consumer?",
 back="<b><code>partition(|x| ...)</code> &mdash; returns a tuple, and you must annotate BOTH halves.</b>"
      "<br><br><code>let (pass, fail): (Vec&lt;_&gt;, Vec&lt;_&gt;) = it.partition(|x| ...);</code>"
      "<br><br>Like <code>collect</code>, it is generic over the container, so the type annotation is the instruction. "
      "The two halves need not be the same container type."
      "<br><br>For <code>Result</code>s specifically, <code>partition</code> gives you both the successes and the "
      "failures &mdash; the honest alternative to <code>flat_map</code> throwing errors away.",
 code='''fn main() {
    let v = 1..=6;
    let (even, odd): (Vec<i32>, Vec<i32>) = v.partition(|n| n % 2 == 0);
    println!("{even:?} {odd:?}");

    let rows = ["1", "x", "3"];
    let (ok, bad): (Vec<_>, Vec<_>) = rows.iter()
        .map(|s| s.parse::<i32>())
        .partition(|r| r.is_ok());
    println!("{} ok, {} bad", ok.len(), bad.len());
}''',
 expect="[2, 4, 6] [1, 3, 5]\n2 ok, 1 bad",
 code_on="back",
 bridge="<b>Python:</b> absent from the stdlib &mdash; the docs give it as an <code>itertools</code> recipe.",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators"),

dict(id="it_return_impl",
 front="You want a function to return an iterator chain. What is the return type?",
 back="<b><code>impl Iterator&lt;Item = T&gt;</code> &mdash; how a function hands back a chain it cannot name.</b>"
      "<br><br>The real type is something like "
      "<code>Filter&lt;Map&lt;slice::Iter&lt;'_, i32&gt;, closure&gt;, closure&gt;</code>, and closures have no "
      "nameable type at all, so writing it out is not merely tedious &mdash; it is impossible."
      "<br><br>Two things go wrong:"
      "<br>• <b>a lifetime</b> the opaque type has to capture &mdash; borrowing an argument means "
      "<code>impl Iterator&lt;Item = &amp;T&gt; + '_</code>"
      "<br>• <b>a second branch</b> returning a different concrete type &mdash; <code>impl Trait</code> is ONE type, so "
      "an <code>if</code> with two different chains needs <code>Box&lt;dyn Iterator&lt;Item = T&gt;&gt;</code> or "
      "<code>Either</code>",
 code='''fn evens(v: &[i32]) -> impl Iterator<Item = i32> + '_ {
    v.iter().copied().filter(|n| n % 2 == 0)
}

fn maybe(flag: bool) -> Box<dyn Iterator<Item = i32>> {
    if flag { Box::new(1..3) } else { Box::new((1..6).filter(|n| n % 2 == 1)) }
}

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{:?}", evens(&v).collect::<Vec<_>>());
    println!("{:?}", maybe(false).collect::<Vec<_>>());
}''',
 expect="[2, 4]\n[1, 3, 5]",
 code_on="back",
 link=("returning_an_iterator", SITE+"24_Iterators/returning_an_iterator/index.html"),
 tags="rust iterators traits"),

dict(id="it_implementing",
 front="What is the minimum you must write to make your own type an iterator &mdash; and what do you get for it?",
 back="<b>An <code>Item</code> type and a <code>next</code> method. That is the whole trait.</b>"
      "<br><br><code>impl Iterator for T { type Item = X; fn next(&amp;mut self) -&gt; Option&lt;Self::Item&gt; }</code> "
      "&mdash; and roughly seventy-five provided methods arrive: <code>map</code>, <code>filter</code>, "
      "<code>zip</code>, <code>sum</code>, <code>collect</code>, all of it."
      "<br><br><b>The warning:</b> implement <code>Iterator</code> on a <i>collection</i> and you have built something "
      "that empties itself the first time anybody reads it. Collections implement <code>IntoIterator</code> instead, "
      "and hand out a separate cursor type &mdash; which is why <code>Vec</code> has <code>vec::IntoIter</code>.",
 code='''struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 { return None; }
        self.0 -= 1;
        Some(self.0 + 1)
    }
}

fn main() {
    println!("{:?}", Countdown(4).collect::<Vec<_>>());
    println!("{}", Countdown(4).filter(|n| n % 2 == 0).sum::<u32>());
}''',
 expect="[4, 3, 2, 1]\n6",
 code_on="back",
 link=("implementing_iterator", SITE+"24_Iterators/implementing_iterator/index.html"),
 tags="rust iterators traits"),

dict(id="it_by_ref",
 front="Consume the first 2 items of an iterator, then keep using the SAME iterator. What do you need?",
 back="<b><code>.by_ref()</code> &mdash; adapters take <code>self</code> by value, so without it the iterator is moved "
      "into the adapter and gone.</b>"
      "<br><br><code>it.by_ref().take(2)</code> borrows instead of moving, so when that chain is dropped the original "
      "<code>it</code> is still usable &mdash; and correctly positioned after the items that were taken."
      "<br><br>The classic use: read a header off a line iterator, then process the rest.",
 code='''fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut it = v.iter();
    let head: Vec<_> = it.by_ref().take(2).collect();
    let tail: Vec<_> = it.collect();
    println!("{head:?} {tail:?}");
}''',
 expect="[1, 2] [3, 4, 5]",
 code_on="back",
 link=("iterators_are_lazy", SITE+"24_Iterators/iterators_are_lazy/index.html"),
 tags="rust iterators ownership"),

dict(id="it_skip_step",
 front="Skip the first n, take every k-th, and take the last n. Which of those three is awkward?",
 back="<b><code>skip(n)</code> and <code>step_by(k)</code> are adapters. <b>There is no <code>take_last</code></b>.</b>"
      "<br><br>Taking from the end needs either a <code>DoubleEndedIterator</code> "
      "(<code>.rev().take(n).rev()</code> &mdash; note it reverses the order twice) or the slice: "
      "<code>&amp;v[v.len() - n..]</code>."
      "<br><br>The general shape: anything needing to know where the END is fits badly on a forward cursor. That is not "
      "an omission, it is the same promise-of-the-sequence idea as <code>DoubleEndedIterator</code>.",
 code='''fn main() {
    let v: Vec<i32> = (1..=10).collect();
    println!("{:?}", v.iter().skip(7).collect::<Vec<_>>());
    println!("{:?}", v.iter().step_by(3).collect::<Vec<_>>());
    println!("{:?}", v.iter().rev().take(3).rev().collect::<Vec<_>>());
    println!("{:?}", &v[v.len() - 3..]);
}''',
 expect="[8, 9, 10]\n[1, 4, 7, 10]\n[8, 9, 10]\n[8, 9, 10]",
 code_on="back",
 bridge="<b>Python:</b> <code>xs[7:]</code>, <code>xs[::3]</code>, <code>xs[-3:]</code> &mdash; slicing does all three, "
        "because a Python list is never a lazy cursor.",
 link=("double_ended_and_exact_size", SITE+"24_Iterators/double_ended_and_exact_size/index.html"),
 tags="rust iterators"),

dict(id="it_inspect",
 front="A long chain gives the wrong answer. How do you see what is flowing through the middle of it?",
 back="<b><code>.inspect(|x| println!(\"{x:?}\"))</code> &mdash; it passes each item through untouched and lets you "
      "look.</b>"
      "<br><br>Drop it anywhere in a chain without changing the types or the result. Because iterators are lazy, "
      "<b>the print order also shows you the interleaving</b>: items go through the whole chain one at a time, not "
      "stage by stage &mdash; which is usually the thing that was confusing.",
 code='''fn main() {
    let out: Vec<i32> = (1..=3)
        .inspect(|x| println!("in  {x}"))
        .map(|x| x * 10)
        .inspect(|x| println!("out {x}"))
        .collect();
    println!("{out:?}");
}''',
 expect="in  1\nout 10\nin  2\nout 20\nin  3\nout 30\n[10, 20, 30]",
 code_on="back",
 link=("iterators_are_lazy", SITE+"24_Iterators/iterators_are_lazy/index.html"),
 tags="rust iterators debugging"),

dict(id="it_loop_wins",
 front="When is a plain <code>for</code> loop the better choice than an iterator chain?",
 back="<b>The fluent style wins for transform-and-keep and loses for everything with real control flow in it.</b>"
      "<br><br>Reach for a loop when you have:"
      "<br>• <b>an error that names its row</b> &mdash; \"line 47 is malformed\"; a chain has no line number in scope"
      "<br>• <b>a work list that grows while you drain it</b> &mdash; you cannot push onto what you are iterating"
      "<br>• <b>a break out of two levels</b> &mdash; there is no labelled break in a closure"
      "<br>• <b>one pass answering three questions</b> &mdash; three chains means three passes, or an awkward "
      "<code>fold</code> over a tuple"
      "<br><br>This is not a performance argument. Both compile to roughly the same thing. It is about which one a "
      "reader can follow.",
 link=("when_a_loop_beats_a_chain", SITE+"24_Iterators/when_a_loop_beats_a_chain/index.html"),
 tags="rust iterators style"),

dict(id="it_sum_empty",
 front="What is <code>[].iter().sum::&lt;i32&gt;()</code>? And <code>.max()</code> on the same?",
 back="<b><code>sum</code> is <code>0</code>. <code>max</code> is <code>None</code>.</b>"
      "<br><br><code>sum</code> and <code>product</code> have an identity element (0 and 1), so they answer for an "
      "empty sequence. <code>max</code>, <code>min</code>, <code>reduce</code>, <code>last</code>, <code>first</code> "
      "do not, so they return <code>Option</code>."
      "<br><br>Reading the return type tells you which kind you are holding &mdash; and an <code>Option</code> coming "
      "back is the API telling you the empty case is real and needs a decision.",
 code='''fn main() {
    let e: Vec<i32> = vec![];
    println!("{} {}", e.iter().sum::<i32>(), e.iter().product::<i32>());
    println!("{:?} {:?}", e.iter().max(), e.iter().last());
    println!("{}", e.iter().copied().max().unwrap_or(i32::MIN));
}''',
 expect="0 1\nNone None\n-2147483648",
 code_on="back",
 link=("fold_and_reduce", SITE+"24_Iterators/fold_and_reduce/index.html"),
 tags="rust iterators option"),

dict(id="it_chain_flatten",
 front="Join two iterators end to end, and flatten a nested one. Which two adapters?",
 back="<b><code>a.chain(b)</code> runs <code>a</code> then <code>b</code>. <code>.flatten()</code> unwraps one level "
      "of nesting.</b>"
      "<br><br><code>flat_map(f)</code> is <code>map(f).flatten()</code> &mdash; use it when the closure produces the "
      "sequence."
      "<br><br><code>flatten()</code> also works on an iterator of <code>Option</code>s or <code>Result</code>s, "
      "silently dropping the empty ones. Convenient, and the same silent-data-loss hazard as "
      "<code>flat_map</code> over a <code>Result</code>.",
 code='''fn main() {
    let a = [1, 2];
    let b = [3, 4];
    println!("{:?}", a.iter().chain(b.iter()).collect::<Vec<_>>());

    let nested = vec![vec![1, 2], vec![3]];
    println!("{:?}", nested.iter().flatten().collect::<Vec<_>>());

    let words = ["ab", "cd"];
    println!("{:?}", words.iter().flat_map(|s| s.chars()).collect::<Vec<_>>());

    let opts = [Some(1), None, Some(3)];
    println!("{:?}", opts.iter().flatten().collect::<Vec<_>>());
}''',
 expect="[1, 2, 3, 4]\n[1, 2, 3]\n['a', 'b', 'c', 'd']\n[1, 3]",
 code_on="back",
 link=("adapters_by_job", SITE+"24_Iterators/adapters_by_job/index.html"),
 tags="rust iterators"),

dict(id="it_chars_not_index",
 front="A chain gives <code>E0599: no method named ...</code>. What are the two usual causes?",
 back="<b>(1) The method is on the <b>collection</b>, not the iterator (<code>windows</code>, <code>chunks</code>, "
      "<code>sort</code>, <code>dedup</code>, <code>len</code>).<br>"
      "(2) A <b>trait is not in scope</b> &mdash; the method exists but its trait is not <code>use</code>d.</b>"
      "<br><br>Cause 2 is the one that reads as a lie, because the method is right there in the docs. "
      "<code>Itertools</code> methods need <code>use itertools::Itertools;</code>; "
      "<code>read_line</code> needs <code>use std::io::BufRead;</code>."
      "<br><br>rustc usually tells you: <i>\"items from traits can only be used if the trait is in scope\"</i>, and "
      "names the <code>use</code> line. Read to the end of the error.",
 code='''use std::io::Write;

fn main() {
    let mut buf: Vec<u8> = Vec::new();
    // write! needs the Write trait in scope -- without the `use`, E0599
    write!(buf, "hi {}", 42).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}''',
 expect="hi 42",
 code_on="back",
 link=("no_method_named", SITE+"12_Traits/no_method_named/index.html"),
 tags="rust iterators traits compile-error"),
]
