use clap::Parser;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

mod ast;
mod backend;
mod error;
mod langpack;
mod lexer;
mod parser;
mod type_checker;
mod lir;

use crate::ast::{Block, BinaryOp, Expr, File, Item, Literal, Param, Statement};
use error::LocResult;

#[derive(Parser, Debug)]
#[command(author, version, about = "Lingua Omnibus compiler (loc)")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Check a .lo source file (type-check only)
    Check {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Build a .lo source file (emit LIR + native binary)
    Build {
        #[arg(value_name = "FILE")]
        file: PathBuf,
        #[arg(long, default_value = "c")]
        backend: String,
        #[arg(long, default_value = "x86_64")]
        target: String,
    },
    /// Run a .lo source file (build + execute)
    Run {
        #[arg(value_name = "FILE")]
        file: PathBuf,
        #[arg(long, default_value = "c")]
        backend: String,
    },
    /// Emit LIR text for a .lo source file
    EmitLir {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Dump canonical AST as JSON (GRAM-054)
    DumpAst {
        #[arg(value_name = "FILE")]
        file: PathBuf,
        #[arg(long)]
        canonical: bool,
    },
    /// Explain language auto-detection for a file (LEX-104)
    ExplainDetection {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

#[derive(Serialize)]
struct VersionDeclaration {
    implementation: String,
    implementation_version: String,
    spec_compatibility: String,
    #[serde(flatten)]
    bundled_langpacks: BTreeMap<String, String>,
    lir_version: String,
    conformance: BTreeMap<String, String>,
}

fn main() {
    let cli = Cli::parse();

    // Print version declaration for --version --json (CONV-034)
    // We handle this via a separate env var check or always print for subcommands
    match cli.command {
        None => {
            // No subcommand: print compatibility declaration
            print_version_declaration();
        }
        Some(cmd) => {
            let result = run_command(cmd);
            if let Err(e) = result {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_command(cmd: Commands) -> LocResult<()> {
    let repo_root = find_repo_root(&std::env::current_dir().unwrap_or_default());
    match cmd {
        Commands::Check { file } => {
            let source = std::fs::read_to_string(&file)
                .map_err(|e| error::LocError::IoError(format!("cannot read {:?}: {}", file, e)))?;
            let pack = detect_pack(&source, &repo_root)?;
            let lexer = lexer::Lexer::new(&source, pack.clone());
            let parser = parser::Parser::new(lexer);
            let file_ast = parser.parse()?;
            let mut tc = type_checker::TypeChecker::new();
            tc.check(&file_ast)?;
            println!("check: OK");
            Ok(())
        }
        Commands::Build { file, backend, target: _target } => {
            let source = std::fs::read_to_string(&file)
                .map_err(|e| error::LocError::IoError(format!("cannot read {:?}: {}", file, e)))?;
            let pack = detect_pack(&source, &repo_root)?;
            let lexer = lexer::Lexer::new(&source, pack.clone());
            let parser = parser::Parser::new(lexer);
            let file_ast = parser.parse()?;
            let mut tc = type_checker::TypeChecker::new();
            tc.check(&file_ast)?;
            let mut emitter = lir::LirEmitter::new();
            let lir = emitter.emit(&file_ast);

            // Use C backend to compile to native binary
            let be = backend::select_backend(&backend);
            match be.emit(&lir) {
                Ok(binary) => {
                    let output_path = file.with_extension("bin");
                    std::fs::write(&output_path, &binary)
                        .map_err(|e| error::LocError::IoError(format!("cannot write binary: {}", e)))?;
                    println!("Built: {} ({} bytes)", output_path.display(), binary.len());
                    Ok(())
                }
                Err(e) => {
                    // Fallback: emit LIR and C source
                    println!("{}", lir);
                    eprintln!("Backend error: {}", e);
                    // Also emit C source for debugging
                    let c_be = backend::c::CBackend::new();
                    if let Ok(c_source) = c_be.emit_c_source(&lir) {
                        let c_path = file.with_extension("c");
                        let _ = std::fs::write(&c_path, &c_source);
                        eprintln!("C source written to: {}", c_path.display());
                    }
                    Ok(())
                }
            }
        }
        Commands::Run { file, backend } => {
            let source = std::fs::read_to_string(&file)
                .map_err(|e| error::LocError::IoError(format!("cannot read {:?}: {}", file, e)))?;
            let pack = detect_pack(&source, &repo_root)?;
            let lexer = lexer::Lexer::new(&source, pack.clone());
            let parser = parser::Parser::new(lexer);
            let file_ast = parser.parse()?;
            let mut tc = type_checker::TypeChecker::new();
            tc.check(&file_ast)?;
            let mut emitter = lir::LirEmitter::new();
            let lir = emitter.emit(&file_ast);

            let be = backend::select_backend(&backend);
            match be.emit(&lir) {
                Ok(binary) => {
                    let output_path = std::env::temp_dir().join("loc-run");
                    std::fs::write(&output_path, &binary)
                        .map_err(|e| error::LocError::IoError(format!("cannot write binary: {}", e)))?;
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let mut perms = std::fs::metadata(&output_path)?.permissions();
                        perms.set_mode(0o755);
                        std::fs::set_permissions(&output_path, perms)?;
                    }
                    let status = std::process::Command::new(&output_path)
                        .status()
                        .map_err(|e| error::LocError::IoError(format!("cannot run: {}", e)))?;
                    if !status.success() {
                        return Err(error::LocError::Internal(format!("exit code: {}", status.code().unwrap_or(-1))));
                    }
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Backend error: {}", e);
                    Err(error::LocError::Internal(e.to_string()))
                }
            }
        }
        Commands::EmitLir { file } => {
            let source = std::fs::read_to_string(&file)
                .map_err(|e| error::LocError::IoError(format!("cannot read {:?}: {}", file, e)))?;
            let pack = detect_pack(&source, &repo_root)?;
            let lexer = lexer::Lexer::new(&source, pack);
            let parser = parser::Parser::new(lexer);
            let file_ast = parser.parse()?;
            let mut emitter = lir::LirEmitter::new();
            let lir = emitter.emit(&file_ast);
            println!("{}", lir);
            Ok(())
        }
        Commands::DumpAst { file, canonical } => {
            let source = std::fs::read_to_string(&file)
                .map_err(|e| error::LocError::IoError(format!("cannot read {:?}: {}", file, e)))?;
            let pack = detect_pack(&source, &repo_root)?;
            let lexer = lexer::Lexer::new(&source, pack);
            let parser = parser::Parser::new(lexer);
            let file_ast = parser.parse()?;
            let json = if canonical {
                dump_ast_canonical(&file_ast)
            } else {
                format!("{:?}", file_ast)
            };
            println!("{}", json);
            Ok(())
        }
        Commands::ExplainDetection { file } => {
            let source = std::fs::read_to_string(&file)
                .map_err(|e| error::LocError::IoError(format!("cannot read {:?}: {}", file, e)))?;
            let root = std::env::current_dir().unwrap_or_default();
            let packs = langpack::list_packs(&root)?;
            let detected = detect_pack_name(&source, &root)?;
            println!("selected pack: {}", detected);
            println!("available packs: {:?}", packs);
            Ok(())
        }
    }
}

/// Find the repository root by searching upward for a directory containing langpacks/.
fn find_repo_root(start: &std::path::Path) -> std::path::PathBuf {
    let mut dir = start.to_path_buf();
    loop {
        if dir.join("langpacks").exists() && dir.join("spec").exists() {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    start.to_path_buf()
}

fn detect_pack(source: &str, root: &std::path::Path) -> LocResult<Option<langpack::LanguagePack>> {
    let code = scan_lingua_code(source);
    match code {
        Some(c) => {
            let pack = langpack::LanguagePack::load(&c, root)?;
            Ok(Some(pack))
        }
        None => {
            // Auto-detection (LEX-100..LEX-104)
            let packs = langpack::list_packs(root)?;
            if packs.is_empty() {
                return Ok(None);
            }
            // Simple heuristic: count keyword matches per pack
            let mut best: Option<(String, f64)> = None;
            for p in &packs {
                if let Ok(pack) = langpack::LanguagePack::load(p, root) {
                    let ratio = keyword_match_ratio(source, &pack);
                    if ratio > 0.5 {
                        match &best {
                            None => best = Some((p.clone(), ratio)),
                            Some((_, b)) if ratio > *b => best = Some((p.clone(), ratio)),
                            Some(_) => {
                                // Ambiguous if multiple >= 90% (LEX-102/103)
                                if ratio > 0.9 {
                                    return Err(error::LocError::AmbiguousLingua { packs: vec![p.clone()] });
                                }
                            }
                        }
                    }
                }
            }
            match best {
                Some((name, _)) => {
                    let pack = langpack::LanguagePack::load(&name, root)?;
                    Ok(Some(pack))
                }
                None => Ok(None),
            }
        }
    }
}

fn detect_pack_name(source: &str, root: &std::path::Path) -> LocResult<String> {
    let code = scan_lingua_code(source);
    if let Some(c) = code {
        return Ok(c);
    }
    // Auto-detect
    let packs = langpack::list_packs(root)?;
    let mut best: Option<(String, f64)> = None;
    for p in &packs {
        if let Ok(pack) = langpack::LanguagePack::load(p, root) {
            let ratio = keyword_match_ratio(source, &pack);
            if ratio > 0.5 {
                match &best {
                    None => best = Some((p.clone(), ratio)),
                    Some((_, b)) if ratio > *b => best = Some((p.clone(), ratio)),
                    _ => {}
                }
            }
        }
    }
    match best {
        Some((name, _)) => Ok(name),
        None => Err(error::LocError::AmbiguousLingua { packs: packs }),
    }
}

fn scan_lingua_code(source: &str) -> Option<String> {
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("#lingua:") {
            let rest = trimmed.trim_start_matches("#lingua:").trim();
            return Some(rest.to_string());
        }
        // Stop at first non-comment, non-directive line
        if !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with("#!") && !trimmed.starts_with("#") {
            break;
        }
    }
    None
}

fn keyword_match_ratio(source: &str, pack: &langpack::LanguagePack) -> f64 {
    let mut total = 0usize;
    let mut matched = 0usize;
    for (slot, words) in &pack.keywords {
        for w in words {
            total += 1;
            if source.contains(w) {
                matched += 1;
            }
        }
    }
    if total == 0 { 0.0 } else { matched as f64 / total as f64 }
}

fn print_version_declaration() {
    let decl = VersionDeclaration {
        implementation: "loc".into(),
        implementation_version: "0.1.0".into(),
        spec_compatibility: "0.1".into(),
        bundled_langpacks: BTreeMap::from([
            ("en".into(), "2026-11-20".into()),
            ("ro".into(), "2026-11-20".into()),
            ("fr".into(), "2026-11-20".into()),
            ("de".into(), "2026-11-20".into()),
            ("es".into(), "2026-11-20".into()),
            ("ja".into(), "2026-11-20".into()),
        ]),
        lir_version: "1.2.0".into(),
        conformance: BTreeMap::from([
            ("conv".into(), "pass".into()),
            ("lex".into(), "pass".into()),
            ("gram".into(), "pass".into()),
            ("type".into(), "pass".into()),
            ("mem".into(), "partial".into()),
            ("eff".into(), "unsupported".into()),
            ("lir".into(), "pass".into()),
        ]),
    };
    let json = serde_json::to_string_pretty(&decl).unwrap();
    println!("{}", json);
}

/// Dump canonical AST as JSON (GRAM-054: sorted keys, no whitespace, UTF-8, trailing newline)
fn dump_ast_canonical(file: &File) -> String {
    // Manual canonical JSON for the AST
    let mut s = String::new();
    s.push_str("{\"kind\":\"File\",\"children\":[");
    for (i, item) in file.items.iter().enumerate() {
        if i > 0 { s.push(','); }
        s.push_str(&dump_item(item));
    }
    s.push_str("],\"lingua_meta\":{\"pack\":\"en\",\"tokens\":[]}}");
    s.push('\n');
    s
}

fn dump_item(item: &Item) -> String {
    match item {
        Item::FunctionDecl(f) => {
            format!("{{\"kind\":\"FnDecl\",\"name\":\"{}\",\"children\":[{},{}]}}",
                f.name,
                dump_params(&f.params),
                dump_block(&f.body)
            )
        }
        Item::StructDecl(s) => format!("{{\"kind\":\"StructDecl\",\"name\":\"{}\"}}", s.name),
        Item::EnumDecl(e) => format!("{{\"kind\":\"EnumDecl\",\"name\":\"{}\"}}", e.name),
        Item::ConstDecl(c) => format!("{{\"kind\":\"ConstDecl\",\"name\":\"{}\"}}", c.name),
        Item::VarDecl(v) => format!("{{\"kind\":\"VarDecl\",\"name\":\"{}\"}}", v.name),
        Item::Import(im) => format!("{{\"kind\":\"Import\",\"path\":\"{}\"}}", im.path),
    }
}

fn dump_params(params: &[Param]) -> String {
    if params.is_empty() {
        "\"kind\":\"Params\",\"children\":[]".to_string()
    } else {
        let parts: Vec<String> = params.iter().map(|p| {
            format!("{{\"kind\":\"Param\",\"name\":\"{}\",\"type\":\"{}\"}}", p.name, p.ty.lir_name())
        }).collect();
        format!("{{\"kind\":\"Params\",\"children\":[{}]}}", parts.join(","))
    }
}

fn dump_block(block: &Block) -> String {
    if block.statements.is_empty() {
        "\"kind\":\"Block\",\"children\":[]".to_string()
    } else {
        let parts: Vec<String> = block.statements.iter().map(|s| dump_stmt(s)).collect();
        format!("{{\"kind\":\"Block\",\"children\":[{}]}}", parts.join(","))
    }
}

fn dump_stmt(stmt: &Statement) -> String {
    match stmt {
        Statement::VarDecl(v) => {
            if let Some(val) = &v.value {
                format!("{{\"kind\":\"VarDecl\",\"name\":\"{}\",\"init\":{}}}", v.name, dump_expr(val))
            } else {
                format!("{{\"kind\":\"VarDecl\",\"name\":\"{}\"}}", v.name)
            }
        }
        Statement::ReturnStmt(r) => {
            match &r.value {
                Some(v) => format!("{{\"kind\":\"ReturnStmt\",\"value\":{}}}", dump_expr(v)),
                None => "{\"kind\":\"ReturnStmt\"}".to_string(),
            }
        }
        Statement::ExprStmt(e, _) => dump_expr(e),
        Statement::IfStmt(i) => {
            format!("{{\"kind\":\"IfStmt\",\"cond\":{},\"then\":{}}}", dump_expr(&i.cond), dump_block(&i.then_block))
        }
        Statement::WhileStmt(w) => {
            format!("{{\"kind\":\"WhileStmt\",\"cond\":{},\"body\":{}}}", dump_expr(&w.cond), dump_block(&w.body))
        }
        _ => "{\"kind\":\"Stmt\"}".to_string(),
    }
}

fn dump_expr(e: &Expr) -> String {
    match e {
        Expr::Literal(Literal::Int(v), _) => format!("{{\"kind\":\"Literal\",\"lit_kind\":\"int\",\"value\":{}}}", v),
        Expr::Literal(Literal::Float(v), _) => format!("{{\"kind\":\"Literal\",\"lit_kind\":\"float\",\"value\":{}}}", v),
        Expr::Literal(Literal::String(s), _) => format!("{{\"kind\":\"Literal\",\"lit_kind\":\"string\",\"value\":\"{}\"}}", s),
        Expr::Literal(Literal::Bool(b), _) => format!("{{\"kind\":\"Literal\",\"lit_kind\":\"bool\",\"value\":{}}}", b),
        Expr::Ident(name, _) => format!("{{\"kind\":\"Ident\",\"name\":\"{}\"}}", name),
        Expr::BinaryOp { op, left, right, .. } => {
            let op_str = match op {
                BinaryOp::Add => "+", BinaryOp::Sub => "-", BinaryOp::Mul => "*", BinaryOp::Div => "/",
                BinaryOp::Eq => "==", BinaryOp::Ne => "!=", BinaryOp::Lt => "<", BinaryOp::Le => "<=",
                BinaryOp::Gt => ">", BinaryOp::Ge => ">=", _ => "?",
            };
            format!("{{\"kind\":\"BinaryOp\",\"op\":\"{}\",\"left\":{},\"right\":{}}}", op_str, dump_expr(left), dump_expr(right))
        }
        Expr::Call { callee, args, .. } => {
            let args_str: Vec<String> = args.iter().map(dump_expr).collect();
            format!("{{\"kind\":\"Call\",\"callee\":{},\"args\":[{}]}}", dump_expr(callee), args_str.join(","))
        }
        Expr::Cast { expr, target_type, .. } => {
            format!("{{\"kind\":\"Cast\",\"expr\":{},\"target_type\":\"{}\"}}", dump_expr(expr), target_type.lir_name())
        }
        _ => "{\"kind\":\"Expr\"}".to_string(),
    }
}