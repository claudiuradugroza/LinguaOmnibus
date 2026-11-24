use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: lofmt <file.lo> [--check] [--ast]");
        std::process::exit(1);
    }
    let file = &args[1];
    let check = args.iter().any(|a| a == "--check");
    let use_ast = args.iter().any(|a| a == "--ast");

    let source = fs::read_to_string(file).expect("cannot read file");

    let formatted = if use_ast {
        // AST-based formatting: parse with loc and re-emit
        format_with_ast(&source, file)
    } else {
        // Basic text formatting
        format_text(&source)
    };

    if check {
        if formatted == source {
            println!("OK: {} is formatted", file);
        } else {
            println!("DIFF: {} needs formatting", file);
            std::process::exit(1);
        }
    } else {
        fs::write(file, &formatted).expect("cannot write file");
        println!("formatted: {}", file);
    }
}

fn format_text(source: &str) -> String {
    // Basic formatter: normalize line endings, trim trailing whitespace, ensure trailing newline
    let mut result = String::new();
    for line in source.lines() {
        result.push_str(line.trim_end());
        result.push('\n');
    }
    while result.ends_with("\n\n") {
        result.pop();
    }
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result
}

fn format_with_ast(source: &str, file: &str) -> String {
    // AST-based formatting using loc parser
    // For now, fall back to text formatting
    // Full AST-based formatting requires loc library integration
    format_text(source)
}