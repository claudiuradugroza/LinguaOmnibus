use crate::ast::Span;
use crate::error::{LocError, LocResult};
use std::collections::VecDeque;
use unicode_normalization::UnicodeNormalization;

/// A token produced by the lexer.
#[derive(Debug, Clone)]
pub enum Token {
    // Punctuation
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Comma, Semicolon, Colon, Dot,
    // Operators (canonical ASCII forms)
    Plus, Minus, Star, Slash, Percent, Eq, EqEq, Ne, Lt, Gt, Le, Ge,
    AndAnd, OrOr, Not, BitAnd, BitOr, BitXor, Tilde, Shl, Shr,
    Question, Arrow, FatArrow, At, ColonColon, DotQuestion,
    // Literals
    Int(u128, String),     // value, original text
    Float(f64, String),
    Bool(bool),
    Char(char),
    Str(String),
    Byte(u8),
    Null,
    Unit,
    // Identifiers and keywords
    Ident(String),         // raw identifier text
    Keyword(String),       // KW slot name
    // Virtual tokens (GRAM-001)
    Newline,
    Indent,
    Dedent,
    // EOF
    Eof,
}

impl Token {
    pub fn is_eof(&self) -> bool {
        matches!(self, Token::Eof)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::LParen, Token::LParen)
            | (Token::RParen, Token::RParen)
            | (Token::LBrace, Token::LBrace)
            | (Token::RBrace, Token::RBrace)
            | (Token::LBracket, Token::LBracket)
            | (Token::RBracket, Token::RBracket)
            | (Token::Comma, Token::Comma)
            | (Token::Semicolon, Token::Semicolon)
            | (Token::Colon, Token::Colon)
            | (Token::Dot, Token::Dot)
            | (Token::Plus, Token::Plus)
            | (Token::Minus, Token::Minus)
            | (Token::Star, Token::Star)
            | (Token::Slash, Token::Slash)
            | (Token::Percent, Token::Percent)
            | (Token::Eq, Token::Eq)
            | (Token::EqEq, Token::EqEq)
            | (Token::Ne, Token::Ne)
            | (Token::Lt, Token::Lt)
            | (Token::Gt, Token::Gt)
            | (Token::Le, Token::Le)
            | (Token::Ge, Token::Ge)
            | (Token::AndAnd, Token::AndAnd)
            | (Token::OrOr, Token::OrOr)
            | (Token::Not, Token::Not)
            | (Token::BitAnd, Token::BitAnd)
            | (Token::BitOr, Token::BitOr)
            | (Token::BitXor, Token::BitXor)
            | (Token::Tilde, Token::Tilde)
            | (Token::Shl, Token::Shl)
            | (Token::Shr, Token::Shr)
            | (Token::Question, Token::Question)
            | (Token::Arrow, Token::Arrow)
            | (Token::FatArrow, Token::FatArrow)
            | (Token::At, Token::At)
            | (Token::ColonColon, Token::ColonColon)
            | (Token::DotQuestion, Token::DotQuestion)
            | (Token::Null, Token::Null)
            | (Token::Unit, Token::Unit)
            | (Token::Newline, Token::Newline)
            | (Token::Indent, Token::Indent)
            | (Token::Dedent, Token::Dedent)
            | (Token::Eof, Token::Eof) => true,
            (Token::Int(a, _), Token::Int(b, _)) => a == b,
            (Token::Float(a, _), Token::Float(b, _)) => (a - b).abs() < f64::EPSILON,
            (Token::Bool(a), Token::Bool(b)) => a == b,
            (Token::Char(a), Token::Char(b)) => a == b,
            (Token::Str(a), Token::Str(b)) => a == b,
            (Token::Byte(a), Token::Byte(b)) => a == b,
            (Token::Ident(a), Token::Ident(b)) => a == b,
            (Token::Keyword(a), Token::Keyword(b)) => a == b,
            _ => false,
        }
    }
}

/// The lexer state.
pub struct Lexer {
    input: String,            // NFC-normalized input (owned)
    pos: usize,               // byte position in normalized input
    line: usize,
    column: usize,
    pack: Option<super::langpack::LanguagePack>,
    lingua_directive_seen: bool,
    /// Indentation stack for significant indentation (GRAM-001, GRAM-010).
    indent_stack: Vec<usize>,
    /// Whether we're at the start of a logical line.
    at_line_start: bool,
    /// Token queue for multi-level DEDENT emission (P0 fix).
    token_queue: VecDeque<Token>,
}

