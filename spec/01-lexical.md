SPEC-01: Lexical Structure
Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1)
Applies to: every .lo source file, every lofmt implementation, every lolsp implementation, every language pack

1. Motivation

The token stream is the first artefact every compiler sees. In a multilingual language, the token stream is the exact point where the natural-language surface ends and universal semantics begins. This document defines:

    how bytes become tokens

    how Unicode is normalized before any decision is made

    how identifiers work across Latin, CJK, Arabic, Hebrew and Devanagari scripts

    how literals express numbers without ambiguity across decimal conventions (1,5 vs 1.5)

    how comments and directives interact with bidirectional (RTL) text

    how a file declares its natural language via #lingua: XX

    how the compiler detects a language when the directive is absent

Every rule is testable. Every rule carries a stable identifier per CONV-010 and is mapped to a conformance test per CONV-012.

2. Normative specification

2.1 Encoding and fundamental constraints

LEX-001. (reifies CONV-070) A .lo source file MUST be UTF-8 encoded without BOM.

LEX-002. If a BOM (U+FEFF) appears as the first code point, the tool MUST reject the file with error L0001_bom_forbidden.

LEX-003. Invalid UTF-8 byte sequences MUST be rejected at lexing time with error L0002_invalid_utf8, reporting the byte offset of the first invalid byte.

LEX-004. (reifies CONV-071) The lexer MUST normalize its input to Unicode NFC before any other processing. Files that are not NFC are accepted; normalization is the compiler's responsibility, not the author's.

LEX-005. A NUL byte (U+0000) MUST be rejected with error L0003_nul_byte.

LEX-006. (reifies CONV-073) Both \n (LF) and \r\n (CRLF) are line terminators. A lone \r not followed by \n MUST be treated as \n. The lexer MUST NOT emit a distinct token for line terminators in spec-0.1.

LEX-007. Source files MUST use extension .lo. Tools MAY accept .lo files from stdin.

LEX-008. If the first line of a source file begins with #!, the entire line (up to and including the next line terminator) MUST be stripped before lexing. Only the first line may be a shebang.

2.2 Tokens and token boundaries

LEX-010. A source file MUST be tokenizable as a sequence of tokens separated by whitespace and comments, ending with exactly one EOF token.

LEX-011. The complete set of token types in spec-0.1 is:

Directives (§2.9) and comments (§2.10) are not emitted as tokens; they are captured into a side-channel metadata record, as specified in the relevant sections.

LEX-012. The lexer MUST apply maximal munch: it consumes the longest prefix of the remaining input that forms a valid token. When two operators share a prefix (< and <=), the longer operator wins.

LEX-013. On a lexing error, an implementation MUST emit the error and MUST attempt recovery by advancing to the next whitespace or punctuation boundary, so that subsequent errors can also be reported. lolsp (§SPEC-09) depends on this behaviour.

2.3 Whitespace

LEX-020. Whitespace is any code point with Unicode property White_Space=Yes except line terminators. This includes U+0020, U+0009, U+000B, U+000C, U+00A0, U+2000–U+200A, U+202F, U+205F, U+3000.

LEX-021. U+000B (vertical tab) SHOULD be accepted as whitespace; lofmt MUST normalize it to U+0020.

LEX-022. U+00A0 (NBSP) MUST be accepted as whitespace; the lexer SHOULD emit warning L0100_nbsp_used with the line and column.

2.4 Identifiers

LEX-030. An identifier MUST start with a code point whose Unicode property is XID_Start, or U+005F LOW LINE (underscore). ASCII A–Z, a–z are included by this rule.

LEX-031. An identifier MUST continue with code points whose Unicode property is XID_Continue, or U+005F LOW LINE.

LEX-032. Implementations MUST accept CJK Unified Ideographs, Hiragana, Katakana, Hangul, Arabic, Hebrew, Devanagari and all Unicode letters within identifiers, by virtue of XID_Start / XID_Continue.

