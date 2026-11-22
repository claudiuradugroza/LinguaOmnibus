use std::fmt;

/// Compiler error codes matching SPEC-01/02/03/04/05 error identifiers.
#[derive(Debug)]
pub enum LocError {
    // LEX-001/002
    BomForbidden { offset: usize },
    // LEX-003
    InvalidUtf8 { byte_offset: usize },
    // LEX-005
    NulByte { offset: usize },
    // LEX-008
    ShebangOnlyFirstLine,
    // LEX-033
    BidiInIdentifier { offset: usize },
    // LEX-061/063/064
    BadDigitSeparator { offset: usize },
    IntegerOverflow { literal: String },
    BadEscape { offset: usize },
    InvalidUtf8InString { offset: usize },
    // LEX-0500/0501
    UnterminatedComment { offset: usize },
    CommentNestingLimit,
    // LEX-0600/0602/0603
    UnknownLangpack { code: String },
    BadDirective { offset: usize },
    MissingPragmaArg,
    // LEX-0700
    AmbiguousLingua { packs: Vec<String> },
    // LEX-0510/0511/0512
    TabOrUnicodeIndent { line: usize },
    UnexpectedIndent { line: usize },
    InconsistentDedent { line: usize },
    // LEX-0520/0521
    ChainedComparison,
    MisplacedTry,
    // LEX-0530/0531/0532
    MissingThen,
    OnelineBlock,
    EffectsPosition,
    // GRAM-072/073/0002/0003
    BadGrammarJson { reason: String },
    KeywordPrefixConflict { phrase1: String, phrase2: String },
    MultiphraseNotDeclared,
    // MEM errors
    UseAfterMove { binding: String },
    PartialMoveCopy,
    BorrowConflict,
    DanglingReference,
    ReturnLocalRef,
    LifetimeMismatch,
    LifetimeTransitivity,
    StackOverflow,
    DoubleDrop,
    PanicInDrop,
    QubitNotReleased,
    QubitAlias,
    QubitConsumedByMeasure,
    // TYPE errors
    CannotInferGeneric,
    BoundNotSatisfied,
    InferenceFailed,
    ReturnTypeMismatch,
    LiteralTypeMismatch,
    NegateUnsigned,
    NullOnNonOption,
    UnhandledEffect,
    EffectMismatch,
    InvalidCast,
    // General
    IoError(String),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
    Unicode(String),
    Internal(String),
    // Conformance
    TestFailure { test: String, reason: String },
}

