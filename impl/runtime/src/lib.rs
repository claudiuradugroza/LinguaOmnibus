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

/// Quantum simulation stub — allocates a qubit in |0⟩ state.
#[no_mangle]
pub extern "C" fn lo_qubit_alloc() -> usize {
    // Returns a handle to a simulated qubit
    0
}

/// Quantum simulation stub — measures a qubit, returns 0 or 1.
#[no_mangle]
pub extern "C" fn lo_qubit_measure(_qubit: usize) -> i32 {
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