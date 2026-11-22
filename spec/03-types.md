SPEC-03: Type System

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1), spec/02-grammar.md (spec-0.1)
Applies to: every .lo source file, every loc implementation, every lolsp implementation, every language pack

1. Motivation

The type system is where SPEC-01's lexical surface and SPEC-02's grammar acquire meaning. It must be precise enough that any two conforming implementations accept or reject the same programs, and it must support linear types for quantum resources (no-cloning at the type level). Every type rule carries a TYPE-NNN identifier and is mapped to at least one conformance test.

2. Normative specification

2.1 Primitive types

TYPE-001. Primitive integer types MUST have exact bit widths: int8, int16, int32, int64, int128 and uint8, uint16, uint32, uint64, uint128. The keyword for each is the pack's KW_INT slot qualified by width, e.g. KW_INT<i32>.

TYPE-002. Primitive floating-point types MUST be f32 and f64, IEEE 754 binary32 and binary64 respectively.

TYPE-003. KW_BOOL MUST be a 1-byte type with exactly two values: KW_TRUE and KW_FALSE.

TYPE-004. KW_CHAR MUST be a 4-byte UTF-32 code point (one Unicode scalar value).

TYPE-005. KW_BYTE MUST be a 1-byte unsigned integer (u8).

TYPE-006. KW_UNIT MUST be a zero-sized type with exactly one value: KW_NULL (also written ()). It is the default return type (GRAM-037).

TYPE-007. KW_STRING MUST be a UTF-8 encoded, growable string type. KW_STRING literals are delimited by " (LEX-070).

TYPE-008. KW_QUBIT MUST be a linear type: exactly one owner, no implicit Copy, no implicit Clone. Copying a qubit MUST be rejected with error T0101_qubit_not_copy.

TYPE-009. KW_QREG <N> MUST be a fixed-size register of N qubits, where N is a positive integer literal. QREG is linear.

TYPE-010. KW_QCIRCUIT and KW_QSTATE MUST be linear types; no implicit duplication.

TYPE-011. All primitive types MUST be declared in the active language pack's types.json mapping TY_X slots to natural-language names (CONV-060).

2.2 Composite types

TYPE-012. Tuple types are written (T1, T2, ..., Tn). A 1-tuple (T) is distinct from T. A 0-tuple () is KW_UNIT.

TYPE-013. Array types are written [T; N] where N is a positive integer constant expression. Arrays are homogeneous.

TYPE-014. Map types are written KW_MAP<K, V>. Keys MUST be comparable (Ord).

TYPE-015. KW_OPTION<T> MUST be an enum with two variants: Some(T) and None. None has no payload.

TYPE-016. KW_RESULT<T, E> MUST be an enum with two variants: Ok(T) and Err(E). KW_NULL MUST NOT be a valid Result value.

TYPE-017. KW_FUNCTION <(T1, ..., Tn)> (KW_RETURNS U)? MUST be a function type. Functions are first-class values.

2.3 Generics with bounds

TYPE-018. Generic parameters are declared as <IDENT : Bound?> where Bound is a trait/interface name. Unbounded generics default to the Sized bound.

TYPE-019. Type inference MUST resolve generic arguments from usage sites. If a generic cannot be inferred, the compiler MUST reject with error T0201_cannot_infer_generic.

TYPE-020. Generic bounds MUST be satisfied at the call site. An unsatisfied bound MUST be rejected with error T0202_bound_not_satisfied.

TYPE-021. Monomorphisation MUST be performed at compile time. No runtime type erasure for generics.

2.4 Type inference

TYPE-022. Local type inference MUST use Hindley-Milner style unification over the unified AST. The algorithm MUST be deterministic.

TYPE-023. Variable declarations with an initializer and no explicit type MUST infer the type from the initializer expression. If inference fails, reject with T0203_inference_failed.

TYPE-024. Function return types MAY be omitted; they default to KW_UNIT (GRAM-037). If a function body returns a non-Unit value and the return type is Unit, reject with T0204_return_type_mismatch.