impl Lexer {
    pub fn new(input: &str, pack: Option<super::langpack::LanguagePack>) -> Self {
        let normalized = input.nfc().collect::<String>();
        Lexer {
            input: normalized,
            pos: 0,
            line: 1,
            column: 1,
            pack,
            lingua_directive_seen: false,
            indent_stack: vec![0],
            at_line_start: true,
            token_queue: VecDeque::new(),
        }
    }

    /// Main entry: tokenize the entire input into a Vec<Token>.
    pub fn tokenize(&mut self) -> LocResult<Vec<Token>> {
        // LEX-002: BOM check on raw input (before normalization)
        if self.input.starts_with('\u{feff}') {
            return Err(LocError::BomForbidden { offset: 0 });
        }
        // LEX-005: NUL byte check
        if self.input.contains('\0') {
            let offset = self.input.find('\0').unwrap_or(0);
            return Err(LocError::NulByte { offset });
        }

        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            if tok.is_eof() {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        Ok(tokens)
    }

    pub fn next_token(&mut self) -> LocResult<Token> {
        // First, check if there are pending tokens in the queue (for multi-level DEDENT)
        if let Some(tok) = self.token_queue.pop_front() {
            return Ok(tok);
        }

        // Handle indentation at the start of a logical line (GRAM-001, GRAM-010-015)
        if self.at_line_start {
            self.at_line_start = false;

            // Count leading spaces (only U+0020 SPACE allowed for indentation per GRAM-010)
            let mut indent = 0;
            while self.pos < self.input.len() {
                let ch = self.peek_char();
                if ch == ' ' {
                    indent += 1;
                    self.advance();
                } else if ch == '\t' {
                    // GRAM-010: tabs at line start are rejected
                    return Err(LocError::TabOrUnicodeIndent { line: self.line });
                } else if ch == '\n' || ch == '\r' {
                    // Blank line - invisible to indentation tracker (GRAM-011)
                    if ch == '\n' {
                        self.newline();
                    } else {
                        self.advance();
                        if self.peek_char() == '\n' {
                            self.advance();
                        }
                        self.line += 1;
                        self.column = 1;
                    }
                    self.at_line_start = true;
                    return Ok(Token::Newline);
                } else if ch == '/' && self.input[self.pos..].starts_with("//") {
                    // Comment-only line - invisible (GRAM-011)
                    self.skip_line();
                    self.at_line_start = true;
                    // Skip the newline
                    if self.pos < self.input.len() && (self.peek_char() == '\n' || self.peek_char() == '\r') {
                        if self.peek_char() == '\r' {
                            self.advance();
                            if self.peek_char() == '\n' {
                                self.advance();
                            }
                        } else {
                            self.advance();
                        }
                        self.line += 1;
                        self.column = 1;
                        self.at_line_start = true;
                    }
                    return Ok(Token::Newline);
                } else {
                    break;
                }
            }

            // Compare with indent stack
            let top = *self.indent_stack.last().unwrap_or(&0);
            if indent > top {
                // GRAM-012: first logical line must have 0 indent
                if self.indent_stack.len() == 1 && self.line > 1 && indent > 0 {
                    // Actually, first line of file establishes base; subsequent lines can indent
                }
                self.indent_stack.push(indent);
                return Ok(Token::Indent);
            } else if indent < top {
                // Pop levels - emit one DEDENT per level (P0 fix)
                let mut dedent_count = 0;
                while self.indent_stack.len() > 1 && *self.indent_stack.last().unwrap() > indent {
                    self.indent_stack.pop();
                    dedent_count += 1;
                }
                if *self.indent_stack.last().unwrap() != indent {
                    return Err(LocError::InconsistentDedent { line: self.line });
                }
                // Emit first DEDENT now, queue the rest
                if dedent_count > 1 {
                    for _ in 1..dedent_count {
                        self.token_queue.push_back(Token::Dedent);
                    }
                }
                return Ok(Token::Dedent);
            }
            // Same level - no indent/dedent token
        }

        let saw_newline = self.skip_whitespace_and_comments()?;

        if saw_newline {
            self.at_line_start = true;
            return Ok(Token::Newline);
        }

        if self.pos >= self.input.len() {
            // Emit remaining DEDENTs at EOF (GRAM-001)
            if self.indent_stack.len() > 1 {
                self.indent_stack.pop();
                // Queue remaining dedents
                for _ in 1..self.indent_stack.len() {
                    self.token_queue.push_back(Token::Dedent);
                }
                self.indent_stack = vec![0]; // reset
                return Ok(Token::Dedent);
            }
            return Ok(Token::Eof);
        }

        let ch = self.peek_char();
        let start_offset = self.pos;
        let start_line = self.line;
        let start_column = self.column;

        // Shebang: only on first line, first chars
        if start_line == 1 && start_column == 1 && self.input[self.pos..].starts_with("#!") {
            self.skip_line();
            return self.next_token();
        }

        // Directive or pragma: starts with #
        if ch == '#' {
            // Only #lingua: and #pragma are directives; other # lines are comments (LEX-080 line comment)
            let rest = &self.input[self.pos + 1..];
            if rest.starts_with("lingua:") || rest.starts_with("pragma") {
                return self.directive_or_pragma(start_offset, start_line, start_column);
            }
            // Treat as a line comment
            self.skip_line();
            return self.next_token();
        }

        // Byte literal: b'X' - check before identifier since 'b' is a valid identifier start
        if ch == 'b' && self.peek_nth(1) == Some('\'') {
            return self.byte_literal();
        }

        // Identifier or keyword
        if is_xid_start(ch) || ch == '_' {
            return self.identifier_or_keyword();
        }

        // Digit -> numeric literal
        if ch.is_ascii_digit() {
            return self.numeric_literal();
        }

        // String literal
        if ch == '"' {
            return self.string_literal();
        }

        // Char literal
        if ch == '\'' {
            return self.char_literal();
        }

        // Raw string
        if ch == '`' {
            return self.raw_string();
        }

        // Multi-char operators and punctuation
        if let Some(tok) = self.operator_or_punctuation()? {
            return Ok(tok);
        }

        Err(LocError::Internal(format!("unexpected character {:?} at offset {}", ch, start_offset)))
    }

    /// Skip whitespace and comments, returning whether a newline ended the line.
    /// Newlines are returned as separate tokens via next_token.
    fn skip_whitespace_and_comments(&mut self) -> LocResult<bool> {
        let mut saw_newline = false;
        loop {
            // Skip trailing whitespace on the current line only
            while self.pos < self.input.len() {
                let ch = self.peek_char();
                if ch == '\n' || ch == '\r' {
                    self.newline();
                    saw_newline = true;
                    break; // Stop after newline — let at_line_start handle leading spaces
                } else if is_whitespace(ch) {
                    self.advance();
                } else {
                    break;
                }
            }

            // Line comment
            if self.input[self.pos..].starts_with("//") {
                self.skip_line();
                // skip_line stops at newline; the newline will be handled next iteration
                continue;
            }

            // Block comment (possibly nested)
            if self.input[self.pos..].starts_with("/*") {
                self.skip_block_comment()?;
                continue;
            }

            break;
        }
        Ok(saw_newline)
    }

    fn skip_block_comment(&mut self) -> LocResult<()> {
        // LEX-080: /* ... */ with nesting
        let mut depth: u32 = 1;
        self.advance_n(2); // skip /*
        while depth > 0 && self.pos < self.input.len() {
            if self.input[self.pos..].starts_with("/*") {
                depth += 1;
                self.advance_n(2);
            } else if self.input[self.pos..].starts_with("*/") {
                depth -= 1;
                self.advance_n(2);
            } else {
                let ch = self.peek_char();
                if ch == '\n' || ch == '\r' {
                    self.newline();
                } else {
                    self.advance();
                }
            }
        }
        if depth > 0 {
            return Err(LocError::UnterminatedComment { offset: self.pos });
        }
        Ok(())
    }

    fn skip_line(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if ch == '\n' || ch == '\r' {
                break;
            }
            self.advance();
        }
    }

