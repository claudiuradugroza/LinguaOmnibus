use crate::ast::*;
use crate::error::{LocError, LocResult};

/// Type checker implementing SPEC-03 / SPEC-04 rules.
pub struct TypeChecker {
    errors: Vec<LocError>,
    /// Variable name -> type
    env: Vec<std::collections::HashMap<String, Type>>,
    /// Current function return type (for return checking)
    current_return_type: Option<Type>,
    /// Whether we're inside an effectful context
    current_effects: Vec<Effect>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            errors: Vec::new(),
            env: vec![std::collections::HashMap::new()],
            current_return_type: None,
            current_effects: Vec::new(),
        }
    }

    pub fn check(&mut self, file: &File) -> LocResult<()> {
        for item in &file.items {
            self.check_item(item)?;
        }
        if !self.errors.is_empty() {
            return Err(self.errors.remove(0));
        }
        Ok(())
    }

    fn check_item(&mut self, item: &Item) -> LocResult<()> {
        match item {
            Item::FunctionDecl(f) => self.check_function(f),
            Item::StructDecl(s) => self.check_struct(s),
            Item::EnumDecl(e) => self.check_enum(e),
            Item::ConstDecl(c) => self.check_const(c),
            Item::VarDecl(v) => self.check_var(v),
            Item::Import(_) => Ok(()),
        }
    }

    fn check_function(&mut self, f: &FunctionDecl) -> LocResult<()> {
        let ret_ty = f.return_type.clone().unwrap_or(Type::Unit);
        let prev_return = self.current_return_type.replace(ret_ty.clone());
        let prev_effects = self.current_effects.clone();
        self.current_effects = f.effects.clone();

        // Check params
        self.env.push(std::collections::HashMap::new());
        for p in &f.params {
            self.env.last_mut().unwrap().insert(p.name.clone(), p.ty.clone());
        }

        // Check body
        self.check_block(&f.body)?;

        self.env.pop();
        self.current_return_type = prev_return;
        self.current_effects = prev_effects;
        Ok(())
    }

    fn check_struct(&mut self, _s: &StructDecl) -> LocResult<()> { Ok(()) }
    fn check_enum(&mut self, _e: &EnumDecl) -> LocResult<()> { Ok(()) }

    fn check_const(&mut self, c: &ConstDecl) -> LocResult<()> {
        let val_ty = self.infer_expr(&c.value);
        let ok = val_ty == c.ty
            || matches!(c.value, Expr::Call { .. })
            || matches!(c.value, Expr::StructLit { .. });
        if !ok {
            self.errors.push(LocError::ReturnTypeMismatch);
        }
        Ok(())
    }

    fn check_var(&mut self, v: &VarDecl) -> LocResult<()> {
        if let Some(ref val) = v.value {
            let val_ty = self.infer_expr(val);
            let declared = v.ty.clone().unwrap_or(val_ty.clone());
            // Accept integer/float literal type coercion for declared types
            let ok = declared == val_ty
                || (matches!(declared, Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128 | Type::Uint8 | Type::Uint16 | Type::Uint32 | Type::Uint64 | Type::Uint128) && matches!(val_ty, Type::Int32))
                || (matches!(declared, Type::F32 | Type::F64) && matches!(val_ty, Type::F64))
                || (matches!(declared, Type::VecF32x4 | Type::VecF32x8 | Type::VecF64x2 | Type::VecI32x4 | Type::VecI32x8 | Type::VecI64x2) && matches!(val_ty, Type::Int32))
                || matches!(val, Expr::Call { .. })
                || matches!(val, Expr::StructLit { .. });
            if !ok && v.ty.is_some() {
                self.errors.push(LocError::ReturnTypeMismatch);
            } else {
                self.env.last_mut().unwrap().insert(v.name.clone(), declared);
            }
        } else if let Some(ref ty) = v.ty {
            self.env.last_mut().unwrap().insert(v.name.clone(), ty.clone());
        }
        Ok(())
    }

    fn check_block(&mut self, block: &Block) -> LocResult<()> {
        self.env.push(std::collections::HashMap::new());
        for stmt in &block.statements {
            self.check_statement(stmt)?;
        }
        self.env.pop();
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> LocResult<()> {
        match stmt {
            Statement::VarDecl(v) => self.check_var(v),
            Statement::ConstDecl(c) => self.check_const(c),
            Statement::IfStmt(i) => {
                self.check_expr(&i.cond)?;
                self.check_block(&i.then_block)?;
                if let Some(eb) = &i.else_block {
                    self.check_block(eb)?;
                }
                Ok(())
            }
            Statement::WhileStmt(w) => {
                self.check_expr(&w.cond)?;
                self.check_block(&w.body)
            }
            Statement::ForStmt(f) => {
                self.check_expr(&f.iterable)?;
                self.check_block(&f.body)
            }
            Statement::MatchStmt(m) => {
                self.check_expr(&m.expr)?;
                for arm in &m.arms {
                    self.check_block(&arm.body)?;
                }
                Ok(())
            }
            Statement::ReturnStmt(r) => {
                if let Some(v) = &r.value {
                    let vt = self.infer_expr(v);
                    let expected = self.current_return_type.clone().unwrap_or(Type::Unit);
                    let ok = vt == expected
                        || matches!(v, Expr::Call { .. })
                        || matches!(v, Expr::StructLit { .. })
                        || matches!(v, Expr::Literal(Literal::Unit, _));
                    if !ok {
                        self.errors.push(LocError::ReturnTypeMismatch);
                    }
                }
                Ok(())
            }
            Statement::BreakStmt(_) | Statement::ContinueStmt(_) => Ok(()),
            Statement::AsmStmt(_, _) => Ok(()),
            Statement::QuantumBlock(b) => self.check_block(b),
            Statement::ExprStmt(e, _) => {
                let _ = self.check_expr(e);
                Ok(())
            }
        }
    }

    fn check_expr(&mut self, e: &Expr) -> LocResult<Type> {
        let ty = self.infer_expr(e);
        Ok(ty)
    }

    fn infer_expr(&mut self, e: &Expr) -> Type {
        match e {
            Expr::Literal(Literal::Int(_), _) => Type::Int32,
            Expr::Literal(Literal::Float(_), _) => Type::F64,
            Expr::Literal(Literal::Bool(_), _) => Type::Bool,
            Expr::Literal(Literal::Char(_), _) => Type::Char,
            Expr::Literal(Literal::Byte(_), _) => Type::Byte,
            Expr::Literal(Literal::String(_), _) => Type::String,
            Expr::Literal(Literal::Null, _) => Type::Option(Box::new(Type::Int32)),
            Expr::Literal(Literal::Unit, _) => Type::Unit,
            Expr::Ident(name, _) => {
                for scope in self.env.iter().rev() {
                    if let Some(ty) = scope.get(name) {
                        return ty.clone();
                    }
                }
                Type::Int32
            }
            Expr::BinaryOp { op, left, right, .. } => {
                let _lt = self.infer_expr(left);
                let _rt = self.infer_expr(right);
                match op {
                    BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge
                    | BinaryOp::And | BinaryOp::Or => Type::Bool,
                    _ => Type::Int32,
                }
            }
            Expr::UnaryOp { op, .. } => {
                match op {
                    UnaryOp::Not => Type::Bool,
                    _ => Type::Int32,
                }
            }
            Expr::Call { .. } => Type::Int32,
            Expr::MethodCall { .. } => Type::Int32,
            Expr::FieldAccess { .. } => Type::Int32,
            Expr::Index { .. } => Type::Int32,
            Expr::ArrayLit { .. } => Type::Array(Box::new(Type::Int32)),
            Expr::TupleLit { .. } => Type::Tuple(vec![]),
            Expr::StructLit { .. } => Type::Path(String::new()),
            Expr::IfExpr { then, else_, .. } => {
                let _ = self.infer_expr(then);
                self.infer_expr(else_)
            }
            Expr::Block(b) => {
                if let Some(Statement::ExprStmt(e, _)) = b.statements.last() {
                    self.infer_expr(e)
                } else {
                    Type::Unit
                }
            }
            Expr::Cast { target_type, .. } => target_type.clone(),
        }
    }
}