TYPE-025. Literal types: integer literals default to int32; float literals default to f64 unless context demands otherwise. A literal in a context expecting a different numeric type MUST be implicitly converted if lossless; otherwise reject with T0205_literal_type_mismatch.

TYPE-026. The unary minus operator on an unsigned integer type MUST be rejected with error T0206_negate_unsigned.

2.5 Linear types and ownership

TYPE-027. A linear type MUST have exactly one owner at any point in its lifetime. Transfer of ownership (move) invalidates the previous binding.

TYPE-028. Using a moved binding MUST be rejected with error T0301_use_after_move.

TYPE-029. Copy types (all primitives except KW_QUBIT, KW_QREG, KW_QCIRCUIT, KW_QSTATE, and user types deriving Copy) MAY be implicitly copied. Non-Copy types MUST be moved, not copied.

TYPE-030. Borrowing: a non-owning reference &T or &mut T MUST not outlive the value it borrows (lifetime rules per SPEC-04). A dangling reference MUST be rejected with error T0302_dangling_reference.

TYPE-031. KW_QUBIT, KW_QREG<N>, KW_QCIRCUIT, KW_QSTATE MUST NOT implement Copy. They MUST NOT be passed by value to a function unless the parameter consumes them (move semantics).

TYPE-032. No-cloning: an implicit or explicit clone of a linear quantum type MUST be rejected with error T0303_quantum_no_clone.

TYPE-033. Drop: a linear value MUST be dropped exactly once when its owner goes out of scope. A double-drop MUST be rejected with error T0304_double_drop.

2.6 Option and Result semantics

TYPE-034. KW_OPTION<T>::None MUST be a valid value of type KW_OPTION<T>. KW_OPTION<T>::Some(v) MUST wrap v.

TYPE-035. The postfix ? operator (GRAM-024) on KW_OPTION<T> extracts Some(v) or propagates None. On KW_RESULT<T, E> it extracts Ok(T) or propagates Err(E). The ? operator is statement-tail only in spec-0.1.

TYPE-036. KW_NULL MUST be of type KW_OPTION<T> for some inferred T, not a universal null. A KW_NULL used where a non-Option type is expected MUST be rejected with error T0401_null_on_non_option.

2.7 Effects as part of function signatures

TYPE-037. Effect declarations KW_IO, KW_DATABASE, KW_NETWORK, KW_FILESYSTEM, KW_QUANTUM MUST be part of a function's type signature (via KW_WITH in GRAM-030).

TYPE-038. A function declaring an effect MUST be callable only from a context that handles that effect (per SPEC-05). Calling an effectful function from a pure context MUST be rejected with error T0501_unhandled_effect.

TYPE-039. Function subtyping: a function with fewer effects MUST be assignable to a function type expecting more effects (effect subsumption). The reverse MUST be rejected.

2.8 Type compatibility and equality

TYPE-040. Two types are equal iff their fully qualified kind and all parameters are structurally equal. Nominal types (structs, enums, interfaces) are equal by name and module path.

TYPE-041. Implicit coercions are permitted only: intN → intM (widening, M > N), f32 → f64, T → KW_OPTION<T> (some), T → KW_RESULT<T, E> (ok). All other conversions MUST be explicit.

TYPE-042. Explicit casts MUST be written as as-expressions: expr as TargetType. An invalid cast MUST be rejected with error T0601_invalid_cast.

3. Formal grammar or tables

(TYPE rules reference the type grammar from SPEC-02 §2.4 GRAM-033. The type table below is normative.)

| Type | Kind | Size | Copy | Linear |
|------|------|------|------|--------|
| int8..int128 | primitive integer | 1..16 bytes | yes | no |
| uint8..uint128 | primitive unsigned | 1..16 bytes | yes | no |
| f32 | primitive float | 4 bytes | yes | no |
| f64 | primitive float | 8 bytes | yes | no |
| bool | primitive | 1 byte | yes | no |
| char | primitive | 4 bytes | yes | no |
| byte | primitive | 1 byte | yes | no |
| unit | primitive | 0 bytes | yes | no |
| string | composite | growable | yes | no |
| qubit | quantum | 1 ref | no | yes |
| qreg<N> | quantum | N refs | no | yes |
| qcircuit | quantum | opaque | no | yes |
| qstate | quantum | opaque | no | yes |
| option<T> | composite | 1 + size(T) | yes | no |
| result<T, E> | composite | 1 + max(size(T), size(E)) | yes | no |

