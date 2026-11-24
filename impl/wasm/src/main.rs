use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: wasm <input.lir> [--output <file.wasm>]");
        std::process::exit(1);
    }
    let input = &args[1];
    let lir_source = fs::read_to_string(input).expect("cannot read LIR file");

    let wat = emit_wat(&lir_source);

    if args.len() >= 4 && args[2] == "--output" {
        let output = &args[3];
        fs::write(output, &wat).expect("cannot write WAT file");
        println!("Written to: {}", output);
    } else {
        println!("{}", wat);
    }
}

/// Emit WebAssembly Text Format (WAT) from LIR text.
fn emit_wat(lir_source: &str) -> String {
    let mut output = String::new();
    output.push_str("(module\n");
    output.push_str("  (import \"wasi_snapshot_preview1\" \"fd_write\" (func $fd_write (param i32 i32 i32 i32) (result i32)))\n");
    output.push_str("  (memory (export \"memory\") 1)\n");
    output.push_str("  (export \"_start\" (func $main))\n\n");

    let mut in_function = false;
    let mut func_name = String::new();
    let mut body = String::new();

    for line in lir_source.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("lir ") || trimmed.starts_with("module") || trimmed == "}" {
            continue;
        }

        if trimmed.starts_with("fn ") {
            if in_function && !body.is_empty() {
                // Emit previous function
                output.push_str(&format!("  (func ${} (export \"{}\")\n", func_name, func_name));
                output.push_str(&body);
                output.push_str("  )\n\n");
                body = String::new();
            }
            in_function = true;
            if let Some(colon_pos) = trimmed.find(" : ") {
                func_name = trimmed[3..colon_pos].trim().to_string();
            } else {
                func_name = "main".to_string();
            }
            continue;
        }

        if trimmed == "entry:" || trimmed.ends_with(':') {
            continue;
        }

        if trimmed.starts_with("ret ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] != "unit" {
                body.push_str(&format!("    ;; return {}\n", parts[1]));
            }
            body.push_str("    return\n");
            continue;
        }

        if trimmed.contains(" = ") {
            let eq_parts: Vec<&str> = trimmed.splitn(2, " = ").collect();
            if eq_parts.len() == 2 {
                let lhs = eq_parts[0].trim().trim_start_matches('%');
                let rhs = eq_parts[1].trim();
                let rhs_parts: Vec<&str> = rhs.split_whitespace().collect();

                if rhs_parts.is_empty() {
                    continue;
                }

                match rhs_parts[0] {
                    "const.str" => {
                        let str_val = if rhs.contains('"') {
                            let start = rhs.find('"').unwrap_or(0);
                            let end = rhs.rfind('"').unwrap_or(rhs.len());
                            &rhs[start..=end]
                        } else {
                            "\"\""
                        };
                        body.push_str(&format!("    ;; {} = const.str {}\n", lhs, str_val));
                        body.push_str(&format!("    (local.set ${} (i32.const 0))\n", lhs));
                    }
                    "const.int" => {
                        if rhs_parts.len() >= 2 {
                            body.push_str(&format!("    (local.set ${} (i64.const {}))\n", lhs, rhs_parts[1]));
                        }
                    }
                    "add" => {
                        if rhs_parts.len() >= 3 {
                            let a = rhs_parts[1].trim_end_matches(',');
                            let b = rhs_parts[2];
                            body.push_str(&format!("    (local.set ${} (i64.add (local.get ${}) (local.get ${})))\n", lhs, a.trim_start_matches('%'), b.trim_start_matches('%')));
                        }
                    }
                    "call" => {
                        let call_str = rhs.trim_start_matches("call ").trim();
                        if let Some(paren_start) = call_str.find('(') {
                            let func_name = &call_str[..paren_start];
                            let func_sanitized = func_name.trim_start_matches('%');
                            body.push_str(&format!("    ;; call {} \n", func_sanitized));
                            body.push_str(&format!("    (local.set ${} (i64.const 0))\n", lhs));
                        }
                    }
                    _ => {
                        body.push_str(&format!("    ;; {} = {}\n", lhs, rhs));
                    }
                }
            }
        }

        if trimmed.starts_with("br ") || trimmed.starts_with("br.cond") {
            body.push_str(&format!("    ;; {}\n", trimmed));
        }
    }

    if in_function && !body.is_empty() {
        output.push_str(&format!("  (func ${} (export \"{}\")\n", func_name, func_name));
        output.push_str(&body);
        output.push_str("  )\n\n");
    }

    // Add main function if not present
    if !output.contains("(func $main") {
        output.push_str("  (func $main (export \"_start\")\n");
        output.push_str("    ;; main entry point\n");
        output.push_str("  )\n\n");
    }

    output.push_str(")\n");
    output
}