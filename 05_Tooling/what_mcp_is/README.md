# What MCP is

**Level:** 201 · working knowledge

**One line:** MCP is JSON-RPC 2.0 with one message per line of text, so an *MCP server* is an ordinary program that reads a request line on stdin and writes a reply line on stdout — and the *"Allow running MCP?"* dialog is your editor asking whether the model may call one of that program's functions on your behalf.

The name is the least helpful part of it. **Model Context Protocol** sounds like a machine-learning concept; it is a message format, and the whole of it fits on this page.

## What the dialog is asking

A prompt like *"Allow running MCP?"* with `Always allow ("rustrover:read_file")` beneath it names two things, and the colon separates them:

| Piece | What it is |
|---|---|
| `rustrover` | the **server** — one program the editor launched, offering some set of functions |
| `read_file` | one **tool** on that server — a single function, with a JSON Schema saying what it takes |

The prefix exists because tool names are only unique *within* a server, so anything aggregating several of them has to disambiguate; the specification recommends exactly this — prefix with a server identifier.

The server is visible from outside the IDE, which is the quickest way to believe any of this. With RustRover open, `ps` shows the editor having launched **itself** a second time — and the agent that talks to it as a third process:

```text
$ ps -ax -o command | grep -i rustrover
…/RustRover.app/Contents/MacOS/rustrover
…/RustRover.app/Contents/MacOS/rustrover stdioMcpServer
~/Library/Caches/JetBrains/RustRover<version>/acp-agents/junie/…
```

`stdioMcpServer` is the process the dialog calls `rustrover`, reading requests from the pipe on its stdin. The agent is the client. There is no third party and no network.

That makes the four buttons four different-sized grants:

| Button | Grants |
|---|---|
| **Yes** | this one call, now |
| **No** | not this call |
| `Always allow ("rustrover:read_file")` | that one function, from now on, without asking |
| `Always allow ("rustrover:*")` | **every** function that server offers — including ones it has not shipped yet |

The wildcard is the one worth pausing on. A server may add tools later and announce it (`notifications/tools/list_changed`), and a blanket grant covers the new ones too. It is also the specification's own position that a tool description is not evidence: clients are told to treat tool annotations as **untrusted** unless the server is trusted. So `rustrover:*` is a judgement about the *server*, not about the tool you were just shown.

The dialog itself is in the specification. Tools are arbitrary code execution, so implementations are told there should always be a human able to deny a call, and that clients should present confirmation prompts. An agent that never asks is not being convenient — it is skipping the step the protocol asks for.

## It is two lines of text

Underneath the dialog there is no API, no SDK requirement, and no network. The model's client writes a line of JSON to a pipe:

```json
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "read_file", "arguments": {"path": "src/main.rs"}}}
```

and the server writes a line of JSON back:

```json
{"jsonrpc": "2.0", "id": 2, "result": {"resultType": "complete", "content": [{"type": "text", "text": "fn main() {}"}], "isError": false}}
```

That is the protocol. The `id` pairs them up, `method` picks the operation, and everything else is arguments. On the **stdio** transport those lines travel on a subprocess's pipes; on **Streamable HTTP** each one is a POST. The messages are identical either way.

## A whole MCP server, with no crates