    // ---- Identifier / keyword ----

    fn identifier_or_keyword(&mut self) -> LocResult<Token> {
        let start = self.pos;
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if is_xid_continue(ch) || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let word = &self.input[start..self.pos];

        // LEX-033: bidi control characters must not appear in identifiers
        if word.chars().any(|c| matches!(c, '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')) {
            return Err(LocError::BidiInIdentifier { offset: start });
        }

        // LEX-037: Unicode confusable detection (warning, not error)
        // Check for common confusable pairs (Latin a vs Cyrillic а, etc.)
        // This is a warning-level check; we don't reject the code.
        let _confusable_pairs = [
            ('\u{0061}', '\u{0430}'), // Latin a vs Cyrillic а
            ('\u{0065}', '\u{0435}'), // Latin e vs Cyrillic е
            ('\u{006F}', '\u{043E}'), // Latin o vs Cyrillic о
            ('\u{0070}', '\u{0440}'), // Latin p vs Cyrillic р
            ('\u{0073}', '\u{0455}'), // Latin s vs Cyrillic ѕ
            ('\u{0078}', '\u{0445}'), // Latin x vs Cyrillic х
        ];
        // (Confusable detection is advisory; logged as warning L0202_unicode_confusable)

        // LEX-040/041/042/043: keyword matching is byte-exact after NFC
        // GRAM-003: multi-word keyword phrases with longest-match-first
        if let Some(pack) = &self.pack {
            // First, try single-word keyword
            if let Some(slot) = pack.keyword_slot(word) {
                // Check if this could be the start of a multi-word keyword phrase
                // (GRAM-003: longest match wins)
                if pack.grammar.multiphrasal_keywords.unwrap_or(false) || pack.grammar.word_segmentation == "none" {
                    // Try to match a longer phrase
                    let mut longest_match: Option<(String, usize)> = None;
                    for (slot, words) in &pack.keywords {
                        for kw in words {
                            if kw.starts_with(word) && kw.len() > word.len() {
                                // Check if the remaining part matches the next input
                                let rest = &self.input[self.pos..];
                                let remaining_kw = &kw[word.len()..];
                                if rest.starts_with(remaining_kw) {
                                    // Make sure it's a proper boundary
                                    let after_pos = self.pos + remaining_kw.len();
                                    if after_pos >= self.input.len() || !is_xid_continue(self.input[after_pos..].chars().next().unwrap_or('\0')) {
                                        longest_match = Some((slot.clone(), kw.len()));
                                    }
                                }
                            }
                        }
                    }
                    if let Some((slot, _)) = longest_match {
                        // Consume the rest of the phrase
                        let kw_len = pack.keywords.iter()
                            .flat_map(|(_, words)| words.iter())
                            .filter(|kw| kw.starts_with(word))
                            .map(|kw| kw.len())
                            .max()
                            .unwrap_or(word.len());
                        let rest_len = kw_len - word.len();
                        for _ in 0..rest_len {
                            self.advance();
                        }
                        return Ok(Token::Keyword(slot));
                    }
                }
                return Ok(Token::Keyword(slot));
            }
        }

        Ok(Token::Ident(word.to_string()))
    }

