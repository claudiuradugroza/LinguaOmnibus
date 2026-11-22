use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let repo_root = find_repo_root();
    let loc_bin = repo_root.join("impl/loc/target/release/loc.exe");
    if !loc_bin.exists() {
        // Try unix name
        let loc_bin2 = repo_root.join("impl/loc/target/release/loc");
        if loc_bin2.exists() {
            run_tests(&loc_bin2, &repo_root);
        } else {
            eprintln!("error: loc binary not found at {:?} or {:?}", loc_bin, loc_bin2);
            std::process::exit(1);
        }
    } else {
        run_tests(&loc_bin, &repo_root);
    }
}

fn find_repo_root() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_default();
    loop {
        if dir.join("langpacks").exists() && dir.join("spec").exists() {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    std::env::current_dir().unwrap_or_default()
}

fn run_tests(loc_bin: &Path, repo_root: &Path) {
    let conformance_dir = repo_root.join("tests/conformance");
    if !conformance_dir.exists() {
        eprintln!("error: tests/conformance/ not found at {:?}", conformance_dir);
        std::process::exit(1);
    }

    let mut total = 0u64;
    let mut passed = 0u64;
    let mut failed = 0u64;
    let mut failures: Vec<String> = Vec::new();

    // Collect all .lo files
    let mut test_files: Vec<PathBuf> = Vec::new();
    collect_lo_files(&conformance_dir, &mut test_files);
    test_files.sort();

    for tf in &test_files {
        total += 1;
        let relative = tf.strip_prefix(repo_root).unwrap_or(tf).to_string_lossy().to_string();

        let output = Command::new(loc_bin)
            .arg("check")
            .arg(tf)
            .output();

        match output {
            Ok(out) => {
                if out.status.success() {
                    passed += 1;
                    println!("PASS: {}", relative);
                } else {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    failed += 1;
                    failures.push(format!("FAIL: {} - {}", relative, stderr.trim()));
                    println!("FAIL: {} - {}", relative, stderr.trim());
                }
            }
            Err(e) => {
                failed += 1;
                failures.push(format!("ERROR: {} - {}", relative, e));
                println!("ERROR: {} - {}", relative, e);
            }
        }
    }

    println!("\n==============================");
    println!("Total: {}, Passed: {}, Failed: {}", total, passed, failed);
    if failed > 0 {
        println!("\nFailures:");
        for f in &failures {
            println!("  {}", f);
        }
        std::process::exit(1);
    }
}

fn collect_lo_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_lo_files(&path, out);
            } else if path.extension().map_or(false, |e| e == "lo") {
                out.push(path);
            }
        }
    }
}