4. Examples

4.1 Primitive types and literals — EN and RO

#lingua: EN
var a: int32 = 42
var b: f64 = 3.14
var c: bool = true
var d: char = 'A'
var e: byte = b'X'

#lingua: RO
variabilă a: int32 = 42
variabilă b: f64 = 3,14
variabilă c: Logic = adevăr
variabilă d: Char = 'A'
variabilă e: Octet = b'X'

Both produce identical LIR type annotations.

4.2 Linear qubit — no cloning

#lingua: EN
var q: Qubit = alloc_qubit()
var q2 = q        # rejected: T0301_use_after_move (q moved)
measure(q)        # q consumed here

#lingua: RO
variabilă q: Qubit = aloca_qubit()
variabilă q2 = q   # refuzat: T0301_use_after_move (q mutate)
măsoară(q)         # q consumat aici

4.3 Option and Result with ?

#lingua: EN
function find(xs: [Integer]) returns Option<Integer>:
    if xs.len() == 0 then:
        return None
    else:
        return Some(xs[0])

function main() returns Unit:
    val = find([1, 2, 3])?   # extracts Integer or propagates None
    return Unit

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: TYPE-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| TYPE-001 … TYPE-006 | conformance/type/001/primitives.lo |
| TYPE-007 … TYPE-011 | conformance/type/002/string_qubit.lo |
| TYPE-012 … TYPE-017 | conformance/type/010/composite.lo |
| TYPE-018 … TYPE-021 | conformance/type/020/generics.lo |
| TYPE-022 … TYPE-026 | conformance/type/030/inference.lo |
| TYPE-027 … TYPE-033 | conformance/type/040/linear.lo |
| TYPE-034 … TYPE-036 | conformance/type/050/option_result.lo |
| TYPE-037 … TYPE-039 | conformance/type/060/effects.lo |
| TYPE-040 … TYPE-042 | conformance/type/070/compatibility.lo |

6. Interactions with other sections

SPEC-00: TYPE rules reify the TYPE prefix (CONV-020). TYPE-001 references exact bit widths (open formats, Principle 8).

SPEC-01: LEX-065 (integer overflow) references TYPE-001 bounds. LEX-073/074 (char/byte) reference TYPE-004/005.

SPEC-02: GRAM-033's type rule is the surface grammar; SPEC-03 assigns meaning and inference. GRAM-037's KW_UNIT default is TYPE-006.

SPEC-04 (MEMORY): TYPE-027…033 (ownership, borrowing, drop) defer lifetime details to SPEC-04.

SPEC-05 (EFFECTS): TYPE-037…039 (effects in signatures) defer effect semantics to SPEC-05.

SPEC-06 (LIR): Type annotations in LIR MUST match SPEC-03 types exactly.

SPEC-09 (TOOL): lolsp hover/type-information uses TYPE inference results.

7. Open questions

Q1. int128/uint128 support. Targeting 32-bit hardware (Principle 11), 128-bit arithmetic may need software emulation. Proposal: keep in spec; mark as optional for constrained targets.

Q2. Higher-kinded types. Not in spec-0.1. Proposal: defer to spec-1.0 via RFC.

Q3. Type-level natural numbers for QREG<N>. Proposal: N is a const generic; defer const generics to spec-1.0.

Q4. Effect polymorphism. Functions generic over effects. Proposal: defer to spec-1.0.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft from info.docx. Defines primitive types, composite types, generics, inference, linear/quantum types, Option/Result, effects in signatures, type compatibility. |

End of SPEC-03 (spec-0.1).