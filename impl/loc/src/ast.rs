use crate::error::LocResult;

/// Source location for error reporting.
#[derive(Debug, Clone, Default)]
pub struct Span {
    pub byte_offset: usize,
    pub line: usize,
    pub column: usize,
}

/// A .lo source file.
#[derive(Debug, Clone)]
pub struct File {
    pub lingua: Option<String>,          // ISO code from #lingua: directive
    pub items: Vec<Item>,
}

/// Top-level declaration.
#[derive(Debug, Clone)]
pub enum Item {
    FunctionDecl(FunctionDecl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    ConstDecl(ConstDecl),
    VarDecl(VarDecl),
    Import(Import),
}

/// A module import.
#[derive(Debug, Clone)]
pub struct Import {
    pub path: String,
    pub alias: Option<String>,
    pub span: Span,
}

/// Function declaration.
#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub effects: Vec<Effect>,
    pub body: Block,
    pub is_extern: bool,
    pub span: Span,
}

/// Function parameter.
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Effect (KW_IO, KW_DATABASE, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    Io,
    Database,
    Network,
    Filesystem,
    Quantum,
    Custom(String),
}

/// Struct declaration.
#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<Field>,
    pub span: Span,
}

/// Struct field.
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Enum declaration.
#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<Variant>,
    pub span: Span,
}

/// Enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    pub name: String,
    pub args: Vec<Type>,
    pub span: Span,
}

/// Constant declaration.
#[derive(Debug, Clone)]
pub struct ConstDecl {
    pub name: String,
    pub ty: Type,
    pub value: Expr,
    pub span: Span,
}

/// Variable declaration.
#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub ty: Option<Type>,
    pub value: Option<Expr>,
    pub span: Span,
}

/// A block of statements (NEWLINE INDENT ... DEDENT).
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// A statement.
#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl(VarDecl),
    ConstDecl(ConstDecl),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    ForStmt(ForStmt),
    MatchStmt(MatchStmt),
    ReturnStmt(ReturnStmt),
    BreakStmt(Span),
    ContinueStmt(Span),
    QuantumBlock(Block),
    ExprStmt(Expr, Span),
    AsmStmt(String, Span),
}

/// An if statement (KW_IF cond KW_THEN ":" block (KW_ELSE ":" block)?).
#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: Expr,
    pub then_block: Block,
    pub else_block: Option<Block>,
    pub span: Span,
}

/// A while statement.
#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub cond: Expr,
    pub body: Block,
    pub span: Span,
}

/// A for statement (KW_FOR pattern KW_IN expr ":" block).
#[derive(Debug, Clone)]
pub struct ForStmt {
    pub pattern: String,
    pub iterable: Expr,
    pub body: Block,
    pub span: Span,
}

/// A match statement.
#[derive(Debug, Clone)]
pub struct MatchStmt {
    pub expr: Expr,
    pub arms: Vec<MatchArm>,
    pub span: Span,
}

/// A match arm.
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Block,
    pub span: Span,
}

/// A return statement.
#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
    pub span: Span,
}

/// A pattern.
#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Ident(String),
    Literal(Literal),
    Tuple(Vec<Pattern>),
    Struct { name: String, fields: Vec<(String, Pattern)> },
    Or(Box<Pattern>, Box<Pattern>),
}

/// An expression.
#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String, Span),
    Literal(Literal, Span),
    BinaryOp { op: BinaryOp, left: Box<Expr>, right: Box<Expr>, span: Span },
    UnaryOp { op: UnaryOp, operand: Box<Expr>, span: Span },
    Assign { target: Box<Expr>, value: Box<Expr>, span: Span },
    Call { callee: Box<Expr>, args: Vec<Expr>, span: Span },
    MethodCall { object: Box<Expr>, method: String, args: Vec<Expr>, span: Span },
    FieldAccess { object: Box<Expr>, field: String, span: Span },
    Index { object: Box<Expr>, index: Box<Expr>, span: Span },
    StructLit { path: String, fields: Vec<(String, Expr)>, span: Span },
    ArrayLit { elements: Vec<Expr>, span: Span },
    TupleLit { elements: Vec<Expr>, span: Span },
    IfExpr { cond: Box<Expr>, then: Box<Expr>, else_: Box<Expr>, span: Span },
    Block(Block),
    Cast { expr: Box<Expr>, target_type: Type, span: Span },
}

