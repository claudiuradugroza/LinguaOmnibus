SPEC-04: Memory Model

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1), spec/02-grammar.md (spec-0.1), spec/03-types.md (spec-0.1)
Applies to: every .lo source file, every loc implementation, every lolsp implementation, runtime, every language pack

1. Motivation

Memory management is where safety meets performance. Lingua Omnibus must run on accessible hardware (32-bit, 512 MB RAM — Principle 11), so it cannot assume a heavyweight garbage collector on every target. This document defines:

    the single-ownership rule for non-Copy values,

    borrowing (shared and mutable references) with lifetime checking,

    move semantics and use-after-move rejection,

    drop semantics (exactly once per owner),

    stack vs heap allocation rules,

    the optional minimal GC and when it MAY be used.

Every rule carries a MEM-NNN identifier and is mapped to at least one conformance test.

2. Normative specification

2.1 Ownership

MEM-001. Every non-Copy value MUST have exactly one owner at any point in its lifetime (reifies the ownership glossary term from SPEC-00 §3.3).

MEM-002. Copy types (all primitives except KW_QUBIT, KW_QREG<N>, KW_QCIRCUIT, KW_QSTATE, and user types deriving Copy) MAY be implicitly copied. Non-Copy types MUST be moved, not copied.

MEM-003. A move transfers ownership from the source binding to the destination. After a move, the source binding is uninitialized and MUST NOT be read.

MEM-004. Reading a moved binding MUST be rejected with error M0101_use_after_move.

MEM-005. Partial moves out of a struct or enum are permitted only for non-Copy fields; moving a field out of a Copy type MUST be rejected with error M0102_partial_move_copy.

MEM-006. Reassigning a binding that owns a non-Copy value drops the old value before the move (drop happens at reassignment point, not at scope end).

2.2 Borrowing

MEM-010. A shared reference &T borrows without taking ownership. Multiple shared references to the same value MAY coexist.

MEM-011. A mutable reference &mut T borrows with exclusive access. While a &mut T is active, no other reference (shared or mutable) to the same value MAY exist.

MEM-012. Creating &mut T while any shared reference &T to the same value is live MUST be rejected with error M0201_borrow_conflict.

MEM-013. Creating &T while any &mut T to the same value is live MUST be rejected with error M0202_borrow_conflict.

MEM-014. A reference MUST not outlive the value it borrows (lifetime rules per §2.4). A dangling reference MUST be rejected with error M0203_dangling_reference.

MEM-015. References to local stack values MUST NOT be returned from a function or stored in a longer-lived location. Returning &local MUST be rejected with error M0204_return_local_ref.

2.3 Lifetimes

MEM-020. A lifetime is a lexical region governing a value's validity (reifies the lifetime glossary term from SPEC-00 §3.3).

MEM-021. Every reference &T or &mut T carries an implicit lifetime parameter. The compiler MUST infer lifetimes using lexical scoping rules.

MEM-022. A reference's lifetime MUST be bounded by the lifetime of the value it borrows. Explicit lifetime annotations '<'a' MAY be written on function signatures and structs.

MEM-023. When explicit lifetime annotations are present, they MUST be consistent: a reference with lifetime 'a MUST not be used where a shorter lifetime 'b ('b: 'a) is expected. Inconsistent annotations MUST be rejected with error M0301_lifetime_mismatch.

MEM-024. The compiler MUST perform drop checking: a value owning non-Copy data MUST be dropped exactly once when its owner's lifetime ends.

MEM-025. Nested lifetimes: if reference r1 borrows from r2, and r2 borrows from v, then r1's lifetime MUST be bounded by v's lifetime (transitivity). Violation → M0302_lifetime_transitivity.

2.4 Stack vs heap

MEM-030. Values whose size is known at compile time and whose lifetime is bounded by a single stack frame MAY be stack-allocated.

MEM-031. Values with unknown size at compile time, or whose lifetime may exceed a single stack frame (e.g. Box<T>, Rc<T>), MUST be heap-allocated.

MEM-032. Box<T> MUST own a heap-allocated T. Moving a Box moves the pointer, not the pointee. Dropping a Box drops T and frees the heap memory.

