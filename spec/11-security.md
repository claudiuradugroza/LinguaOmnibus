SPEC-11: Security

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1), spec/04-memory.md (spec-0.1), spec/05-effects.md (spec-0.1), spec/09-toolchain.md (spec-0.1)
Applies to: every loc implementation, every lopm implementation, every runtime, every language pack

1. Motivation

Security is enforced through sandboxing, capabilities, and a threat model. Lingua Omnibus must run untrusted code safely. This document defines:

    the sandbox model (default-deny),

    capabilities (declared permissions required by effects),

    the threat model (what we protect against, what we don't).

2. Normative specification

SEC-001. Capabilities are declared permissions required by effects (reifies the capability glossary term from SPEC-00 §3.3; see SPEC-05 EFF-040…042).

SEC-002. The sandbox MUST restrict untrusted code by default: no file system, no network, no database access unless the capability is granted.

SEC-003. Capabilities are granted via lopm.toml package manifest or command-line flag. A program without the filesystem capability MUST NOT open files.

SEC-004. Quantum operations require the quantum capability. A program without it MUST NOT allocate qubits.

SEC-005. The sandbox boundary is the process: a LO program runs in a single process with restricted syscalls.

SEC-006. lopm MUST verify package signatures (when provided) before installation. Unsigned packages MAY be installed with a warning.

SEC-007. Telemetry MUST be disabled by default (Principle 7). No data is sent without explicit, audited opt-in.

SEC-008. The runtime MUST NOT execute arbitrary code from untrusted sources (no eval, no dynamic code generation in spec-0.1).

3. Formal grammar or tables

Table — capabilities and corresponding effects:

| Capability | Required by effect | Default |
|------------|-------------------|---------|
| io | KW_IO | denied |
| database | KW_DATABASE | denied |
| network | KW_NETWORK | denied |
| filesystem | KW_FILESYSTEM | denied |
| quantum | KW_QUANTUM | denied |

4. Examples

4.1 Capability declaration in lopm.toml

[capabilities]
io = true
filesystem = false
network = false
quantum = false

4.2 Program requiring filesystem capability

#lingua: EN
function read_file() with [FILESYSTEM] returns String:
    # requires filesystem capability
    return std.io.read_file("config.txt")

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: SEC-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| SEC-001 … SEC-008 | conformance/sec/001/security.lo |

6. Interactions with other sections

SPEC-00: SEC rules reify the SEC prefix (CONV-020); CONV-034 declaration may include capability profile.

SPEC-05: Effects require capabilities (EFF-040…042); SEC enforces them.

SPEC-04: Panic unwinding and drop must not leak capabilities (MEM-043).

SPEC-09: lopm enforces capabilities at install time (SEC-003, SEC-006).

7. Open questions

Q1. Capability granularity. Are capabilities per-package or per-function? Proposal: per-package in lopm.toml, per-function in source.

Q2. Sandbox implementation. seccomp-bpf on Linux, AppContainer on Windows, sandbox-exec on macOS? Proposal: abstract sandbox interface; per-OS backends.

Q3. Package signing. What key format? Proposal: Ed25519 signatures.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. Defines sandbox, capabilities, threat model. |

End of SPEC-11 (spec-0.1).