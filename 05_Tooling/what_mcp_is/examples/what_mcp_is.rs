//! A whole MCP server, with no crates, no network, and no magic.
//!
//! MCP messages are JSON-RPC 2.0, one message per line. Over the stdio
//! transport the client launches this program as a subprocess and talks to it
//! through the pipes: requests arrive on stdin, replies leave on stdout.
//!
//! The real `main` is three lines and reads the pipe:
//!
//!     for line in std::io::stdin().lock().lines() {
//!         if let Some(reply) = handle(&line.unwrap()) { println!("{reply}"); }
//!     }
//!
//! This one feeds `handle` a fixed transcript instead, so the output is an
//! answer key rather than whatever an editor happened to ask today. Everything
//! below the transcript is what a real server runs.

/// What a client writes to this server's stdin, one message per line.
///
/// The first request carries the `_meta` block every request must have in
/// protocol revision 2026-07-28; the rest omit it, exactly as the spec's own
/// examples do, because it is the same three fields every time.
const INBOX: [&str; 5] = [
    r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{},"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientInfo":{"name":"a-code-editor","version":"1.0"},"io.modelcontextprotocol/clientCapabilities":{}}}"#,
    r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"word_count","arguments":{"text":"the borrow checker is not your enemy"}}}"#,
    r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"snake_case","arguments":{"text":"BorrowChecker"}}}"#,
    r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"word_count","arguments":{"text":""}}}"#,
    r#"{"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":4}}"#,
];

fn main() {
    for line in INBOX {
        // stderr is yours: the spec says a server MAY write anything here, and
        // that a client SHOULD NOT read it as failure. It is the only place a
        // server is allowed to talk to a human.
        eprintln!("<- {line}");

        // stdout is the wire. One line per reply, and nothing else, ever.
        if let Some(reply) = handle(line) {
            println!("{reply}");
        }
    }
}

/// Answer one request line. `None` means "say nothing" — a notification has no
/// `id`, so there is nothing to reply to.
fn handle(request: &str) -> Option<String> {
    // A `println!("about to dispatch")` here would put a line on stdout that is
    // not a JSON-RPC message, and the client's parser would stop at it. Debug
    // with `eprintln!` in an MCP server, always:
    // println!("about to dispatch");   <- breaks the protocol

    let id = number_field(request, "id")?;
    let method = string_field(request, "method")?;
    let params = value_after(request, "params").unwrap_or("{}");

    match method {
        "tools/list" => Some(tool_list(id)),
        "tools/call" => Some(tool_call(id, params)),
        _ => Some(error(id, -32601, &format!("Method not found: {method}"))),
    }
}

/// Every tool this server offers, with the JSON Schema a model reads to work
/// out how to call it. The description is not decoration — it is the entire
/// user manual the model gets.
fn tool_list(id: u32) -> String {
    let schema = r#"{"type":"object","properties":{"text":{"type":"string"}},"required":["text"]}"#;
    format!(
        concat!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"resultType":"complete","tools":["#,
            r#"{{"name":"word_count","description":"Count whitespace-separated words.","inputSchema":{}}},"#,
            r#"{{"name":"snake_case","description":"Rewrite CamelCase as snake_case.","inputSchema":{}}}"#,
            r#"]}}}}"#
        ),
        id, schema, schema
    )
}

fn tool_call(id: u32, params: &str) -> String {
    let Some(name) = string_field(params, "name") else {
        return error(id, -32602, "Missing tool name");
    };
    let text = string_field(params, "text").unwrap_or("");

    match name {
        // A tool that ran and failed is NOT a JSON-RPC error. It is a normal
        // result carrying `isError: true`, because the model is meant to read
        // the message and try again.
        "word_count" | "snake_case" if text.is_empty() => {
            result(id, "text must not be empty", true)
        }
        "word_count" => result(id, &text.split_whitespace().count().to_string(), false),
        "snake_case" => result(id, &snake_case(text), false),
        // An unknown tool is a broken request, not a failed call, so it takes
        // the JSON-RPC error channel instead.
        _ => error(id, -32602, &format!("Unknown tool: {name}")),
    }
}

fn snake_case(text: &str) -> String {
    let mut out = String::new();
    for (i, ch) in text.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.extend(ch.to_lowercase());
    }
    out
}

fn result(id: u32, text: &str, is_error: bool) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"result":{{"resultType":"complete","content":[{{"type":"text","text":"{text}"}}],"isError":{is_error}}}}}"#
    )
}

fn error(id: u32, code: i32, message: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":{code},"message":"{message}"}}}}"#)
}

// ---------------------------------------------------------------------------
// A toy JSON scanner, so this file needs no crates. It reads a key's value by
// finding the key's text, which is wrong for any input it was not written for:
// a string containing `"method"`, an escaped quote, a key nested one level too
// deep. A real server hands the line to `serde_json` and never writes this.
// ---------------------------------------------------------------------------

/// Everything after `"key":`, with the whitespace eaten.
fn value_after<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let quoted = format!("\"{key}\"");
    let at = json.find(&quoted)?;
    let rest = json[at + quoted.len()..].trim_start();
    Some(rest.strip_prefix(':')?.trim_start())
}

fn string_field<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let value = value_after(json, key)?.strip_prefix('"')?;
    Some(&value[..value.find('"')?])
}

fn number_field(json: &str, key: &str) -> Option<u32> {
    let value = value_after(json, key)?;
    let end = value
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(value.len());
    value[..end].parse().ok()
}
