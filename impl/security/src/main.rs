use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: security <command> [args...]");
        eprintln!("Commands: sandbox, capabilities, version");
        std::process::exit(1);
    }
    match args[1].as_str() {
        "version" => println!("security 0.1.0"),
        "sandbox" => {
            println!("Sandbox: default-deny (SEC-002)");
            println!("Capabilities: io, database, network, filesystem, quantum");
            println!("Status: stub — full sandbox deferred to spec-1.0");
        }
        "capabilities" => {
            println!("Capability         Required by    Default");
            println!("io                 KW_IO          denied");
            println!("database           KW_DATABASE    denied");
            println!("network            KW_NETWORK     denied");
            println!("filesystem         KW_FILESYSTEM  denied");
            println!("quantum            KW_QUANTUM     denied");
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
        // Parse lopm.toml [capabilities] section
        let mut checker = Self::new();
        for line in manifest.lines() {
            let line = line.trim();
            if line.starts_with("io ") || line.starts_with("io=") {
                checker.io = line.contains("true");
            } else if line.starts_with("database ") || line.starts_with("database=") {
                checker.database = line.contains("true");
            } else if line.starts_with("network ") || line.starts_with("network=") {
                checker.network = line.contains("true");
            } else if line.starts_with("filesystem ") || line.starts_with("filesystem=") {
                checker.filesystem = line.contains("true");
            } else if line.starts_with("quantum ") || line.starts_with("quantum=") {
                checker.quantum = line.contains("true");
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
}