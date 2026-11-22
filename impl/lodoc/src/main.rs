use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: lodoc <file.lo> [--output <dir>]");
        std::process::exit(1);
    }
    let file = &args[1];
    let source = fs::read_to_string(file).expect("cannot read file");

    // Extract doc comments (/// lines) preceding declarations
    let mut docs = Vec::new();
    let mut current_doc = String::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("///") {
            current_doc.push_str(&trimmed[3..]);
            current_doc.push('\n');
        } else if !current_doc.is_empty() {
            // Next non-doc line is a declaration
            if trimmed.starts_with("function") || trimmed.starts_with("funcție")
                || trimmed.starts_with("struct") || trimmed.starts_with("structură")
                || trimmed.starts_with("enum") || trimmed.starts_with("enumerare") {
                docs.push((trimmed.to_string(), current_doc.clone()));
            }
            current_doc.clear();
        }
    }

    // Output as Markdown
    println!("# Documentation for {}\n", file);
    for (decl, doc) in &docs {
        println!("## {}\n", decl);
        println!("{}", doc);
    }
    if docs.is_empty() {
        println!("No documented declarations found.");
    }
}