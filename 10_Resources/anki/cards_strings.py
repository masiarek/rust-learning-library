# Anki cards: Rust String & &str.  Every `code` block is compiled with
# rustc 1.98.0 --edition 2024 and run; `expect` must match stdout exactly.
# `fails` cards must NOT compile, with that error code.

SITE = "https://masiarek.github.io/rust-learning-library/"

DECK = "Rust::Strings"

CARDS = [

dict(id="str_two_types",
 front="Rust has two text types. What is each one, in terms of what sits on the <b>stack</b>?",
 back="<b><code>&amp;str</code> = a borrowed view: pointer + length (16 bytes). "
      "<code>String</code> = an owned buffer: pointer + length + capacity (24 bytes).</b>"
      "<br><br>Only <code>String</code> can grow, because only <code>String</code> owns the allocation. "
      "Everything about <i>reading</i> text lives on <code>str</code> — and a <code>String</code> gets all of it free, "
      "because it derefs to <code>str</code>.",
 bridge="<b>Python:</b> one <code>str</code> type, always owned, always immutable. Rust splits <i>ownership</i> from <i>view</i>, "
        "which is the whole reason there are two types.<br><b>ABAP:</b> <code>DATA(lv_b) = lv_a</code> always copies. "
        "A <code>&amp;str</code> is the copy you did <i>not</i> make.",
 link=("string_vs_str", SITE+"14_Strings/string_vs_str/index.html"),
 tags="rust strings core"),

dict(id="str_len_is_bytes",
 front="What does this print?",
 code='''fn main() {
    let s = "héllo";
    println!("{} {}", s.len(), s.chars().count());
}''',
 code_on="front",
 expect="6 5",
 back="<b><code>len()</code> is BYTES. Always.</b> The <code>é</code> is 2 bytes in UTF-8, so 6 bytes hold 5 characters."
      "<br><br>There is no O(1) character count in Rust — <code>chars().count()</code> walks the whole string. "
      "That is the price of storing UTF-8 instead of fixed-width code points.",
 bridge="<b>Python:</b> <code>len(\"héllo\")</code> is <b>5</b> — Python counts code points. This is the single most common "
        "thing a Python programmer gets wrong in Rust.",
 link=("string_len", SITE+"14_Strings/string_methods/string_len/index.html"),
 tags="rust strings gotcha unicode"),

dict(id="str_no_index",
 front="Does this compile?<br><br>Give the reason, and the fix if you want the first character.",
 code='''fn main() {
    let s = String::from("hello");
    let first = s[0];
    println!("{first}");
}''',
 code_on="front",
 fails="E0277",
 back="<b>No — <code>E0277</code>: <code>String</code> cannot be indexed by <code>{integer}</code>.</b>"
      "<br><br>A byte index into UTF-8 is meaningless on its own: index 0 might be a whole character or the first "
      "third of one. Rust refuses rather than guess.<br><br>Fixes:"
      "<br>• first character → <code>s.chars().next()</code> → <code>Option&lt;char&gt;</code>"
      "<br>• first byte → <code>s.as_bytes()[0]</code>"
      "<br>• a range → <code>&amp;s[0..3]</code> <i>does</i> compile (and panics off a boundary)",
 bridge="<b>Python:</b> <code>s[0]</code> is free and safe. In Rust the equivalent is <code>s.chars().next().unwrap()</code>, "
        "and the extra words are the point.",
 link=("walking_a_string", SITE+"14_Strings/walking_a_string/index.html"),
 tags="rust strings gotcha compile-error"),

dict(id="str_slice_boundary",
 front="What does this print?",
 code='''fn main() {
    let s = "héllo";
    println!("{}", &s[0..1]);
    println!("{}", s.is_char_boundary(2));
    println!("{}", &s[0..3]);
}''',
 code_on="front",
 expect="h\nfalse\nhé",
 back="<b>Slicing a string uses BYTE offsets, and panics if an end lands inside a character.</b>"
      "<br><br><code>&amp;s[0..2]</code> would have panicked, and the message names the character it cut: "
      "<code>end byte index 2 is not a char boundary; it is inside 'é' (bytes 1..3 of string)</code>. "
      "The <code>é</code> is bytes 1..3, so 1 and 3 are legal cuts and 2 is not."
      "<br><br><code>is_char_boundary(n)</code> is the test. The two repairs go <i>opposite</i> ways: "
      "<code>floor_char_boundary(2)</code> is <b>1</b> — back up, dropping the <code>é</code>; "
      "<code>ceil_char_boundary(2)</code> is <b>3</b> — forward, keeping it. Both stable since 1.91.",
 bridge="<b>Python:</b> <code>s[0:2]</code> can never fail. In Rust a string slice is the one indexing operation that "
        "panics on <i>valid-looking</i> input.",
 link=("string_slices", SITE+"14_Strings/string_slices/index.html"),
 tags="rust strings gotcha unicode panic"),

dict(id="str_to_owned",
 front="Three ways to turn <code>&amp;str</code> into <code>String</code>. Name them — and say which to reach for.",
 back="<b><code>.to_string()</code> · <code>String::from(s)</code> · <code>.to_owned()</code> — all identical here.</b>"
      "<br><br>Pick by intent:<br>"
      "• <code>String::from(\"x\")</code> — you are <i>constructing</i> a String<br>"
      "• <code>s.to_owned()</code> — you have a borrow and need to own it (says exactly that)<br>"
      "• <code>s.to_string()</code> — most common; comes from <code>Display</code>, so it also works on numbers"
      "<br><br>Off <code>&amp;str</code> they stop being alternatives at all. <code>to_owned()</code> is not a "
      "stringifying operation — it returns the <i>source's</i> owned twin, so <code>42.to_owned()</code> is an "
      "<code>i32</code> while <code>42.to_string()</code> is a <code>String</code>."
      "<br><br>Coming back is free: <code>.as_str()</code>, or <code>&amp;my_string</code> <i>where a "
      "<code>&amp;str</code> is expected</i> — on its own that expression is a <code>&amp;String</code>.",
 link=("making_a_string", SITE+"14_Strings/making_a_string/index.html"),
 tags="rust strings conversion"),

dict(id="str_param_type",
 front="You are writing a function that only <b>reads</b> its text argument. What type should the parameter be — and why not the other one?",
 back="<b><code>fn f(s: &amp;str)</code> — never <code>&amp;String</code>.</b>"
      "<br><br>Deref coercion means a <code>&amp;String</code> converts to <code>&amp;str</code> at the call site for free, so "
      "<code>&amp;str</code> costs existing callers nothing and additionally accepts string literals, "
      "<code>&amp;s[1..4]</code> slices, and anything else that derefs to <code>str</code>."
      "<br><br><code>&amp;String</code> accepts one of those four. It is strictly worse with no upside.",
 code='''fn shout(s: &str) -> String { s.to_uppercase() }

fn main() {
    let owned = String::from("hello");
    println!("{}", shout(&owned));   // &String coerces
    println!("{}", shout("world"));  // literal works too
}''',
 expect="HELLO\nWORLD",
 code_on="back",
 link=("string_vs_str", SITE+"14_Strings/string_vs_str/index.html"),
 tags="rust strings api-design"),

dict(id="str_push",
 front="Append one character vs append text — which method for each?",
 back="<b><code>push(char)</code> for one character · <code>push_str(&amp;str)</code> for text.</b>"
      "<br><br><code>push</code> takes a <code>char</code> (single quotes: <code>'!'</code>), and grows <code>len</code> by "
      "<b>1 to 4 bytes</b> depending on the character. <code>push_str</code> is the workhorse.",
 code='''fn main() {
    let mut s = String::new();
    s.push('a');
    println!("{}", s.len());
    s.push('é');
    println!("{}", s.len());
    s.push('👋');
    println!("{}", s.len());
    s.push_str(" done");
    println!("{s} {}", s.len());
}''',
 expect="1\n3\n7\naé👋 done 12",
 code_on="back",
 bridge="<b>Python:</b> <code>s += \"x\"</code> builds a new string every time; <code>push_str</code> appends into the "
        "existing buffer, which is why Rust string-building in a loop is cheap.",
 link=("building_a_string", SITE+"14_Strings/building_a_string/index.html"),
 tags="rust strings"),

dict(id="str_plus_moves",
 front="Does this compile?",
 code='''fn main() {
    let a = String::from("Hello, ");
    let b = String::from("world");
    let c = a + &b;
    println!("{a} {c}");
}''',
 code_on="front",
 fails="E0382",
 back="<b>No — <code>E0382</code>: borrow of moved value <code>a</code>.</b>"
      "<br><br><code>+</code> on strings is <code>fn add(self, &amp;str)</code> — it <b>consumes the left operand</b> and reuses "
      "its buffer, which is why the right side must be a <code>&amp;str</code> and the left must be owned."
      "<br><br>Use <code>format!(\"{a}{b}\")</code> when you need both afterwards — it borrows everything and moves nothing.",
 bridge="<b>Python:</b> <code>a + b</code> leaves both alive. In Rust <code>+</code> is the <i>optimised</i> path (no new "
        "allocation) and <code>format!</code> is the convenient one.",
 link=("concatenating_strings", SITE+"14_Strings/concatenating_strings/index.html"),
 tags="rust strings ownership compile-error"),

dict(id="str_split_ws",
 front="What does this print? (mind the double space)",
 code='''fn main() {
    let s = "a  b";
    println!("{:?}", s.split(' ').collect::<Vec<_>>());
    println!("{:?}", s.split_whitespace().collect::<Vec<_>>());
}''',
 code_on="front",
 expect='["a", "", "b"]\n["a", "b"]',
 back="<b><code>split(' ')</code> emits an empty string between the two spaces. <code>split_whitespace()</code> does not.</b>"
      "<br><br><code>split(pat)</code> is literal and mechanical: <i>n</i> separators always give <i>n+1</i> pieces, empties "
      "included. <code>split_whitespace()</code> treats any run of whitespace as one separator and skips leading/trailing runs."
      "<br><br>Splitting on a <code>char</code> is the fast path; splitting on <code>&amp;str</code> and on a closure "
      "<code>|c: char| ...</code> also work.",
 bridge="<b>Python:</b> exactly the same split: <code>\"a  b\".split(' ')</code> → <code>['a','','b']</code>, "
        "<code>\"a  b\".split()</code> → <code>['a','b']</code>. Same trap, same fix.",
 link=("inside_a_split", SITE+"14_Strings/inside_a_split/index.html"),
 tags="rust strings gotcha"),

dict(id="str_split_is_lazy",
 front="<code>s.split(',')</code> — what type comes back, and how do you get a <code>Vec</code>?",
 back="<b>An iterator (<code>Split&lt;'_, char&gt;</code>), not a collection. Nothing has been scanned yet.</b>"
      "<br><br><code>.collect::&lt;Vec&lt;_&gt;&gt;()</code> materialises it. The <code>_</code> is inferred as <code>&amp;str</code> — "
      "the pieces <b>borrow the original string</b>, so no text is copied and the source must outlive them."
      "<br><br>Want owned pieces? <code>.map(String::from).collect::&lt;Vec&lt;String&gt;&gt;()</code>.",
 code='''fn main() {
    let s = "a,b,c";
    let borrowed: Vec<&str> = s.split(',').collect();
    let owned: Vec<String> = s.split(',').map(String::from).collect();
    println!("{borrowed:?} {owned:?}");
    println!("{}", s.split(',').count());
}''',
 expect='["a", "b", "c"] ["a", "b", "c"]\n3',
 code_on="back",
 bridge="<b>Python:</b> <code>str.split</code> returns a <i>list</i> immediately. Rust returns a lazy iterator, so "
        "<code>.next()</code> on a 1&nbsp;GB string costs one field, not a billion.",
 link=("inside_a_split", SITE+"14_Strings/inside_a_split/index.html"),
 tags="rust strings iterators"),

dict(id="str_trim_returns",
 front="Does <code>s.trim()</code> modify <code>s</code>? What does it return?",
 back="<b>No. It returns a <code>&amp;str</code> — a narrower window onto the same bytes.</b>"
      "<br><br>Nothing is allocated and nothing is written. That is true of the whole reading half of the API: "
      "<code>trim</code>, <code>split</code>, <code>strip_prefix</code>, <code>lines</code> all hand back views."
      "<br><br>Want to keep it? <code>let t = s.trim().to_string();</code> — and note <code>trim</code> "
      "borrows <code>s</code>, so <code>s</code> must stay alive as long as the view does.",
 code='''fn main() {
    let s = String::from("  hi  ");
    let t = s.trim();
    println!("[{t}] [{s}] {} {}", s.len(), t.len());
}''',
 expect="[hi] [  hi  ] 6 2",
 code_on="back",
 bridge="<b>Python:</b> <code>s.strip()</code> also returns a new value rather than mutating — but Python <i>copies</i>, "
        "Rust just moves two pointers.",
 link=("str_trim", SITE+"14_Strings/str_methods/str_trim/index.html"),
 tags="rust strings"),

dict(id="str_parse",
 front="Turn <code>\"42\"</code> into an <code>i32</code>. Write the line — and say what type comes back.",
 back="<b><code>let n: i32 = \"42\".parse()?;</code> — or <code>\"42\".parse::&lt;i32&gt;()</code> with the turbofish.</b>"
      "<br><br><code>parse</code> returns <code>Result&lt;F, F::Err&gt;</code>, never the number directly, because text can lie. "
      "The target type must be known — from the annotation on the left, or the <code>::&lt;&gt;</code> on the right."
      "<br><br><code>unwrap_or(0)</code>, <code>?</code>, or a <code>match</code> — but never ignore it.",
 code='''fn main() {
    let good = "42".parse::<i32>();
    let bad  = "4x".parse::<i32>();
    println!("{good:?}");
    println!("{}", bad.is_err());
    let n: i32 = "7".parse().unwrap_or(0);
    println!("{n}");
}''',
 expect="Ok(42)\ntrue\n7",
 code_on="back",
 bridge="<b>Python:</b> <code>int(\"4x\")</code> <i>raises</i>. Rust hands you the failure as a value you must open, "
        "which is why there is no unhandled-exception equivalent.",
 link=("parsing_a_string", SITE+"14_Strings/parsing_a_string/index.html"),
 tags="rust strings result"),

dict(id="str_three_walks",
 front="Three ways to walk a string: <code>chars()</code>, <code>bytes()</code>, <code>char_indices()</code>. What does each yield?",
 back="<b><code>chars()</code> → <code>char</code> · <code>bytes()</code> → <code>u8</code> · <code>char_indices()</code> → "
      "<code>(usize, char)</code> where the usize is a BYTE offset.</b>"
      "<br><br>The byte offset in <code>char_indices</code> is the one you can feed back into <code>&amp;s[..]</code> — "
      "<code>enumerate()</code> would give you a <i>character</i> counter, which is not a valid slice index.",
 code='''fn main() {
    let s = "hé";
    println!("{:?}", s.chars().collect::<Vec<_>>());
    println!("{:?}", s.bytes().collect::<Vec<_>>());
    println!("{:?}", s.char_indices().collect::<Vec<_>>());
}''',
 expect="['h', 'é']\n[104, 195, 169]\n[(0, 'h'), (1, 'é')]",
 code_on="back",
 link=("walking_a_string", SITE+"14_Strings/walking_a_string/index.html"),
 tags="rust strings iterators unicode"),

dict(id="str_find_byte_index",
 front="What does <code>\"héllo\".find('l')</code> return — the value AND the type?",
 back="<b><code>Some(3)</code> — an <code>Option&lt;usize&gt;</code> holding a BYTE offset.</b>"
      "<br><br>Not 2. The <code>é</code> is two bytes, so the first <code>l</code> sits at byte 3 even though it is the "
      "third character. That is deliberate: the number is directly usable as <code>&amp;s[3..]</code>."
      "<br><br><code>None</code> when absent — so <code>if let Some(i) = s.find(..)</code>, never a <code>-1</code> sentinel.",
 code='''fn main() {
    let s = "héllo";
    println!("{:?}", s.find('l'));
    println!("{:?}", s.find('z'));
    println!("{}", &s[s.find('l').unwrap()..]);
}''',
 expect="Some(3)\nNone\nllo",
 code_on="back",
 bridge="<b>Python:</b> <code>s.find</code> returns <code>-1</code> when missing and counts <i>characters</i>. Rust returns "
        "<code>None</code> and counts <i>bytes</i>. Both halves differ.",
 link=("searching_a_string", SITE+"14_Strings/searching_a_string/index.html"),
 tags="rust strings option gotcha"),

dict(id="str_replace_allocates",
 front="Does <code>s.replace(\"a\", \"b\")</code> change <code>s</code> in place?",
 back="<b>No — it allocates and returns a brand-new <code>String</code>.</b> There is no in-place replace, "
      "because the replacement can be a different byte length."
      "<br><br>Two in-place tools do exist when the shape allows it:"
      "<br>• <code>s.retain(|c| ...)</code> — delete characters, no allocation"
      "<br>• <code>s.replace_range(a..b, \"x\")</code> — swap one byte range",
 code='''fn main() {
    let s = String::from("banana");
    let t = s.replace('a', "o");
    println!("{s} {t}");

    let mut r = String::from("b-a-n");
    r.retain(|c| c != '-');
    println!("{r}");
}''',
 expect="banana bonono\nban",
 code_on="back",
 link=("str_replace", SITE+"14_Strings/str_methods/str_replace/index.html"),
 tags="rust strings"),

dict(id="str_case_returns_string",
 front="Why does <code>to_uppercase()</code> return <code>String</code> and not <code>&amp;str</code>?",
 back="<b>Because case mapping can change the length — it cannot be a view into the original bytes.</b>"
      "<br><br>German <code>ß</code> uppercases to <b>two</b> characters, <code>SS</code>. So the result is new text and "
      "needs a new allocation. Same reason <code>replace</code> allocates."
      "<br><br>(<code>to_ascii_uppercase</code> exists and is cheaper — but it leaves every non-ASCII byte alone.)",
 code='''fn main() {
    let s = "straße";
    println!("{} {}", s.len(), s.to_uppercase());
    println!("{}", s.to_uppercase().len());
    println!("{}", s.to_ascii_uppercase());
}''',
 expect="7 STRASSE\n7\nSTRAßE",
 code_on="back",
 link=("str_to_uppercase", SITE+"14_Strings/str_methods/str_to_uppercase/index.html"),
 tags="rust strings unicode"),

dict(id="str_literal_type",
 front="What is the exact type of the literal <code>\"hello\"</code>?",
 back="<b><code>&amp;'static str</code></b> — a borrowed view, with a lifetime that lasts the whole program."
      "<br><br>The bytes are baked into the binary's read-only data. Nothing is allocated at runtime, nothing is freed, "
      "and it can be handed anywhere without a lifetime worry — which is why <code>&amp;'static str</code> is the "
      "no-friction choice for constant text.",
 code='''fn main() {
    let s: &'static str = "hello";
    let n = s.len();
    println!("{s} {n}");
}''',
 expect="hello 5",
 code_on="back",
 link=("static_str", SITE+"14_Strings/static_str/index.html"),
 tags="rust strings lifetimes"),

dict(id="str_unsized",
 front="Does this compile?<br><br>And what does it tell you about why you always meet <code>&amp;str</code> rather than bare <code>str</code>?",
 code='''fn main() {
    let s: str = *"hello";
    println!("{s}");
}''',
 code_on="front",
 fails="E0277",
 back="<b>No &mdash; <code>E0277</code>: the size for values of type <code>str</code> cannot be known at compilation time.</b>"
      "<br><br>A <code>str</code> is a run of UTF-8 bytes and nothing else &mdash; no length travels with it, so the compiler "
      "cannot say how much stack one needs. <code>Sized</code> is not implemented for it, and rustc's own help line is the fix: "
      "<i>consider borrowing here</i>."
      "<br><br>The length lives in the <b>pointer</b>, which is why you only ever meet a <code>str</code> behind one: "
      "<code>&amp;str</code>, <code>Box&lt;str&gt;</code>, <code>Rc&lt;str&gt;</code>. That is what the std docs mean by "
      "<i>usually seen in its borrowed form</i> &mdash; and the same rule, for the same reason, governs <code>[T]</code> "
      "and <code>dyn Trait</code>."
      "<br><br>Sharper still: <code>size_of::&lt;str&gt;()</code> does not compile either. The size is a property of the "
      "value, not of the type.",
 bridge="<b>Python:</b> every value is already a pointer to a heap object, so the question never arises. Rust makes you say "
        "which of the two you are holding.<br><b>ABAP:</b> the same split as <code>TYPE c LENGTH 5</code> against "
        "<code>TYPE string</code> &mdash; a sized field you can declare, versus a handle to something whose length is not "
        "in the type.",
 link=("str is unsized", SITE+"14_Strings/str_is_unsized/index.html"),
 tags="rust strings types compile-error"),

dict(id="str_fat_pointer",
 front="What does this print?<br><br>Why is the reference to the <b>borrowed</b> type bigger than the reference to the <b>owned</b> one?",
 code='''fn main() {
    println!("{} {} {} {}",
        size_of::<&str>(), size_of::<&String>(),
        size_of::<String>(), size_of::<Box<str>>());
}''',
 code_on="front",
 expect="16 8 24 16",
 back="<b>16 8 24 16.</b> <code>&amp;str</code> is a <b>fat pointer</b> &mdash; address + length, two words &mdash; because "
      "the <code>str</code> at the far end carries no length of its own. <code>&amp;String</code> is one word, a plain "
      "address, because the <code>String</code> it points at already holds its own len and capacity."
      "<br><br>Those 8 extra bytes are exactly what buys the ability to point <i>into the middle</i> of a string: every "
      "<code>&amp;s[1..4]</code>, every <code>split</code> item, every <code>trim</code> result is a length the pointer had "
      "to carry. A <code>&amp;String</code> can only ever name a whole one."
      "<br><br><code>Box&lt;str&gt;</code> is the same fat pointer owning instead of borrowing: 16 bytes, no capacity field "
      "&mdash; which is the 8 it saves over <code>String</code>.",
 bridge="<b>Python:</b> <code>sys.getsizeof</code> measures the object; a reference has no size you can ask about, let alone "
        "one that varies by what it points at.<br><b>ABAP:</b> a <code>REF TO</code> is one handle whatever sits behind it.",
 link=("str is unsized", SITE+"14_Strings/str_is_unsized/index.html"),
 tags="rust strings types memory"),

dict(id="str_capacity",
 front="What does this print? (<code>len</code> vs <code>capacity</code>)",
 code='''fn main() {
    let mut s = String::new();
    println!("{} {}", s.len(), s.capacity());
    s.push_str("hello");
    println!("{} {}", s.len(), s.capacity());
}''',
 code_on="front",
 expect="0 0\n5 8",
 back="<b><code>String::new()</code> allocates NOTHING — capacity 0. The first push buys the buffer.</b>"
      "<br><br><code>len</code> is what you wrote; <code>capacity</code> is what you paid for. Growth is amortised doubling, "
      "and the exact numbers are this std's choice, not a promise — so <b>never assert on a capacity in a test</b>."
      "<br><br>Two strings with the same text and different capacities are equal and hash the same."
      "<br><br>Know the size up front? <code>String::with_capacity(n)</code> — one allocation instead of several.",
 link=("string_capacity", SITE+"14_Strings/string_methods/string_capacity/index.html"),
 tags="rust strings memory"),

dict(id="str_pop",
 front="<code>s.pop()</code> on a <code>String</code> — what comes back, and what unit is removed?",
 back="<b><code>Option&lt;char&gt;</code> — one whole character off the end (1–4 bytes), or <code>None</code> if empty.</b>"
      "<br><br>It is the one removal that cannot land mid-character, because it works backwards from a known boundary. "
      "<code>remove(i)</code> and <code>truncate(n)</code> take <b>byte</b> offsets and panic inside a character.",
 code='''fn main() {
    let mut s = String::from("hi👋");
    println!("{}", s.len());
    println!("{:?}", s.pop());
    println!("{} {}", s, s.len());
    let mut e = String::new();
    println!("{:?}", e.pop());
}''',
 expect="6\nSome('👋')\nhi 2\nNone",
 code_on="back",
 bridge="<b>Python:</b> <code>list.pop()</code> on empty <i>raises</i> <code>IndexError</code>. Rust hands back "
        "<code>None</code>, so the empty case is impossible to forget.",
 link=("string_pop", SITE+"14_Strings/string_methods/string_pop/index.html"),
 tags="rust strings option"),

dict(id="str_join",
 front="Join <code>[\"a\", \"b\", \"c\"]</code> with <code>\", \"</code>. Write it — and note where the method lives.",
 back="<b><code>parts.join(\", \")</code> — the method is on the SLICE, not on the separator.</b>"
      "<br><br>This is backwards from Python and catches everyone once. It works on <code>&amp;[&amp;str]</code>, "
      "<code>Vec&lt;String&gt;</code>, and <code>&amp;[Vec&lt;T&gt;]</code> alike. <code>concat()</code> is the same thing "
      "with an empty separator.",
 code='''fn main() {
    let parts = ["a", "b", "c"];
    println!("{}", parts.join(", "));
    println!("{}", parts.concat());
    let owned: Vec<String> = vec!["x".into(), "y".into()];
    println!("{}", owned.join("-"));
}''',
 expect="a, b, c\nabc\nx-y",
 code_on="back",
 bridge="<b>Python:</b> <code>\", \".join(parts)</code> — separator first. Rust puts it the other way round."
        "<br><b>ABAP:</b> <code>CONCATENATE LINES OF itab INTO lv SEPARATED BY ', '</code>.",
 link=("concatenating_strings", SITE+"14_Strings/concatenating_strings/index.html"),
 tags="rust strings gotcha"),

dict(id="str_reverse",
 front="Reverse a string. Write the line — and name the case where it is wrong.",
 back="<b><code>s.chars().rev().collect::&lt;String&gt;()</code></b>"
      "<br><br>Reversing <code>bytes()</code> would produce invalid UTF-8, so it has to go through <code>chars</code>."
      "<br><br><b>Still not always right:</b> it reverses <i>code points</i>, not what a reader calls characters. "
      "A flag emoji, an <code>e</code> + combining accent, or a family emoji will come apart. For real text use a "
      "grapheme-cluster crate — but this repo is bare-<code>rustc</code>, and for ASCII and most European text it is fine.",
 code='''fn main() {
    let s = "héllo";
    let r: String = s.chars().rev().collect();
    println!("{r}");
}''',
 expect="olléh",
 code_on="back",
 link=("meet_the_char", SITE+"14_Strings/meet_the_char/index.html"),
 tags="rust strings iterators unicode"),

dict(id="str_compare",
 front="Does <code>my_string == \"hello\"</code> compile, with <code>my_string: String</code>?",
 back="<b>Yes.</b> <code>String</code> implements <code>PartialEq&lt;&amp;str&gt;</code> in both directions, so the "
      "comparison you would write naturally just works."
      "<br><br>Comparison is <b>byte-for-byte</b>, so it is exact, fast — and <i>not</i> Unicode-aware: two strings that "
      "look identical on screen can differ if one uses a combining accent and the other a precomposed character.",
 code='''fn main() {
    let s = String::from("hello");
    println!("{}", s == "hello");
    println!("{}", "hello" == s);
    println!("{}", s.as_str() == "hello");

    let precomposed = "\\u{e9}";      // é as one code point
    let combining   = "e\\u{301}";    // e + combining acute
    println!("{precomposed} {combining} {} {} {}",
             precomposed == combining, precomposed.len(), combining.len());
}''',
 expect="true\ntrue\ntrue\n\u00e9 e\u0301 false 2 3",
 code_on="back",
 link=("comparing_strings", SITE+"14_Strings/comparing_strings/index.html"),
 tags="rust strings unicode gotcha"),

dict(id="str_struct_lifetime",
 front="Does this compile?",
 code='''struct User {
    name: &str,
}

fn main() {
    let u = User { name: "ada" };
    println!("{}", u.name);
}''',
 code_on="front",
 fails="E0106",
 back="<b>No — <code>E0106</code>: missing lifetime specifier.</b>"
      "<br><br>A struct holding a borrow must declare how long that borrow lives: <code>struct User&lt;'a&gt; { name: &amp;'a str }</code>. "
      "The struct may not outlive the text it points at, and the compiler needs that written down."
      "<br><br><b>The usual fix is not the lifetime — it is <code>String</code>.</b> Make the field owned unless you have "
      "measured a reason not to; borrowing in a struct is an optimisation, and it spreads <code>&lt;'a&gt;</code> through "
      "every type that touches it.",
 link=("string_vs_str", SITE+"14_Strings/string_vs_str/index.html"),
 tags="rust strings lifetimes compile-error"),

dict(id="str_bytes_roundtrip",
 front="You have a <code>Vec&lt;u8&gt;</code> from a file. How do you get a <code>String</code> — and what can go wrong?",
 back="<b><code>String::from_utf8(bytes)</code> → <code>Result</code>. It validates; it does not copy.</b>"
      "<br><br>Rust strings are UTF-8 <i>by type</i>, so arbitrary bytes have to be checked at the boundary. On failure "
      "the error hands the original <code>Vec</code> back, so nothing is lost."
      "<br><br><code>String::from_utf8_lossy(&amp;bytes)</code> never fails — bad bytes become <code>�</code>. It returns a "
      "<code>Cow</code>, so clean input costs no allocation at all.",
 code='''fn main() {
    let good = vec![104, 105];
    println!("{:?}", String::from_utf8(good));

    let bad = vec![104, 0xFF];
    println!("{}", String::from_utf8(bad.clone()).is_err());
    println!("{}", String::from_utf8_lossy(&bad));
}''',
 expect='Ok("hi")\ntrue\nh�',
 code_on="back",
 link=("string_from_utf8", SITE+"14_Strings/string_methods/string_from_utf8/index.html"),
 tags="rust strings result"),

dict(id="str_from_utf8_borrowed",
 front="You have a <code>&amp;[u8]</code> and want a <code>&amp;str</code>, with no allocation. Which function &mdash; and how does it differ from <code>String::from_utf8</code>?",
 code='''fn main() {
    let bytes: &[u8] = b"hello";
    let s: &str = std::str::from_utf8(bytes).unwrap();
    println!("{s} {}", s.len());
}''',
 expect="hello 5",
 code_on="back",
 back="<b><code>std::str::from_utf8(&amp;[u8]) -&gt; Result&lt;&amp;str, Utf8Error&gt;</code></b> &mdash; it validates the "
      "bytes where they lie and hands back a borrowed view. Nothing is copied and nothing is allocated."
      "<br><br><code>String::from_utf8(Vec&lt;u8&gt;)</code> is the owned twin: it takes the vector <i>by value</i> and "
      "reuses that exact allocation as the <code>String</code>'s buffer. Same UTF-8 check, different ownership &mdash; so "
      "the question is never which is faster, but which one you are holding: a borrow, or a <code>Vec</code>."
      "<br><br>This is the split the docs point at with <i>see also the <code>std::str</code> module</i>: the <b>type</b> "
      "<code>str</code> carries the methods you call on text you already have; the <b>module</b> <code>std::str</code> "
      "carries the free functions that make a <code>&amp;str</code> out of something that is not one yet.",
 bridge="<b>Python:</b> <code>bytes.decode()</code> always builds a new <code>str</code>. The borrowed form has no direct "
        "equivalent &mdash; <code>memoryview</code> is the nearest thing, and it exists for the same reason."
        "<br><b>ABAP:</b> converting an <code>xstring</code> to a <code>string</code> always produces a new value; there is "
        "no view-over-the-bytes form at all.",
 link=("string_from_utf8", SITE+"14_Strings/string_methods/string_from_utf8/index.html"),
 tags="rust strings result api-design"),

dict(id="str_lines",
 front="Split text into lines. Which method &mdash; and what does it do about <code>\\r\\n</code> and a trailing newline?",
 back="<b><code>s.lines()</code> &mdash; an iterator of <code>&amp;str</code>.</b>"
      "<br><br>It strips <code>\\n</code> <b>and</b> a preceding <code>\\r</code>, so Windows files need no special case. "
      "A trailing newline does <b>not</b> produce a final empty line &mdash; unlike <code>split('\\n')</code>, which does."
      "<br><br>Both are lazy iterators, so a huge file costs one line at a time.",
 code=r'''fn main() {
    let s = "a\r\nb\n";
    println!("{:?}", s.lines().collect::<Vec<_>>());
    println!("{:?}", s.split('\n').collect::<Vec<_>>());
}''',
 expect=r'''["a", "b"]
["a\r", "b", ""]''',
 code_on="back",
 bridge="<b>Python:</b> <code>s.splitlines()</code> behaves the same way; <code>s.split(&quot;&#92;n&quot;)</code> has the same "
        "trailing-empty trap.",
 link=("str_lines", SITE+"14_Strings/str_methods/str_lines/index.html"),
 tags="rust strings"),

dict(id="str_format",
 front="<code>format!</code>, <code>println!</code>, <code>write!</code> — what does each one do with the text?",
 back="<b><code>format!</code> returns a <code>String</code> · <code>println!</code> writes it to stdout · "
      "<code>write!</code> appends into a buffer you supply.</b>"
      "<br><br>Same formatting language for all three. Inline captures work in every one: <code>{name}</code> reads a "
      "variable in scope; <code>{}</code> takes the next positional argument."
      "<br><br>Useful specs: <code>{v:?}</code> debug · <code>{v:#?}</code> pretty debug · <code>{n:>8}</code> right-align · "
      "<code>{x:.2}</code> two decimals · <code>{n:04}</code> zero-pad · <code>{{</code> a literal brace.",
 code='''fn main() {
    let name = "ada";
    let x = 3.14159;
    let s = format!("{name} {x:.2} {:>5} {:04}", "hi", 7);
    println!("{s}");
    println!("{:?}", vec![1, 2]);
}''',
 expect="ada 3.14    hi 0007\n[1, 2]",
 code_on="back",
 bridge="<b>Python:</b> <code>f\"{name} {x:.2f}\"</code> — nearly the same mini-language, and <code>{x:.2}</code> in Rust "
        "already means fixed-point, so there is no <code>f</code>.",
 link=("the_format_language", SITE+"14_Strings/the_format_language/index.html"),
 tags="rust strings formatting"),

dict(id="str_starts_contains",
 front="Test that a string starts with, ends with, or contains something. What do the three methods take as an argument?",
 back="<b>All three take a <code>Pattern</code>: a <code>char</code>, a <code>&amp;str</code>, or a closure "
      "<code>|c: char| -&gt; bool</code>.</b>"
      "<br><br><code>starts_with</code> · <code>ends_with</code> · <code>contains</code> — all return <code>bool</code>. "
      "<code>find</code> returns the byte offset as <code>Option&lt;usize&gt;</code>."
      "<br><br><code>strip_prefix</code> / <code>strip_suffix</code> are the better pair when you were going to slice "
      "afterwards: they test <i>and</i> remove in one step, returning <code>Option&lt;&amp;str&gt;</code>.",
 code='''fn main() {
    let s = "report_2026.csv";
    println!("{}", s.starts_with("report"));
    println!("{}", s.ends_with(".csv"));
    println!("{}", s.contains(|c: char| c.is_ascii_digit()));
    println!("{:?}", s.strip_suffix(".csv"));
}''',
 expect='true\ntrue\ntrue\nSome("report_2026")',
 code_on="back",
 link=("searching_a_string", SITE+"14_Strings/searching_a_string/index.html"),
 tags="rust strings"),

dict(id="str_chars_nth",
 front="Get the 5th character of a string. Why is there no <code>s.char_at(5)</code>?",
 back="<b><code>s.chars().nth(5)</code> → <code>Option&lt;char&gt;</code> — and it walks the first five characters to get there.</b>"
      "<br><br>UTF-8 is variable-width, so character <i>n</i> has no computable address. Rust does not offer a method that "
      "would hide an O(n) scan behind indexing syntax."
      "<br><br>If you index characters repeatedly in a loop, that is the signal to convert once: "
      "<code>let cs: Vec&lt;char&gt; = s.chars().collect();</code> then <code>cs[5]</code> is O(1) — at the cost of 4 bytes "
      "per character.",
 code='''fn main() {
    let s = "héllo";
    println!("{:?}", s.chars().nth(1));
    println!("{:?}", s.chars().nth(99));
    let cs: Vec<char> = s.chars().collect();
    println!("{} {}", cs[1], cs.len());
}''',
 expect="Some('é')\nNone\né 5",
 code_on="back",
 link=("walking_a_string", SITE+"14_Strings/walking_a_string/index.html"),
 tags="rust strings unicode performance"),

dict(id="str_empty",
 front="Test whether a string is empty — and what is wrong with <code>s.len() == 0</code>?",
 back="<b><code>s.is_empty()</code>.</b> Nothing is <i>wrong</i> with <code>len() == 0</code> — it is correct and equally "
      "fast — but <code>is_empty</code> says the intent and clippy will tell you so."
      "<br><br>Careful: <code>\"   \".is_empty()</code> is <b>false</b>. For blank-not-just-empty use "
      "<code>s.trim().is_empty()</code>.",
 code='''fn main() {
    let a = String::new();
    let b = "   ";
    println!("{} {} {}", a.is_empty(), b.is_empty(), b.trim().is_empty());
}''',
 expect="true false true",
 code_on="back",
 bridge="<b>ABAP:</b> <code>IF lv_text IS INITIAL</code> is the same test — and has the same blank-vs-empty subtlety.",
 link=("string_is_empty", SITE+"14_Strings/string_methods/string_is_empty/index.html"),
 tags="rust strings"),

dict(id="str_repeat",
 front="Build a separator line of 40 dashes. Write it.",
 back="<b><code>\"-\".repeat(40)</code> → <code>String</code>.</b>"
      "<br><br>Also on the formatting side: <code>{:-&lt;40}</code> pads with a fill character, and "
      "<code>{:width$}</code> takes the width from a variable.",
 code='''fn main() {
    println!("{}", "-".repeat(10));
    println!("{}", "ab".repeat(3));
    let w = 10;
    println!("[{:>width$}]", "hi", width = w);
    println!("[{:-<10}]", "hi");
}''',
 expect="----------\nababab\n[        hi]\n[hi--------]",
 code_on="back",
 bridge="<b>Python:</b> <code>\"-\" * 40</code>. Rust has no <code>*</code> on strings — the method is the way.",
 link=("the_format_language", SITE+"14_Strings/the_format_language/index.html"),
 tags="rust strings formatting"),

dict(id="str_six_kinds",
 front="Beyond <code>String</code> and <code>&amp;str</code>, name the other string types you will actually meet — and what each is for.",
 back="<b><code>String</code> · <code>&amp;str</code> · <code>Box&lt;str&gt;</code> · <code>Cow&lt;str&gt;</code> · "
      "<code>OsString/&amp;OsStr</code> · <code>CString/&amp;CStr</code></b>"
      "<br><br>• <code>Box&lt;str&gt;</code> — owned, <b>not</b> growable; drops the capacity field, saving 8 bytes per value "
      "when you are storing millions and will never append"
      "<br>• <code>Cow&lt;str&gt;</code> — \"borrowed unless I had to change it\"; what <code>from_utf8_lossy</code> returns, "
      "so the clean path allocates nothing"
      "<br>• <code>OsStr</code> — paths and env vars, which are not guaranteed UTF-8 on any real OS"
      "<br>• <code>CStr</code> — NUL-terminated, for FFI"
      "<br><br>You will write <code>String</code> and <code>&amp;str</code> 95% of the time. Recognising the other four is "
      "enough.",
 link=("six_kinds_of_string", SITE+"14_Strings/six_kinds_of_string/index.html"),
 tags="rust strings types"),
]
