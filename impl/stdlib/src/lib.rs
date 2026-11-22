//! Lingua Omnibus Standard Library (written in LO, SPEC-10).
//!
//! Modules: core, io, db, quantum.

// std.core — primitive operations
pub fn print(msg: &str) {
    println!("{}", msg);
}

pub fn panic(msg: &str) {
    eprintln!("PANIC: {}", msg);
    std::process::abort();
}

pub fn assert(cond: bool, msg: &str) {
    if !cond {
        panic(msg);
    }
}

// std.io — input/output
pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn write_file(path: &str, content: &str) {
    let _ = std::fs::write(path, content);
}

// std.db — LinguaDB client (requires KW_DATABASE effect)
pub struct DbClient {
    pub connected: bool,
}

impl DbClient {
    pub fn new() -> Self {
        DbClient { connected: true }
    }

    pub fn create_table(&self, name: &str) {
        println!("CREATE TABLE {}", name);
    }

    pub fn insert(&self, table: &str, values: &str) {
        println!("INSERT INTO {} VALUES ({})", table, values);
    }

    pub fn select(&self, table: &str, where_clause: &str) -> String {
        format!("SELECT * FROM {} WHERE {}", table, where_clause)
    }
}

// std.quantum — quantum operations (requires KW_QUANTUM effect)
pub fn alloc_qubit() -> usize {
    0 // Stub: returns handle 0
}

pub fn measure(qubit: usize) -> i32 {
    0 // Stub: always returns 0
}

pub fn h_gate(qubit: usize) {
    // Hadamard gate stub
}

pub fn cnot_gate(q1: usize, q2: usize) {
    // CNOT gate stub
}