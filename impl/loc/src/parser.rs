use crate::ast::*;
use crate::error::{LocError, LocResult};
use crate::lexer::{Lexer, Token};

/// Recursive-descent parser implementing SPEC-02 grammar.
pub struct Parser {
    lexer: Lexer,
    tokens: Vec<Token>,
    pos: usize,
    /// Current indentation level stack (for significant indentation).
    indent_stack: Vec<usize>,
    /// Base indentation (must be 0 per GRAM-012).
    base_indent: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            tokens: Vec::new(),
            pos: 0,
            lexer,
            indent_stack: vec![0],
            base_indent: 0,
        }
    }

    /// Parse the entire input into a File AST.
    pub fn parse(mut self) -> LocResult<File> {
        // Collect all tokens first
        loop {
            let tok = self.lexer.next_token()?;
            self.tokens.push(tok.clone());
            if tok.is_eof() {
                break;
            }
        }

        // Extract #lingua: directive from first __lingua_XX token if present
        let _lingua = self.directive_lingua();

        // Parse top-level items
        let mut items = Vec::new();
        while !self.is_eof() {
            // Skip extra newlines / indentation tokens between items
            loop {
                let before = self.pos;
                self.eat_newlines();
                self.skip_indent_tokens();
                if self.pos == before {
                    break;
                }
            }
            if self.is_eof() {
                break;
            }
            let item = self.parse_item()?;
            items.push(item);
        }

        Ok(File {
            lingua: _lingua,
            items,
        })
    }

    /// Extract the #lingua: code from the first __lingua_XX token if present.
    fn directive_lingua(&mut self) -> Option<String> {
        if self.pos < self.tokens.len() {
            if let Token::Ident(s) = &self.tokens[self.pos] {
                if s.starts_with("__lingua_") {
                    let code = s.trim_start_matches("__lingua_").to_string();
                    self.pos += 1;
                    return Some(code);
                }
            }
        }
        None
    }

    // ---------- helpers ----------

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn peek_nth(&self, n: usize) -> &Token {
        self.tokens.get(self.pos + n).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        tok
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    /// Consume only Newline tokens (not INDENT/DEDENT).
    /// Separated from indentation handling per P0 fix.
    fn eat_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) {
            self.advance();
        }
    }

    /// Skip INDENT/DEDENT tokens (used when transitioning between blocks).
    fn skip_indent_tokens(&mut self) {
        while matches!(self.peek(), Token::Indent | Token::Dedent) {
            self.advance();
        }
    }

    fn skip_to_newline(&mut self) {
        while !matches!(self.peek(), Token::Newline | Token::Eof) {
            self.advance();
        }
        self.eat_newlines();
    }

    fn expect(&mut self, expected: &Token) -> LocResult<Token> {
        let tok = self.advance();
        if &tok == expected {
            Ok(tok)
        } else {
            Err(LocError::Internal(format!("expected {:?}, got {:?}", expected, tok)))
        }
    }

    fn expect_keyword(&mut self, slot: &str) -> LocResult<Span> {
        let tok = self.advance();
        if let Token::Keyword(s) = &tok {
            if s == slot {
                return Ok(Span::default());
            }
        }
        Err(LocError::Internal(format!("expected keyword {}, got {:?}", slot, tok)))
    }

    // ---------- item ----------

    fn parse_item(&mut self) -> LocResult<Item> {
        // Skip directive/pragma/lingua markers left by the lexer
        loop {
            match self.peek() {
                Token::Ident(s) if s.starts_with("__pragma_") || s.starts_with("__directive_") || s.starts_with("__lingua_") => {
                    self.advance();
                }
                Token::Keyword(s) if s == "KW_PUB" => {
                    self.advance(); // skip pub
                }
                Token::Newline | Token::Indent | Token::Dedent => {
                    self.advance();
                }
                _ => break,
            }
        }

        let tok = self.peek();

        match tok {
            Token::Keyword(slot) => match slot.as_str() {
                "KW_FUNCTION" => Ok(Item::FunctionDecl(self.parse_function_decl(false)?)),
                "KW_STRUCT" => Ok(Item::StructDecl(self.parse_struct_decl()?)),
                "KW_ENUM" => Ok(Item::EnumDecl(self.parse_enum_decl()?)),
                "KW_CONST" => Ok(Item::ConstDecl(self.parse_const_decl()?)),
                "KW_VAR" => Ok(Item::VarDecl(self.parse_var_decl()?)),
                "KW_IMPORT" => Ok(Item::Import(self.parse_import()?)),
                "KW_PUB" => {
                    self.advance(); // skip pub
                    self.parse_item() // re-parse
                },
                "KW_DB" => {
                    // Skip db statement
                    self.advance();
                    self.skip_to_newline();
                    Ok(Item::Import(Import { path: String::new(), alias: None, span: Span::default() }))
                },
                "KW_QUANTUM" => {
                    self.advance();
                    if matches!(self.peek(), Token::Colon) {
                        self.advance();
                    }
                    self.eat_newlines();
                    let _ = self.parse_block();
                    Ok(Item::Import(Import { path: String::new(), alias: None, span: Span::default() }))
                },
                "KW_INTERFACE" => {
                    self.advance();
                    Err(LocError::Internal("interface parsing not yet implemented".into()))
                }
                "KW_IMPL" => {
                    self.advance();
                    Err(LocError::Internal("impl block parsing not yet implemented".into()))
                }
                "KW_MODULE" => {
                    self.advance();
                    let _name = match self.advance() {
                        Token::Ident(s) => s,
                        t => return Err(LocError::Internal(format!("expected module name, got {:?}", t))),
                    };
                    self.eat_newlines();
                    Ok(Item::Import(self.parse_import()?))
                }
                _ => Err(LocError::Internal(format!("unexpected keyword {} at item level", slot))),
            },
            Token::Ident(s) if s.eq_ignore_ascii_case("extern") => {
                self.advance(); // consume 'extern'
                // optional ABI string: extern "C" function ...
                if let Token::Str(_abi) = self.peek() {
                    self.advance();
                }
                Ok(Item::FunctionDecl(self.parse_function_decl(true)?))
            }
            Token::Ident(s) => {
                if s.starts_with("__pragma_") || s.starts_with("__directive_") || s.starts_with("__lingua_") {
                    self.advance();
                    return self.parse_item();
                }
                if s == "type" {
                    // Skip type alias: type Name = ...
                    self.advance(); // skip type
                    // skip name
                    self.advance();
                    // skip = and the rest
                    self.skip_to_newline();
                    return Ok(Item::Import(Import { path: String::new(), alias: None, span: Span::default() }));
                }
                Err(LocError::Internal(format!("unexpected identifier {} at item level", s)))
            }
            _ => Err(LocError::Internal(format!("unexpected token {:?} at item level", tok))),
        }
    }

    fn parse_import(&mut self) -> LocResult<Import> {
        self.expect_keyword("KW_IMPORT")?;
        // Parse dotted path: std.io.db
        let mut path_parts = Vec::new();
        loop {
            let part = match self.advance() {
                Token::Ident(s) => s,
                Token::Keyword(s) => s,
                t => return Err(LocError::Internal(format!("expected import path part, got {:?}", t))),
            };
            path_parts.push(part);
            if matches!(self.peek(), Token::Dot) {
                self.advance(); // skip dot
            } else {
                break;
            }
        }
        let path = path_parts.join(".");
        let alias = if matches!(self.peek(), Token::Ident(s) if s == "as") {
            self.advance();
            match self.advance() {
                Token::Ident(s) => Some(s),
                t => return Err(LocError::Internal(format!("expected alias, got {:?}", t))),
            }
        } else {
            None
        };
        self.eat_newlines();
        Ok(Import { path, alias, span: Span::default() })
    }

    // ---------- function ----------

    fn parse_function_decl(&mut self, is_extern: bool) -> LocResult<FunctionDecl> {
        if !is_extern {
            self.expect_keyword("KW_FUNCTION")?;
        } else {
            // extern function: 'function' keyword optional
            if matches!(self.peek(), Token::Keyword(s) if s == "KW_FUNCTION") {
                self.advance();
            } else if matches!(self.peek(), Token::Ident(s) if s.eq_ignore_ascii_case("function")) {
                self.advance();
            }
        }
        let name = match self.advance() {
            Token::Ident(s) => s,
            t => return Err(LocError::Internal(format!("expected function name, got {:?}", t))),
        };
        self.expect(&Token::LParen)?;
        let params = self.parse_param_list()?;
        self.expect(&Token::RParen)?;

        let return_type = if matches!(self.peek(), Token::Keyword(s) if s == "KW_RETURN") {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let effects = self.parse_effects_clause()?;

        let body = if is_extern {
            // Optional trailing colon/body for extern declarations
            if matches!(self.peek(), Token::Colon) {
                self.advance();
                self.eat_newlines();
                // consume a body block if present (ignored)
                if !matches!(self.peek(), Token::Eof) && !matches!(self.peek(), Token::Keyword(_) ) {
                    let _ = self.parse_block();
                }
            }
            Block { statements: Vec::new(), span: Span::default() }
        } else {
            self.expect(&Token::Colon)?;
            self.eat_newlines();
            self.parse_block()?
        };

        Ok(FunctionDecl {
            name,
            params,
            return_type,
            effects,
            body,
            is_extern,
            span: Span::default(),
        })
    }

    fn parse_param_list(&mut self) -> LocResult<Vec<Param>> {
        let mut params = Vec::new();
        if matches!(self.peek(), Token::RParen) {
            return Ok(params);
        }
        loop {
            let name = match self.advance() {
                Token::Ident(s) => s,
                t => return Err(LocError::Internal(format!("expected param name, got {:?}", t))),
            };
            self.expect(&Token::Colon)?;
            let ty = self.parse_type()?;
            params.push(Param { name, ty, span: Span::default() });
            if matches!(self.peek(), Token::Comma) {
                self.advance();
                if matches!(self.peek(), Token::RParen) {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(params)
    }

    fn parse_effects_clause(&mut self) -> LocResult<Vec<Effect>> {
        let mut effects = Vec::new();
        if matches!(self.peek(), Token::Keyword(s) if s == "KW_WITH") || matches!(self.peek(), Token::Ident(s) if s == "with") {
            if matches!(self.peek(), Token::Keyword(_)) {
                self.advance(); // skip KW_WITH
            } else {
                self.advance(); // skip "with" Ident
            }
            self.expect(&Token::LBracket)?;
            loop {
                let eff = match self.advance() {
                    Token::Keyword(s) => match s.as_str() {
                        "KW_IO" => Effect::Io,
                        "KW_DATABASE" => Effect::Database,
                        "KW_NETWORK" => Effect::Network,
                        "KW_FILESYSTEM" => Effect::Filesystem,
                        "KW_QUANTUM" => Effect::Quantum,
                        other => return Err(LocError::Internal(format!("unknown effect {}", other))),
                    },
                    Token::Ident(s) => match s.to_uppercase().as_str() {
                        "IO" => Effect::Io,
                        "DATABASE" => Effect::Database,
                        "NETWORK" => Effect::Network,
                        "FILESYSTEM" => Effect::Filesystem,
                        "QUANTUM" => Effect::Quantum,
                        other => return Err(LocError::Internal(format!("unknown effect {}", other))),
                    },
                    t => return Err(LocError::Internal(format!("expected effect, got {:?}", t))),
                };
                effects.push(eff);
                if matches!(self.peek(), Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            self.expect(&Token::RBracket)?;
        }
        Ok(effects)
    }

    // ---------- struct ----------

    fn parse_struct_decl(&mut self) -> LocResult<StructDecl> {
        self.expect_keyword("KW_STRUCT")?;
        let name = match self.advance() {
            Token::Ident(s) => s,
            t => return Err(LocError::Internal(format!("expected struct name, got {:?}", t))),
        };
        let mut fields = Vec::new();
        if matches!(self.peek(), Token::LBrace) {
            self.advance(); // skip {
            while !matches!(self.peek(), Token::RBrace) {
                let fname = match self.advance() {
                    Token::Ident(s) => s,
                    t => return Err(LocError::Internal(format!("expected field name, got {:?}", t))),
                };
                self.expect(&Token::Colon)?;
                let fty = self.parse_type()?;
                fields.push(Field { name: fname, ty: fty, span: Span::default() });
                if matches!(self.peek(), Token::Comma) {
                    self.advance();
                }
            }
            self.expect(&Token::RBrace)?;
        } else if matches!(self.peek(), Token::Colon) {
            self.advance(); // skip :
            self.eat_newlines();
            self.skip_indent_tokens();
            // Indentation-based fields
            loop {
                self.eat_newlines();
                // skip stray indents but NOT dedent (dedent ends the block)
                while matches!(self.peek(), Token::Indent) {
                    self.advance();
                }
                if self.is_eof() {
                    break;
                }
                if matches!(self.peek(), Token::Dedent) {
                    self.advance();
                    break;
                }
                // Check if we've hit a top-level keyword
                if let Token::Keyword(slot) = self.peek() {
                    if ["KW_FUNCTION", "KW_STRUCT", "KW_ENUM", "KW_CONST", "KW_VAR", "KW_IMPORT", "KW_INTERFACE", "KW_IMPL", "KW_PUB"].contains(&slot.as_str()) {
                        break;
                    }
                }
                // Also stop at __lingua_ or __directive_ markers
                if let Token::Ident(s) = self.peek() {
                    if s.starts_with("__") {
                        break;
                    }
                }
                // Parse field: name: type
                let fname = match self.advance() {
                    Token::Ident(s) => s,
                    Token::Keyword(s) => s,
                    t => return Err(LocError::Internal(format!("expected field name, got {:?}", t))),
                };
                self.expect(&Token::Colon)?;
                let fty = self.parse_type()?;
                fields.push(Field { name: fname, ty: fty, span: Span::default() });
                self.eat_newlines();
            }
        }
        self.eat_newlines();
        Ok(StructDecl { name, fields, span: Span::default() })
    }

    // ---------- enum ----------

    fn parse_enum_decl(&mut self) -> LocResult<EnumDecl> {
        self.expect_keyword("KW_ENUM")?;
        let name = match self.advance() {
            Token::Ident(s) => s,
            t => return Err(LocError::Internal(format!("expected enum name, got {:?}", t))),
        };
        let mut variants = Vec::new();
        if matches!(self.peek(), Token::LBrace) {
            self.advance(); // skip {
            while !matches!(self.peek(), Token::RBrace) {
                let vname = match self.advance() {
                    Token::Ident(s) => s,
                    t => return Err(LocError::Internal(format!("expected variant name, got {:?}", t))),
                };
                let args = if matches!(self.peek(), Token::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.peek(), Token::RParen) {
                        args.push(self.parse_type()?);
                        if matches!(self.peek(), Token::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(&Token::RParen)?;
                    args
                } else {
                    Vec::new()
                };
                variants.push(Variant { name: vname, args, span: Span::default() });
                if matches!(self.peek(), Token::Comma) {
                    self.advance();
                }
            }
            self.expect(&Token::RBrace)?;
        }
        self.eat_newlines();
        Ok(EnumDecl { name, variants, span: Span::default() })
    }

    // ---------- const ----------

    fn parse_const_decl(&mut self) -> LocResult<ConstDecl> {
        self.expect_keyword("KW_CONST")?;
        let name = match self.advance() {
            Token::Ident(s) => s,
            t => return Err(LocError::Internal(format!("expected const name, got {:?}", t))),
        };
        self.expect(&Token::Colon)?;
        let ty = self.parse_type()?;
        self.expect(&Token::Eq)?;
        let value = self.parse_expr()?;
        self.eat_newlines();
        Ok(ConstDecl { name, ty, value, span: Span::default() })
    }

    // ---------- var ----------

    fn parse_var_decl(&mut self) -> LocResult<VarDecl> {
        self.expect_keyword("KW_VAR")?;
        let name = match self.advance() {
            Token::Ident(s) => s,
            t => return Err(LocError::Internal(format!("expected var name, got {:?}", t))),
        };
        let ty = if matches!(self.peek(), Token::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let value = if matches!(self.peek(), Token::Eq) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.eat_newlines();
        Ok(VarDecl { name, ty, value, span: Span::default() })
    }

    // ---------- type ----------

    fn parse_type(&mut self) -> LocResult<Type> {
        let tok = self.advance();
        match tok {
            // Array type [T]
            Token::LBracket => {
                let inner = self.parse_type()?;
                self.expect(&Token::RBracket)?;
                Ok(Type::Array(Box::new(inner)))
            },
            Token::Keyword(slot) => match slot.as_str() {
                "KW_INT" => Ok(Type::Int32),
                "KW_FLOAT" => Ok(Type::F64),
                "KW_BOOL" => Ok(Type::Bool),
                "KW_STRING" => Ok(Type::String),
                "KW_CHAR" => Ok(Type::Char),
                "KW_BYTE" => Ok(Type::Byte),
                "KW_UNIT" => Ok(Type::Unit),
                "KW_QUBIT" => Ok(Type::Qubit),
                "KW_OPTION" => {
                    self.expect(&Token::Lt)?;
                    let inner = self.parse_type()?;
                    self.expect(&Token::Gt)?;
                    Ok(Type::Option(Box::new(inner)))
                }
                "KW_RESULT" => {
                    self.expect(&Token::Lt)?;
                    let t = self.parse_type()?;
                    self.expect(&Token::Comma)?;
                    let e = self.parse_type()?;
                    self.expect(&Token::Gt)?;
                    Ok(Type::Result(Box::new(t), Box::new(e)))
                }
                "KW_MAP" => {
                    self.expect(&Token::Lt)?;
                    let k = self.parse_type()?;
                    self.expect(&Token::Comma)?;
                    let v = self.parse_type()?;
                    self.expect(&Token::Gt)?;
                    Ok(Type::Map(Box::new(k), Box::new(v)))
                }
                other => Ok(Type::Path(other.to_string())),
            },
            Token::Ident(s) => {
                // Check for primitive type names
                let ty = match s.as_str() {
                    "int8" => Type::Int8,
                    "int16" => Type::Int16,
                    "int32" => Type::Int32,
                    "int64" => Type::Int64,
                    "int128" => Type::Int128,
                    "uint8" => Type::Uint8,
                    "uint16" => Type::Uint16,
                    "uint32" => Type::Uint32,
                    "uint64" => Type::Uint64,
                    "uint128" => Type::Uint128,
                    "f32" => Type::F32,
                    "f64" => Type::F64,
                    "f32x4" => Type::VecF32x4,
                    "f32x8" => Type::VecF32x8,
                    "f64x2" => Type::VecF64x2,
                    "i32x4" => Type::VecI32x4,
                    "i32x8" => Type::VecI32x8,
                    "i64x2" => Type::VecI64x2,
                    "bool" => Type::Bool,
                    "char" => Type::Char,
                    "byte" => Type::Byte,
                    "unit" => Type::Unit,
                    "string" => Type::String,
                    "String" => Type::String,
                    "qubit" => Type::Qubit,
                    "Result" => Type::Result(Box::new(Type::Int32), Box::new(Type::Int32)),
                    _ => {
                        // Check for generic type like QReg<5>, Option<Integer>
                        if matches!(self.peek(), Token::Lt) {
                            self.advance(); // skip <
                            let args = self.parse_type_args()?;
                            self.expect(&Token::Gt)?;
                            if s == "QReg" {
                                Type::QReg
                            } else if s == "QCircuit" {
                                Type::QCircuit
                            } else if s == "QState" {
                                Type::QState
                            } else {
                                Type::Path(format!("{}<{}>", s, args.join(", ")))
                            }
                        } else if matches!(self.peek(), Token::LBracket) {
                            // Array type [T]
                            self.advance(); // skip [
                            let inner = self.parse_type()?;
                            self.expect(&Token::RBracket)?;
                            Type::Array(Box::new(inner))
                        } else {
                            Type::Path(s)
                        }
                    }
                };
                Ok(ty)
            }
            Token::LParen => {
                if matches!(self.peek(), Token::RParen) {
                    self.advance(); // skip )
                    return Ok(Type::Unit);
                }
                // Tuple type (T1, T2, ...)
                let mut types = Vec::new();
                loop {
                    types.push(self.parse_type()?);
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                        if matches!(self.peek(), Token::RParen) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                self.expect(&Token::RParen)?;
                Ok(Type::Tuple(types))
            }
            t => Err(LocError::Internal(format!("expected type, got {:?}", t))),
        }
    }

    fn parse_type_args(&mut self) -> LocResult<Vec<String>> {
        let mut args = Vec::new();
        loop {
            let arg = match self.advance() {
                Token::Int(v, _) => v.to_string(),
                Token::Ident(s) => s,
                Token::Keyword(s) => s,
                t => return Err(LocError::Internal(format!("expected type arg, got {:?}", t))),
            };
            args.push(arg);
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(args)
    }

    // ---------- block ----------

    fn parse_block(&mut self) -> LocResult<Block> {
        let mut statements = Vec::new();

        // Consume leading indentation / newline tokens (not DEDENT, which ends a block)
        while matches!(self.peek(), Token::Indent | Token::Newline) {
            self.advance();
        }

        // Support both { ... } and indentation-based blocks
        if matches!(self.peek(), Token::LBrace) {
            self.advance(); // skip {
            // consume leading indentation inside the braces
            while matches!(self.peek(), Token::Indent | Token::Newline) {
                self.advance();
            }
            while !matches!(self.peek(), Token::RBrace) {
                if self.is_eof() {
                    break;
                }
                while matches!(self.peek(), Token::Indent | Token::Dedent | Token::Newline) {
                    self.advance();
                }
                if matches!(self.peek(), Token::RBrace) {
                    break;
                }
                let stmt = self.parse_statement()?;
                statements.push(stmt);
                while matches!(self.peek(), Token::Indent | Token::Dedent | Token::Newline) {
                    self.advance();
                }
            }
            self.expect(&Token::RBrace)?;
        } else {
            // Strict indentation-based block parsing
            // Consume leading INDENT (one level) and newlines at entry
            while matches!(self.peek(), Token::Indent | Token::Newline) {
                self.advance();
            }

            loop {
                // Skip newlines between statements
                while matches!(self.peek(), Token::Newline) {
                    self.advance();
                }
                // Skip stray indents
                while matches!(self.peek(), Token::Indent) {
                    self.advance();
                }

                if self.is_eof() {
                    break;
                }

                // DEDENT signals end of this block: consume one level and stop
                if matches!(self.peek(), Token::Dedent) {
                    self.advance();
                    break;
                }

                // Check for top-level declaration keywords that end the block
                if let Token::Keyword(slot) = self.peek() {
                    if ["KW_FUNCTION", "KW_STRUCT", "KW_ENUM",
                        "KW_IMPORT", "KW_INTERFACE", "KW_IMPL",
                        "KW_ELSE", "KW_RBRACE"].contains(&slot.as_str()) {
                        break;
                    }
                }

                // End of braced block
                if matches!(self.peek(), Token::RBrace) {
                    break;
                }

                let stmt = self.parse_statement()?;
                statements.push(stmt);
            }
        }

        Ok(Block { statements, span: Span::default() })
    }

    // ---------- statement ----------

    fn parse_statement(&mut self) -> LocResult<Statement> {
        // Skip leading indentation/newline tokens (not DEDENT, which ends a block)
        while matches!(self.peek(), Token::Indent | Token::Newline) {
            self.advance();
        }
        let tok = self.peek();

        // Inline assembly: asm "..." or asm { "..." }
        if let Token::Ident(s) = tok {
            if s.eq_ignore_ascii_case("asm") {
                self.advance(); // consume asm
                let code = if matches!(self.peek(), Token::LBrace) {
                    self.advance(); // skip {
                    self.eat_newlines();
                    let mut parts = Vec::new();
                    while !matches!(self.peek(), Token::RBrace) {
                        if self.is_eof() { break; }
                        if let Token::Str(s) = self.peek() {
                            parts.push(s.clone());
                            self.advance();
                        } else {
                            self.advance();
                        }
                        self.eat_newlines();
                    }
                    self.expect(&Token::RBrace)?;
                    parts.join(" ")
                } else {
                    match self.advance() {
                        Token::Str(s) => s,
                        t => return Err(LocError::Internal(format!("expected asm string, got {:?}", t))),
                    }
                };
                self.eat_newlines();
                return Ok(Statement::AsmStmt(code, Span::default()));
            }
        }

        match tok {
            Token::Keyword(slot) => match slot.as_str() {
                "KW_VAR" => Ok(Statement::VarDecl(self.parse_var_decl()?)),
                "KW_CONST" => Ok(Statement::ConstDecl(self.parse_const_decl()?)),
                "KW_IF" => Ok(Statement::IfStmt(self.parse_if_stmt()?)),
                "KW_WHILE" => Ok(Statement::WhileStmt(self.parse_while_stmt()?)),
                "KW_FOR" => Ok(Statement::ForStmt(self.parse_for_stmt()?)),
                "KW_MATCH" => Ok(Statement::MatchStmt(self.parse_match_stmt()?)),
                "KW_RETURN" => Ok(Statement::ReturnStmt(self.parse_return_stmt()?)),
                "KW_BREAK" => {
                    self.advance();
                    self.eat_newlines();
                    Ok(Statement::BreakStmt(Span::default()))
                }
                "KW_CONTINUE" => {
                    self.advance();
                    self.eat_newlines();
                    Ok(Statement::ContinueStmt(Span::default()))
                }
                "KW_QUANTUM" => {
                    self.advance();
                    self.expect(&Token::Colon)?;
                    self.eat_newlines();
                    let body = self.parse_block()?;
                    Ok(Statement::QuantumBlock(body))
                }
                _ => Ok(Statement::ExprStmt(self.parse_expr()?, Span::default())),
            },
            _ => Ok(Statement::ExprStmt(self.parse_expr()?, Span::default())),
        }
    }

    fn parse_if_stmt(&mut self) -> LocResult<IfStmt> {
        self.expect_keyword("KW_IF")?;
        let cond = self.parse_expr()?;
        self.expect_keyword("KW_THEN")?;
        self.expect(&Token::Colon)?;
        self.eat_newlines();
        let then_block = self.parse_block()?;

        let else_block = if matches!(self.peek(), Token::Keyword(s) if s == "KW_ELSE") {
            self.advance();
            self.expect(&Token::Colon)?;
            self.eat_newlines();
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(IfStmt { cond, then_block, else_block, span: Span::default() })
    }

    fn parse_while_stmt(&mut self) -> LocResult<WhileStmt> {
        self.expect_keyword("KW_WHILE")?;
        let cond = self.parse_expr()?;
        self.expect(&Token::Colon)?;
        self.eat_newlines();
        let body = self.parse_block()?;
        Ok(WhileStmt { cond, body, span: Span::default() })
    }

    fn parse_for_stmt(&mut self) -> LocResult<ForStmt> {
        self.expect_keyword("KW_FOR")?;
        let pattern = match self.advance() {
            Token::Ident(s) => s,
            t => return Err(LocError::Internal(format!("expected for pattern, got {:?}", t))),
        };
        self.expect_keyword("KW_IN")?;
        let iterable = self.parse_expr()?;
        self.expect(&Token::Colon)?;
        self.eat_newlines();
        let body = self.parse_block()?;
        Ok(ForStmt { pattern, iterable, body, span: Span::default() })
    }

    fn parse_match_stmt(&mut self) -> LocResult<MatchStmt> {
        self.expect_keyword("KW_MATCH")?;
        let expr = self.parse_expr()?;
        self.expect(&Token::Colon)?;
        self.eat_newlines();
        let mut arms = Vec::new();
        loop {
            self.eat_newlines();
            if self.is_eof() {
                break;
            }
            let pattern = self.parse_pattern()?;
            self.expect(&Token::Colon)?;
            self.eat_newlines();
            let body = self.parse_block()?;
            arms.push(MatchArm { pattern, body, span: Span::default() });
            if self.is_eof() {
                break;
            }
        }
        Ok(MatchStmt { expr, arms, span: Span::default() })
    }

    fn parse_pattern(&mut self) -> LocResult<Pattern> {
        let tok = self.advance();
        match tok {
            Token::Ident(s) => Ok(Pattern::Ident(s)),
            Token::Keyword(slot) => match slot.as_str() {
                "KW_TRUE" => Ok(Pattern::Literal(Literal::Bool(true))),
                "KW_FALSE" => Ok(Pattern::Literal(Literal::Bool(false))),
                "KW_NULL" => Ok(Pattern::Literal(Literal::Null)),
                other => Ok(Pattern::Ident(other.to_string())),
            },
            Token::Int(v, _) => Ok(Pattern::Literal(Literal::Int(v as i128))),
            Token::Str(s) => Ok(Pattern::Literal(Literal::String(s))),
            _ => Ok(Pattern::Wildcard),
        }
    }

    fn parse_return_stmt(&mut self) -> LocResult<ReturnStmt> {
        self.expect_keyword("KW_RETURN")?;
        let value = if self.is_expr_start() {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.eat_newlines();
        Ok(ReturnStmt { value, span: Span::default() })
    }

    fn is_expr_start(&self) -> bool {
        matches!(self.peek(),
            Token::Ident(_) | Token::Int(_,_) | Token::Float(_,_) | Token::Str(_) |
            Token::Char(_) | Token::Byte(_) | Token::Keyword(_) | Token::LParen |
            Token::LBracket | Token::LBrace | Token::Not | Token::Minus | Token::Tilde
        )
    }

    // ---------- expressions (precedence climbing) ----------

    fn parse_expr(&mut self) -> LocResult<Expr> {
        self.parse_assign_expr()
    }

    fn parse_assign_expr(&mut self) -> LocResult<Expr> {
        // Try assignment: lvalue = expr
        let saved_pos = self.pos;
        let lvalue = self.parse_postfix();
        if let Ok(lhs) = lvalue {
            if matches!(self.peek(), Token::Eq) {
                self.advance(); // =
                let value = self.parse_expr()?;
                self.eat_newlines();
                return Ok(Expr::Assign { target: Box::new(lhs), value: Box::new(value), span: Span::default() });
            }
        }
        // Not an assignment: restore and parse normal expression
        self.pos = saved_pos;
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> LocResult<Expr> {
        let mut left = self.parse_logical_and()?;
        while matches!(self.peek(), Token::OrOr) || matches!(self.peek(), Token::Keyword(s) if s == "KW_OR") {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expr::BinaryOp { op: BinaryOp::Or, left: Box::new(left), right: Box::new(right), span: Span::default() };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> LocResult<Expr> {
        let mut left = self.parse_bit_or()?;
        while matches!(self.peek(), Token::AndAnd) || matches!(self.peek(), Token::Keyword(s) if s == "KW_AND") {
            self.advance();
            let right = self.parse_bit_or()?;
            left = Expr::BinaryOp { op: BinaryOp::And, left: Box::new(left), right: Box::new(right), span: Span::default() };
        }
        Ok(left)
    }

    fn parse_bit_or(&mut self) -> LocResult<Expr> {
        self.parse_binary_chain(Self::parse_bit_xor, &[Token::BitOr], BinaryOp::BitOr)
    }

    fn parse_bit_xor(&mut self) -> LocResult<Expr> {
        self.parse_binary_chain(Self::parse_bit_and, &[Token::BitXor], BinaryOp::BitXor)
    }

    fn parse_bit_and(&mut self) -> LocResult<Expr> {
        self.parse_binary_chain(Self::parse_equality, &[Token::BitAnd], BinaryOp::BitAnd)
    }

    fn parse_equality(&mut self) -> LocResult<Expr> {
        self.parse_binary_chain(Self::parse_comparison, &[Token::EqEq, Token::Ne], BinaryOp::Eq)
    }

    fn parse_comparison(&mut self) -> LocResult<Expr> {
        // GRAM-022: no chained comparisons
        let left = self.parse_shift()?;
        if matches!(self.peek(), Token::Lt | Token::Le | Token::Gt | Token::Ge) {
            let op_tok = self.advance();
            let right = self.parse_shift()?;
            // Check for chaining
            if matches!(self.peek(), Token::Lt | Token::Le | Token::Gt | Token::Ge) {
                return Err(LocError::ChainedComparison);
            }
            let op = match op_tok {
                Token::Lt => BinaryOp::Lt,
                Token::Le => BinaryOp::Le,
                Token::Gt => BinaryOp::Gt,
                Token::Ge => BinaryOp::Ge,
                _ => unreachable!(),
            };
            return Ok(Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right), span: Span::default() });
        }
        Ok(left)
    }

    fn parse_shift(&mut self) -> LocResult<Expr> {
        self.parse_binary_chain(Self::parse_additive, &[Token::Shl, Token::Shr], BinaryOp::Add)
    }

    fn parse_additive(&mut self) -> LocResult<Expr> {
        self.parse_binary_chain(Self::parse_multiplicative, &[Token::Plus, Token::Minus], BinaryOp::Add)
    }

    fn parse_multiplicative(&mut self) -> LocResult<Expr> {
        self.parse_binary_chain(Self::parse_unary, &[Token::Star, Token::Slash, Token::Percent], BinaryOp::Mul)
    }

    fn parse_binary_chain<F>(&mut self, next: F, ops: &[Token], _default_op: BinaryOp) -> LocResult<Expr>
    where F: Fn(&mut Self) -> LocResult<Expr> {
        let mut left = next(self)?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Rem,
                Token::EqEq => BinaryOp::Eq,
                Token::Ne => BinaryOp::Ne,
                Token::Lt => BinaryOp::Lt,
                Token::Le => BinaryOp::Le,
                Token::Gt => BinaryOp::Gt,
                Token::Ge => BinaryOp::Ge,
                Token::AndAnd => BinaryOp::And,
                Token::OrOr => BinaryOp::Or,
                Token::BitAnd => BinaryOp::BitAnd,
                Token::BitOr => BinaryOp::BitOr,
                Token::BitXor => BinaryOp::BitXor,
                Token::Shl => BinaryOp::Shl,
                Token::Shr => BinaryOp::Shr,
                _ => break,
            };
            // Only continue if this op is in our allowed set
            let tok = self.peek().clone();
            let allowed = match tok {
                Token::Plus => ops.contains(&Token::Plus),
                Token::Minus => ops.contains(&Token::Minus),
                Token::Star => ops.contains(&Token::Star),
                Token::Slash => ops.contains(&Token::Slash),
                Token::Percent => ops.contains(&Token::Percent),
                Token::EqEq => ops.contains(&Token::EqEq),
                Token::Ne => ops.contains(&Token::Ne),
                Token::Lt => ops.contains(&Token::Lt),
                Token::Le => ops.contains(&Token::Le),
                Token::Gt => ops.contains(&Token::Gt),
                Token::Ge => ops.contains(&Token::Ge),
                Token::AndAnd => ops.contains(&Token::AndAnd),
                Token::OrOr => ops.contains(&Token::OrOr),
                Token::BitAnd => ops.contains(&Token::BitAnd),
                Token::BitOr => ops.contains(&Token::BitOr),
                Token::BitXor => ops.contains(&Token::BitXor),
                Token::Shl => ops.contains(&Token::Shl),
                Token::Shr => ops.contains(&Token::Shr),
                _ => false,
            };
            if !allowed {
                break;
            }
            self.advance();
            let right = next(self)?;
            left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right), span: Span::default() };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> LocResult<Expr> {
        match self.peek() {
            Token::Not => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp { op: UnaryOp::Not, operand: Box::new(operand), span: Span::default() })
            }
            Token::Keyword(s) if s == "KW_NOT" => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp { op: UnaryOp::Not, operand: Box::new(operand), span: Span::default() })
            }
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp { op: UnaryOp::Neg, operand: Box::new(operand), span: Span::default() })
            }
            Token::Tilde => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp { op: UnaryOp::BitNot, operand: Box::new(operand), span: Span::default() })
            }
            Token::BitAnd => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp { op: UnaryOp::Neg, operand: Box::new(operand), span: Span::default() }) // placeholder for reference
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> LocResult<Expr> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek() {
                Token::Dot => {
                    self.advance();
                    let field = match self.advance() {
                        Token::Ident(s) => s,
                        t => return Err(LocError::Internal(format!("expected field name, got {:?}", t))),
                    };
                    expr = Expr::FieldAccess { object: Box::new(expr), field, span: Span::default() };
                }
                Token::ColonColon => {
                    self.advance();
                    let part = match self.advance() {
                        Token::Ident(s) => s,
                        Token::Keyword(s) => s,
                        t => return Err(LocError::Internal(format!("expected path part, got {:?}", t))),
                    };
                    expr = Expr::FieldAccess { object: Box::new(expr), field: part, span: Span::default() };
                }
                Token::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Token::RParen) {
                        loop {
                            args.push(self.parse_expr()?);
                            if matches!(self.peek(), Token::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(&Token::RParen)?;
                    expr = Expr::Call { callee: Box::new(expr), args, span: Span::default() };
                }
                Token::LBracket => {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect(&Token::RBracket)?;
                    expr = Expr::Index { object: Box::new(expr), index: Box::new(index), span: Span::default() };
                }
                Token::Question => {
                    self.advance();
                    // GRAM-024: ? only at statement tail - we accept it syntactically here
                }
                Token::Ident(s) if s == "as" => {
                    self.advance();
                    let target_type = self.parse_type()?;
                    expr = Expr::Cast { expr: Box::new(expr), target_type, span: Span::default() };
                }
                Token::LBrace => {
                    // Struct literal: Path { field: value, ... }
                    self.advance(); // skip {
                    let mut fields = Vec::new();
                    if !matches!(self.peek(), Token::RBrace) {
                        loop {
                            let field_name = match self.advance() {
                                Token::Ident(s) => s,
                                t => return Err(LocError::Internal(format!("expected field name, got {:?}", t))),
                            };
                            self.expect(&Token::Colon)?;
                            let value = self.parse_expr()?;
                            fields.push((field_name, value));
                            if matches!(self.peek(), Token::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(&Token::RBrace)?;
                    expr = Expr::StructLit { path: String::new(), fields, span: Span::default() };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> LocResult<Expr> {
        let tok = self.advance();
        match tok {
            Token::Ident(s) => Ok(Expr::Ident(s, Span::default())),
            Token::Int(v, _) => Ok(Expr::Literal(Literal::Int(v as i128), Span::default())),
            Token::Float(v, _) => Ok(Expr::Literal(Literal::Float(v), Span::default())),
            Token::Str(s) => Ok(Expr::Literal(Literal::String(s), Span::default())),
            Token::Char(c) => Ok(Expr::Literal(Literal::Char(c), Span::default())),
            Token::Byte(b) => Ok(Expr::Literal(Literal::Byte(b), Span::default())),
            Token::Keyword(slot) => match slot.as_str() {
                "KW_TRUE" => Ok(Expr::Literal(Literal::Bool(true), Span::default())),
                "KW_FALSE" => Ok(Expr::Literal(Literal::Bool(false), Span::default())),
                "KW_NULL" => Ok(Expr::Literal(Literal::Null, Span::default())),
                "KW_UNIT" => Ok(Expr::Literal(Literal::Unit, Span::default())),
                other => Ok(Expr::Ident(other.to_string(), Span::default())),
            },
            Token::LParen => {
                if matches!(self.peek(), Token::RParen) {
                    self.advance(); // skip )
                    return Ok(Expr::Literal(Literal::Unit, Span::default()));
                }
                // Could be tuple (1, 2) or parenthesized expr (1 + 2)
                let mut elements = Vec::new();
                elements.push(self.parse_expr()?);
                if matches!(self.peek(), Token::Comma) {
                    // It's a tuple
                    loop {
                        if matches!(self.peek(), Token::Comma) {
                            self.advance();
                        }
                        if matches!(self.peek(), Token::RParen) {
                            break;
                        }
                        elements.push(self.parse_expr()?);
                    }
                    self.expect(&Token::RParen)?;
                    Ok(Expr::TupleLit { elements, span: Span::default() })
                } else {
                    // Parenthesized expression
                    let inner = elements.into_iter().next().unwrap();
                    self.expect(&Token::RParen)?;
                    Ok(inner)
                }
            }
            Token::LBracket => {
                let mut elements = Vec::new();
                if !matches!(self.peek(), Token::RBracket) {
                    loop {
                        elements.push(self.parse_expr()?);
                        if matches!(self.peek(), Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&Token::RBracket)?;
                Ok(Expr::ArrayLit { elements, span: Span::default() })
            }
            t => Err(LocError::Internal(format!("unexpected token {:?} in expression", t))),
        }
    }
}