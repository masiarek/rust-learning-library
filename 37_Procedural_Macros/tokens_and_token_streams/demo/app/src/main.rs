use inspect_tokens::{Text, text, text_attr, token_trees};

type Meters = u32;

/// A road.
#[derive(Text)]
struct Road {
    length: Meters,
}

/// Greets.
#[text_attr]
fn greet() {}

/// `$e` reaches `token_trees!` wrapped in an invisible group.
macro_rules! twice {
    ($e:expr) => {
        token_trees!($e * 2)
    };
}

/// Hands its tokens to `$inspect!` unchanged, as `println!` and `vec!` do.
macro_rules! forward {
    ($inspect:ident, $($tokens:tt)*) => {
        $inspect!($($tokens)*)
    };
}

/// Every line of `text`, indented under its label.
fn show(label: &str, text: &str) {
    println!("{label}");
    for line in text.lines() {
        println!("    {line}");
    }
}

fn main() {
    show("1. Four kinds of token tree", token_trees!(GET /users/{id} => list_users));
    show("2. Spacing", token_trees!(x += 1; &'a str; 'a'; -1));
    show("3. Delimiters", token_trees!((a) [b] {c}));
    show("   and the invisible one, around a macro_rules! $e", twice!(1 + 1));

    let direct = token_trees! {
        /// hi
    };
    let forwarded = forward! { token_trees,
        /// hi
    };
    show("4. A doc comment, straight to the macro", direct);
    show("   and forwarded by macro_rules!", forwarded);
    let direct = text! {
        /// hi
    };
    let forwarded = forward! { text,
        /// hi
    };
    show("   to_string, straight to a function-like macro", direct);
    show("   to_string, forwarded by macro_rules!", forwarded);
    show("   to_string in a derive", DERIVE_INPUT);
    show("   to_string in an attribute", ATTRIBUTE_INPUT);

    let spacing = [text!(x:i32), text!(x : i32), text!(x
                                                      :     i32)];
    show("5. to_string and whitespace", &spacing.join("\n"));

    greet();
    println!("road.length = {}", Road { length: 5 }.length);
}
