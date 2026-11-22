SPEC-02: Grammar

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1)
Applies to: every .lo source file, every language pack, every lofmt, lolsp, lodoc implementation

1. Motivation

Grammar is where the multilingual promise either holds together or collapses. A Japanese function declaration must produce the same unified AST as an English one, even though its token order may differ. This document therefore defines two things at once:

    The reference grammar — a single EBNF expressed over keyword slots (KW_FUNCTION, KW_IF, …), not over English words. This grammar is the canonical one; every pack derives from it.

    The derivation contract — the exact, checkable rules by which a language pack turns the reference grammar into a working parser for its own natural language.

At spec-0.1, derivation is slot substitution only: word order is preserved verbatim across all packs. The 6 initial packs (EN, RO, FR, DE, ES, JA) all use SVO or SVO-compatible order for the constructs we support, so spec-0.1 covers them. Order-transforming derivation (SOV/VSO) is designed into the pack schema but rejected at pack-load time until spec-1.0 (see §2.5, §7-Q3). This satisfies CONV-061 (no compiler change per language) while not overpromising.

Additionally, this document extends SPEC-01 with three small, additive rules:

    Virtual NEWLINE / INDENT / DEDENT tokens for significant indentation.

    Keyword phrases (multi-word keywords like German gibt zurück).

    Unicode operator aliases (≤, ≥, ≠ → <=, >=, !=).

All three are additive and preserve SPEC-01 byte-for-byte.

2. Normative specification

2.1 Lexical extensions (additive to SPEC-01)

GRAM-001. The lexer MUST additionally emit three virtual tokens that never appear in source bytes:

At end of file, the lexer MUST emit one DEDENT per open indentation level, then EOF.

