SPEC-05: Effects & Concurrency

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1), spec/02-grammar.md (spec-0.1), spec/03-types.md (spec-0.1), spec/04-memory.md (spec-0.1)
Applies to: every .lo source file, every loc implementation, every lolsp implementation, runtime

1. Motivation

Effects are the mechanism by which Lingua Omnibus tracks side effects in function signatures, enabling safe concurrency and capability-based security. This document defines:

    effect classes (IO, DATABASE, NETWORK, FILESYSTEM, QUANTUM),

    async/await and spawn concurrency primitives,

    capability checking (SPEC-11 integration),

    the interaction between effects and the memory model (SPEC-04).

Every rule carries an EFF-NNN identifier and is mapped to at least one conformance test.

2. Normative specification

2.1 Effect classes

EFF-001. The following effect classes are predefined: KW_IO, KW_DATABASE, KW_NETWORK, KW_FILESYSTEM, KW_QUANTUM. No other effects exist in spec-0.1.

EFF-002. A function declaration MAY declare an effects clause: KW_WITH [ effect_list ]. The effects clause appears after the optional return type and before : (GRAM-038).

EFF-003. A function with no effects clause is pure: it performs no side effects and MAY be called from any context.

EFF-004. A function declaring KW_IO performs input/output. Calling it from a pure context is rejected (TYPE-038 / EFF-010).

EFF-005. A function declaring KW_DATABASE performs database operations (SPEC-08).

EFF-006. A function declaring KW_NETWORK performs network I/O.

EFF-007. A function declaring KW_FILESYSTEM performs file system operations.

EFF-008. A function declaring KW_QUANTUM performs quantum operations (alloc_qubit, measure, quantum gates).

2.2 Effect subtyping and subsumption

EFF-010. Function subtyping: a function with fewer effects MUST be assignable to a function type expecting more effects (effect subsumption). E.g. a pure function MAY be used where KW_IO is expected.

EFF-011. A function with MORE effects than expected MUST be rejected with error E0101_effect_mismatch. E.g. a KW_IO function MUST NOT be assigned to a pure function type.

EFF-012. Effect polymorphism (generic over effects) is NOT part of spec-0.1. Deferred to spec-1.0 (see Q4).

2.3 Async / await

EFF-020. The KW_ASYNC keyword marks an asynchronous function. async functions return a future implicitly; the return type is wrapped in an implicit Future<T>.

EFF-021. The KW_AWAIT keyword awaits a future. await MAY only appear inside an async function or an async block. Using await outside async → E0201_await_outside_async.

EFF-022. An async function automatically carries the KW_IO effect (async I/O is the only async effect in spec-0.1).

EFF-023. The .await postfix syntax (GRAM-024 postfix ? is separate) — await is a keyword, not an operator.

2.4 Concurrency — spawn

EFF-030. KW_SPAWN spawns a new lightweight thread executing a closure. spawn returns a JoinHandle<T>.

EFF-031. The closure passed to spawn MUST be 'static (no references to non-'static data) or all captured references MUST have lifetimes outliving the spawned thread. Violation → E0301_non_static_spawn.

EFF-032. Data races are prevented by the type system: a value sent across threads MUST be Send. Non-Copy values are moved; Copy values are copied. Shared references &T are Send if T: Sync.

EFF-033. Channels for inter-thread communication are provided by the standard library (SPEC-10), not built into the language.

2.5 Capabilities (integration with SPEC-11)

EFF-040. Capabilities are declared permissions required by effects (SPEC-11). A function declaring KW_FILESYSTEM requires the filesystem capability at its call site.

EFF-041. Capability checking is performed at compile time for direct calls, and at runtime for dynamic dispatch (function pointers, trait objects).

EFF-042. A program lacking a required capability MUST be rejected with error E0401_missing_capability (compile time) or abort with E0402_capability_violation (runtime).

2.6 Effect inference

EFF-050. Effects are NOT inferred. Every effectful function MUST declare its effects explicitly. The compiler MUST NOT infer effects from the body.