impl fmt::Display for LocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocError::BomForbidden { offset } => write!(f, "L0001_bom_forbidden: BOM at byte offset {}", offset),
            LocError::InvalidUtf8 { byte_offset } => write!(f, "L0002_invalid_utf8: invalid UTF-8 at byte offset {}", byte_offset),
            LocError::NulByte { offset } => write!(f, "L0003_nul_byte: NUL byte at offset {}", offset),
            LocError::ShebangOnlyFirstLine => write!(f, "only the first line may be a shebang"),
            LocError::BidiInIdentifier { offset } => write!(f, "L0200_bidi_in_identifier: bidi control character in identifier at offset {}", offset),
            LocError::BadDigitSeparator { offset } => write!(f, "L0310_bad_digit_separator: invalid digit grouping at offset {}", offset),
            LocError::IntegerOverflow { literal } => write!(f, "L0311_integer_overflow: integer literal {} exceeds width", literal),
            LocError::BadEscape { offset } => write!(f, "L0400_bad_escape: invalid escape sequence at offset {}", offset),
            LocError::InvalidUtf8InString { offset } => write!(f, "L0401_invalid_utf8_in_string: invalid UTF-8 in string at offset {}", offset),
            LocError::UnterminatedComment { offset } => write!(f, "L0500_unterminated_comment: unterminated block comment at offset {}", offset),
            LocError::CommentNestingLimit => write!(f, "L0501_comment_nesting_limit: block comment nesting exceeded"),
            LocError::UnknownLangpack { code } => write!(f, "L0600_unknown_langpack: language pack '{}' not found", code),
            LocError::BadDirective { offset } => write!(f, "L0602_bad_directive: directive must be lowercase at offset {}", offset),
            LocError::MissingPragmaArg => write!(f, "L0603_missing_pragma_arg: pragma missing required argument"),
            LocError::AmbiguousLingua { packs } => write!(f, "L0700_ambiguous_lingua: multiple packs match: {:?}", packs),
            LocError::TabOrUnicodeIndent { line } => write!(f, "L0510_tab_or_unicode_indent: line {} uses non-space indentation", line),
            LocError::UnexpectedIndent { line } => write!(f, "L0511_unexpected_indent: unexpected indent at line {}", line),
            LocError::InconsistentDedent { line } => write!(f, "L0512_inconsistent_dedent: inconsistent dedent at line {}", line),
            LocError::ChainedComparison => write!(f, "L0520_chained_comparison: chained comparisons not allowed"),
            LocError::MisplacedTry => write!(f, "L0521_misplaced_try: ? operator only at statement tail"),
            LocError::MissingThen => write!(f, "L0530_missing_then: then-keyword required after condition"),
            LocError::OnelineBlock => write!(f, "L0531_oneline_block: one-liner bodies not allowed"),
            LocError::EffectsPosition => write!(f, "L0532_effects_position: effects clause in wrong position"),
            LocError::BadGrammarJson { reason } => write!(f, "G0002_bad_grammar_json: {}", reason),
            LocError::KeywordPrefixConflict { phrase1, phrase2 } => write!(f, "G0003_keyword_prefix_conflict: '{}' is prefix of '{}'", phrase1, phrase2),
            LocError::MultiphraseNotDeclared => write!(f, "G0004_multiphrase_not_declared: multiphrasal keyword not declared"),
            LocError::UseAfterMove { binding } => write!(f, "M0101_use_after_move: use of moved binding '{}'", binding),
            LocError::PartialMoveCopy => write!(f, "M0102_partial_move_copy: partial move out of Copy type"),
            LocError::BorrowConflict => write!(f, "M0201/M0202_borrow_conflict: conflicting borrows"),
            LocError::DanglingReference => write!(f, "M0203_dangling_reference: dangling reference"),
            LocError::ReturnLocalRef => write!(f, "M0204_return_local_ref: returning reference to local"),
            LocError::LifetimeMismatch => write!(f, "M0301_lifetime_mismatch: lifetime annotation inconsistent"),
            LocError::LifetimeTransitivity => write!(f, "M0302_lifetime_transitivity: lifetime transitivity violation"),
            LocError::StackOverflow => write!(f, "M0304_stack_overflow: stack overflow"),
            LocError::DoubleDrop => write!(f, "M0401_double_drop: double drop of non-Copy value"),
            LocError::PanicInDrop => write!(f, "M0402_panic_in_drop: panic in drop function"),
            LocError::QubitNotReleased => write!(f, "M0601_qubit_not_released: qubit not measured or released"),
            LocError::QubitAlias => write!(f, "M0602_qubit_alias: qubit aliasing not allowed"),
            LocError::QubitConsumedByMeasure => write!(f, "M0603_qubit_consumed_by_measure: qubit consumed by measure"),
            LocError::CannotInferGeneric => write!(f, "T0201_cannot_infer_generic: cannot infer generic argument"),
            LocError::BoundNotSatisfied => write!(f, "T0202_bound_not_satisfied: generic bound not satisfied"),
            LocError::InferenceFailed => write!(f, "T0203_inference_failed: type inference failed"),
            LocError::ReturnTypeMismatch => write!(f, "T0204_return_type_mismatch: return type mismatch"),
            LocError::LiteralTypeMismatch => write!(f, "T0205_literal_type_mismatch: literal type mismatch"),
            LocError::NegateUnsigned => write!(f, "T0206_negate_unsigned: cannot negate unsigned integer"),
            LocError::NullOnNonOption => write!(f, "T0401_null_on_non_option: null on non-Option type"),
            LocError::UnhandledEffect => write!(f, "E0501_unhandled_effect: effect not handled in context"),
            LocError::EffectMismatch => write!(f, "E0101_effect_mismatch: effect mismatch in subtyping"),
            LocError::InvalidCast => write!(f, "T0601_invalid_cast: invalid cast"),
            LocError::IoError(msg) => write!(f, "IO error: {}", msg),
            LocError::Io(e) => write!(f, "IO error: {}", e),
            LocError::SerdeJson(e) => write!(f, "JSON error: {}", e),
            LocError::Unicode(msg) => write!(f, "Unicode error: {}", msg),
            LocError::Internal(msg) => write!(f, "internal error: {}", msg),
            LocError::TestFailure { test, reason } => write!(f, "test failure '{}': {}", test, reason),
        }
    }
}

impl std::error::Error for LocError {}

impl From<std::io::Error> for LocError {
    fn from(e: std::io::Error) -> Self {
        LocError::Io(e)
    }
}

impl From<serde_json::Error> for LocError {
    fn from(e: serde_json::Error) -> Self {
        LocError::SerdeJson(e)
    }
}

pub type LocResult<T> = Result<T, LocError>;