GRAM-002. Logical lines end at \n unless the physical line ends inside an unclosed (, [ or {, in which case the line terminator is treated as ordinary whitespace. This is the sole continuation rule; backslash continuation is not part of spec-0.1.

GRAM-003. (Keyword phrases, extends LEX-040) A keyword MAY be a phrase — a sequence of one or more identifier-shaped words on the same logical line, separated by whitespace not containing a line terminator. The lexer MUST build a keyword-phrase trie from the active pack and MUST emit at most one Keyword token per matched phrase. Longest match wins (LEX-012). If two keyword phrases of a pack share a prefix, the pack is invalid (see GRAM-072).

GRAM-004. (Unicode operators, extends LEX-051) The following surface forms MUST be accepted by the lexer and canonicalized to the corresponding ASCII token before parsing:

This introduces no new token kind — it is a surface-form canonicalization, so LEX-051 remains byte-identical.

GRAM-005. The canonicalization in GRAM-004 MUST be performed after NFC normalization (LEX-004) and MUST NOT apply inside string, char, byte or raw-string literals.

2.2 Significant indentation

GRAM-010. Indentation is the sequence of leading U+0020 SPACE characters at the start of a logical line, before any non-whitespace, non-comment character. Every other whitespace code point (LEX-020) at line start MUST be rejected with error L0510_tab_or_unicode_indent unless it is U+0020.

GRAM-011. A blank line (only whitespace and/or line terminator) or a comment-only line MUST NOT produce NEWLINE, INDENT or DEDENT. They are invisible to the indentation tracker.

GRAM-012. The first logical line of a file establishes the base indentation, which MUST be zero. A file that begins with an indented logical line MUST be rejected with error L0511_unexpected_indent.

GRAM-013. A logical line whose indentation does not match any level in the current indentation stack MUST be rejected with error L0512_inconsistent_dedent.

GRAM-014. A DEDENT that would pop to zero indentation at end-of-file MUST be emitted once per level as specified by GRAM-001, not once in total.

GRAM-015. The lexer MUST NOT reject tabs used inside a line, inside a comment, or inside a string; only line-leading indentation is restricted.

2.3 Expression precedence

GRAM-020. The following precedence table is normative and universal. No language pack MAY redefine it. All parsers, regardless of pack, MUST produce the same precedence tree for the same token sequence.

GRAM-021. Parentheses ( … ) MUST override precedence. Any expression grammar rule MUST accept a parenthesized form.

GRAM-022. Comparison operators are not chainable. a < b < c MUST be rejected with error L0520_chained_comparison — users must write a < b && b < c. This avoids the Python/Rust divergence on what a < b < c means.

GRAM-023. +, -, !, ~ as prefix unary operators bind tighter than any binary operator at level 10 or below, and looser than any postfix operator at level 12 or above.

GRAM-024. The postfix ? operator MUST be a statement-tail-only syntactic form: it may appear only as the last sub-expression of a return statement or of an expression-statement whose result is discarded. Anywhere else MUST be rejected with error L0521_misplaced_try in spec-0.1; full expression-level error propagation is deferred to SPEC-05.

2.4 Reference grammar (keyword slots)

The reference grammar below is stated exactly once and is the canonical form. UPPERCASE_SLOTS are keyword slots substituted by the active pack (CONV-063 / LEX-040). Terminals in " are literal punctuation. NEWLINE, INDENT, DEDENT are the virtual tokens from GRAM-001. Non-terminals in snake_case are grammar rules. IDENT, INT, FLOAT, STRING, CHAR, BYTE, RAW_STRING are token types from LEX-011.

GRAM-030 — declarations.

file            = shebang? directive* module_decl? import* item* EOF ;
module_decl     = KW_MODULE path NEWLINE ;
import          = KW_IMPORT path ( "as" IDENT )? NEWLINE ;
item            = function_decl
                | struct_decl
                | enum_decl
                | interface_decl
                | impl_block
                | const_decl
                | var_decl        (* top-level global var; rare in practice *) ;
function_decl   = doc? visibility? KW_FUNCTION IDENT "(" param_list? ")"
                  ( KW_RETURNS type )? effects_clause? ":" block ;
param_list      = param ( "," param )* ","? ;
param           = IDENT ":" type ;
effects_clause  = KW_WITH "[" effect_list "]" ;
effect_list     = effect ( "," effect )* ;
effect          = KW_IO | KW_DATABASE | KW_NETWORK | KW_FILESYSTEM | KW_QUANTUM ;
visibility      = KW_PUBLIC | KW_PRIVATE ;
doc             = (* doc-comment metadata attached to next token, per LEX-084 *) ;
struct_decl     = KW_STRUCT IDENT generic_params? "{" field_list? "}" ;
field_list      = field ( "," field )* ","? ;
field           = IDENT ":" type ;
enum_decl       = KW_ENUM IDENT generic_params? "{" variant_list? "}" ;
variant_list    = variant ( "," variant )* ","? ;
variant         = IDENT ( "(" type_list ")" )? ;
interface_decl  = KW_INTERFACE IDENT generic_params? "{" method_sig* "}" ;
method_sig      = KW_FUNCTION IDENT "(" param_list? ")" ( KW_RETURNS type )? effects_clause? ;
impl_block      = KW_IMPL IDENT KW_FOR type "{" function_decl* "}" ;
generic_params  = "<" generic_param ( "," generic_param )* ">" ;
generic_param   = IDENT ( ":" IDENT )? ;
const_decl      = KW_CONST IDENT ":" type "=" expr NEWLINE ;
var_decl        = KW_VAR IDENT ( ":" type )? ( "=" expr )? NEWLINE ;

GRAM-031 — statements.

block           = NEWLINE INDENT statement+ DEDENT ;
statement       = var_decl
                | const_decl
                | if_stmt
                | while_stmt
                | for_stmt
                | match_stmt
                | return_stmt
                | break_stmt
                | continue_stmt
                | quantum_block
                | expr_stmt ;
if_stmt         = KW_IF expr KW_THEN ":" block ( KW_ELSE ":" ( if_stmt | block ) )? ;
while_stmt      = KW_WHILE expr ":" block ;
for_stmt        = KW_FOR pattern KW_IN expr ":" block ;
match_stmt      = KW_MATCH expr ":" NEWLINE INDENT match_arm+ DEDENT ;
match_arm       = pattern ":" block ;
return_stmt     = KW_RETURN expr? NEWLINE ;
break_stmt      = KW_BREAK NEWLINE ;
continue_stmt   = KW_CONTINUE NEWLINE ;
quantum_block   = KW_QUANTUM ":" block ;
expr_stmt       = expr NEWLINE ;

GRAM-032 — expressions.

expr            = assign_expr ;
assign_expr     = IDENT "=" expr                 (* statement-form; see GRAM-021 *)
                | logical_or ;
logical_or      = logical_and ( "\|\|" logical_and )* ;
logical_and     = bit_or ( "&&" bit_or )* ;
bit_or          = bit_xor ( "\|" bit_xor )* ;
bit_xor         = bit_and ( "^" bit_and )* ;
bit_and         = equality ( "&" equality )* ;
equality        = comparison ( ( "==" | "!=" ) comparison )* ;
comparison      = shift ( ( "<" | "<=" | ">" | ">=" ) shift )? ;     (* non-chaining, GRAM-022 *)
shift           = additive ( ( "<<" | ">>" ) additive )* ;
additive        = multiplicative ( ( "+" | "-" ) multiplicative )* ;
multiplicative  = unary ( ( "*" | "/" | "%" ) unary )* ;
unary           = ( "!" | "-" | "~" ) unary | postfix ;
postfix         = primary postfix_op* ;
postfix_op      = "." IDENT
                | "?." IDENT
                | "::" IDENT
                | "[" expr "]"
                | "(" arg_list? ")"
                | "?" ;                            (* see GRAM-024 *)
primary         = IDENT
                | INT | FLOAT | STRING | CHAR | BYTE | RAW_STRING
                | KW_TRUE | KW_FALSE | KW_NULL
                | "(" expr ")"
                | array_lit
                | tuple_lit
                | struct_lit ;
array_lit       = "[" ( expr ( "," expr )* ","? )? "]" ;
tuple_lit       = "(" expr "," ( expr ( "," expr )* ","? )? ")" ;
struct_lit      = path "{" ( field_init ( "," field_init )* ","? )? "}" ;
field_init      = IDENT ":" expr ;
arg_list        = expr ( "," expr )* ","? ;
path            = IDENT ( "::" IDENT )* ;

GRAM-033 — types.

type            = KW_INT | KW_FLOAT | KW_BOOL | KW_STRING | KW_CHAR | KW_BYTE | KW_UNIT
                | KW_QUBIT | KW_QREG "<" INT ">" | KW_QCIRCUIT | KW_QSTATE
                | path generic_args?
                | "(" type ( "," type )* ","? ")"     (* tuple type *)
                | "[" type "]"                        (* array type *)
                | KW_MAP "<" type "," type ">"
                | KW_OPTION "<" type ">"
                | KW_RESULT "<" type "," type ">"
                | KW_FUNCTION "(" type_list? ")" ( KW_RETURNS type )? ;
generic_args    = "<" type ( "," type )* ">" ;
type_list       = type ( "," type )* ;

GRAM-034 — patterns.

pattern         = "_"
                | IDENT
                | INT | FLOAT | STRING | CHAR | KW_TRUE | KW_FALSE | KW_NULL
                | "(" pattern ( "," pattern )* ","? ")"
                | path "(" pattern_list? ")"
                | path "{" field_pat ( "," field_pat )* ","? "}"
                | pattern "\|" pattern ;
pattern_list    = pattern ( "," pattern )* ;
field_pat       = IDENT ( ":" pattern )? ;

GRAM-035 — the if … then: redundancy rule. In every pack, the then-equivalent keyword (then in EN, atunci in RO, ならば in JA, …) is required after the condition. It documents the boundary between condition and body and mirrors natural-language conditional structures in all target packs. There is no shorthand form; if cond: without a then-keyword MUST be rejected with error L0530_missing_then.

GRAM-036 — the block-after-colon rule. A : that opens a block MUST be followed by NEWLINE INDENT statement+ DEDENT. One-liner bodies (if x: return 1) are not part of spec-0.1 and MUST be rejected with error L0531_oneline_block. This removes an entire class of grammar ambiguity and keeps the AST identical across packs whose scripts differ in whitespace conventions.

GRAM-037 — the function-return-type default. When KW_RETURNS type is omitted from a function_decl, the return type MUST be KW_UNIT. There is no void; KW_UNIT serves that role.

GRAM-038 — the effects clause position. When present, KW_WITH [ … ] MUST appear after the optional KW_RETURNS type and before :. Any other position MUST be rejected with error L0532_effects_position.

2.5 Language pack derivation

GRAM-040. A language pack MUST provide, alongside keywords.json, a machine-readable file grammar.json conforming to the schema in §9. The file MUST declare at minimum:

{
  "word_order": "svo",
  "word_segmentation": "space",
  "requires_explicit_lingua_directive": false,
  "script": "Latin",
  "bidi": "ltr",
  "preferred_decimal_sep": ".",
  "indent_unit": 4,
  "multiphrasal_keywords": false
}

GRAM-041. The compiler MUST support word_order: "svo" for any script at spec-0.1. If a pack declares "sov" or "vso", the compiler MUST reject the pack at load time with error G0001_unsupported_word_order and MUST list the supported values. Order-transforming derivation is deferred to spec-1.0 (see §7-Q3).

GRAM-042. The compiler MUST support word_segmentation: "space" and "none". When "none", keyword phrase matching uses longest-match-first per LEX-042 and GRAM-003.

GRAM-043. When multiphrasal_keywords: true, the compiler MUST build a keyword-phrase trie per GRAM-003 and MUST run the prefix-conflict check in GRAM-072.

GRAM-044. When bidi: "rtl" or "bidi", the compiler MUST NOT apply any grammatical change; source files are always read in logical order (UAX #9 §3.3). Bidi is a rendering concern only. The one exception is error message formatting, which lolsp and loc MUST produce with proper bidi embedding per UAX #9 for RTL diagnostics.

GRAM-045. The indent_unit field is a hint for lofmt, not a lexer rule. The lexer uses only relative indentation (GRAM-010–015) and never requires a specific unit.

GRAM-046. A pack with requires_explicit_lingua_directive: true MUST include the directive #lingua: XX in every file; the compiler MUST NOT auto-detect (reifies LEX-044, LEX-100).

2.6 Unified AST contract

GRAM-050. The parser MUST build a unified AST whose node kinds are drawn exclusively from the fixed set in §8. No implementation MAY introduce AST node kinds outside §8 at spec-0.1.

GRAM-051. A node MUST be an object with at minimum:

{
  "kind": "<NodeKind>",           // one of §8
  "children": [ ... ],             // ordered; order is normative
  "lingua_meta": {                 // debug/source info; MUST NOT affect LIR
    "pack": "en",
    "tokens": [ ... ]              // byte offsets into the source, for lolsp only
  }
}

lingua_meta MUST be stripped before LIR generation (SPEC-06). Two ASTs that differ only in lingua_meta MUST be considered identical for the purposes of GRAM-053.

GRAM-052. Every production rule in §2.4 that is visible (i.e. has a name and appears on the right-hand side of item or statement or expr) MUST map to exactly one AST node kind from §8. The mapping is:

GRAM-053. The AST identity test is normative. For any two .lo files A and B that differ only in their #lingua: directive and in the pack that produced them, the ASTs of A and B after stripping lingua_meta MUST be structurally identical (same node kinds, same children, same order) and MUST serialize to byte-identical JSON under loc dump-ast --canonical. This is the operational meaning of "unified AST".

GRAM-054. loc dump-ast --canonical MUST emit JSON with: keys sorted lexicographically, no insignificant whitespace, UTF-8, and one trailing newline. This gives a byte-comparable artefact for CI (see ci/conformance-gate.sh).

3. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: GRAM-NNN, … header per CONV-013.

That is 20+ files. Combined with the 36 from SPEC-01 and the 37 from SPEC-00, the repository is well over the M0 minimum of 30.

4. Examples

4.1 The same Fibonacci, three word orders, one AST

English (#lingua: EN):

function fib(n: Integer) returns Integer:
    if n <= 1 then:
        return n
    else:
        return fib(n-1) + fib(n-2)

Romanian (#lingua: RO):

funcție fib(n: Întreg) returnează Întreg:
    dacă n ≤ 1 atunci:
        returnează n
    altfel:
        returnează fib(n-1) + fib(n-2)

Japanese (#lingua: JA):

関数 fib(n: 整数) 返す 整数:
    もし n ≤ 1 ならば:
        返す n
    そうでなければ:
        返す fib(n-1) + fib(n-2)

Canonical loc dump-ast --canonical output for all three (identical, modulo lingua_meta, which is stripped here): {"children":[{"children":[{"children":[{"children":[1],"kind":"Literal","lit_kind":"int"}],"kind":"Ident","name":"n"}],"kind":"Param"},{"children":[{"kind":"PrimType","name":"int"}],"kind":"ReturnType"}],"kind":"FnDecl","name":"fib","children_body":[{"children":[{"children":[{"children":[{"children":[{"children":[1],"kind":"Literal","lit_kind":"int"}],"kind":"Ident","name":"n"},{"children":[{"children":[{"children":[1],"kind":"Literal","lit_kind":"int"}],"kind":"Ident","name":"n"}],"kind":"Literal","lit_kind":"int"}],"kind":"BinaryOp","op":"<="}],"kind":"BinaryOp","op":"<="}],"kind":"Cond"},{"children":[{"children":[{"kind":"Ident","name":"n"}],"kind":"Expr"}],"kind":"Block"},{"children":[...]}],"kind":"IfStmt"}],"kind":"Block","body":[...]}],"kind":"File"}

The Japanese and Romanian versions produce the same tree because the reference grammar is over slots (§2.4), and the pack only substitutes strings for slots. The ≤ surface form canonicalizes to <= before parsing (GRAM-004).

4.2 A German pack with multi-word keywords

German keywords.json (excerpt):

{
  "KW_RETURNS": ["gibt zurück"],
  "KW_IF": ["wenn"],
  "KW_THEN": ["dann"],
  "KW_ELSE": ["sonst"]
}

German source:

#lingua: DE
Funktion fib(n: Ganzzahl) gibt zurück Ganzzahl:
    wenn n ≤ 1 dann:
        zurückgeben n
    sonst:
        zurückgeben fib(n-1) + fib(n-2)

At lexing, gibt zurück is recognized as a single Keyword(KW_RETURNS) token (GRAM-003), and the AST is identical to EN/RO/JA.

4.3 Error cases

5. Interactions with other sections

SPEC-00: GRAM-003 reifies CONV-062 (requires_explicit_lingua_directive), GRAM-041 reifies CONV-061 (no compiler change per language), GRAM-050 reifies the unified AST glossary term.

SPEC-01: GRAM-001–005 are additive extensions; they do not remove any LEX rule.

SPEC-03 (TYPES): §2.4's type rule and §2.6's AST types are the surface; SPEC-03 assigns meaning and inference rules.

SPEC-04 (MEMORY): var_decl vs const_decl distinction is syntactic here; ownership is SPEC-04.

SPEC-05 (EFFECTS): GRAM-030's effects_clause position (GRAM-038) is normative; effect semantics live in SPEC-05.

SPEC-06 (LIR): GRAM-051 requires lingua_meta stripping; GRAM-054's canonical AST is a CI artefact, not an LIR artefact.

SPEC-09 (TOOL): loc dump-ast --canonical implements GRAM-054; lofmt consumes GRAM-045's indent_unit; lolsp consumes lingua_meta byte offsets for hover.

SPEC-12 (COMPAT): GRAM-041's rejection of SOV/VSO is temporary and MUST be removed at spec-1.0 after RFC-0005.

6. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft from info.docx. Defines lexical extensions, indentation, precedence, reference grammar over keyword slots, pack derivation contract, unified AST, examples, conformance tests, interactions, open questions. |

7. Open questions

Q1. then/else redundancy. GRAM-035 requires then after the condition. Some packs (e.g. Arabic, which is VSO and doesn't use an English-style then) may find this awkward. Proposal: keep for spec-0.1 for readability and cross-language consistency; revisit in RFC-0006 if a pack argues convincingly.

Q2. ? operator scope. GRAM-024 restricts ? to statement-tail positions. Rust allows it anywhere. Our restriction is more conservative but easier to spec. Proposal: keep; full expression-level ? deferred to spec-1.0.

Q3. Order-transforming derivation. The biggest linguistic risk of the whole project. spec-0.1 accepts SVO only. Japanese happens to be SVO-compatible in the constructs we support, but a truly SOV pack (e.g. a more aggressive Japanese style) would need reordering. Proposal: RFC-0005 to define the derivation block of grammar.json, and lift GRAM-041 to spec-1.0.

Q4. interface vs trait. §2.4 uses KW_INTERFACE. The glossary in SPEC-00 §3.3 says "trait/interface". Proposal: standardize on interface in spec-0.1; add KW_TRAIT as a synonym in a MINOR bump if community pressure emerges.

Q5. impl block syntax. GRAM-030 has KW_IMPL IDENT KW_FOR type "{" … "}". This matches Rust's style but is somewhat unusual. The alternative (methods inside the struct declaration) is more Java-like. Proposal: keep Rust-style for now; defer to SPEC-03 for the full picture.

Q6. KW_NULL semantics. null appears in the grammar as a literal. Its type and interaction with Option<T> is a SPEC-03 decision. Proposal: SPEC-03 should make null of type Option<T> for some T, not a universal null.

Q7. Canonical AST JSON. GRAM-054 fixes lexicographic key order and no whitespace. This is arbitrary but testable. Proposal: keep; if a second implementation finds it too strict, relax to "any JSON that round-trips through a canonical form".

Q8. KW_UNIT vs (). §2.4 uses KW_UNIT as the type name. Rust uses (). Python uses None. Our keyword is a natural-language word, which fits the project's philosophy. Proposal: keep; () as a tuple-literal for empty tuple is a separate thing.

Q9. Where do union types live? The original prompt listed Union in the type set, but §2.4 has no union_decl. Proposal: add union_decl in spec-0.2 if needed; enum covers most uses in spec-0.1.

Q10. Default parameter values. param in §2.4 has no default. Proposal: defer to spec-1.0; use Option<T> for now.

8. Appendix A — Unified AST node kinds (normative)

The following is the complete list of node kinds permitted at spec-0.1. Any implementation that emits a node kind outside this list is non-conforming (GRAM-050).

Declarations

Statements

Expressions

Types

Patterns

Supporting nodes (not first-class but required by the schema)

Param (name, type), Field (name, type), Variant (name, args), MethodSig (name, params, ret, effects), GenericParam (name, bound), Effect (name), MatchArm (pattern, body), FieldInit (name, value), FieldPattern (name, pattern).

9. Appendix B — grammar.json schema (normative)

{
  "$schema": "https://lingua-omnibus.org/schema/grammar-0.1.json",
  "type": "object",
  "required": ["word_order", "word_segmentation", "script", "bidi"],
  "additionalProperties": false,
  "properties": {
    "word_order": {
      "type": "string",
      "enum": ["svo", "sov", "vso"]
    },
    "word_segmentation": {
      "type": "string",
      "enum": ["space", "none"]
    },
    "requires_explicit_lingua_directive": { "type": "boolean", "default": false },
    "script": {
      "type": "string",
      "enum": ["Latin", "CJK", "Arabic", "Hebrew", "Devanagari",
               "Cyrillic", "Greek", "Hangul", "Thai", "Other"]
    },
    "bidi": {
      "type": "string",
      "enum": ["ltr", "rtl", "bidi"]
    },
    "preferred_decimal_sep": { "type": "string", "enum": [".", ","], "default": "." },
    "indent_unit": { "type": "integer", "minimum": 1, "default": 4 },
    "multiphrasal_keywords": { "type": "boolean", "default": false }
  }
}

GRAM-070. The compiler MUST validate grammar.json against this schema at pack load. Any violation MUST be rejected with error G0002_bad_grammar_json.

GRAM-071. Unknown keys MUST be rejected (schema additionalProperties: false). Forward compatibility is handled by schema versioning (grammar-0.2.json, etc.).

GRAM-072. (Prefix-conflict check) If multiphrasal_keywords: true, the compiler MUST verify that no keyword phrase in keywords.json is a proper prefix of another keyword phrase for a different slot. Violation MUST be rejected with error G0003_keyword_prefix_conflict, with the two conflicting phrases and their slots named.

GRAM-073. If multiphrasal_keywords: false, all keyword phrases in keywords.json MUST contain exactly one word. Violation MUST be rejected with error G0004_multiphrase_not_declared.

10. Appendix C — Normative references

    UAX #9 — Unicode Bidirectional Algorithm (GR-044 relies on §3.3: source is read in logical order)

    RFC 5234 — Augmented BNF for Syntax Specifications (the EBNF notation used here)

    ISO 639-1 — two-letter language codes (#lingua: values)

    SPEC-00 §3.3 — glossary; SPEC-01 §2.4 — token types; SPEC-01 §2.5 — keyword slots

End of SPEC-02 (spec-0.1).