/// A literal value.
#[derive(Debug, Clone)]
pub enum Literal {
    Int(i128),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
    Byte(u8),
    Null,
    Unit,
}

/// Binary operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Rem,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
    BitAnd, BitOr, BitXor, Shl, Shr,
}

/// Unary operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Not, Neg, BitNot,
}

/// A type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int8, Int16, Int32, Int64, Int128,
    Uint8, Uint16, Uint32, Uint64, Uint128,
    F32, F64,
    // SIMD vector types
    VecF32x4, VecF32x8, VecF64x2, VecI32x4, VecI32x8, VecI64x2,
    Bool, Char, Byte, Unit,
    String,
    Qubit, QReg, QCircuit, QState,
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Map(Box<Type>, Box<Type>),
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Function(Vec<Type>, Box<Type>),
    Path(String),
}

impl Type {
    pub fn is_copy(&self) -> bool {
        match self {
            Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128
            | Type::Uint8 | Type::Uint16 | Type::Uint32 | Type::Uint64 | Type::Uint128
            | Type::F32 | Type::F64
            | Type::VecF32x4 | Type::VecF32x8 | Type::VecF64x2
            | Type::VecI32x4 | Type::VecI32x8 | Type::VecI64x2
            | Type::Bool | Type::Char | Type::Byte | Type::Unit
            | Type::String => true,
            Type::Qubit | Type::QReg | Type::QCircuit | Type::QState => false,
            Type::Option(_) | Type::Result(_, _) | Type::Map(_, _) | Type::Array(_) => true,
            Type::Tuple(ts) => ts.iter().all(|t| t.is_copy()),
            Type::Function(_, _) => true,
            Type::Path(_) => true, // conservative: user types assumed Copy unless derived otherwise
        }
    }
}

/// Convert a Type to its LIR type name.
impl Type {
    pub fn lir_name(&self) -> String {
        match self {
            Type::Int8 => "i8".into(), Type::Int16 => "i16".into(), Type::Int32 => "i32".into(),
            Type::Int64 => "i64".into(), Type::Int128 => "i128".into(),
            Type::Uint8 => "u8".into(), Type::Uint16 => "u16".into(), Type::Uint32 => "u32".into(),
            Type::Uint64 => "u64".into(), Type::Uint128 => "u128".into(),
            Type::F32 => "f32".into(), Type::F64 => "f64".into(),
            Type::VecF32x4 => "v4f32".into(), Type::VecF32x8 => "v8f32".into(),
            Type::VecF64x2 => "v2f64".into(), Type::VecI32x4 => "v4i32".into(),
            Type::VecI32x8 => "v8i32".into(), Type::VecI64x2 => "v2i64".into(),
            Type::Bool => "bool".into(), Type::Char => "char".into(), Type::Byte => "byte".into(),
            Type::Unit => "unit".into(),
            Type::String => "string".into(),
            Type::Qubit => "qubit".into(), Type::QReg => "qreg".into(),
            Type::QCircuit => "qcircuit".into(), Type::QState => "qstate".into(),
            Type::Option(t) => format!("option<{}>", t.lir_name()),
            Type::Result(t, e) => format!("result<{}, {}>", t.lir_name(), e.lir_name()),
            Type::Map(k, v) => format!("map<{}, {}>", k.lir_name(), v.lir_name()),
            Type::Array(t) => format!("[{}]", t.lir_name()),
            Type::Tuple(ts) => format!("({})", ts.iter().map(|t| t.lir_name()).collect::<Vec<_>>().join(", ")),
            Type::Function(args, ret) => format!("fn({}) -> {}", args.iter().map(|a| a.lir_name()).collect::<Vec<_>>().join(", "), ret.lir_name()),
            Type::Path(p) => p.clone(),
        }
    }
}