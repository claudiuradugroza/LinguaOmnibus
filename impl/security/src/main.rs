use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: security <command> [args...]");
        eprintln!("Commands: sandbox, capabilities, check, version");
        std::process::exit(1);
    }
    match args[1].as_str() {
        "version" => println!("security 0.1.0"),
        "sandbox" => {
            println!("Sandbox: default-deny (SEC-002)");
            println!("Capabilities: io, database, network, filesystem, quantum");
            println!("Default: all denied");
            println!("Status: functional (basic)");
        }
        "capabilities" => {
            println!("Capability         Required by    Default");
            println!("io                 KW_IO          denied");
            println!("database           KW_DATABASE    denied");
            println!("network            KW_NETWORK     denied");
            println!("filesystem         KW_FILESYSTEM  denied");
            println!("quantum            KW_QUANTUM     denied");
        }
        "check" => {
            // Check capabilities from lopm.toml
            let manifest = std::fs::read_to_string("lopm.toml").unwrap_or_default();
            let checker = CapabilityChecker::from_manifest(&manifest);
            println!("Capability check:");
            for cap in &["io", "database", "network", "filesystem", "quantum"] {
                println!("  {}: {}", cap, if checker.check(cap) { "GRANTED" } else { "DENIED" });
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}

/// Capability checker (SPEC-11).
pub struct CapabilityChecker {
    pub io: bool,
    pub database: bool,
    pub network: bool,
    pub filesystem: bool,
    pub quantum: bool,
}

impl CapabilityChecker {
    pub fn new() -> Self {
        CapabilityChecker {
            io: false,
            database: false,
            network: false,
            filesystem: false,
            quantum: false,
        }
    }

    pub fn from_manifest(manifest: &str) -> Self {
        let mut checker = Self::new();
        let mut in_caps = false;
        for line in manifest.lines() {
            let line = line.trim();
            if line.starts_with("[capabilities]") {
                in_caps = true;
                continue;
            }
            if line.starts_with('[') {
                in_caps = false;
                continue;
            }
            if in_caps {
                if let Some((key, val)) = line.split_once('=') {
                    let key = key.trim();
                    let val = val.trim().trim_matches('"');
                    let granted = val == "true";
                    match key {
                        "io" => checker.io = granted,
                        "database" => checker.database = granted,
                        "network" => checker.network = granted,
                        "filesystem" => checker.filesystem = granted,
                        "quantum" => checker.quantum = granted,
                        _ => {}
                    }
                }
            }
        }
        checker
    }

    pub fn check(&self, capability: &str) -> bool {
        match capability {
            "io" => self.io,
            "database" => self.database,
            "network" => self.network,
            "filesystem" => self.filesystem,
            "quantum" => self.quantum,
            _ => false,
        }
    }

    pub fn require(&self, capability: &str) -> Result<(), String> {
        if self.check(capability) {
            Ok(())
        } else {
            Err(format!("Missing capability: {}", capability))
        }
    }
}