EFF-051. If a function calls an effectful function but does not declare the effect, the compiler MUST reject with error E0501_undeclared_effect.

3. Formal grammar or tables

effect_clause = KW_WITH "[" effect ("," effect)* "]" ;
effect        = KW_IO | KW_DATABASE | KW_NETWORK | KW_FILESYSTEM | KW_QUANTUM ;
async_fn      = KW_ASYNC KW_FUNCTION IDENT "(" param_list? ")" ( KW_RETURNS type )? ":" block ;
await_expr    = KW_AWAIT expr ;
spawn_expr    = KW_SPAWN "(" closure ")" ;

Table — effect classes:

| Effect | Meaning | Capability required |
|--------|---------|---------------------|
| KW_IO | Print, file read/write, stdin/stdout | IO capability |
| KW_DATABASE | LinguaDB queries (SPEC-08) | Database capability |
| KW_NETWORK | HTTP, TCP, UDP | Network capability |
| KW_FILESYSTEM | File create/delete, directory listing | Filesystem capability |
| KW_QUANTUM | qubit allocation, measurement, gates | Quantum capability |

4. Examples

4.1 Pure vs effectful — EN and RO

#lingua: EN
function pure_add(a: Integer, b: Integer) returns Integer:
    return a + b

function read_config() with [IO] returns String:
    return "config"

#lingua: RO
funcție adunare_pura(a: Întreg, b: Întreg) returnează Întreg:
    returnează a + b

funcție citește_config() cu [IO] returnează Șir:
    returnează "config"

4.2 Async / await

#lingua: EN
async function fetch_data() returns String:
    var data = await http_get("https://example.com")
    return data

4.3 Spawn

#lingua: EN
function main() returns Unit:
    var handle = spawn(|| {
        var x = 1 + 2
        return x
    })
    var result = handle.join()
    return Unit

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: EFF-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| EFF-001 … EFF-008 | conformance/eff/001/effects.lo |
| EFF-010 … EFF-012 | conformance/eff/010/subtyping.lo |
| EFF-020 … EFF-023 | conformance/eff/020/async_await.lo |

6. Interactions with other sections

SPEC-00: EFF rules reify the EFF prefix (CONV-020); effect glossary term (SPEC-00 §3.3) defined here.

SPEC-02: GRAM-030 effects_clause is the surface syntax; SPEC-05 defines effect semantics.

SPEC-03: TYPE-037…TYPE-039 (effects in signatures, effect subsumption) are the type-level rules; SPEC-05 gives operational semantics.

SPEC-04: MEM-044 panic unwinding interacts with effects (panics in effectful functions).

SPEC-06 (LIR): LIR must encode effect annotations on function declarations so that CONV-042 (byte-identical LIR) holds.

SPEC-08 (LINGUADB): KW_DATABASE effect is required for all database operations.

SPEC-09 (TOOL): lolsp effect checking and diagnostics use EFF codes.

SPEC-11 (SEC): Capabilities (EFF-040…042) are enforced by the security sandbox.

7. Open questions

Q1. Structured concurrency. Should spawn be tied to a scope (like Kotlin's structured concurrency) so that all spawned tasks complete before the scope exits? Proposal: defer to spec-1.0; spawn in spec-0.1 is fire-and-forget with JoinHandle.

Q2. Effect generics. EFF-012 defers effect polymorphism. Use case: higher-order functions that are generic over the effects of their callback. Deferred to spec-1.0.

Q3. async block syntax. Should { async { … } } blocks be supported in addition to async functions? Proposal: keep async functions only in spec-0.1.

Q4. Channel primitives. Should channels be language builtins or stdlib-only? Proposal: stdlib-only (EFF-033).

Q5. Effect checking for function pointers. EFF-041 says runtime for dynamic dispatch. Is there a way to make it static? Proposal: function pointer types carry effect annotations (TYPE-017).

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. Defines effect classes, subtyping, async/await, spawn, capabilities, effect inference. |

End of SPEC-05 (spec-0.1).