LEX-033. The following bidi control characters MUST NOT appear in an identifier: U+200E, U+200F, U+202A–U+202E, U+2066–U+2069. If present, the lexer MUST reject with error L0200_bidi_in_identifier.

LEX-034. All identifier and keyword matching is byte-exact after NFC (LEX-004). No case-folding, no decomposition, no locale-specific normalization. This rule eliminates the Turkish i / İ / ı / I ambiguity entirely.

LEX-035. Identifiers beginning with two underscores (__) MUST be reserved for the implementation and standard library. User code that declares such an identifier SHOULD trigger warning L0201_reserved_prefix and MUST be accepted.

LEX-036. Implementations MUST support identifiers of at least 255 code points. Implementations SHOULD support 1024.

LEX-037. Implementations SHOULD emit warning L0202_unicode_confusable when two identifiers in the same scope differ only by Unicode confusables (e.g. Latin a vs Cyrillic а). They MUST NOT reject such code.

2.5 Keywords

LEX-040. (reifies CONV-063) A keyword is a token whose byte sequence (after NFC, LEX-004) exactly matches one of the language pack's keyword strings, and which is bounded by non-XID_Continue characters on both sides (for scripts with word segmentation) or by longest-match tokenization (§LEX-042, for scripts without word segmentation).

LEX-041. Keyword matching is byte-exact after NFC (LEX-034). No case-folding.

LEX-042. (ideographic scripts) For packs that declare "word_segmentation": "none" in grammar-notes.md, keyword matching MUST use longest-match-first against all declared keyword strings, then fall back to identifier rules. This is required because CJK scripts do not insert whitespace between words.

LEX-043. A pack with "word_segmentation": "none" MUST NOT declare a keyword that is a proper prefix of another declared keyword in the same pack. This guarantees longest-match tokenization is unambiguous.

LEX-044. (reifies CONV-062) A pack MAY declare "requires_explicit_lingua_directive": true. In that case the compiler MUST require a valid #lingua: directive (LEX-090) and MUST NOT perform auto-detection (LEX-100).

LEX-045. The 40 keyword slots required by CONV-063 are fixed. A pack MAY declare additional slots using identifiers of the form KW_X_{NAME} (CONV-064); such slots MUST be listed in grammar-notes.md.

2.6 Universal punctuation and operators

LEX-050. The set of punctuation tokens is exactly:

( ) { } [ ] , ; : .

These MUST appear verbatim in every language pack and MUST NOT be remapped. Punctuation is the visual spine of code and must remain stable across natural languages.

LEX-051. The set of operator tokens is exactly:

+ - * / % = == != < > <= >= && || ! & | ^ ~ << >> ? -> => @ :: ?.

LEX-052. # is reserved for directives (§2.9) and MUST NOT appear as an operator or punctuation.

LEX-053. $ is reserved for future use (RFC-0003, macro hygiene). Implementations MUST reject $ outside string literals with error L0300_reserved_dollar.