MEM-033. Stack overflow MUST be detected at runtime where feasible and reported as error M0304_stack_overflow. Implementations targeting 32-bit / 512 MB RAM SHOULD set a conservative default stack limit (e.g. 1 MiB per thread).

2.5 Drop semantics

MEM-040. Every owner of a non-Copy value MUST drop that value exactly once when the owner's lifetime ends (reifies the drop glossary term).

MEM-041. The drop order for a scope is reverse declaration order (last declared, first dropped).

MEM-042. A double-drop of the same non-Copy value MUST be rejected at compile time where possible, and MUST abort with M0401_double_drop at runtime if not statically detectable.

MEM-043. Panic unwinding MUST drop all live owners in the unwinding scope. A panic in a drop function MUST abort the program (M0402_panic_in_drop).

MEM-044. The #![no_drop] attribute MAY be applied to a type to suppress automatic drop; the programmer then assumes full responsibility for manual deallocation. Use of #![no_drop] on a type containing non-Copy fields without a manual drop impl MUST produce warning M0403_no_drop_unsafe.

2.6 Optional garbage collector

MEM-050. A garbage collector is OPTIONAL in spec-0.1. An implementation MAY ship with no GC, a minimal mark-and-sweep GC, or a region-based GC.

MEM-051. If a GC is present, it MUST NOT be used to silently fix use-after-move or borrow conflicts: those are compile-time errors per MEM-004, MEM-012, MEM-013.

MEM-052. The GC, if present, MUST collect only unreachable heap objects. It MUST NOT collect stack values or values with live owners.

MEM-053. An implementation WITHOUT a GC MUST NOT leak memory for programs that pass the borrow checker: every heap allocation MUST be freed by drop (MEM-040) or Box drop (MEM-032).

MEM-054. The presence or absence of a GC MUST be declared in the implementation's compatibility declaration (CONV-034) under a "gc" field with value "none", "minimal", or "full".

2.7 Quantum memory (linear)

MEM-060. KW_QUBIT and KW_QREG<N> are linear quantum resources (reifies the qubit glossary term). They MUST NOT implement Copy.

MEM-061. A qubit MUST be allocated via alloc_qubit() and MUST be measured or explicitly released before its owner's lifetime ends. Failure to measure/release → M0601_qubit_not_released.

MEM-062. Qubit aliasing (two owners for the same physical qubit) MUST be rejected at compile time with error M0602_qubit_alias.

MEM-063. Measuring a qubit collapses it and consumes ownership; the measurement result is a Copy bool. After measure(q), q is moved and MUST NOT be used (M0603_qubit_consumed_by_measure).

3. Formal grammar or tables

Lifetime annotations (additive to SPEC-02 grammar):

type_ref = "&" lifetime? "mut"? type ;
lifetime = "'" IDENT ;
fn_sig_with_lifetimes = KW_FUNCTION IDENT "(" param_list? ")" ( KW_RETURNS type )? ("<" lifetime ("," lifetime)* ">")? effects_clause? ":" block ;

Table — allocation and drop rules:

| Construct | Storage | Copy | Move | Drop |
|-----------|---------|------|------|------|
| intN, uintN | stack | yes | n/a | none |
| f32, f64 | stack | yes | n/a | none |
| bool, char, byte | stack | yes | n/a | none |
| KW_UNIT | stack | yes | n/a | none |
| String | stack header + heap data | yes (clone) | yes (shallow) | frees heap |
| T[] / [T; N] | stack or heap | element-wise | yes | drops elements |
| Box<T> | heap | no | yes (pointer) | frees T + heap |
| Qubit | heap | no | yes (linear) | M0601 if not measured |
| QReg<N> | heap | no | yes (linear) | M0601 if not measured |

4. Examples

4.1 Move semantics — EN and RO

#lingua: EN
var s1 = String("hello")
var s2 = s1          # s1 moved to s2
# print(s1)          # M0101_use_after_move — s1 is uninitialized
var s3 = s2.clone()  # Copy via explicit clone — OK