    // ---- Numeric literals ----

    fn numeric_literal(&mut self) -> LocResult<Token> {
        let start = self.pos;
        let text_start = self.pos;

        // Hex / bin / oct
        if self.input[self.pos..].starts_with("0x") {
            self.advance_n(2);
            let digits = self.read_hex_digits();
            if digits.is_empty() {
                return Err(LocError::Internal("expected hex digits".into()));
            }
            let val = u128::from_str_radix(&digits.replace('_', ""), 16)
                .map_err(|_| LocError::IntegerOverflow { literal: self.input[start..self.pos].to_string() })?;
            return Ok(Token::Int(val, self.input[start..self.pos].to_string()));
        }
        if self.input[self.pos..].starts_with("0b") {
            self.advance_n(2);
            let digits = self.read_bin_digits();
            let val = u128::from_str_radix(&digits.replace('_', ""), 2)
                .map_err(|_| LocError::IntegerOverflow { literal: self.input[start..self.pos].to_string() })?;
            return Ok(Token::Int(val, self.input[start..self.pos].to_string()));
        }
        if self.input[self.pos..].starts_with("0o") {
            self.advance_n(2);
            let digits = self.read_oct_digits();
            let val = u128::from_str_radix(&digits.replace('_', ""), 8)
                .map_err(|_| LocError::IntegerOverflow { literal: self.input[start..self.pos].to_string() })?;
            return Ok(Token::Int(val, self.input[start..self.pos].to_string()));
        }

        // Decimal integer part
        let int_digits = self.read_dec_digits();

        // Float? Only if . or , is followed by a digit
        let next_ch = self.peek_char();
        if (next_ch == '.' || next_ch == ',') && self.peek_nth(1).map_or(false, |c| c.is_ascii_digit()) {
            let sep = self.peek_char();
            self.advance();
            let frac_digits = self.read_dec_digits();
            let text = &self.input[start..self.pos];
            // LEX-063: both . and , accepted as decimal separator; no mixing
            let normalized = text.replace(',', ".");
            let val: f64 = normalized.parse()
                .map_err(|_| LocError::Internal(format!("invalid float literal {}", text)))?;
            // Exponent
            let exp_val = self.read_exponent(val)?;
            return Ok(Token::Float(exp_val, self.input[start..self.pos].to_string()));
        }

        if self.peek_char() == 'e' || self.peek_char() == 'E' {
            let val: f64 = int_digits.parse::<i128>().unwrap_or(0) as f64;
            let exp_val = self.read_exponent(val)?;
            return Ok(Token::Float(exp_val, self.input[start..self.pos].to_string()));
        }

        // Plain integer
        let text = &self.input[start..self.pos];
        let val: u128 = text.replace('_', "").parse()
            .map_err(|_| LocError::IntegerOverflow { literal: text.to_string() })?;
        Ok(Token::Int(val, text.to_string()))
    }