LEX-054. ` (backtick) introduces a raw string (LEX-075) or a multiline raw string (LEX-076).

2.7 Numeric literals

LEX-060. An Integer literal is one of:

LEX-061. A digit grouping separator _ MUST appear only between digits. Leading, trailing, or doubled _ MUST be rejected with error L0310_bad_digit_separator.

LEX-062. A Float literal MUST match one of:

LEX-061. A digit grouping separator _ MUST appear only between digits. Leading, trailing, or doubled _ MUST be rejected with error L0310_bad_digit_separator.

LEX-062. A Float literal MUST match one of:

digits ( "." digits? | "," digits? ) ( [eE] [+-]? digits )?

digits [eE] [+-]? Digits

where digits is [0-9]+ (_ [0-9]+)*.

LEX-063. Both . and , MUST be accepted as the decimal separator. A Float literal MUST NOT contain more than one decimal separator, and MUST NOT mix . and , in the same literal.

LEX-064. 1.500 and 1,500 both denote the Float value 1.5. They MUST NOT be interpreted as the integer 1500. To write the integer 1500, use 1500 or 1_500. This rule is universal: the pack's number_format.decimal_sep (CONV-060/074) is a hint for lofmt output only, and MUST NOT affect lexing.

LEX-065. An Integer literal that exceeds the implementation's declared integer width (SPEC-03) MUST be reported at lexing with error L0311_integer_overflow, not deferred to code generation.

LEX-066. Leading zeros MUST be accepted: 007 is the decimal integer 7. lofmt MUST normalize to 7. 0 alone is decimal zero.

LEX-067. Numeric literal type suffixes (1i32, 1.0f64) MUST NOT exist in spec-0.1. Numeric typing is performed by SPEC-03.

2.8 String, character and byte literals

LEX-070. A String literal is delimited by " and " and occupies a single line (no unescaped line terminator). It MUST be validated as UTF-8 after unescaping.

LEX-071. The following escape sequences are valid in a String:

\n  \t  \r  \\  \"  \'  \0

\xNN              (exactly 2 hex digits; byte value 0x00–0xFF)

\u{H…H}           (1 to 6 hex digits; Unicode scalar value)

Any other backslash sequence MUST be rejected with error L0400_bad_escape.

LEX-072. An escape sequence that produces a byte outside the UTF-8 grammar (e.g. \x80 alone inside a "…" literal) MUST be rejected with L0401_invalid_utf8_in_string.

LEX-073. A Char literal is delimited by ' and ' and MUST contain exactly one Unicode scalar value (one code point or one escape sequence). Empty '' or multi-codepoint 'ab' MUST be rejected.

LEX-074. A Byte literal is written b'…' and MUST contain exactly one ASCII character or one \xNN escape sequence.

LEX-075. A RawString literal is delimited by ` and ` on a single line. It MUST NOT process any escape sequences. A backtick cannot appear inside a raw string; users who need a backtick MUST use a "…" string with `.

LEX-076. A multiline String is """…""" and processes escape sequences; a multiline RawString is ```…``` and does not. Both preserve newlines literally. Neither performs automatic indentation stripping. Users who want dedenting MUST call an explicit standard-library function.

LEX-077. Adjacent String literals separated only by whitespace, comments, or a single line terminator MUST be concatenated by the lexer into a single String token, without any interposed content. This allows long strings to be broken across lines.

LEX-078. Unicode U+2028 (LINE SEPARATOR) and U+2029 (PARAGRAPH SEPARATOR) MUST be preserved literally inside String literals but MUST be re-encoded as \u{2028} / \u{2029} in LIR output (SPEC-06).

LEX-079. Bidi control characters (U+200E, U+200F, U+202A–U+202E, U+2066–U+2069) inside a String or Char literal MUST be preserved as data (they are legitimate content). When the token is emitted to LIR, they MUST be re-encoded as \u{XXXX} escapes so that the LIR file is neutral to bidi display.

2.9 Comments

LEX-080. A line comment starts with // and ends at the next line terminator. A block comment starts with /* and ends with the matching */, counting nested pairs. An unterminated block comment MUST be rejected with error L0500_unterminated_comment.

LEX-081. Implementations MUST support nesting of block comments to at least depth 256. Deeper nesting MAY be accepted; if the limit is exceeded the tool MUST reject with error L0501_comment_nesting_limit.

LEX-082. Comments MUST NOT appear in the token stream. They are removed at lexing time.

LEX-083. Inside a comment, any Unicode code point is legal, including ", ', unbalanced /*, and bidi controls.

LEX-084. A doc-comment — a line comment starting with /// or a block comment starting with /** — MUST be captured as side-channel metadata attached to the next non-comment token. Doc-comments MUST NOT appear as tokens in the regular stream. The metadata schema is defined in SPEC-10; lodoc consumes it.

LEX-085. Bidi control characters inside a doc-comment MUST be preserved as data in the metadata record. lodoc MUST strip them from HTML/Markdown output unless --preserve-bidi is passed.

2.10 Directives and pragmas

LEX-090. A language directive is a line of the form:

#lingua: XX

