SPEC-09: Toolchain

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1), spec/06-lir.md (spec-0.1), spec/08-linguadb.md (spec-0.1)
Applies to: loc, lopm, lolsp, lofmt, lodoc, lotest implementations

1. Motivation

The toolchain comprises six tools: loc (compiler), lopm (package manager), lolsp (language server), lofmt (formatter), lodoc (doc generator), lotest (test runner). Each is a sub-project with its own spec. This document defines:

    the compatibility declaration (CONV-034) exposed by all tools,

    lopm registry modes (local and remote, CONV-080),

    offline-first behaviour (Principle 6),

    CLI contracts for each tool.

2. Normative specification

TOOL-001. All tools MUST expose the compatibility declaration of CONV-034 in machine-readable version output (loc --version --json).

TOOL-002. lopm MUST support local mode (vendor/ directory or file:// path) with zero configuration. lopm install --vendor MUST work with no network (CONV-080).

TOOL-003. lopm MUST support remote mode (any HTTP(S) endpoint serving the standard registry layout). No mandatory "official" registry.

TOOL-004. lopm MUST allow arbitrary registries via lopm.toml:

[registries]
default = "https://example.com/lopm"
mirror  = "file:///srv/lopm-mirror"

TOOL-005. Tooling MUST function offline-first (Principle 6): all operations MUST work with vendor/ directory or local cache, no network required.

TOOL-006. loc MUST: build, check, run, emit LIR (loc emit-lir), dump AST (loc dump-ast --canonical per GRAM-054).

TOOL-007. lolsp MUST provide: autocomplete, goto definition, hover (type info), diagnostics (using LEX-013 error recovery, MEM/EFF error codes).

TOOL-008. lofmt MUST normalize source files: indentation (GRAM-045), decimal separator per pack (LEX-064), vertical tab → space (LEX-021).

TOOL-009. lodoc MUST generate documentation from doc-comment metadata (LEX-084).

TOOL-010. lotest MUST run conformance tests and unit tests, reporting pass/fail per rule identifier.

2.1 lopm.toml format

[registries]
default = "https://registry.example.com/lopm"
mirror  = "file:///srv/lopm-mirror"

[registries.policy]
fallback = ["default", "mirror"]
offline  = false

2.2 Tool contracts

Table — tool summary:

| Tool | Spec | Purpose | Language |
|------|------|---------|----------|
| loc | SPEC-09 | Compiler | Rust (bootstrap) → LO |
| lopm | SPEC-09 | Package manager | Rust |
| lolsp | SPEC-09 | Language server | Rust |
| lofmt | SPEC-09 | Formatter | Rust |
| lodoc | SPEC-09 | Doc generator | Rust |
| lotest | SPEC-09 | Test runner | Rust |
| runtime | SPEC-04/05 | Minimal runtime | Rust + asm per target |

3. Formal grammar or tables

lopm.toml = "[registries]" newline registry* policy? ;
registry   = ident "=" string newline ;
policy     = "[registries.policy]" newline "fallback" "=" array newline "offline" "=" bool newline ;

4. Examples

4.1 loc --version --json output

{
  "implementation": "loc",
  "implementation_version": "0.9.0",
  "spec_compatibility": "0.1",
  "bundled_langpacks": { "en": "2026-01-01", "ro": "2026-01-01" },
  "lir_version": "0.3.0",
  "conformance": {
    "conv": "pass",
    "lex": "pass",
    "gram": "pass",
    "type": "pass",
    "mem": "partial",
    "eff": "unsupported",
    "lir": "pass"
  }
}

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: TOOL-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| TOOL-001 … TOOL-005 | conformance/tool/001/toolchain.lo |
| TOOL-006 … TOOL-010 | conformance/tool/010/tool_contracts.lo |

6. Interactions with other sections

SPEC-00: TOOL rules reify the TOOL prefix (CONV-020); CONV-034 declaration format; CONV-080 federated registry.

SPEC-06 (LIR): loc emits LIR; loc emit-lir outputs LIR per SPEC-06.

SPEC-08 (LINGUADB): Database client operations in toolchain.

SPEC-10 (STDLIB): Modules are compilation units (SPEC-09).

7. Open questions

Q1. Plugin system for lolsp. Should lolsp support third-party plugins? Proposal: defer to spec-1.0.

Q2. Registry authentication. How does lopm authenticate to private registries? Proposal: token-based via lopm.toml [auth] section.

Q3. Cross-compilation. How does loc cross-compile for ARM64 from x86_64? Proposal: target triple flag --target.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. Defines toolchain tools, CONV-034 declaration, lopm registry modes, offline-first, CLI contracts. |

End of SPEC-09 (spec-0.1).