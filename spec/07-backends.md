SPEC-07: Backends

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1), spec/02-grammar.md (spec-0.1), spec/03-types.md (spec-0.1), spec/04-memory.md (spec-0.1), spec/05-effects.md (spec-0.1), spec/06-lir.md (spec-0.1)
Applies to: every backend implementation (LLVM, WASM, QASM), every loc implementation

1. Motivation

Backends consume LIR (SPEC-06) and produce target-specific machine code or intermediate representation. Supported targets: x86_64, ARM64, RISC-V, PowerPC, WebAssembly, and quantum backends (OpenQASM 3.0). This document defines:

    the backend contract (consume LIR, produce target output),

    target triple and ABI specification,

    quantum backend (quantum { } → OpenQASM 3.0),

    backend-independent requirements (bit-identical output per CONV-042).

2. Normative specification

BACK-001. Each backend MUST consume LIR (text or binary per SPEC-06) and produce target-specific output.

BACK-002. The supported target triples are: x86_64-unknown-linux, x86_64-pc-windows, aarch64-unknown-linux, riscv64-unknown-linux, powerpc64-unknown-linux, wasm32-unknown-unknown.

BACK-003. The ABI for each target MUST follow the platform's standard ABI (System V AMD64 ABI for x86_64, AAPCS64 for ARM64, etc.).

BACK-004. Quantum { } blocks MUST compile to OpenQASM 3.0.

BACK-005. Backend output MUST be deterministic: same LIR input → same target output (CONV-042 reproducibility).

BACK-006. A backend MAY declare itself as "unsupported" for a target (e.g. a compiler without quantum support declares quantum: unsupported per CONV-040). It MUST NOT silently accept LIR it cannot honour.

BACK-007. WebAssembly backend MUST produce valid wasm32 modules conforming to the WebAssembly 2.0 specification.

BACK-008. LLVM backend MUST target LLVM IR version 15 or later.

3. Formal grammar or tables

Table — target triples and ABIs:

| Target triple | Architecture | OS | ABI |
|---------------|-------------|-----|-----|
| x86_64-unknown-linux | x86_64 | Linux | System V AMD64 ABI |
| x86_64-pc-windows | x86_64 | Windows | Microsoft x64 calling convention |
| aarch64-unknown-linux | ARM64 | Linux | AAPCS64 |
| riscv64-unknown-linux | RISC-V | Linux | RISC-V ELF ABI |
| powerpc64-unknown-linux | PowerPC | Linux | PowerPC ELF ABI v2 |
| wasm32-unknown-unknown | WebAssembly | none | WebAssembly ABI |

Table — quantum backends:

| Backend | Output format | Status |
|---------|--------------|--------|
| OpenQASM 3.0 | .qasm | spec-0.1 |
| IBM Quantum | optional | spec-0.1 (optional integration) |

4. Examples

4.1 LIR → x86_64 assembly

lir 1.2.0
module hello {
  fn main : () -> unit {
  entry:
    %msg = const.str "Hello, world!"
    call @std.io.print(%msg)
    ret unit
  }
}

Produces x86_64 System V ABI assembly calling std.io.print.

4.2 quantum { } → OpenQASM 3.0

#lingua: EN
quantum:
    var q = alloc_qubit()
    var r = measure(q)

Compiles to OpenQASM 3.0:

qubit[1] q;
q[0] = measure(q[0]);

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: BACK-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| BACK-001 … BACK-008 | conformance/back/001/backends.lo |

6. Interactions with other sections

SPEC-00: BACK rules reify the BACK prefix (CONV-020).

SPEC-06 (LIR): BACK-001 consumes LIR; LIR byte-identical output (CONV-042) requires backend determinism.

SPEC-04 (MEM): Memory model per target (stack size, heap layout) — MEM-033.

SPEC-05 (EFFECTS): Quantum backend implements KW_QUANTUM effect.

SPEC-09 (TOOL): loc selects backend based on target triple.

7. Open questions

Q1. Cranelift vs LLVM. Should we use Cranelift (faster compile) instead of LLVM for the default backend? Proposal: LLVM for spec-0.1; Cranelift as optional in spec-1.0.

Q2. Multiple quantum backends. Should we support Braket, Q#, etc.? Proposal: OpenQASM 3.0 is the interchange format; vendor backends are optional.

Q3. WASM garbage collection. Should the WASM backend use the proposed GC proposal? Proposal: no GC in spec-0.1; manual memory management per SPEC-04.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. Defines backend contract, target triples, ABIs, quantum backend (OpenQASM 3.0), WASM/LLVM requirements. |

End of SPEC-07 (spec-0.1).