<!-- source:what_mcp_is -->
*[`what_mcp_is.rs`](examples/what_mcp_is.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
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
```
<!-- /source -->

Its stdout — which *is* the wire:

<!-- output:what_mcp_is -->
*Verified output of [`what_mcp_is.rs`](examples/what_mcp_is.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
{"jsonrpc":"2.0","id":1,"result":{"resultType":"complete","tools":[{"name":"word_count","description":"Count whitespace-separated words.","inputSchema":{"type":"object","properties":{"text":{"type":"string"}},"required":["text"]}},{"name":"snake_case","description":"Rewrite CamelCase as snake_case.","inputSchema":{"type":"object","properties":{"text":{"type":"string"}},"required":["text"]}}]}}
{"jsonrpc":"2.0","id":2,"result":{"resultType":"complete","content":[{"type":"text","text":"7"}],"isError":false}}
{"jsonrpc":"2.0","id":3,"result":{"resultType":"complete","content":[{"type":"text","text":"borrow_checker"}],"isError":false}}
{"jsonrpc":"2.0","id":4,"result":{"resultType":"complete","content":[{"type":"text","text":"text must not be empty"}],"isError":true}}
```
<!-- /output -->

Five requests went in and four replies came out. The fifth was a *notification*: no `id`, so nothing to reply to, so the server says nothing at all.

## The four nouns

| | |
|---|---|
| **Host** | the application holding the model — the IDE, the chat client |
| **Client** | the connector inside the host that speaks to one server |
| **Server** | the program offering capabilities. The example above is one |
| **Transport** | how the bytes move: **stdio** (a subprocess and its pipes) or **Streamable HTTP** (a POST per message) |

A server can offer three kinds of thing. **Tools** are functions for the model to call — the only kind the example implements, and the kind nearly every permission prompt is about. **Resources** are context and data to read. **Prompts** are templated workflows offered to the user. In the other direction a client can offer **elicitation**: the server asking the user for something part-way through a call.

You already run a protocol shaped like this all day. MCP is openly modelled on the **Language Server Protocol**, and as [Choosing an editor](../editors/README.md) puts it, every editor here is a front end for the same `rust-analyzer` — a program your editor launches and talks JSON-RPC to over a pipe. Swap *"which type is this symbol"* for *"call this function"* and you have MCP.

## Two kinds of failure, and they are not interchangeable

The example answers a bad request two different ways, on purpose:

```json
{"result": {"content": [{"type": "text", "text": "text must not be empty"}], "isError": true}}
```

```json
{"error": {"code": -32602, "message": "Unknown tool: delete_everything"}}
```

The distinction is about **who can fix it**. `isError: true` is a normal result carrying a message the model is meant to read and retry against — a bad date format, a value out of range, an API that was down. A JSON-RPC `error` means the request itself made no sense: an unknown tool, a malformed call, and no rewording of the arguments will help. Reaching for the error channel to report a failed-but-valid call throws away the model's chance to correct itself.

## The trap: `println!` breaks the protocol

On the stdio transport, stdout **is** the wire. The rule is flat: a server must not write anything to stdout that is not a valid MCP message, and each message must be one line with no embedded newlines. One stray debug print and the client's parser meets a line that is not JSON.

```rust
fn handle_call(name: &str) {
    eprintln!("looking up {name}");   // fine — stderr is yours
    // println!("looking up {name}"); // corrupts the stream
}
```

stderr is explicitly the free channel: a server may write anything there, and a client is told **not** to read output on it as a sign of failure. It is the only place an MCP server is allowed to talk to a human.

This is the one Rust-specific hazard in the whole protocol, because `println!` is the first thing anyone reaches for. A logging crate configured to stdout does it just as effectively, and more quietly.

The example enforces the rule by construction rather than describing it: `main` writes its commentary with `eprintln!` and its replies with `println!`, and the answer key recorded beside it captures **stdout only** — the same discipline, applied by the test runner.

## If you are coming from another language

**Python.** The closest thing you have already written is a `json.loads()` loop over `sys.stdin` — that is the entire transport. The official `mcp` package hides it behind a `@mcp.tool()` decorator that reads your type hints and generates the JSON Schema, which is what the hand-written `inputSchema` above would otherwise be. What does *not* transfer is printing: in a Python MCP server `print()` is the same bug as `println!`. `logging` writes to stderr by default, so it happens to be safe — by luck, not by design.

**ABAP.** The nearest shape is an RFC-enabled function module: a named callable with a declared parameter interface that a foreign caller invokes without knowing the implementation. `inputSchema` is the IMPORTING signature, and the JSON Schema does the job SE37's typed parameters do — except that the caller is a language model reading the `description` field, so that one string carries the weight a colleague's tribal knowledge usually carries. The difference worth naming: an RFC destination is configured once in SM59 by a Basis administrator, whereas MCP puts the authorization decision in a dialog box in front of the end user, per call.

**Anyone.** The protocol is stateless as of revision `2026-07-28` — every request carries its own protocol version and capabilities in a `_meta` block, and there is no connection-scoped session. Earlier revisions opened with an `initialize` handshake, so material written against those describes a session that no longer exists. A server needing state across calls has to hand back an explicit handle and take it as an argument next time, exactly as an HTTP API would.

## Writing one for real

The example above needs no crates because it parses JSON with `str::find`, which is a stunt — and is labelled as one in the source. A real server uses the Tier 1 Rust SDK:

```bash
cargo add rmcp --features server
```

[`rmcp` ↗](https://github.com/modelcontextprotocol/rust-sdk) is the official Rust implementation, and it is async — it brings `tokio`, a heavier dependency than anything else in this library. It generates the schemas, owns the transport, and leaves you writing the function bodies.

Worth knowing before you start: **the interesting problem is not the protocol.** It is deciding what a tool should do, naming it so a model picks the right one, and writing a description precise enough to serve as the entire manual. The wire format is the easy half, and this page is most of it.

## Sources

- [Specification ↗](https://modelcontextprotocol.io/specification/latest) — revision `2026-07-28`; the overview, the tools page and the stdio transport page are where every rule above comes from
- [`rust-sdk` ↗](https://github.com/modelcontextprotocol/rust-sdk) — the `rmcp` crate
- [JSON-RPC 2.0 ↗](https://www.jsonrpc.org/specification) — the message format MCP carries, unchanged

## See also

- [Choosing an editor](../editors/README.md) — the LSP servers your editor already launches exactly this way
- [RustRover setup](../rustrover_setup/README.md) — the rest of what that IDE needs told
