SPEC-10: Standard Library

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1), spec/03-types.md (spec-0.1), spec/09-toolchain.md (spec-0.1)
Applies to: every standard library implementation, every .lo source file importing std modules

1. Motivation

The standard library is written in LO itself (dogfooding). It provides core, io, db, and quantum modules. API stability follows SemVer for the language version. This document defines:

    the standard library modules and their APIs,

    the stability guarantee,

    the module system (compilation units per SPEC-09).

2. Normative specification

STD-001. The standard library MUST be written in LO.

STD-002. The standard library API MUST follow SemVer for the language version: MAJOR bumps break API, MINOR adds features, PATCH fixes bugs.

STD-003. The core module MUST provide: print, panic, assert, type introspection functions.

STD-004. The io module MUST provide: file read/write, stdin/stdout, network (requires KW_IO effect).

STD-005. The db module MUST provide: database client operations (requires KW_DATABASE effect, SPEC-08).

STD-006. The quantum module MUST provide: alloc_qubit, measure, quantum gates (requires KW_QUANTUM effect).

STD-007. Option<T> and Result<T, E> MUST be in the core module with standard combinators (map, and_then, unwrap, etc.).

STD-008. Every public function MUST have a doc-comment (/// per LEX-084) consumable by lodoc.

3. Formal grammar or tables

Table — standard library modules:

| Module | Purpose | Effects required |
|--------|---------|------------------|
| std.core | Primitive operations, Option, Result, print, panic | none |
| std.io | Input/output, file system | KW_IO |
| std.db | LinguaDB client | KW_DATABASE |
| std.quantum | Qubit allocation, measurement, gates | KW_QUANTUM |

4. Examples

4.1 Using std.io — EN and RO

#lingua: EN
import std.io
function main() returns Unit:
    std.io.print("Hello, world!")

#lingua: RO
import std.io
funcție main() returnează Unit:
    std.io.print("Hello, world!")

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: STD-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| STD-001 … STD-008 | conformance/std/001/stdlib.lo |

6. Interactions with other sections

SPEC-00: STD rules reify the STD prefix (CONV-020).

SPEC-03: Standard library types are SPEC-03 types.

SPEC-05: std.io, std.db, std.quantum require KW_IO, KW_DATABASE, KW_QUANTUM effects respectively.

SPEC-08: std.db is the client for LinguaDB.

SPEC-09: Modules are compilation units; lopm distributes std packages.

7. Open questions

Q1. Is std part of the compiler or a separate package? Proposal: std is shipped with the compiler but versioned independently.

Q2. Should std be usable without the standard library (no_std)? Proposal: defer to spec-1.0 for embedded targets.

Q3. Module path resolution. How does import resolve std.io → std/io.lo? Proposal: lopm-managed module search path.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. Defines std modules, stability guarantee, doc-comment requirement. |

End of SPEC-10 (spec-0.1).