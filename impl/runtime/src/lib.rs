//! Lingua Omnibus minimal runtime.
//!
//! Provides panic handler, GC (optional), target-specific assembly,
//! and quantum simulation stubs.
//! See SPEC-04 (Memory Model) and SPEC-05 (Effects & Concurrency).

use std::alloc::{self, Layout};
use std::ptr;

/// Panic handler — aborts the program with an error message.
#[no_mangle]
pub extern "C" fn lo_panic(msg: *const u8, len: usize) {
    let message = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(msg, len))
    };
    eprintln!("PANIC: {}", message);
    std::process::abort();
}

/// Minimal allocation tracker (GC stub).
pub struct AllocTracker {
    pub allocations: usize,
    pub total_bytes: usize,
}

impl AllocTracker {
    pub fn new() -> Self {
        AllocTracker { allocations: 0, total_bytes: 0 }
    }

    pub fn track(&mut self, size: usize) {
        self.allocations += 1;
        self.total_bytes += size;
    }
}

/// Stack overflow guard (MEM-033).
pub fn check_stack_depth(depth: usize) -> bool {
    depth < 1024 * 1024 // 1 MiB default stack limit
}

/// Heap allocation (Box<T> backend).
pub unsafe fn lo_alloc(size: usize, align: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, align).unwrap_or_else(|_| {
        eprintln!("PANIC: invalid allocation layout");
        std::process::abort();
    });
    let ptr = alloc::alloc(layout);
    if ptr.is_null() {
        eprintln!("PANIC: out of memory");
        std::process::abort();
    }
    ptr
}

/// Heap deallocation.
pub unsafe fn lo_dealloc(ptr: *mut u8, size: usize, align: usize) {
    let layout = Layout::from_size_align(size, align).unwrap_or_else(|_| {
        eprintln!("PANIC: invalid deallocation layout");
        std::process::abort();
    });
    alloc::dealloc(ptr, layout);
}

// ─────────────────────────────────────────────────────────────
// Hybrid Quantum Runtime
// ─────────────────────────────────────────────────────────────

/// Quantum state vector simulator (hybrid classical-quantum).
/// Simulates up to 20 qubits (2^20 = 1M complex amplitudes).
pub struct QuantumState {
    pub num_qubits: usize,
    pub amplitudes: Vec<(f64, f64)>, // (real, imag) pairs
}

