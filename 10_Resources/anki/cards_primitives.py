# Anki cards: Rust's primitive types and their literals.  Every `code` block is
# compiled with rustc 1.98.0 --edition 2024 and run; `expect` must match stdout
# exactly.  `fails` cards must NOT compile, with that error code.

SITE = "https://masiarek.github.io/rust-learning-library/"

DECK = "Rust::Primitives"

CARDS = [

# ---------------------------------------------------------------- literals --

dict(id="prim_base_prefixes",
 front="Write 42 in all four bases Rust accepts. What is the rule for remembering the three prefixes?",
 back="<b><code>0x2A</code> hex &middot; <code>0o52</code> octal &middot; <code>0b10_1010</code> binary &middot; <code>42</code> decimal.</b>"
      "<br><br>The rule: <b>the <code>0</code> means &lsquo;not decimal&rsquo;, and the letter after it is the first letter of "
      "the base&rsquo;s English name</b> &mdash; he<b>x</b>adecimal, <b>o</b>ctal, <b>b</b>inary."
      "<br><br>There is no fourth prefix and no per-type variation, so that is the whole rule. Digit case does not "
      "matter (<code>0xff</code> == <code>0xFF</code>); the prefix letter is always lowercase.",
 code='''fn main() {
    println!("{} {} {} {}", 0x2A, 0o52, 0b10_1010, 42);
}''',
 code_on="back",
 expect="42 42 42 42",
 bridge="<b>Python:</b> the same three prefixes, exactly &mdash; <code>0x2A</code>, <code>0o52</code>, <code>0b101010</code>. "
        "This one transfers with no adjustment at all.",
 link=("writing_a_number_down", SITE+"19_Numbers/writing_a_number_down/index.html"),
 tags="rust primitives literals"),

dict(id="prim_leading_zero",
 front="What does this print? (Think before you answer &mdash; the C habit is a trap here.)",
 code='''fn main() {
    println!("{} {}", 0755, 0o755);
}''',
 code_on="front",
 expect="755 493",
 back="<b>A bare leading zero means nothing in Rust.</b> <code>0755</code> is seven hundred fifty-five, decimal. "
      "Only <code>0o</code> asks for base 8."
      "<br><br>In C, <code>0755</code> <i>is</i> octal &mdash; the bare leading zero is the prefix &mdash; which has "
      "produced a long tail of bugs where a zero-padded decimal quietly became a different number. Rust never "
      "adopted the form, so leading zeros are inert padding.",
 bridge="<b>Python:</b> made the same call, and its error message names the fix: "
        "<code>SyntaxError: leading zeros in decimal integer literals are not permitted; use an 0o prefix for octal "
        "integers</code>. Python 2 <i>did</i> read <code>0755</code> as octal; Python 3 removed it.",
 link=("writing_a_number_down", SITE+"19_Numbers/writing_a_number_down/index.html"),
 tags="rust primitives literals gotcha"),

dict(id="prim_suffix",
 front="Three ways to say &ldquo;this 57 is a <code>u8</code>&rdquo;. Write them.",
 back="<b><code>57u8</code> &middot; <code>57_u8</code> &middot; <code>let c: u8 = 57;</code></b>"
      "<br><br>All three are one instruction to the compiler, said in three places. The underscore before the suffix "
      "is optional and purely cosmetic."
      "<br><br>Reach for the <b>suffix</b> when the value goes straight to something (<code>vec![0u8; 4]</code>, "
      "<code>x as f32 * 2.0_f32</code>) and the <b>annotation</b> when the value is being named.",
 code='''fn main() {
    let a = 57u8;
    let b = 57_u8;
    let c: u8 = 57;
    println!("{}", a == b && b == c);
}''',
 code_on="back",
 expect="true",
 bridge="<b>Python:</b> no suffix exists, because there is one <code>int</code>. Choosing a width is a decision Rust "
        "asks you to make and Python never does.",
 link=("writing_a_number_down", SITE+"19_Numbers/writing_a_number_down/index.html"),
 tags="rust primitives literals"),

dict(id="prim_underscore",
 front="What is the underscore doing in <code>1_000_000</code>, and where may it go?",
 back="<b>Nothing, and anywhere.</b> It is a digit separator for the reader; the compiler ignores it entirely, "
      "in every base and any number of times."
      "<br><br>Group at the boundary that matters: thousands in a decimal quantity, <b>bytes</b> in hex "
      "(<code>0xDEAD_BEEF</code>), <b>nibbles</b> in binary (<code>0b1011_1110</code>). "
      "<code>1_0_0</code> is legal, and nobody should.",
 code='''fn main() {
    println!("{} {} {}", 1_000_000, 0xDEAD_BEEFu32, 1_0_0);
}''',
 code_on="back",
 expect="1000000 3735928559 100",
 bridge="<b>Python:</b> <code>1_000_000</code>, identical, since 3.6.",
 link=("writing_a_number_down", SITE+"19_Numbers/writing_a_number_down/index.html"),
 tags="rust primitives literals"),

dict(id="prim_byte_literal_vs_python",
 front="What does <code>b'A'</code> mean in Rust &mdash; and is it the same as Python&rsquo;s <code>b'A'</code>?",
 code='''fn main() {
    let byte = b'A';
    let bytes = b"Hi";
    println!("{} {:?} {}", byte, bytes, b'A' + 2);
}''',
 code_on="front",
 expect="65 [72, 105] 67",
 back="<b>In Rust <code>b'A'</code> IS the number 65 &mdash; a <code>u8</code>, no container.</b> You can do arithmetic "
      "on it directly. <code>b\"Hi\"</code> is <code>&amp;[u8; 2]</code>."
      "<br><br>The character between the quotes must be one ASCII byte: <code>b'&eacute;'</code> does not compile, "
      "because &eacute; is two bytes in UTF-8.",
 bridge="<b>Python &mdash; same spelling, different thing.</b> Python&rsquo;s <code>b'A'</code> is a <code>bytes</code> "
        "object of <b>length 1</b> &mdash; a container &mdash; so you need <code>b'A'[0]</code> to get 65."
        "<br>&bull; Rust <code>b'A'</code> &harr; Python <code>b'A'[0]</code> or <code>ord('A')</code>"
        "<br>&bull; Rust <code>b\"Hi\"</code> &harr; Python <code>b'Hi'</code>"
        "<br>The familiar syntax maps to the <b>plural</b> form, which is the direction that produces a confusing "
        "type error rather than a wrong number.",
 link=("writing_a_number_down", SITE+"19_Numbers/writing_a_number_down/index.html"),
 tags="rust primitives literals python-trap"),

dict(id="prim_u8_is_the_byte",
 front="&ldquo;<code>u8</code> is the only integer type suitable for a byte.&rdquo; True? Say what is actually going on.",
 back="<b>True narrowly, misleading broadly.</b> Nothing stops you storing 65 in an <code>i32</code>."
      "<br><br>What is true: <b><code>u8</code> is the only type whose range is <i>exactly</i> a byte&rsquo;s</b> &mdash; "
      "<code>0..=255</code>, no value spare and none missing. So it is what the language and std use wherever the "
      "thing genuinely <i>is</i> a byte:"
      "<br>&bull; <code>b'A'</code> produces one<br>&bull; <code>b\"Hi\"</code> is an array of them"
      "<br>&bull; <code>String::as_bytes()</code> returns <code>&amp;[u8]</code>"
      "<br>&bull; <code>File::read</code> fills a <code>&amp;mut [u8]</code>"
      "<br><br>Picking <code>i32</code> there is picking a type that can hold <code>-1</code> and <code>300</code>, "
      "neither of which any byte is.",
 link=("meet_the_byte", SITE+"19_Numbers/meet_the_byte/index.html"),
 tags="rust primitives integers"),

# ------------------------------------------------------------------ widths --

dict(id="prim_signed_range",
 front="What range can a signed <code>iN</code> hold, as a formula in N? And <code>uN</code>?",
 back="<b>signed <code>iN</code>: &minus;2<sup>N&minus;1</sup> ..= 2<sup>N&minus;1</sup> &minus; 1</b>"
      "<br><b>unsigned <code>uN</code>: 0 ..= 2<sup>N</sup> &minus; 1</b>"
      "<br><br>So <code>i8</code> is &minus;128..=127 and <code>u8</code> is 0..=255."
      "<br><br>The shape comes from two&rsquo;s complement: the top bit of a signed integer carries the sign, which "
      "halves the magnitude and shifts the range down by one &mdash; so there is exactly <b>one more value below "
      "zero than above it</b>.",
 code='''fn main() {
    println!("{} {} {} {}", i8::MIN, i8::MAX, u8::MIN, u8::MAX);
}''',
 code_on="back",
 expect="-128 127 0 255",
 bridge="<b>Python:</b> no formula to learn &mdash; <code>int</code> is arbitrary precision, so <code>2 ** 200</code> is "
        "exact. Every width here is a constraint Python does not have.<br><b>ABAP:</b> <code>b</code>/<code>s</code>/"
        "<code>i</code>/<code>int8</code> are 1/2/4/8 bytes, all <b>signed</b> &mdash; ABAP has no unsigned integer at all.",
 link=("writing_a_number_down", SITE+"19_Numbers/writing_a_number_down/index.html"),
 tags="rust primitives integers"),

dict(id="prim_min_has_no_abs",
 front="What does this print? (The answer is the sharpest consequence of two&rsquo;s complement.)",
 code='''fn main() {
    println!("{:?} {:?}", i8::MIN.checked_neg(), i8::MIN.checked_abs());
}''',
 code_on="front",
 expect="None None",
 back="<b><code>-128</code> has no positive counterpart in <code>i8</code>.</b> The signed range is asymmetric &mdash; "
      "one more value below zero than above &mdash; so negating or taking the absolute value of <code>MIN</code> "
      "overflows."
      "<br><br>Which means <b><code>i8::MIN.abs()</code> panics in debug and returns <code>-128</code> in release</b> "
      "&mdash; an absolute value that is negative. That is why <code>checked_abs</code> exists."
      "<br><br>Unsigned types have no such hole: 0 is its own negation and there is nothing below it.",
 link=("writing_a_number_down", SITE+"19_Numbers/writing_a_number_down/index.html"),
 tags="rust primitives integers gotcha"),

dict(id="prim_overflow_split",
 front="<code>255u8 + 1</code>. What happens &mdash; and what is the <i>bad</i> part of the answer?",
 back="<b>A debug build panics (<code>attempt to add with overflow</code>); a release build wraps to 0.</b> "
      "Both are documented and deliberate, neither is undefined behaviour."
      "<br><br>The bad part: <b>the two builds do not agree</b>, so an overflow bug can pass every test you run and "
      "still wrap in production."
      "<br><br>The fix is to stop using bare <code>+</code> where the width is in question and say which you meant:"
      "<br>&bull; <code>wrapping_add(1)</code> &rarr; 0 &mdash; modular, on purpose"
      "<br>&bull; <code>checked_add(1)</code> &rarr; <code>None</code> &mdash; the Option makes you handle it"
      "<br>&bull; <code>saturating_add(1)</code> &rarr; 255 &mdash; clamp at the ceiling"
      "<br>&bull; <code>overflowing_add(1)</code> &rarr; <code>(0, true)</code> &mdash; the value and whether it wrapped",
 code='''fn main() {
    let m = u8::MAX;
    println!("{} {:?} {} {:?}",
        m.wrapping_add(1), m.checked_add(1), m.saturating_add(1), m.overflowing_add(1));
}''',
 code_on="back",
 expect="0 None 255 (0, true)",
 bridge="<b>Python:</b> cannot happen on an <code>int</code>, but <i>does</i> happen the moment you opt into a fixed "
        "width elsewhere &mdash; <code>numpy.uint8(255) + 1</code> is 0, as are <code>struct</code>, "
        "<code>array</code>, <code>ctypes</code> and every integer column in your database."
        "<br><b>ABAP:</b> raises <code>CX_SY_ARITHMETIC_OVERFLOW</code>. Three languages, three different choices.",
 link=("meet_the_byte", SITE+"19_Numbers/meet_the_byte/index.html"),
 tags="rust primitives integers gotcha"),

dict(id="prim_literal_out_of_range",
 front="<code>let n = 100_000u16;</code> &mdash; does that compile? Quote what rustc says.",
 back="<b>No. The LITERAL is rejected, before any arithmetic happens:</b>"
      "<br><br><code>error: literal out of range for `u16`</code>"
      "<br><code>= note: the literal `100_000u16` does not fit into the type `u16` whose range is `0..=65535`</code>"
      "<br><code>= note: `#[deny(overflowing_literals)]` on by default</code>"
      "<br><br>Note there is no <code>E0000</code> number: this is a <b>deny-by-default lint</b>, not a type error, "
      "which is why the message reads differently from the ones you are used to. The note prints the range, so the "
      "compiler tells you the answer to &lsquo;what does <code>u16</code> hold&rsquo; at the moment you need it."
      "<br><br>Contrast with arithmetic: <code>65535u16 + 1</code> compiles fine and is a <i>run-time</i> question "
      "&mdash; panic in debug, wrap in release.",
 link=("writing_a_number_down", SITE+"19_Numbers/writing_a_number_down/index.html"),
 tags="rust primitives integers compile-error"),

dict(id="prim_usize_when",
 front="When do you reach for <code>usize</code> rather than <code>u32</code>? Give the rule and an example.",
 code='''fn main() {
    let names = ["Ada", "Ben", "Cara"];
    let n: usize = names.len();
    let i: usize = 1;
    println!("{} of {n}", names[i]);
}''',
 code_on="front",
 expect="Ben of 3",
 back="<b>A <i>quantity</i> is a <code>u32</code>; a <i>position</i> in a collection is a <code>usize</code>.</b>"
      "<br><br><code>usize</code> is the machine&rsquo;s pointer width &mdash; 8 bytes on a 64-bit target, 4 on a "
      "32-bit one &mdash; and its job is <b>positions in memory</b>. Every length, index, capacity and byte count "
      "in the standard library is one, which is why <code>.len()</code> hands you a <code>usize</code> and why "
      "indexing demands one."
      "<br><br>So a vote total is <code>u32</code>; the position of a candidate in a slice is <code>usize</code>. "
      "That is also where <code>scores[i as usize]</code> comes from, when <code>i</code> arrived as a <code>u32</code>."
      "<br><br><code>isize</code> is rare: pointer offsets and FFI.",
 bridge="<b>Python:</b> not exposed &mdash; you index with any <code>int</code>, and a negative one counts from the "
        "end. Rust has no negative indexing at all, which is half the reason the index type is unsigned.",
 link=("values", SITE+"15_First_Programs/values/index.html"),
 tags="rust primitives integers"),

# ------------------------------------------------------------------ floats --

dict(id="prim_no_unsigned_float",
 front="How do you write an <b>unsigned</b> floating-point literal in Rust?",
 back="<b>You cannot. There is no <code>uf32</code> or <code>uf64</code>, and there cannot be.</b>"
      "<br><br>IEEE 754 puts a sign bit at the top of every float, so signedness is part of the <i>format</i>, not a "
      "choice the type makes. The bit is there even at zero, which is why floats have two zeros that compare equal:"
      "<br><br>If you need &ldquo;a float that cannot be negative&rdquo;, that is a newtype with a checked "
      "constructor, not a primitive.",
 code='''fn main() {
    println!("{} {}", -0.0 == 0.0, (-0.0f64).is_sign_negative());
}''',
 code_on="back",
 expect="true true",
 link=("writing_a_number_down", SITE+"19_Numbers/writing_a_number_down/index.html"),
 tags="rust primitives floats"),

dict(id="prim_float_default_precision",
 front="What is the default float type, why, and what does &ldquo;double precision&rdquo; actually count?",
 code='''fn main() {
    println!("{} {} {} {}",
        f32::MANTISSA_DIGITS, f32::DIGITS, f64::MANTISSA_DIGITS, f64::DIGITS);
}''',
 code_on="front",
 expect="24 6 53 15",
 back="<b><code>f64</code> is the default</b> &mdash; roughly as fast as <code>f32</code> on any machine you will "
      "meet, and it remembers more than twice as much."
      "<br><br><b>Precision counts significant BITS, not decimal places:</b>"
      "<br>&bull; <code>f32</code> = IEEE 754 binary32 = <b>single</b> precision: 24 significand bits, ~6 reliable "
      "decimal digits"
      "<br>&bull; <code>f64</code> = IEEE 754 binary64 = <b>double</b> precision: 53 significand bits, ~15 digits"
      "<br><br>&ldquo;Double&rdquo; is a historical name, from when the narrow one was the machine&rsquo;s native "
      "float and the wide one took two registers."
      "<br><br>More bits is not <i>exact</i>: <code>0.1</code> is representable in neither. <code>f64</code> gets the "
      "same wrong answer 29 bits further right.",
 bridge="<b>Python:</b> <code>float</code> is always a C double &mdash; i.e. always <code>f64</code>. There is no "
        "single-precision float in the language, so <code>f32</code> is a choice Rust offers that Python does not.",
 link=("what_a_float_stores", SITE+"19_Numbers/what_a_float_stores/index.html"),
 tags="rust primitives floats"),

dict(id="prim_float_dot",
 front="Does this compile?<br><br>If not, what did rustc think you wrote?",
 code='''fn main() {
    let f = 2.f32;
    println!("{f}");
}''',
 code_on="front",
 fails="E0610",
 back="<b>No &mdash; <code>E0610</code>. rustc parsed <code>2.f32</code> as <i>field <code>f32</code> of the number "
      "2</i>,</b> and primitives have no fields."
      "<br><br>Its help line gives the fix: <i>&ldquo;if intended to be a floating point literal, consider adding a "
      "<code>0</code> after the period&rdquo;</i>."
      "<br><br>Write <code>2.0f32</code> or <code>2_f32</code>. Note what <i>does</i> work:"
      "<br>&bull; <code>2.0</code> &rarr; <code>f64</code>"
      "<br>&bull; <code>2.</code> &rarr; <code>f64</code> (legal! plain <code>2</code> would be an integer)"
      "<br>&bull; <code>1e6</code> &rarr; <code>f64</code>"
      "<br>&bull; <code>2_f32</code> &rarr; no dot needed once the suffix says float",
 link=("writing_a_number_down", SITE+"19_Numbers/writing_a_number_down/index.html"),
 tags="rust primitives floats compile-error"),

# ------------------------------------------------------------------- bool ---

dict(id="prim_bool_size",
 front="How big is a <code>bool</code>, and how big is <code>Option&lt;bool&gt;</code>?",
 code='''fn main() {
    println!("{} {} {}",
        size_of::<bool>(), size_of::<Option<bool>>(), size_of::<[bool; 8]>());
}''',
 code_on="front",
 expect="1 1 8",
 back="<b>One byte &mdash; one bit of information in one byte of space</b>, because a byte is the smallest thing a "
      "machine can address. Eight bools in an array are eight BYTES."
      "<br><br><code>Option&lt;bool&gt;</code> is <b>also</b> one byte: a <code>bool</code> uses two of its 256 bit "
      "patterns, so <code>None</code> is stored <i>in</i> the value rather than beside it. That is the niche "
      "optimization, and <code>bool</code> is its clearest case."
      "<br><br>Packing eight bools into one byte is bit flags, and it is deliberate work you opt into.",
 link=("meet_the_bool", SITE+"15_First_Programs/meet_the_bool/index.html"),
 tags="rust primitives bool"),

dict(id="prim_no_truthiness",
 front="Does this compile?<br><br>Name the rule, and where else it applies.",
 code='''fn main() {
    let n = 1;
    if n {
        println!("yes");
    }
}''',
 code_on="front",
 fails="E0308",
 back="<b>No &mdash; <code>E0308</code>: expected <code>bool</code>, found integer. There is no truthiness anywhere "
      "in Rust.</b>"
      "<br><br>Not just <code>if</code>: the same rule holds in <code>while</code>, <code>match</code> guards, "
      "<code>filter</code> and <code>assert!</code>, because nothing in the language converts a value to a truth."
      "<br><br>A <code>Vec</code> is not false when empty; an <code>Option</code> is not false when <code>None</code>; "
      "<code>0.0</code> is not false. Each has its own question &mdash; <code>is_empty()</code>, "
      "<code>is_none()</code>, <code>!= 0.0</code> &mdash; and you pick the one you meant.",
 bridge="<b>Python:</b> truthiness everywhere &mdash; <code>if []</code>, <code>if \"\"</code>, <code>if 0</code>, "
        "<code>if None</code> are all false, and <code>__bool__</code> lets any class join in. The habit that "
        "transfers badly is <code>if my_list:</code> &rarr; write <code>if !v.is_empty()</code>."
        "<br><b>ABAP:</b> no boolean type at all &mdash; <code>abap_bool</code> is a char with "
        "<code>abap_true = 'X'</code>, so <code>IF lv_flag = abap_true</code> is a char comparison. Rust&rsquo;s "
        "<code>if flag</code> is the condition itself.",
 link=("meet_the_bool", SITE+"15_First_Programs/meet_the_bool/index.html"),
 tags="rust primitives bool python-trap"),

dict(id="prim_bool_not_a_number",
 front="Python counts <code>True</code>s with <code>sum(flags)</code>. What is the Rust line &mdash; and why is it not a sum?",
 code='''fn main() {
    let flags = [true, false, true, true, false];
    println!("{}", flags.iter().filter(|&&b| b).count());
}''',
 code_on="front",
 expect="3",
 back="<b><code>bool</code> is not a number, so counting is a <i>count</i>, not a sum.</b> "
      "<code>true + 1</code> is <code>error[E0369]: cannot add {integer} to bool</code>."
      "<br><br><code>flags.iter().map(|&amp;b| b as u32).sum()</code> gives the same 3, but the reader has to work "
      "out from a cast that counting was the point. Prefer <code>filter(..).count()</code>."
      "<br><br>Both directions need saying out loud:"
      "<br>&bull; <code>bool</code> &rarr; number: <code>b as u8</code> or <code>u8::from(b)</code> &mdash; "
      "<code>true</code> is 1, guaranteed"
      "<br>&bull; number &rarr; <code>bool</code>: <code>n != 0</code>. <code>1u8 as bool</code> is "
      "<code>E0054</code>, and rustc&rsquo;s help says <i>&ldquo;compare with zero instead&rdquo;</i>",
 bridge="<b>Python:</b> <code>bool</code> is a <b>subclass of <code>int</code></b>, so <code>True + 1 == 2</code> and "
        "<code>[False, True][flag]</code> indexes a list with a boolean. None of that has a Rust translation.",
 link=("meet_the_bool", SITE+"15_First_Programs/meet_the_bool/index.html"),
 tags="rust primitives bool python-trap"),

dict(id="prim_amp_vs_ampamp",
 front="<code>&amp;&amp;</code> and <code>&amp;</code> both compile on bools and return the same value. What is the difference, and when does it bite?",
 back="<b><code>&amp;&amp;</code> short-circuits; <code>&amp;</code> always evaluates both sides.</b> Nothing warns you, "
      "because both type-check as <code>bool &rarr; bool &rarr; bool</code>."
      "<br><br>It bites hardest on a <b>guard</b>, where the left half exists to make the right half safe to ask:"
      "<br><br><code>(i &lt; v.len()) &amp;&amp; (v[i] &gt; 0)</code> &mdash; safe, returns false"
      "<br><code>(i &lt; v.len()) &amp; (v[i] &gt; 0)</code> &mdash; <b>panics</b>, index out of bounds"
      "<br><br>Same shape with a side effect instead of a panic: <code>cache_ok &amp; fetch_from_network()</code> does "
      "the fetch every time. Use <code>&amp;</code>/<code>|</code> on bools only when you want both sides evaluated "
      "on purpose &mdash; rare enough to deserve a comment.",
 code='''fn main() {
    let v = vec![5u8, 3, 0];
    let i = 7;
    println!("{}", (i < v.len()) && (v[i] > 0));
}''',
 code_on="back",
 expect="false",
 bridge="<b>Python:</b> the same pair &mdash; <code>and</code>/<code>or</code> short-circuit, <code>&amp;</code>/"
        "<code>|</code> on bools do not. Same trap, though Rust gives you more ways to feel it, since the right "
        "side may panic rather than merely be slow.",
 link=("meet_the_bool", SITE+"15_First_Programs/meet_the_bool/index.html"),
 tags="rust primitives bool gotcha"),

# ------------------------------------------------------------------- char ---

dict(id="prim_char_four_bytes",
 front="How many bytes is a <code>char</code>, and <b>why that number</b>?",
 code='''fn main() {
    println!("{} {:?} {}", size_of::<char>(), char::MAX, char::MAX as u32);
}''',
 code_on="front",
 expect="4 '\\u{10ffff}' 1114111",
 back="<b>Four. Because a <code>char</code> is one Unicode scalar value, and the largest is U+10FFFF &mdash; which "
      "needs 21 bits.</b>"
      "<br><br>21 bits does not fit in 8 or 16, so the next byte-sized width is 32."
      "<br><br>It is fixed-width <i>because</i> it is <b>decoded</b>: you can compare it, classify it, and range over "
      "it (<code>'0'..='9'</code>). Inside a <code>String</code> the same character is <b>1 to 4 UTF-8 bytes</b>, "
      "because encoded bytes want to be small."
      "<br><br>So <code>'A'</code> costs 4 bytes as a value and 1 byte inside a string.",
 bridge="<b>C:</b> <code>char</code> <i>is</i> the byte &mdash; the same word for a different object, which is the "
        "single most common confusion coming from C."
        "<br><b>Python:</b> no character type at all; a 1-length <code>str</code>.",
 link=("meet_the_char", SITE+"14_Strings/meet_the_char/index.html"),
 tags="rust primitives char unicode"),

dict(id="prim_quotes",
 front="Rust has four literal forms that differ only in quotes and a prefix. Name all four and their types.",
 back="<b><code>'A'</code> &rarr; <code>char</code></b> (4 bytes, one Unicode scalar value)"
      "<br><b><code>\"A\"</code> &rarr; <code>&amp;str</code></b> (UTF-8 bytes)"
      "<br><b><code>b'A'</code> &rarr; <code>u8</code></b> (the number 65)"
      "<br><b><code>b\"A\"</code> &rarr; <code>&amp;[u8; 1]</code></b>"
      "<br><br>Single quotes are a character, double quotes are a string, and a leading <code>b</code> means bytes "
      "rather than text. Two of the four print the same and are not the same type.",
 code='''fn main() {
    let c: char = 'A';
    let s: &str = "A";
    let b: u8 = b'A';
    let bs: &[u8; 1] = b"A";
    println!("{} {} {} {:?}", c, s, b, bs);
}''',
 code_on="back",
 expect="A A 65 [65]",
 bridge="<b>Python:</b> <code>'A'</code> and <code>\"A\"</code> are the <i>same thing</i> &mdash; quote style is pure "
        "preference. In Rust they are different types, which is the first thing to unlearn.",
 link=("meet_the_char", SITE+"14_Strings/meet_the_char/index.html"),
 tags="rust primitives char literals python-trap"),

# ---------------------------------------------------------------- compound --

dict(id="prim_tuple_type",
 front="Does this compile?<br><br>What is the rule about a tuple&rsquo;s type?",
 code='''fn main() {
    let t: (u8, char, bool) = (5, 'A', true);
    let (a, b) = t;
    println!("{a} {b}");
}''',
 code_on="front",
 fails="E0308",
 back="<b>No. A tuple&rsquo;s <i>length and element types ARE its type</i></b>, so a three-element tuple cannot be "
      "destructured into two names."
      "<br><br><code>(u8, char, bool)</code> and <code>(u8, bool, char)</code> are different types, and neither is "
      "<code>(u8, char)</code>."
      "<br><br>Getting values out, two ways:"
      "<br>&bull; destructure: <code>let (a, b, c) = t;</code>"
      "<br>&bull; index by position: <code>t.0</code>, <code>t.1</code>, <code>t.2</code>",
 bridge="<b>Python:</b> <code>a, b = (1, 2, 3)</code> raises <code>ValueError: too many values to unpack</code> "
        "&mdash; <b>at run time</b>. Rust moves the same error to compile time. Also missing in Rust: "
        "<code>*rest</code> unpacking and <code>t + (1,)</code>. What <i>does</i> transfer exactly: the comma rule, "
        "<code>(7)</code> is 7 and <code>(7,)</code> is a one-tuple.",
 link=("tuples", SITE+"26_Collections/tuples/index.html"),
 tags="rust primitives tuples compile-error python-trap"),

dict(id="prim_unit_type",
 front="What is <code>()</code> called, how many values does it have, and how big is it?",
 code='''fn main() {
    println!("{:?} {} {}", (), size_of::<()>(), size_of::<[(); 1000]>());
}''',
 code_on="front",
 expect="() 0 0",
 back="<b>The <i>unit type</i> &mdash; the empty tuple. Exactly ONE value, and ZERO bytes.</b>"
      "<br><br>A type with one value carries no information, so there is nothing to store; a thousand of them still "
      "occupy nothing. It is both the type and its single value, written the same way."
      "<br><br>Counting values orders the primitives: <code>u8</code> has 256, <code>bool</code> has 2, "
      "<code>()</code> has 1, and <code>!</code> (never) has 0."
      "<br><br>Where it comes from: <b>a function with no <code>-&gt;</code></b> returns it, and <b>a semicolon</b> "
      "produces it &mdash; <code>{ 7; }</code> is <code>()</code> while <code>{ 7 }</code> is 7. That one rule is "
      "behind every <i>&ldquo;expected <code>i32</code>, found <code>()</code>&rdquo;</i>.",
 bridge="<b>Python:</b> the nearest thing is <code>None</code>, and the resemblance misleads. Python&rsquo;s "
        "<code>None</code> does two jobs &mdash; &ldquo;no return value&rdquo; <i>and</i> &ldquo;missing&rdquo;. "
        "Rust splits them: <code>()</code> is &ldquo;done, nothing to report&rdquo;, <code>Option::None</code> is "
        "&ldquo;there is no value here&rdquo;.",
 link=("the_unit_type", SITE+"15_First_Programs/the_unit_type/index.html"),
 tags="rust primitives unit tuples"),

dict(id="prim_sort_returns_unit",
 front="What does this print? (Then say what the second value is, and why.)",
 code='''fn main() {
    let mut v = vec![3u8, 1, 5];
    let x = v.sort();
    println!("{v:?} {x:?}");
}''',
 code_on="front",
 expect="[1, 3, 5] ()",
 back="<b><code>x</code> is <code>()</code> &mdash; you took the receipt instead of the result.</b> Every in-place "
      "method returns unit, because the answer was written back into the receiver."
      "<br><br>Nothing complains until you use <code>x</code>, and then the message names the type outright: "
      "<code>error[E0599]: no method named 'len' found for unit type '()'</code>."
      "<br><br><code>push</code>, <code>dedup</code>, <code>retain</code>, <code>clear</code> and "
      "<code>sort_unstable</code> all do this &mdash; which is also why none of them chains: "
      "<code>v.push(9).dedup()</code> does not compile. <b>Mutate, then use</b>; or clone first and sort the copy.",
 bridge="<b>Python:</b> the identical bug &mdash; <code>x = lst.sort()</code> gives <code>None</code>, and you find "
        "out at run time with <code>AttributeError: 'NoneType' object has no attribute ...</code>. Both languages "
        "made <code>sort</code> return nothing <i>deliberately</i>, so a mutation cannot be mistaken for a copy.",
 link=("the_unit_type", SITE+"15_First_Programs/the_unit_type/index.html"),
 tags="rust primitives unit gotcha python-trap"),

dict(id="prim_ok_unit",
 front="What does <code>Result&lt;(), String&gt;</code> describe, and how do you write its success value?",
 code='''fn main() {
    fn check(score: u8) -> Result<(), String> {
        if score <= 5 { Ok(()) } else { Err(format!("{score} out of range")) }
    }
    println!("{:?} {:?}", check(5), check(9));
}''',
 code_on="front",
 expect='Ok(()) Err("9 out of range")',
 back="<b>A job that either works with no payload, or fails with a reason. The success value is <code>Ok(())</code>.</b>"
      "<br><br>It reads oddly the first time and says something precise: <i>it worked, and there is nothing to hand "
      "back</i>. A validator, a <code>write!</code>, a <code>File::set_len</code> &mdash; all this shape."
      "<br><br><code>?</code> works as it always does; on success it unwraps a value that carries nothing, so nothing "
      "is lost.",
 bridge="<b>ABAP:</b> this is <code>sy-subrc</code> with the discipline added. Same idea &mdash; success carries no "
        "payload, failure carries a reason &mdash; but <code>sy-subrc</code> can be ignored by not looking at it, "
        "while <code>Result</code> is <code>#[must_use]</code> and the compiler warns when you drop it.",
 link=("the_unit_type", SITE+"15_First_Programs/the_unit_type/index.html"),
 tags="rust primitives unit result"),

dict(id="prim_array_init",
 front="Two ways to initialise an array. Write both, and say what is in the type.",
 code='''fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    println!("{a:?} {b:?}");
}''',
 code_on="front",
 expect="[1, 2, 3, 4, 5] [3, 3, 3, 3, 3]",
 back="<b>Element by element, <code>[1, 2, 3, 4, 5]</code>; or all at once, <code>[3; 5]</code> &mdash; the value, a "
      "semicolon, the count.</b>"
      "<br><br><b>The length is part of the type.</b> <code>[i32; 5]</code> and <code>[i32; 3]</code> are as "
      "different as <code>u32</code> and <code>String</code>, so a function taking one will not accept the other."
      "<br><br>Arrays are fixed length, always. The elements sit inline &mdash; <code>size_of::&lt;[u32; 5]&gt;()</code> "
      "is 20, five values and no header.",
 bridge="<b>Python:</b> <code>[3] * 5</code> is the counterpart of <code>[3; 5]</code>, and the famous Python trap "
        "&mdash; <code>[[0] * 3] * 2</code> aliases the inner list &mdash; has no Rust equivalent here, because "
        "<code>[[0; 3]; 2]</code> copies.",
 link=("array_or_vec", SITE+"26_Collections/array_or_vec/index.html"),
 tags="rust primitives arrays"),

dict(id="prim_array_len_is_const",
 front="Does this compile?<br><br>State the rule in one sentence.",
 code='''fn main() {
    let ballots = ["a", "b", "c"];
    let n = ballots.len();
    let counts = [0u32; n];
    println!("{counts:?}");
}''',
 code_on="front",
 fails="E0435",
 back="<b>No &mdash; <code>E0435</code>: attempt to use a non-constant value in a constant. An array length is a "
      "CONSTANT, full stop.</b>"
      "<br><br><code>n</code> came from a value, so <code>vec![0u32; n]</code> is not a fallback &mdash; it is the "
      "only thing that expresses what you meant."
      "<br><br>This is the real reason the default is <code>Vec</code>: almost every length in a real program comes "
      "from input, and input length is not knowable when you compile.",
 link=("array_or_vec", SITE+"26_Collections/array_or_vec/index.html"),
 tags="rust primitives arrays compile-error"),

dict(id="prim_array_or_vec",
 front="Array or <code>Vec</code>? Give the default and the four things an array actually buys.",
 back="<b>Default to <code>Vec</code></b> &mdash; because almost every length comes from input, and input length is "
      "not knowable at compile time."
      "<br><br><b>An array wins when the length is a fact about the PROBLEM:</b>"
      "<br>1. <b>The wrong length is a compile error.</b> A fn taking <code>[u8; 32]</code> cannot be handed 31 "
      "bytes; the <code>Vec</code> version compiles and produces a wrong answer."
      "<br>2. <b>An array of <code>Copy</code> is <code>Copy</code>.</b> <code>let snap = arr;</code> is a real copy, "
      "no <code>.clone()</code>, and the original stays usable."
      "<br>3. <b>It works in a <code>const</code>.</b> <code>const W: [u32; 6] = [..]</code>; a <code>Vec</code> "
      "cannot, since there is no allocator at compile time."
      "<br>4. <b>No allocator at all</b> &mdash; which is why <code>no_std</code> and embedded code use arrays."
      "<br><br>And the question mostly dissolves: take <code>&amp;[T]</code> in the signature and both callers work, "
      "so this is a <i>storage</i> decision, not an API one.",
 code='''fn main() {
    fn total(counts: &[u32]) -> u32 { counts.iter().sum() }
    let fixed = [4u32, 0, 2];
    let grown = vec![4u32, 0, 2];
    println!("{} {}", total(&fixed), total(&grown));
}''',
 code_on="back",
 expect="6 6",
 bridge="<b>Python:</b> nothing to choose &mdash; <code>list</code> is <code>Vec</code>, and there is no array. The "
        "habit that transfers badly is reaching for the list every time, which in Rust means a heap allocation for "
        "a three-element lookup table that could have been a <code>const</code>."
        "<br><b>ABAP:</b> <code>Vec&lt;T&gt;</code> is a <code>STANDARD TABLE OF</code>; an array is closer to a "
        "fixed-length structure. But <code>SORTED</code>/<code>HASHED</code> tables are <code>BTreeMap</code>/"
        "<code>HashMap</code> in Rust, not flavours of <code>Vec</code>.",
 link=("array_or_vec", SITE+"26_Collections/array_or_vec/index.html"),
 tags="rust primitives arrays vec"),

# ------------------------------------------------------- expressions & misc --

dict(id="prim_block_is_expression",
 front="What are <code>x</code> and <code>y</code>? Explain <b>both</b> reasons.",
 code='''fn main() {
    let x = 10;
    let y = {
        let x = 3;
        x + 1
    };
    println!("{x} {y}");
}''',
 code_on="front",
 expect="10 4",
 back="<b><code>x</code> is 10 and <code>y</code> is 4, for two independent reasons.</b>"
      "<br><br>1. <b>The block is an expression.</b> Its value is its last expression <i>without</i> a semicolon &mdash; "
      "<code>x + 1</code> has no <code>;</code>, so the block evaluates to 4."
      "<br>2. <b>The inner <code>let x = 3</code> shadows</b> the outer <code>x</code> only inside the braces, so the "
      "outer one is still 10."
      "<br><br>The <code>;</code> after the closing <code>}</code> ends the <code>let</code> statement &mdash; "
      "required, and unrelated to either. Put a semicolon on <code>x + 1;</code> and the block&rsquo;s value becomes "
      "<code>()</code>.",
 bridge="<b>Python:</b> has neither &mdash; no block expressions, and no shadowing in an inner scope (a nested "
        "<code>x = 3</code> in the same function rebinds the <i>same</i> name). The Rust habit this enables has no "
        "Python counterpart: <code>let config = { ...build it...  built };</code>, where the scratch variables die "
        "at the brace.",
 link=("a_block_is_an_expression", SITE+"15_First_Programs/a_block_is_an_expression/index.html"),
 tags="rust primitives expressions"),

dict(id="prim_statement_vs_expression",
 front="Statement or expression &mdash; what is the difference, and which one is <code>let x = 5;</code>?",
 back="<b>An expression evaluates to a value; a statement performs an action and does not.</b> "
      "<code>let x = 5;</code> is a <b>statement</b>."
      "<br><br>Which is why <code>let a = (let b = 5);</code> does not compile &mdash; there is no value to bind."
      "<br><br>Rust is expression-heavy, so the things that are statements in other languages have values here:"
      "<br>&bull; <code>let size = if n &lt; 10 { \"small\" } else { \"large\" };</code>"
      "<br>&bull; <code>let v = match c { .. };</code>"
      "<br>&bull; <code>let v = loop { break 7; };</code>"
      "<br>&bull; a function body&rsquo;s last expression is its return value, no <code>return</code> needed"
      "<br><br>A <code>;</code> turns an expression into a statement by discarding its value and leaving "
      "<code>()</code>.",
 code='''fn main() {
    let n = 4;
    let size = if n < 10 { "small" } else { "large" };
    let doubled = { n * 2 };
    println!("{size} {doubled}");
}''',
 code_on="back",
 expect="small 8",
 bridge="<b>Python:</b> the conditional expression <code>\"small\" if n &lt; 10 else \"large\"</code> is the one "
        "counterpart; <code>match</code> and <code>loop</code> have none. Python&rsquo;s statement/expression line "
        "is drawn in a very different place, which is why <code>return</code> is mandatory there and optional here.",
 link=("a_block_is_an_expression", SITE+"15_First_Programs/a_block_is_an_expression/index.html"),
 tags="rust primitives expressions"),

dict(id="prim_reverse_an_array",
 front="Reverse an array. Write the two ways, and say what each returns.",
 code='''fn main() {
    let a = [1, 2, 3];
    let backwards: Vec<i32> = a.iter().rev().copied().collect();
    let mut b = [1, 2, 3];
    b.reverse();
    println!("{backwards:?} {b:?} {a:?}");
}''',
 code_on="front",
 expect="[3, 2, 1] [3, 2, 1] [1, 2, 3]",
 back="<b><code>a.iter().rev()</code></b> returns a <b>lazy iterator</b>, not a reversed array &mdash; nothing has "
      "moved, and the original is untouched. Add <code>.collect()</code> if you want a container."
      "<br><br><b><code>b.reverse()</code></b> reverses <b>in place</b> and returns <code>()</code>, like every "
      "other in-place method."
      "<br><br><code>.rev()</code> needs <code>DoubleEndedIterator</code>, which slice and array iterators are &mdash; "
      "but a <code>HashMap</code> iterator is not, so <code>.rev()</code> on one does not compile.",
 bridge="<b>Python:</b> the same split, same names almost &mdash; <code>reversed(a)</code> is the lazy view "
        "(<code>list(...)</code> to materialise) and <code>a.reverse()</code> is in place and returns "
        "<code>None</code>. This one maps across cleanly.",
 link=("double_ended_and_exact_size", SITE+"24_Iterators/double_ended_and_exact_size/index.html"),
 tags="rust primitives arrays iterators"),

]
