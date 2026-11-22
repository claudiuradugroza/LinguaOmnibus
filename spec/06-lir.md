SPEC-06: Intermediate Representation (LIR)

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1), spec/02-grammar.md (spec-0.1), spec/03-types.md (spec-0.1), spec/04-memory.md (spec-0.1), spec/05-effects.md (spec-0.1)
Applies to: every loc implementation, every backend implementation, every LIR consumer

1. Motivation

LIR (Lingua Intermediate Representation) is the universal semantic form produced by every language pack's compiler front end and consumed by every back end. LIR has its own version, independent of the language version (CONV-031). Two conforming implementations compiling the same program MUST produce byte-identical LIR (CONV-042). This document defines:

    LIR text format and binary format,

    the LIR instruction set,

    LIR versioning (independent SemVer),

    LIR identity requirements (CONV-042).

2. Normative specification

2.1 LIR versions

LIR-001. The LIR format MUST have its own SemVer version, independent of the language version (CONV-031).

LIR-002. The LIR file header MUST declare the LIR version: lir {MAJOR}.{MINOR}.{PATCH}.

LIR-003. The current LIR version is 1.2.0 (example in SPEC-00 §4.1).

2.2 LIR text format

LIR-010. LIR text files MUST be UTF-8 encoded without BOM (CONV-070).

LIR-011. A LIR module is declared as: module <name> { ... }.

LIR-012. Functions are declared as: fn <name> : (<params>) -> <ret> { ... }.

LIR-013. Instructions are in three-address code (SSA-like): %result = <op> %arg1, %arg2.

LIR-014. Basic blocks are labeled: entry:, block1:, etc.

LIR-015. Control flow: br (unconditional branch), br.cond (conditional branch), ret (return).

LIR-016. All values are SSA (single static assignment): a virtual register (%name) is assigned exactly once.

2.3 LIR instruction set

LIR-020. Arithmetic: add, sub, mul, div, rem for int and float types.

LIR-021. Comparison: eq, ne, lt, le, gt, ge.

LIR-022. Logic: and, or, xor, not, shl, shr.

LIR-023. Memory: alloc, load, store, copy, move, drop.

LIR-024. Control: call, ret, br, br.cond, switch.

LIR-025. Quantum: qubit.alloc, qubit.measure, qubit.gate.

LIR-026. Effects: effect.enter, effect.exit (annotate effect scope).

2.4 LIR identity (CONV-042)

LIR-030. Two conforming implementations compiling the same .lo program (same #lingua: semantics, same pack version, same spec_compatibility, same lir_version) MUST produce byte-identical LIR after optimization passes documented as optional in this spec.

LIR-031. Optional optimization passes: dead code elimination, constant folding, inlining. These MUST NOT change observable semantics.

LIR-032. The LIR version in the header MUST match the lir_version declared in CONV-034.

2.5 LIR binary format

LIR-040. The binary format is a straightforward encoding of the text format: magic number "LOIR", version, then a sequence of records (function, instruction, block).

LIR-041. The binary format MUST be losslessly convertible to/from the text format (round-trip guaranteed).

LIR-042. Binary LIR MUST NOT embed absolute addresses or implementation-specific data.

3. Formal grammar or tables

module     = "lir" version "module" ident "{" function* "}" ;
version    = DIGIT "." DIGIT "." DIGIT ;
function   = "fn" ident ":" "(" param* ")" "->" type "{" block* "}" ;
block      = ident ":" instruction* terminator ;
instruction  = "%" ident "=" opcode operand* ;
terminator   = "ret" operand | "br" label | "br.cond" operand label label | "switch" operand label+ ;

Table — LIR opcodes:

| Opcode | Operands | Semantics |
|--------|----------|-----------|
| add | %a, %b | integer/float addition |
| sub | %a, %b | subtraction |
| mul | %a, %b | multiplication |
| div | %a, %b | division |
| rem | %a, %b | remainder |
| eq | %a, %b | equality |
| ne | %a, %b | inequality |
| lt | %a, %b | less than |
| le | %a, %b | less than or equal |
| gt | %a, %b | greater than |
| ge | %a, %b | greater than or equal |
| and | %a, %b | bitwise/logical and |
| or | %a, %b | bitwise/logical or |
| xor | %a, %b | bitwise xor |
| not | %a | bitwise/not |
| shl | %a, %b | shift left |
| shr | %a, %b | shift right |
| load | %ptr | load from pointer |
| store | %ptr, %val | store to pointer |
| copy | %src | copy (Copy types) |
| move | %src | move (linear types) |
| drop | %val | drop value |
| call | %fn, %arg+ | function call |
| ret | %val | return value |
| br | %label | unconditional branch |
| br.cond | %cond, %label_true, %label_false | conditional branch |
| qubit.alloc | | allocate qubit |
| qubit.measure | %qubit | measure qubit → bool |
| qubit.gate | %qubit, %gate | apply quantum gate |
| effect.enter | %effect | enter effect scope |
| effect.exit | %effect | exit effect scope |

4. Examples

4.1 Hello world LIR (byte-identical for EN and RO)

lir 1.2.0
module hello {
  fn main : () -> unit {
  entry:
    %msg = const.str "Hello, world!"
    call @std.io.print(%msg)
    ret unit
  }
}

4.2 Add function LIR

lir 1.2.0
module math {
  fn add : (i32, i32) -> i32 {
  entry:
    %result = add %a, %b
    ret %result
  }
}

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: LIR-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| LIR-001 … LIR-003 | conformance/lir/001/version.lo |
| LIR-010 … LIR-016 | conformance/lir/010/text_format.lo |
| LIR-020 … LIR-026 | conformance/lir/020/instruction_set.lo |
| LIR-030 … LIR-032 | conformance/lir/030/identity.lo |
| LIR-040 … LIR-042 | conformance/lir/030/binary_format.lo |

6. Interactions with other sections

SPEC-00: LIR rules reify the LIR prefix (CONV-020); CONV-031 (independent version), CONV-034 (declared LIR version), CONV-042 (byte-identical LIR).

SPEC-01: LEX-078/079 (U+2028/U+2029, bidi escapes in LIR strings) are LIR requirements.

SPEC-02: GRAM-051 (lingua_meta stripped before LIR generation) is an LIR requirement.

SPEC-03: LIR type annotations MUST match SPEC-03 types exactly.

SPEC-04: LIR must encode move, borrow, drop instructions for ownership semantics.

SPEC-05: LIR must encode effect annotations on function declarations.

SPEC-07 (BACKENDS): Backends consume LIR and produce target-specific output.

SPEC-09 (TOOL): loc emits LIR; loc --version --json declares lir_version (CONV-034).

7. Open questions

Q1. SSA vs register-based. LIR-016 mandates SSA. Should LIR use register-based (virtual registers can be reassigned) for simpler codegen? Proposal: keep SSA; it simplifies optimizations and identity proofs.

Q2. LIR binary encoding. LIR-040 defines a simple record format. Should we use a more compact encoding (LEB128, etc.)? Proposal: keep simple text-compatible binary for spec-0.1; optimize later.

Q3. Debug info in LIR. Should LIR carry source locations? Proposal: debug info in a separate section, not in the core LIR format.

Q4. LIR version bump policy. When does LIR version increment independently of language version? Proposal: whenever the instruction set or binary format changes.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. Defines LIR versions, text format, instruction set, identity rules (CONV-042), binary format. |

End of SPEC-06 (spec-0.1).