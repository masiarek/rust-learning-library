//! Three jobs that sound like they need a procedural macro, done with
//! `macro_rules!` and a `const fn`.

/// Counts its arguments without recursion: one `()` per name, in a slice
/// whose length is a constant.
#[macro_export]
macro_rules! count {
    ($($name:ident),* $(,)?) => {
        <[()]>::len(&[$($crate::count!(@unit $name)),*])
    };
    (@unit $name:ident) => {
        ()
    };
}

/// Checks a string literal's contents while compiling. The macro cannot look
/// inside the literal, but a `const` it expands to can.
#[macro_export]
macro_rules! hex {
    ($text:literal) => {{
        const _: () = assert!($crate::all_hex($text), "not a hex string");
        $text
    }};
}

/// An error message of the macro's own, naming the token it refused.
#[macro_export]
macro_rules! method {
    (GET) => {
        "GET"
    };
    (POST) => {
        "POST"
    };
    ($other:ident) => {
        compile_error!(concat!("unknown method `", stringify!($other), "`"))
    };
}

pub const fn all_hex(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if !bytes[i].is_ascii_hexdigit() {
            return false;
        }
        i += 1;
    }
    true
}
