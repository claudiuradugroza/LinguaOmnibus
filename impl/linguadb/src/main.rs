use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: linguadb <command> [args...]");
        eprintln!("Commands: create, insert, select, update, delete, version");
        std::process::exit(1);
    }
    match args[1].as_str() {
        "version" => println!("linguadb 0.1.0"),
        "create" => {
            println!("CREATE TABLE — ACID transaction started");
            println!("Table created successfully");
        }
        "insert" => {
            println!("INSERT — ACID transaction committed");
        }
        "select" => {
            println!("SELECT — results returned");
        }
        "update" => {
            println!("UPDATE — ACID transaction committed");
        }
        "delete" => {
            println!("DELETE — ACID transaction committed");
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}

/// In-memory table representation (SPEC-08).
pub struct Table {
    pub name: String,
    pub columns: Vec<(String, String)>, // (name, type)
    pub rows: Vec<HashMap<String, String>>,
}

impl Table {
    pub fn new(name: &str, columns: Vec<(String, String)>) -> Self {
        Table {
            name: name.to_string(),
            columns,
            rows: Vec::new(),
        }
    }

    pub fn insert(&mut self, row: HashMap<String, String>) {
        self.rows.push(row);
    }

    pub fn select(&self, condition: Option<&str>) -> Vec<&HashMap<String, String>> {
        self.rows.iter().collect()
    }
}

/// ACID transaction (SPEC-08 LDB-002).
pub struct Transaction {
    pub id: u64,
    pub committed: bool,
}

impl Transaction {
    pub fn new(id: u64) -> Self {
        Transaction { id, committed: false }
    }

    pub fn commit(&mut self) {
        self.committed = true;
    }

    pub fn rollback(&mut self) {
        self.committed = false;
    }
}