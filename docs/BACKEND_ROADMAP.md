# Lingua Omnibus — Backend Roadmap

## Strategy: 3 Phases

### Phase 1: C-transpiler (MVP) ✅ Complete
**Status:** Functional

The C backend transpiles LIR to portable C99 code, which is then compiled by clang/gcc to native x86_64 binaries.

- **Backend:** `impl/loc/src/backend/c.rs`
- **Targets:** x86_64 (via C)
- **Verified:** Hello World, Fibonacci(10) = 55
- **Advantages:** Bootstrappable, reproducible, zero new dependencies
- **Limitations:** No optimization beyond what gcc/clang provides

### Phase 2: Native x86_64 (current)
**Status:** Functional

Direct x86_64 assembly generation from LIR, bypassing C intermediate.

- **Backend:** `impl/loc/src/backend/x86_64.rs`
- **Targets:** x86_64 (native assembly)
- **Verified:** Hello World, Fibonacci(10) = 55
- **Advantages:** No C compiler dependency, direct control over codegen
- **Limitations:** No optimization, no register allocation (uses stack slots)

### Phase 3: LLVM (v0.5+)
**Status:** Planned

LIR → LLVM IR → LLVM → native code for all architectures.

- **Backend:** `impl/loc/src/backend/llvm.rs` (planned)
- **Targets:** x86_64, ARM64, RISC-V, PowerPC, WASM
- **Advantages:** Real optimizations, PGO, inlining, multi-target
- **Dependencies:** LLVM 15+, inkwell or llvm-sys
- **Timeline:** After Phase 1+2 are stable and tested

### Phase 4: Multi-arch + Quantum (v1.0+)
**Status:** Planned

- **ARM64:** Native codegen + QEMU user-mode testing
- **RISC-V:** Native codegen + QEMU user-mode testing
- **PowerPC:** Native codegen + QEMU user-mode testing
- **WASM:** WebAssembly 2.0 codegen + wasmtime testing
- **OpenQASM 3.0:** Quantum circuit codegen (already functional via qasm3 backend)
- **IBM Quantum:** Optional integration

## Testing Strategy

### QEMU User-Mode (for ARM64, RISC-V, PowerPC)
```
ARM64:   qemu-aarch64-static ./binary
RISC-V:  qemu-riscv64-static ./binary
PowerPC: qemu-ppc64-static ./binary
```

CI workflows: `.github/workflows/backend-arm64.yml`, `backend-riscv.yml`, `backend-powerpc.yml`

### WASM Runtime (wasmtime)
```
wasmtime run hello.wasm
```

CI workflow: `.github/workflows/backend-wasm.yml`

### LLVM (llvm-lit)
```
opt -verify < ir.ll
llc -verify-machineinstrs < ir.ll
```

## Why C first, then native, then LLVM

1. **C validates LIR**: If LIR → C produces correct programs, the LIR is correct. With LLVM, bugs in LIR and codegen are indistinguishable.
2. **Bootstrappable (Principle 4)**: TCC + C-transpiler = complete bootstrap in ~3MB. LLVM cannot be bootstrapped without LLVM.
3. **Reproducible (Principle 3)**: gcc/clang produce deterministic binaries. Tested for 20+ years.
4. **Time to first binary**: 2 weeks (C) vs 2 months (LLVM).
5. **Native x86_64**: Validates that we can generate machine code directly, without C dependency.
6. **LLVM comes last**: Provides optimization, not correctness. Optimization can come after correctness is proven.

## Current Backend Status

| Backend | Status | Targets | Verified |
|---------|--------|---------|----------|
| C (Phase 1) | ✅ Functional | x86_64 (via C) | Hello World, Fibonacci |
| x86_64 (Phase 2) | ✅ Functional | x86_64 (native) | Hello World, Fibonacci |
| OpenQASM 3.0 | ✅ Functional | Quantum | Bell circuit, Hadamard |
| WASM | ⚠️ Stub | wasm32 | WAT emitter only |
| LLVM | 📋 Planned | All | Not started |
| ARM64 | 📋 Planned | ARM64 | QEMU CI ready |
| RISC-V | 📋 Planned | RISC-V | QEMU CI ready |
| PowerPC | 📋 Planned | PowerPC | QEMU CI ready |

## Roadmap

| Phase | Backend | Milestone | Status |
|-------|---------|-----------|--------|
| 1 | C-transpiler | M2 (Hello World runs) | ✅ Complete |
| 2 | x86_64 native | M2 (no C dependency) | ✅ Complete |
| 3 | LLVM | v0.5 (optimizations) | 📋 Planned |
| 4 | Multi-arch + Quantum | v1.0 (all targets) | 📋 Planned |