//! Optional function arguments — in a language with no default parameters.
//!
//! Rust has no default argument values and no overloading. So "this argument is
//! optional" has to be built out of something, and `Option<T>` is only one of the
//! five ways — usually not the best one.
//!
//!   rustc --edition 2024 optional_arguments.rs -o /tmp/oa && /tmp/oa

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
// What std does when there are exactly two shapes: two functions.
fn open(path: &str) -> String {
    open_with_mode(path, "read")
}
fn open_with_mode(path: &str, mode: &str) -> String {
    format!("{path} [{mode}]")
}

fn step1() {
    banner(1, "No default arguments — so std writes a second function");

    println!("  open(\"a.txt\")                     -> {}", open("a.txt"));
    println!("  open_with_mode(\"a.txt\", \"append\") -> {}", open_with_mode("a.txt", "append"));
    println!("      This is Vec::new / Vec::with_capacity, HashMap::new / with_capacity.");
    println!("      Two names beat one name plus a None nobody can read.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn connect(host: &str, port: Option<u16>, timeout: Option<u32>) -> String {
    format!(
        "{host}:{} (timeout {}s)",
        port.unwrap_or(443),
        timeout.unwrap_or(30)
    )
}

fn step2() {
    banner(2, "The Option parameter, and what it costs the CALLER");

    println!("  connect(\"a.io\", None, None)          -> {}", connect("a.io", None, None));
    println!("  connect(\"a.io\", Some(80), None)      -> {}", connect("a.io", Some(80), None));
    println!("  connect(\"a.io\", None, Some(5))       -> {}", connect("a.io", None, Some(5)));
    println!("      Every call site must spell out every argument, including the ones it");
    println!("      does not care about — and `None, None` says nothing about WHICH knobs");
    println!("      were skipped. Two Nones is tolerable; five is a puzzle.");
}

// ─────────────────────────────────────────────────────────── Step 3
// Take Option<&T>, not &Option<T>. The first accepts strictly more callers.
fn greet_flexible(name: Option<&str>) -> String {
    format!("hello, {}", name.unwrap_or("stranger"))
}
fn greet_rigid(name: &Option<String>) -> String {
    format!("hello, {}", name.as_deref().unwrap_or("stranger"))
}

fn step3() {
    banner(3, "Option<&T> in argument position, never &Option<T>");

    let owned: Option<String> = Some("Ada".to_string());
    let borrowed: &str = "Ben";

    println!("  greet_flexible(Some(\"Ben\"))       -> {}", greet_flexible(Some(borrowed)));
    println!("  greet_flexible(owned.as_deref())  -> {}", greet_flexible(owned.as_deref()));
    println!("  greet_flexible(None)              -> {}", greet_flexible(None));
    println!("  greet_rigid(&owned)               -> {}", greet_rigid(&owned));
    println!("      greet_rigid demands a real Option<String> to point at. A caller holding");
    println!("      a plain &str has to BUILD one — allocating a String and an Option — to");
    println!("      call it. Option<&T> takes both, so it is the one to write.");
}

// ─────────────────────────────────────────────────────────── Step 4
// The trick that makes `None` optional-ish at the call site.
fn retries(times: impl Into<Option<u32>>) -> u32 {
    times.into().unwrap_or(3)
}

fn step4() {
    banner(4, "impl Into<Option<T>>: pass the bare value OR None");

    println!("  retries(5)     -> {}", retries(5));
    println!("  retries(None)  -> {}", retries(None));
    println!("      `impl From<T> for Option<T>` exists, so 5 converts to Some(5).");
    println!("      Cute, and rare on purpose: it weakens inference, it does not compose");
    println!("      past one or two arguments, and the signature stops being obvious.");
}

// ─────────────────────────────────────────────────────────── Step 5
#[derive(Debug, Default)]
struct ConnectOpts {
    port: Option<u16>,
    timeout: Option<u32>,
    retries: Option<u32>,
}

fn connect_with(host: &str, opts: ConnectOpts) -> String {
    format!(
        "{host}:{} (timeout {}s, {} retries)",
        opts.port.unwrap_or(443),
        opts.timeout.unwrap_or(30),
        opts.retries.unwrap_or(3)
    )
}

fn step5() {
    banner(5, "An options struct: the arguments get their names back");

    println!("  {}", connect_with("a.io", ConnectOpts::default()));
    println!(
        "  {}",
        connect_with("a.io", ConnectOpts { timeout: Some(5), ..Default::default() })
    );
    println!("      `timeout: Some(5)` says at the call site which knob was turned.");
    println!("      Adding a fourth option later does not touch a single existing caller.");
}

// ─────────────────────────────────────────────────────────── Step 6
struct Request {
    url: String,
    method: String,
    retries: u32,
}

struct RequestBuilder {
    url: String,
    method: String,
    retries: u32,
}

impl RequestBuilder {
    fn new(url: &str) -> Self {
        Self { url: url.to_string(), method: "GET".into(), retries: 3 }
    }
    fn method(mut self, m: &str) -> Self {
        self.method = m.to_string();
        self
    }
    fn retries(mut self, n: u32) -> Self {
        self.retries = n;
        self
    }
    fn build(self) -> Request {
        Request { url: self.url, method: self.method, retries: self.retries }
    }
}

fn step6() {
    banner(6, "The builder: for when there are many, and defaults are real values");

    let r = RequestBuilder::new("https://a.io").method("POST").retries(5).build();
    println!("  {} {} ({} retries)", r.method, r.url, r.retries);

    let d = RequestBuilder::new("https://a.io").build();
    println!("  {} {} ({} retries)   <- untouched defaults", d.method, d.url, d.retries);
    println!("      Note there is no Option anywhere: the defaults are ordinary values held");
    println!("      by the builder. Option only appears when 'unset' differs from any value.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    println!();
}