impl QuantumState {
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut amplitudes = vec![(0.0, 0.0); size];
        amplitudes[0] = (1.0, 0.0); // |0...0⟩
        QuantumState { num_qubits, amplitudes }
    }

    /// Apply Hadamard gate to qubit.
    pub fn h(&mut self, qubit: usize) {
        let n = self.num_qubits;
        let size = 1 << n;
        let mut new_amps = self.amplitudes.clone();
        for i in 0..size {
            if (i >> qubit) & 1 == 0 {
                let j = i | (1 << qubit);
                let (a_re, a_im) = self.amplitudes[i];
                let (b_re, b_im) = self.amplitudes[j];
                let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();
                new_amps[i] = ((a_re + b_re) * inv_sqrt2, (a_im + b_im) * inv_sqrt2);
                new_amps[j] = ((a_re - b_re) * inv_sqrt2, (a_im - b_im) * inv_sqrt2);
            }
        }
        self.amplitudes = new_amps;
    }

    /// Apply Pauli-X gate to qubit.
    pub fn x(&mut self, qubit: usize) {
        let size = 1 << self.num_qubits;
        for i in 0..size {
            if (i >> qubit) & 1 == 0 {
                let j = i | (1 << qubit);
                self.amplitudes.swap(i, j);
            }
        }
    }

    /// Apply Pauli-Y gate to qubit.
    pub fn y(&mut self, qubit: usize) {
        let size = 1 << self.num_qubits;
        let mut new_amps = self.amplitudes.clone();
        for i in 0..size {
            if (i >> qubit) & 1 == 0 {
                let j = i | (1 << qubit);
                let (a_re, a_im) = self.amplitudes[i];
                let (b_re, b_im) = self.amplitudes[j];
                // Y = [[0, -i], [i, 0]]
                new_amps[i] = (b_im, -b_re);
                new_amps[j] = (-a_im, a_re);
            }
        }
        self.amplitudes = new_amps;
    }

    /// Apply Pauli-Z gate to qubit.
    pub fn z(&mut self, qubit: usize) {
        let size = 1 << self.num_qubits;
        for i in 0..size {
            if (i >> qubit) & 1 == 1 {
                let (re, im) = self.amplitudes[i];
                self.amplitudes[i] = (-re, -im);
            }
        }
    }

    /// Apply CNOT gate (control, target).
    pub fn cnot(&mut self, control: usize, target: usize) {
        let size = 1 << self.num_qubits;
        let mut new_amps = self.amplitudes.clone();
        for i in 0..size {
            if (i >> control) & 1 == 1 {
                let j = i ^ (1 << target);
                new_amps[j] = self.amplitudes[i];
            }
        }
        // Need to handle the case where control=0 (identity)
        for i in 0..size {
            if (i >> control) & 1 == 0 {
                new_amps[i] = self.amplitudes[i];
            }
        }
        self.amplitudes = new_amps;
    }

    /// Measure a qubit, collapsing the state. Returns 0 or 1.
    pub fn measure(&mut self, qubit: usize) -> i32 {
        let size = 1 << self.num_qubits;
        // Calculate probability of measuring 0
        let mut prob_0 = 0.0;
        for i in 0..size {
            if (i >> qubit) & 1 == 0 {
                let (re, im) = self.amplitudes[i];
                prob_0 += re * re + im * im;
            }
        }
        // For deterministic simulation, use a simple threshold
        let result = if prob_0 > 0.5 { 0 } else { 1 };

        // Collapse state
        let norm = if result == 0 {
            1.0 / prob_0.sqrt()
        } else {
            1.0 / (1.0 - prob_0).sqrt()
        };
        for i in 0..size {
            if (i >> qubit) & 1 == result as usize {
                let (re, im) = self.amplitudes[i];
                self.amplitudes[i] = (re * norm, im * norm);
            } else {
                self.amplitudes[i] = (0.0, 0.0);
            }
        }
        result
    }

    /// Get probability of measuring |0⟩ for a qubit.
    pub fn prob_zero(&self, qubit: usize) -> f64 {
        let size = 1 << self.num_qubits;
        let mut prob = 0.0;
        for i in 0..size {
            if (i >> qubit) & 1 == 0 {
                let (re, im) = self.amplitudes[i];
                prob += re * re + im * im;
            }
        }
        prob
    }
}

/// Quantum simulation stub — allocates a qubit in |0⟩ state.
#[no_mangle]
pub extern "C" fn lo_qubit_alloc() -> usize {
    0 // Returns handle 0
}

/// Quantum simulation stub — measures a qubit, returns 0 or 1.
#[no_mangle]
pub extern "C" fn lo_qubit_measure(qubit: usize) -> i32 {
    0 // Stub: always returns 0
}

/// GC: mark-and-sweep (optional, MEM-050).
pub struct GcHeap {
    pub objects: Vec<*mut u8>,
    pub sizes: Vec<usize>,
    pub aligns: Vec<usize>,
    pub marked: Vec<bool>,
}

impl GcHeap {
    pub fn new() -> Self {
        GcHeap { objects: Vec::new(), sizes: Vec::new(), aligns: Vec::new(), marked: Vec::new() }
    }

    pub fn alloc(&mut self, size: usize, align: usize) -> *mut u8 {
        let ptr = unsafe { lo_alloc(size, align) };
        self.objects.push(ptr);
        self.sizes.push(size);
        self.aligns.push(align);
        self.marked.push(false);
        ptr
    }

    pub fn mark(&mut self, idx: usize) {
        if idx < self.marked.len() {
            self.marked[idx] = true;
        }
    }

    pub fn sweep(&mut self) {
        let mut i = 0;
        while i < self.objects.len() {
            if !self.marked[i] {
                unsafe { lo_dealloc(self.objects[i], self.sizes[i], self.aligns[i]) };
                self.objects.remove(i);
                self.sizes.remove(i);
                self.aligns.remove(i);
                self.marked.remove(i);
            } else {
                self.marked[i] = false;
                i += 1;
            }
        }
    }
}