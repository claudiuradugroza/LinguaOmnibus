SPEC-12: Compatibility & Versioning

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1), spec/02-grammar.md (spec-0.1), spec/03-types.md (spec-0.1), spec/04-memory.md (spec-0.1), spec/05-effects.md (spec-0.1), spec/06-lir.md (spec-0.1), spec/07-backends.md (spec-0.1), spec/08-linguadb.md (spec-0.1), spec/09-toolchain.md (spec-0.1), spec/10-stdlib.md (spec-0.1), spec/11-security.md (spec-0.1)
Applies to: every conforming implementation, every language pack, the LTS migration tool

1. Motivation

Compatibility and versioning ensure the LTS promise (Principle 13). Each major version ships with lo-migrate for automated migration. Deprecation follows CONV-050…CONV-052. This document defines:

    SemVer for the language (CONV-030),

    migration tool (lo-migrate),

    deprecation and removal rules,

    language pack deprecation (CONV-032 dates).

2. Normative specification

COMP-001. Each major version MUST ship with lo-migrate, a migration tool that automatically updates source code from version N to N+1.

COMP-002. lo-migrate MUST be able to migrate any conforming program from version N to version N+1 without semantic change (modulo deprecated features).

COMP-003. Deprecated rule identifiers MUST remain normative for at least one spec-MAJOR cycle (CONV-050, CONV-051).

COMP-004. A deprecated rule identifier MUST be removed only in a spec-MAJOR bump, documented in MIGRATION-{MAJOR}.md at the repository root (CONV-052, Principle 13).

COMP-005. MIGRATION-{MAJOR}.md MUST contain: list of removed identifiers, migration steps for each, and an example before/after.

COMP-006. Language pack deprecation: a pack more than 24 months old on a spec-STABLE release is deprecated; implementations MAY still accept it with a warning (Q4 in SPEC-00).

COMP-007. The conformance suite version follows the spec set version (spec-{MAJOR}.{MINOR}).

COMP-008. An implementation MUST declare which spec-{MAJOR}.{MINOR} it implements (CONV-034). Running a program requiring a newer spec version MUST be rejected.

3. Formal grammar or tables

Table — versioning scheme summary:

| Artefact | Scheme | Example |
|----------|--------|---------|
| Language | SemVer 2.0.0 | 1.4.0 |
| LIR format | SemVer 2.0.0 | 1.2.0 |
| Language pack | ISO 8601 date | 2026-01-01 |
| Spec set | spec-{MAJOR}.{MINOR} | spec-0.1 |
| Conformance suite | follows spec set | spec-0.1 |

4. Examples

4.1 Migration example

# Before (spec 0.1):
function main() returns Unit:
    IF true THEN:       # deprecated uppercase keyword
        pass

# After (spec 1.0, via lo-migrate):
function main() returns Unit:
    if true then:
        pass

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: COMP-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| COMP-001 … COMP-008 | conformance/comp/001/compat.lo |

6. Interactions with other sections

SPEC-00: COMP extends CONV-030…CONV-034 with migration rules; deprecation per CONV-050…052.

SPEC-09: lo-migrate is distributed via lopm.

SPEC-13: Governance governs deprecation proposals (RFC process).

7. Open questions

Q1. lo-migrate implementation language. Should lo-migrate be written in LO itself? Proposal: Rust for spec-0.1; LO in spec-1.0 (bootstrapping).

Q2. Migration guarantees. Can lo-migrate guarantee 100% semantic preservation? Proposal: best-effort with manual review for edge cases.

Q3. Language pack deprecation enforcement. Should compilers reject packs older than 24 months by default? Proposal: warning only, configurable.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. Defines SemVer, lo-migrate, deprecation/removal, language pack deprecation. |

End of SPEC-12 (spec-0.1).