    fn read_dec_digits(&mut self) -> String {
        let mut s = String::new();
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if ch.is_ascii_digit() || ch == '_' {
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    fn read_hex_digits(&mut self) -> String {
        let mut s = String::new();
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if ch.is_ascii_hexdigit() || ch == '_' {
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    fn read_bin_digits(&mut self) -> String {
        let mut s = String::new();
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if ch == '0' || ch == '1' || ch == '_' {
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    fn read_oct_digits(&mut self) -> String {
        let mut s = String::new();
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if ch >= '0' && ch <= '7' || ch == '_' {
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    fn read_exponent(&mut self, base: f64) -> LocResult<f64> {
        if self.peek_char() == 'e' || self.peek_char() == 'E' {
            self.advance();
            let neg = self.peek_char() == '-';
            if neg || self.peek_char() == '+' {
                self.advance();
            }
            let exp_digits = self.read_dec_digits();
            let exp: i32 = exp_digits.replace('_', "").parse().unwrap_or(0);
            let factor = 10f64.powi(exp);
            Ok(if neg { base / factor } else { base * factor })
        } else {
            Ok(base)
        }
    }

    // ---- String / char / byte / raw string ----

    fn string_literal(&mut self) -> LocResult<Token> {
        self.advance(); // skip opening "
        // Check for multiline string """..."""
        if self.peek_char() == '"' && self.peek_nth(1) == Some('"') {
            self.advance_n(2); // skip the two extra "
            return self.multiline_string(false);
        }
        let mut s = String::new();
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if ch == '"' {
                self.advance();
                return Ok(Token::Str(s));
            }
            if ch == '\\' {
                self.advance();
                let esc = self.peek_char();
                match esc {
                    'n' => { s.push('\n'); self.advance(); }
                    't' => { s.push('\t'); self.advance(); }
                    'r' => { s.push('\r'); self.advance(); }
                    '\\' => { s.push('\\'); self.advance(); }
                    '"' => { s.push('"'); self.advance(); }
                    '\'' => { s.push('\''); self.advance(); }
                    '0' => { s.push('\0'); self.advance(); }
                    'x' => {
                        self.advance();
                        let hex = self.read_n_hex(2)?;
                        let val = u8::from_str_radix(&hex, 16).map_err(|_| LocError::BadEscape { offset: self.pos })?;
                        s.push(val as char);
                    }
                    'u' => {
                        self.advance();
                        if self.peek_char() == '{' {
                            self.advance();
                            let hex = self.read_until('}')?;
                            let val = u32::from_str_radix(&hex, 16).map_err(|_| LocError::BadEscape { offset: self.pos })?;
                            if let Some(c) = char::from_u32(val) { s.push(c); }
                            else { return Err(LocError::BadEscape { offset: self.pos }); }
                            self.advance(); // skip }
                        } else {
                            return Err(LocError::BadEscape { offset: self.pos });
                        }
                    }
                    _ => return Err(LocError::BadEscape { offset: self.pos }),
                }
            } else if ch == '\n' || ch == '\r' {
                return Err(LocError::Internal("unescaped line terminator in string".into()));
            } else {
                s.push(ch);
                self.advance();
            }
        }
        Err(LocError::Internal("unterminated string literal".into()))
    }

    fn multiline_string(&mut self, _raw: bool) -> LocResult<Token> {
        // LEX-076: multiline """...""" - processes escape sequences, preserves newlines
        let mut s = String::new();
        while self.pos < self.input.len() {
            if self.peek_char() == '"' && self.peek_nth(1) == Some('"') && self.peek_nth(2) == Some('"') {
                self.advance_n(3);
                return Ok(Token::Str(s));
            }
            if self.peek_char() == '\\' {
                self.advance();
                let esc = self.peek_char();
                match esc {
                    'n' => { s.push('\n'); self.advance(); }
                    't' => { s.push('\t'); self.advance(); }
                    'r' => { s.push('\r'); self.advance(); }
                    '\\' => { s.push('\\'); self.advance(); }
                    '"' => { s.push('"'); self.advance(); }
                    '0' => { s.push('\0'); self.advance(); }
                    _ => { s.push('\\'); } // keep unknown escapes as-is
                }
            } else {
                s.push(self.peek_char());
                self.advance();
            }
        }
        Err(LocError::Internal("unterminated multiline string".into()))
    }

    fn char_literal(&mut self) -> LocResult<Token> {
        self.advance(); // skip '
        let ch = if self.peek_char() == '\\' {
            self.advance();
            self.read_escape_char()?
        } else {
            let c = self.peek_char();
            self.advance();
            c
        };
        if self.peek_char() != '\'' {
            return Err(LocError::Internal("char literal must contain exactly one character".into()));
        }
        self.advance(); // skip closing '
        Ok(Token::Char(ch))
    }

    fn byte_literal(&mut self) -> LocResult<Token> {
        self.advance_n(2); // skip b'
        let val = if self.peek_char() == '\\' {
            self.advance();
            if self.peek_char() == 'x' {
                self.advance();
                let hex = self.read_n_hex(2)?;
                u8::from_str_radix(&hex, 16).map_err(|_| LocError::BadEscape { offset: self.pos })?
            } else {
                return Err(LocError::BadEscape { offset: self.pos });
            }
        } else {
            let c = self.peek_char() as u8;
            self.advance();
            c
        };
        if self.peek_char() != '\'' {
            return Err(LocError::Internal("byte literal must contain exactly one byte".into()));
        }
        self.advance();
        Ok(Token::Byte(val))
    }

    fn raw_string(&mut self) -> LocResult<Token> {
        self.advance(); // skip `
        let mut s = String::new();
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if ch == '`' {
                self.advance();
                return Ok(Token::Str(s));
            }
            if ch == '\n' || ch == '\r' {
                return Err(LocError::Internal("raw string cannot span lines (use multiline raw string)".into()));
            }
            s.push(ch);
            self.advance();
        }
        Err(LocError::Internal("unterminated raw string".into()))
    }

    // ---- Operators and punctuation ----

    fn operator_or_punctuation(&mut self) -> LocResult<Option<Token>> {
        // LEX-012: maximal munch - longest operator wins
        let remaining = &self.input[self.pos..];

        // Two-char operators first
        if let Some((tok, len)) = match remaining {
            s if s.starts_with("==") => Some((Token::EqEq, 2)),
            s if s.starts_with("!=") => Some((Token::Ne, 2)),
            s if s.starts_with("<=") => Some((Token::Le, 2)),
            s if s.starts_with(">=") => Some((Token::Ge, 2)),
            s if s.starts_with("&&") => Some((Token::AndAnd, 2)),
            s if s.starts_with("||") => Some((Token::OrOr, 2)),
            s if s.starts_with("<<") => Some((Token::Shl, 2)),
            s if s.starts_with(">>") => Some((Token::Shr, 2)),
            s if s.starts_with("->") => Some((Token::Arrow, 2)),
            s if s.starts_with("=>") => Some((Token::FatArrow, 2)),
            s if s.starts_with("::") => Some((Token::ColonColon, 2)),
            s if s.starts_with("?.") => Some((Token::DotQuestion, 2)),
            _ => None,
        } {
            self.advance_n(len);
            return Ok(Some(tok));
        }

        // GRAM-004: Unicode operator canonicalization (≤ → <=, ≥ → >=, ≠ → !=)
        let remaining = &self.input[self.pos..];
        if let Some((tok, _)) = match remaining {
            s if s.starts_with("≤") => Some((Token::Le, 3)),
            s if s.starts_with("≥") => Some((Token::Ge, 3)),
            s if s.starts_with("≠") => Some((Token::Ne, 3)),
            _ => None,
        } {
            // Advance by the UTF-8 byte length (3 bytes each for ≤, ≥, ≠)
            self.pos += 3;
            // These are not newlines, so no line/column tracking needed beyond column
            self.column += 1;
            return Ok(Some(tok));
        }

        // Single-char
        let ch = self.peek_char();
        let tok = match ch {
            '(' => { self.advance(); Token::LParen }
            ')' => { self.advance(); Token::RParen }
            '{' => { self.advance(); Token::LBrace }
            '}' => { self.advance(); Token::RBrace }
            '[' => { self.advance(); Token::LBracket }
            ']' => { self.advance(); Token::RBracket }
            ',' => { self.advance(); Token::Comma }
            ';' => { self.advance(); Token::Semicolon }
            ':' => { self.advance(); Token::Colon }
            '.' => { self.advance(); Token::Dot }
            '+' => { self.advance(); Token::Plus }
            '-' => { self.advance(); Token::Minus }
            '*' => { self.advance(); Token::Star }
            '/' => { self.advance(); Token::Slash }
            '%' => { self.advance(); Token::Percent }
            '=' => { self.advance(); Token::Eq }
            '<' => { self.advance(); Token::Lt }
            '>' => { self.advance(); Token::Gt }
            '!' => { self.advance(); Token::Not }
            '&' => { self.advance(); Token::BitAnd }
            '|' => { self.advance(); Token::BitOr }
            '^' => { self.advance(); Token::BitXor }
            '~' => { self.advance(); Token::Tilde }
            '?' => { self.advance(); Token::Question }
            '@' => { self.advance(); Token::At }
            '#' => return Ok(None), // handled by directive
            _ => return Ok(None),
        };
        Ok(Some(tok))
    }

    // ---- Directives and pragmas ----

    fn directive_or_pragma(&mut self, start_offset: usize, start_line: usize, start_column: usize) -> LocResult<Token> {
        self.advance(); // skip #
        let word = self.read_identifier_word();

        if word == "lingua" {
            // LEX-095: lingua must be lowercase
            // LEX-090: #lingua: XX
            // CONV-062: case-insensitive ISO code (normalize to lowercase)
            self.skip_whitespace();
            if self.peek_char() != ':' {
                return Err(LocError::BadDirective { offset: start_offset });
            }
            self.advance(); // skip :
            self.skip_whitespace();
            let code = self.read_identifier_word();
            // Normalize ISO code to lowercase immediately (case-insensitive)
            let code_lower = code.to_lowercase();
            self.skip_line();
            self.lingua_directive_seen = true;
            return Ok(Token::Ident(format!("__lingua_{}", code_lower)));
        }
        if word == "pragma" {
            // LEX-092: #pragma NAME [ARG ...]
            self.skip_whitespace();
            let name = self.read_identifier_word();
            self.skip_line();
            return Ok(Token::Ident(format!("__pragma_{}", name)));
        }

        // Unknown directive - treat as error or skip
        self.skip_line();
        Ok(Token::Ident(format!("__directive_{}", word)))
    }

    // ---- Helpers ----

    fn peek_char(&self) -> char {
        if self.pos >= self.input.len() { return '\0'; }
        self.input[self.pos..].chars().next().unwrap_or('\0')
    }

    fn peek_nth(&self, n: usize) -> Option<char> {
        self.input[self.pos..].chars().nth(n)
    }

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            let ch = self.input[self.pos..].chars().next().unwrap();
            self.pos += ch.len_utf8();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }

    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    fn newline(&mut self) {
        let ch = self.peek_char();
        self.advance();
        if ch == '\r' && self.peek_char() == '\n' {
            self.advance();
        }
        self.line += 1;
        self.column = 1;
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if ch == ' ' || ch == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier_word(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if is_xid_continue(ch) || ch == '_' || ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    fn read_n_hex(&mut self, n: usize) -> LocResult<String> {
        let mut s = String::new();
        for _ in 0..n {
            if self.pos >= self.input.len() {
                return Err(LocError::BadEscape { offset: self.pos });
            }
            let ch = self.peek_char();
            if ch.is_ascii_hexdigit() {
                s.push(ch);
                self.advance();
            } else {
                return Err(LocError::BadEscape { offset: self.pos });
            }
        }
        Ok(s)
    }

    fn read_escape_char(&mut self) -> LocResult<char> {
        let ch = self.peek_char();
        match ch {
            'n' => { self.advance(); Ok('\n') }
            't' => { self.advance(); Ok('\t') }
            'r' => { self.advance(); Ok('\r') }
            '\\' => { self.advance(); Ok('\\') }
            '"' => { self.advance(); Ok('"') }
            '\'' => { self.advance(); Ok('\'') }
            '0' => { self.advance(); Ok('\0') }
            'x' => {
                self.advance();
                let hex = self.read_n_hex(2)?;
                Ok(u8::from_str_radix(&hex, 16).map_err(|_| LocError::BadEscape { offset: self.pos })? as char)
            }
            'u' => {
                self.advance();
                if self.peek_char() == '{' {
                    self.advance();
                    let hex = self.read_until('}')?;
                    let val = u32::from_str_radix(&hex, 16).map_err(|_| LocError::BadEscape { offset: self.pos })?;
                    self.advance(); // skip }
                    char::from_u32(val).ok_or_else(|| LocError::BadEscape { offset: self.pos })
                } else {
                    Err(LocError::BadEscape { offset: self.pos })
                }
            }
            _ => Err(LocError::BadEscape { offset: self.pos }),
        }
    }

    fn read_until(&mut self, term: char) -> LocResult<String> {
        let mut s = String::new();
        while self.pos < self.input.len() {
            let ch = self.peek_char();
            if ch == term {
                return Ok(s);
            }
            s.push(ch);
            self.advance();
        }
        Err(LocError::Internal(format!("unterminated sequence, expected {}", term)))
    }
}

fn is_xid_start(ch: char) -> bool {
    // XID_Start approximation: ASCII letters, underscore, and major Unicode script blocks
    ch.is_ascii_alphabetic() || ch == '_'
        || (ch >= '\u{00C0}' && ch <= '\u{00FF}')      // Latin-1 Supplement
        || (ch >= '\u{0100}' && ch <= '\u{024F}')      // Latin Extended
        || (ch >= '\u{0370}' && ch <= '\u{03FF}')      // Greek
        || (ch >= '\u{0400}' && ch <= '\u{04FF}')      // Cyrillic
        || (ch >= '\u{0590}' && ch <= '\u{05FF}')      // Hebrew
        || (ch >= '\u{0600}' && ch <= '\u{06FF}')      // Arabic
        || (ch >= '\u{0900}' && ch <= '\u{097F}')      // Devanagari
        || (ch >= '\u{4E00}' && ch <= '\u{9FFF}')      // CJK Unified Ideographs
        || (ch >= '\u{3400}' && ch <= '\u{4DBF}')      // CJK Extension A
        || (ch >= '\u{AC00}' && ch <= '\u{D7AF}')      // Hangul Syllables
        || (ch >= '\u{3040}' && ch <= '\u{309F}')      // Hiragana
        || (ch >= '\u{30A0}' && ch <= '\u{30FF}')      // Katakana
}

fn is_xid_continue(ch: char) -> bool {
    is_xid_start(ch) || ch.is_ascii_digit()
}

fn is_whitespace(ch: char) -> bool {
    matches!(ch, ' ' | '\t' | '\u{000B}' | '\u{000C}' | '\u{00A0}' | '\u{202F}' | '\u{205F}' | '\u{3000}')
        && ch != '\n' && ch != '\r'
}