where XX is an ISO 639-1 two-letter code, or an ISO 639-2 three-letter code if no 639-1 exists. It MUST appear before the first non-comment, non-directive token. At most one language directive MUST appear per file. Case of lingua is fixed lowercase (§LEX-095); case of XX is case-insensitive.

LEX-091. If #lingua: XX names a pack not present in the implementation's bundle, the compiler MUST reject with error L0600_unknown_langpack and MUST list the available packs.

LEX-092. A pragma is a line of the form:

#pragma NAME [ARG ...]

Recognized pragmas in spec-0.1:

LEX-093. Unknown pragmas MUST produce a warning L0601_unknown_pragma and MUST NOT affect parsing. #pragma extension X-vendor for an unknown vendor MUST be treated as unknown.

LEX-094. Directives and pragmas MUST be removed from the token stream after being recorded in the metadata channel.

LEX-095. The literal tokens lingua and pragma MUST be lowercase. Uppercase variants MUST be rejected with error L0602_bad_directive.

LEX-096. A pragma line that is missing a required argument (e.g. #pragma extension with no vendor) MUST be rejected with error L0603_missing_pragma_arg.

2.11 Language auto-detection

LEX-100. Language auto-detection is a tool feature, not a language feature. Implementations MAY perform auto-detection only when #lingua: is absent and #pragma no_auto_detect is absent and the pack does not declare requires_explicit_lingua_directive (LEX-044).

LEX-101. Auto-detection MUST consider, for each candidate pack, the fraction of keyword slot candidates in the file (i.e. identifier tokens that could match any declared keyword string in any pack) that the pack claims.

LEX-102. If exactly one pack claims more than 90% of keyword slot candidates, that pack MUST be selected.

LEX-103. If zero packs claim ≥ 50%, or more than one pack claims ≥ 90%, the tool MUST reject with error L0700_ambiguous_lingua, list the tying packs, and require an explicit #lingua: directive.

LEX-104. The tool MUST record the outcome of auto-detection, including the selected pack and its match ratio, so that loc --explain-detection can report it verbatim.

3. Formal grammar

EBNF, language-independent. Language pack keyword strings substitute for <keyword>. Directives and comments are elided here for brevity; see §2.9 and §2.10.

file            = shebang? directive* token* EOF ;
shebang         = "#!" [^\n]* newline ;
directive       = lingua_directive | pragma ;
lingua_directive= "#lingua:" ws* iso_code ws* newline ;
pragma          = "#pragma" ws+ pragma_name (ws+ pragma_arg)* ws* newline ;
pragma_name     = [a-z_] [a-z0-9_]* ;
pragma_arg      = [^ \t\r\n]+ ;
token           = identifier | keyword | integer | float
                | string_ | char_ | byte_ | raw_string
                | multiline_string | multiline_raw_string
                | punctuation | operator ;
identifier      = xid_start xid_continue* ;
xid_start       = "_" | <XID_Start> ;
xid_continue    = "_" | <XID_Continue> ;
keyword         = <byte-exact match against pack keyword strings, per LEX-040/042> ;
integer         = dec_int | hex_int | bin_int | oct_int ;
dec_int         = digit ( digit | "_" digit )* ;
hex_int         = "0x" hexdigit ( hexdigit | "_" hexdigit )* ;
bin_int         = "0b" [01] ( [01] | "_" [01] )* ;
oct_int         = "0o" [0-7] ( [0-7] | "_" [0-7] )* ;
float           = dec_int ( "." digit* | "," digit* ) exponent?
                | dec_int exponent ;
exponent        = [eE] [+-]? digit+ ;
string_         = '"' ( escape | [^"\\\n\r] )* '"' ;
char_           = "'" ( escape | [^'\\\n\r] ) "'" ;
byte_           = "b'" ( escape | [\x00-\x7F] ) "'" ;
raw_string      = '`' [^`\n\r]* '`' ;
multiline_string= '"""' ( escape | [^"\\] | '"' [^"] | '""' [^"] )* '"""' ;
multiline_raw   = '```' [^`]* '```' ;
escape          = "\\" ( "n" | "t" | "r" | "\\" | '"' | "'" | "0"
                        | "x" hexdigit hexdigit
                        | "u{" hexdigit{1,6} "}" ) ;
punctuation     = "(" | ")" | "{" | "}" | "[" | "]" | "," | ";" | ":" | "." ;
operator        = "==" | "!=" | "<=" | ">=" | "&&" | "||" | "<<" | ">>"
                | "->" | "=>" | "?." | "::"
                | "+" | "-" | "*" | "/" | "%" | "=" | "<" | ">"
                | "!" | "&" | "|" | "^" | "~" | "?" | "@" ;
ws              = <White_Space=Yes, excluding LF and CR> ;
digit           = [0-9] ;
hexdigit        = [0-9a-fA-F] ;
newline         = "\n" | "\r\n" | "\r" ;

Rule. The operator alternation is ordered by decreasing length on purpose: it documents LEX-012 (maximal munch) in the grammar itself.

4. Examples

4.1 Hello world (EN + RO) — token-identical

Already canonical from SPEC-00 §4.1. Both produce identical LIR. No further tokens are introduced in either file.

4.2 Numeric literals — pack-independent

#lingua: EN
var pi_en = 3.14159
var eu_en = 1_000_000.5
#lingua: RO
variabilă pi_ro = 3,14159
variabilă eu_ro = 1_000_000,5

Both files compile to the same Float LIR constants. Per LEX-063/064: . and , are both decimal separators, and _ is the only grouping separator. lofmt normalizes each file to its pack's preferred_decimal_sep.

4.3 CJK identifiers and longest-match tokenization

#lingua: JA
関数 加算(数: 整数) 返す 整数:
    変数 結果 = 数 + 1
    返す 結果

Tokens: Keyword(関数), Identifier(加算), Punctuation, Identifier(数), Punctuation, Keyword(整数), Punctuation, Keyword(返す), Keyword(整数), … The lexer's longest-match-first rule (LEX-042) is what disambiguates 関数加算 into Keyword(関数) + Identifier(加算) rather than any other split.

4.4 Arabic — bidi in identifiers and strings

#lingua: AR
دالة جمع(أ: عدد) ترجع عدد:
    أرجع أ + 1

ة (KW_FUNCTION), جمع (identifier), ترجع (KW_RETURNS), أرجع (KW_RETURN) parse naturally. Bidi marks (U+200E, U+200F, etc.) are not part of the source bytes; they exist only in the editor's display. If a user pastes a string containing bidi marks as data, those marks are preserved per LEX-079 and re-encoded as \u{XXXX} in LIR.

4.5 String escaping across scripts

#lingua: EN
var arabic = "مرحبا\u{200F} بالعالم"
var cjk    = "こんにちは世界"
var emoji  = "hello 👋"

All three are valid UTF-8 strings. The Arabic one contains an explicit RTL mark as \u{200F} so the LIR representation is neutral to display.

4.6 Doc-comments

#lingua: EN
/// Computes the sum of two integers.
/// @param a first addend
/// @param b second addend
function add(a: Integer, b: Integer) returns Integer:
    return a + b

The three /// lines are captured as doc-comment metadata (LEX-084) attached to the function token. lodoc consumes them.

4.7 Nested comments and pragmas

#lingua: EN
#pragma prefer_decimal "."
/* Outer comment
   /* inner nested comment */
   still inside outer */
function main() returns Unit:
    print("ok")

The block comment is depth-2 nested (LEX-081). The pragma is recorded and removed from the stream (LEX-092, LEX-094).

5. Conformance tests

Minimum required at spec-0.1. Each file carries a # covers: LEX-NNN, LEX-NNN header per CONV-013.

That is 36 test files for LEX, matching the M0 minimum of ≥30. Note that LEX-032, LEX-037, LEX-038, LEX-045, LEX-054, LEX-073, LEX-084, LEX-093 are exercised indirectly via other tests; any rule without a dedicated file MUST be added as a sibling test before spec-1.0.

6. Interactions with other sections

SPEC-00: LEX-001…007 reify CONV-070…073. LEX-040…045 reify CONV-060…064. LEX-090…096 provide the concrete grammar for the #lingua: directive referenced by CONV-062.

SPEC-02 (GRAM): consumes the token stream from SPEC-01. Grammar productions reference token types by name.

SPEC-03 (TYPES): LEX-065 (integer overflow) references the integer type bounds defined in SPEC-03.

SPEC-06 (LIR): LEX-078 and LEX-079 require LIR to escape U+2028, U+2029 and bidi controls in string literals.

SPEC-09 (TOOL): lofmt is bound by LEX-021, LEX-064, LEX-066, LEX-085, LEX-092. lolsp depends on LEX-013 for error recovery. lodoc consumes LEX-084 metadata. loc --explain-detection implements LEX-104.

SPEC-10 (STDLIB): the doc-comment metadata schema is defined there (see LEX-084).

7. Open questions

Q1. Grouping _ vs per-pack decimal. LEX-063/064 choose _ for grouping and both ./, for decimals. The alternative — per-pack decimal separator — would prevent source portability (an EN library could not be vendored into a RO project). Proposal: keep the current rule; revisit if a real portability issue emerges.

Q2. Block comment nesting depth. LEX-081 requires depth 256. Some languages (C) disallow nesting. We chose nesting for /* commented-out /* inner */ */ ergonomics. Proposal: keep. Revisit if the implementation cost is high.

Q3. Raw string delimiter. LEX-075 uses a single backtick. Rust uses r#"…"#. Backtick is visually cleaner but cannot contain backticks. Proposal: keep; if RFC-0004 is accepted, add r#"…"# as an alternative.

Q4. Directive name case. LEX-095 requires lowercase lingua and pragma. Alternative: case-insensitive. Lowercase keeps byte-exact matching uniform with keywords (LEX-041). Proposal: keep.

Q5. Directive position. LEX-090 requires #lingua: before first non-comment token. Alternative: allow anywhere. Before-first-code avoids "which directive wins?" ambiguity. Proposal: keep.

Q6. Doc-comment attachment scope. LEX-084 attaches doc-comments to the next token. Alternative: attach to next item (function/struct/etc.). The current approach is more general but pushes more logic to SPEC-10. Proposal: defer to SPEC-10 RFC review.

Q7. Auto-detection threshold. LEX-102 uses 90% / LEX-103 uses 50%. These are heuristics. Proposal: make the thresholds configurable via lopm.toml, but keep the 90% default normative.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft from info.docx. Defines encoding, tokens, identifiers, keywords, punctuation, operators, literals, comments, directives, auto-detection, EBNF grammar, examples, conformance tests, interactions, open questions. |

9. Appendix A — Conformance test file skeleton

# conformance/lex/030/identifiers-cjk.lo
# covers: LEX-030, LEX-031, LEX-042
#lingua: JA
# All of these are valid identifiers under XID_Start / XID_Continue.
関数 test(名前: 文字列) 返す 文字列:
    返す 名前

The test harness asserts loc check returns exit code 0 for this file, and that the token stream reports Keyword(関数), Identifier(test), Punctuation, Identifier(名前), …

10. Appendix B — Normative references

    RFC 3629 — UTF-8, a transformation format of ISO 10646

    RFC 5892 — The Unicode Code Points and Internationalized Domain Names for Applications (IDNA) — normative for XID_Start and XID_Continue

    Unicode 15.0+ — normative for XID_Start, XID_Continue, White_Space, NFC

    UAX #9 — Unicode Bidirectional Algorithm — normative for editor display semantics; non-normative for lexing (the lexer sees logical order only)

    UAX #15 — Unicode Normalization Forms — normative for NFC

    ISO 639-1 / 639-2 / 639-3 — language code identifiers

End of SPEC-01 (spec-0.1).