#lingua: RO
variabilă s1 = String("hello")
variabilă s2 = s1    # s1 mutat în s2
# print(s1)          # M0101_use_after_move — s1 e neinițializat
variabilă s3 = s2.clone()  # Copiere explicită — OK

Both produce identical LIR.

4.2 Borrowing — shared vs mutable

#lingua: EN
var x = 42
var r1 = &x          # shared borrow — OK
var r2 = &x          # multiple shared — OK
var r3 = &mut x      # M0201_borrow_conflict — r1, r2 still live
print(*r1)
print(*r2)

4.3 Lifetimes — dangling reference rejected

#lingua: EN
function bad() -> &Integer:
    var local = 7
    return &local    # M0204_return_local_ref — local outlives function

function good(x: &Integer) -> &Integer:
    return x         # lifetime bounded by caller — OK

4.4 Box — heap ownership

#lingua: EN
var b = Box::new(42) # heap allocated
var b2 = b           # Box moved — pointer copied, pointee not duplicated
# b is now uninitialized
drop(b2)             # frees 42 and the heap allocation

4.5 Qubit — linear quantum resource

#lingua: EN
quantum:
    var q = alloc_qubit()
    var r = measure(q)   # q consumed — result is bool
    # q is unusable here — M0603_qubit_consumed_by_measure if used

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: MEM-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| MEM-001 … MEM-006 | conformance/mem/001/ownership.lo |
| MEM-010 … MEM-015 | conformance/mem/010/borrowing.lo |
| MEM-020 … MEM-025 | conformance/mem/020/lifetimes.lo |
| MEM-030 … MEM-033 | conformance/mem/030/stack_heap.lo |
| MEM-040 … MEM-044 | conformance/mem/040/drop.lo |
| MEM-050 … MEM-054 | conformance/mem/050/gc_optional.lo |
| MEM-060 … MEM-063 | conformance/mem/060/quantum_linear.lo |

6. Interactions with other sections

SPEC-00: MEM rules reify the MEM prefix (CONV-020); ownership/borrow/lifetime glossary terms (SPEC-00 §3.3) are defined here.

SPEC-01: LEX-065 (integer overflow) and numeric typing interact with stack layout (MEM-030).

SPEC-02: GRAM-030 declarations (var_decl vs const_decl) are syntactic here; ownership distinguishes them semantically. GRAM-034 patterns bind ownership.

SPEC-03: TYPE-027…TYPE-033 (linear types, move, borrow, drop) are the type-level rules; SPEC-04 gives the operational memory semantics.

SPEC-05: Effects and capabilities (KW_IO, etc.) interact with MEM-044 panic unwinding.

SPEC-06 (LIR): LIR must encode move, borrow, and drop instructions so that two implementations produce identical LIR for the same ownership program (CONV-042).

SPEC-09 (TOOL): lolsp borrow-checker diagnostics use MEM-004…MEM-014 error codes. lofmt does not alter ownership semantics.

SPEC-11 (SEC): The sandbox (SEC-002) restricts heap allocation size per package (MEM-031, MEM-033 stack limits).

7. Open questions

Q1. Stack limit default. Principle 11 requires 32-bit / 512 MB RAM support. Proposal: 1 MiB default stack per thread, configurable via --stack-size. Revisit if embedded targets need less.

Q2. GC presence declaration. MEM-054 adds a "gc" field to CONV-034. Is this sufficiently standardized, or should it be a separate capability in SPEC-11?

Q3. #![no_drop] safety. MEM-044 is a warning, not an error. Should manual memory management without drop be an error in safe LO? Proposal: keep as warning; safe LO requires drop.

Q4. Qubit release without measure. MEM-061 requires measure or release. Is an explicit release_qubit(q) needed? Proposal: add release_qubit in SPEC-06 LIR intrinsics.

Q5. Move out of const. Can a const binding own non-Copy data? Proposal: const implies immutable reference, not ownership; const + non-Copy → M0103_const_owns_noncopy error. Deferred to RFC-0007.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. Defines ownership, borrowing, lifetimes, stack/heap, drop semantics, optional GC, quantum linear memory. |

End of SPEC-04 (spec-0.1).