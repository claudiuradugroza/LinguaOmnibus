use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: lofmt <file.lo> [--check]");
        std::process::exit(1);
    }
    let file = &args[1];
    let check = args.iter().any(|a| a == "--check");
    let source = fs::read_to_string(file).expect("cannot read file");
    let formatted = format_source(&source);
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

fn format_source(source: &str) -> String {
    // Basic formatter: normalize line endings, trim trailing whitespace, ensure trailing newline
    let mut result = String::new();
    for line in source.lines() {
        result.push_str(line.trim_end());
        result.push('\n');
    }
    // Ensure single trailing newline
    while result.ends_with("\n\n") {
        result.pop();
    }
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result
}