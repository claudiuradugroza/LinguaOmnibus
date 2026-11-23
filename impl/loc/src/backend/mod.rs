//! Backend module for Lingua Omnibus.
//!
//! Provides the Backend trait and concrete backend implementations.
//! Phase 1: C-transpiler (see RFC-0003).

use std::io;

/// Backend error types.
#[derive(Debug)]
pub enum BackendError {
    /// An LIR instruction is not supported by this backend.
    UnsupportedInstruction(String),
    /// I/O error.
    IoError(io::Error),
    /// Linker/compiler error.
    LinkerError(String),
    /// Parse error in LIR.
    ParseError(String),
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendError::UnsupportedInstruction(s) => write!(f, "Unsupported instruction: {}", s),
            BackendError::IoError(e) => write!(f, "IO error: {}", e),
            BackendError::LinkerError(s) => write!(f, "Linker error: {}", s),
            BackendError::ParseError(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl std::error::Error for BackendError {}

impl From<io::Error> for BackendError {
    fn from(e: io::Error) -> Self {
        BackendError::IoError(e)
    }
}

/// Backend trait for code generation.
pub trait Backend {
    fn name(&self) -> &'static str;
    fn emit(&self, lir_source: &str) -> Result<Vec<u8>, BackendError>;
}

/// Select a backend by name.
pub fn select_backend(name: &str) -> Box<dyn Backend> {
    match name {
        "c" | "x86_64-via-c" => Box::new(c::CBackend::new()),
        _ => panic!("Unknown backend: {}", name),
    }
}

pub mod c;