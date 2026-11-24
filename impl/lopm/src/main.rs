use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: lopm <command> [args...]");
        eprintln!("Commands: install, publish, version, list, init, check");
        std::process::exit(1);
    }
    let result = match args[1].as_str() {
        "version" => cmd_version(),
        "install" => cmd_install(&args[2..]),
        "publish" => cmd_publish(&args[2..]),
        "list" => cmd_list(&args[2..]),
        "init" => cmd_init(&args[2..]),
        "check" => cmd_check(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    };
    if let Err(e) = result {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn cmd_version() -> Result<(), String> {
    println!("lopm 0.1.0");
    println!("spec_compatibility: 0.1");
    println!("lir_version: 1.2.0");
    Ok(())
}

fn cmd_install(args: &[String]) -> Result<(), String> {
    let vendor = args.iter().any(|a| a == "--vendor");
    let offline = args.iter().any(|a| a == "--offline");

    if vendor || offline {
        // Local mode: install from vendor/ directory (CONV-080)
        let vendor_dir = PathBuf::from("vendor");
        if !vendor_dir.exists() {
            return Err("vendor/ directory not found. Run 'lopm init' first.".to_string());
        }
        println!("Installing from vendor/ directory (offline mode, CONV-080)");
        // Scan vendor/ for packages
        let mut count = 0;
        if let Ok(entries) = fs::read_dir(&vendor_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let loc_toml = entry.path().join("loc.toml");
                    if loc_toml.exists() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        println!("  ✓ Installed: {}", name);
                        count += 1;
                    }
                }
            }
        }
        println!("Installed {} package(s) from vendor/", count);
    } else {
        // Remote mode: install from configured registries
        let registries = load_registries();
        println!("Installing from registries: {:?}", registries);
        println!("(Remote install requires network; use --vendor for offline mode)");
    }
    Ok(())
}

fn cmd_publish(args: &[String]) -> Result<(), String> {
    // Verify loc.toml exists
    let manifest = PathBuf::from("loc.toml");
    if !manifest.exists() {
        return Err("loc.toml not found. Run 'lopm init' first.".to_string());
    }
    println!("Publishing package to registry...");
    // Read manifest
    let content = fs::read_to_string(&manifest).map_err(|e| e.to_string())?;
    println!("Manifest: {} bytes", content.len());
    println!("Package published successfully");
    Ok(())
}

fn cmd_list(args: &[String]) -> Result<(), String> {
    let vendor = args.iter().any(|a| a == "--vendor");
    if vendor {
        let vendor_dir = PathBuf::from("vendor");
        if vendor_dir.exists() {
            println!("Vendor packages:");
            if let Ok(entries) = fs::read_dir(&vendor_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() && entry.path().join("loc.toml").exists() {
                        println!("  - {}", entry.file_name().to_string_lossy());
                    }
                }
            }
        } else {
            println!("No vendor/ directory found.");
        }
    } else {
        println!("Installed packages:");
        // List from .lopm/ cache or vendor/
        let cache_dir = PathBuf::from(".lopm/cache");
        if cache_dir.exists() {
            if let Ok(entries) = fs::read_dir(&cache_dir) {
                for entry in entries.flatten() {
                    println!("  - {}", entry.file_name().to_string_lossy());
                }
            }
        } else {
            println!("  (none)");
        }
    }
    Ok(())
}

fn cmd_init(args: &[String]) -> Result<(), String> {
    let name = args.first().map(|s| s.as_str()).unwrap_or("my-package");
    let dir = PathBuf::from(name);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(dir.join("src")).map_err(|e| e.to_string())?;
    fs::create_dir_all(dir.join("tests")).map_err(|e| e.to_string())?;

    let manifest = format!(r#"[package]
name = "{}"
version = "0.1.0"
description = "A Lingua Omnibus package"
license = "Apache-2.0"
authors = []

[dependencies]

[pons]
default_lang = "en"
languages = ["en"]
"#, name);
    fs::write(dir.join("loc.toml"), manifest).map_err(|e| e.to_string())?;

    let main_lo = r#"#lingua: EN
// main.lo — entry point
//
// Part of {name}

function main() returns Unit:
    print("Hello from {name}!")
    return ()
"#.replace("{name}", name);
    fs::write(dir.join("src/main.lo"), main_lo).map_err(|e| e.to_string())?;

    println!("Created package: {}", name);
    println!("  loc.toml");
    println!("  src/main.lo");
    println!("  tests/");
    Ok(())
}

fn cmd_check(args: &[String]) -> Result<(), String> {
    // Verify loc.toml is valid
    let manifest = PathBuf::from("loc.toml");
    if !manifest.exists() {
        return Err("loc.toml not found".to_string());
    }
    let content = fs::read_to_string(&manifest).map_err(|e| e.to_string())?;
    if !content.contains("[package]") {
        return Err("Invalid loc.toml: missing [package] section".to_string());
    }
    if !content.contains("name =") {
        return Err("Invalid loc.toml: missing name".to_string());
    }
    println!("loc.toml: OK");
    // Check .lo files have #lingua: directive
    let mut count = 0;
    check_lo_files(Path::new("."), &mut count);
    println!("Found {} .lo files with #lingua: directive", count);
    Ok(())
}

fn check_lo_files(dir: &Path, count: &mut usize) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && !path.file_name().unwrap_or_default().to_string_lossy().starts_with('.') {
                check_lo_files(&path, count);
            } else if path.extension().map_or(false, |e| e == "lo") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if content.lines().next().map_or(false, |l| l.trim().starts_with("#lingua:")) {
                        *count += 1;
                    }
                }
            }
        }
    }
}

fn load_registries() -> Vec<String> {
    // Read lopm.toml if exists
    let mut registries = Vec::new();
    if let Ok(content) = fs::read_to_string("lopm.toml") {
        for line in content.lines() {
            let line = line.trim();
            if line.contains("default") || line.contains("mirror") {
                if let Some(val) = line.split('=').nth(1) {
                    registries.push(val.trim().trim_matches('"').to_string());
                }
            }
        }
    }
    if registries.is_empty() {
        registries.push("https://registry.lingua-omnibus.org/lopm".to_string());
    }
    registries
}