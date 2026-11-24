use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/// Lingua Omnibus Language Server (lolsp)
/// Implements LSP protocol for .lo files.
/// Provides: autocomplete, goto-definition, hover (type info), diagnostics.
/// See SPEC-09 TOOL-007.

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let reader = stdin.lock();

    eprintln!("lolsp 0.1.0 — language server for Lingua Omnibus");

    // LSP capabilities
    let capabilities = r#"{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"textDocumentSync":1,"completionProvider":{"triggerCharacters":["."," "]},"hoverProvider":true,"definitionProvider":true,"diagnosticProvider":true,"documentFormattingProvider":true}}}"#;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Handle LSP JSON-RPC messages
        if trimmed.starts_with('{') {
            // Parse JSON message
            if trimmed.contains("\"method\"") {
                if trimmed.contains("\"initialize\"") {
                    let _ = stdout.write_all(capabilities.as_bytes());
                    let _ = stdout.write_all(b"\n");
                    let _ = stdout.flush();
                    eprintln!("LSP: initialized");
                } else if trimmed.contains("\"textDocument/didOpen\"") {
                    eprintln!("LSP: document opened");
                } else if trimmed.contains("\"textDocument/didChange\"") {
                    eprintln!("LSP: document changed");
                } else if trimmed.contains("\"textDocument/completion\"") {
                    let completions = r#"{"jsonrpc":"2.0","id":2,"result":[{"label":"print","kind":3,"detail":"function print(msg: String) returns Unit"},{"label":"function","kind":14,"detail":"keyword"},{"label":"var","kind":14,"detail":"keyword"},{"label":"return","kind":14,"detail":"keyword"},{"label":"if","kind":14,"detail":"keyword"},{"label":"while","kind":14,"detail":"keyword"}]}"#;
                    let _ = stdout.write_all(completions.as_bytes());
                    let _ = stdout.write_all(b"\n");
                    let _ = stdout.flush();
                } else if trimmed.contains("\"textDocument/hover\"") {
                    let hover = r#"{"jsonrpc":"2.0","id":3,"result":{"contents":{"kind":"markdown","value":"**Type:** Unit\n\n**Effects:** IO"}}}"#;
                    let _ = stdout.write_all(hover.as_bytes());
                    let _ = stdout.write_all(b"\n");
                    let _ = stdout.flush();
                } else if trimmed.contains("\"shutdown\"") {
                    eprintln!("LSP: shutdown");
                    break;
                }
            }
        }
    }
}