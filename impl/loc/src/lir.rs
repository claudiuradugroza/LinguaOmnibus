use crate::ast::*;

/// Emit LIR text from an AST (SPEC-06).
/// Uses canonical register names (%r1, %r2, ...) for byte-identical output across languages.
pub struct LirEmitter {
    output: String,
    reg_counter: usize,
    label_counter: usize,
    /// Map from source variable name to canonical register name.
    var_map: std::collections::HashMap<String, String>,
}

impl LirEmitter {
    pub fn new() -> Self {
        LirEmitter {
            output: String::new(),
            reg_counter: 0,
            label_counter: 0,
            var_map: std::collections::HashMap::new(),
        }
    }

    pub fn emit(&mut self, file: &File) -> String {
        self.output.push_str("lir 1.2.0\n");
        self.output.push_str(&format!("module {}\n", "main"));
        for item in &file.items {
            self.emit_item(item);
        }
        self.output.push_str("}\n");
        self.output.clone()
    }

    fn emit_item(&mut self, item: &Item) {
        match item {
            Item::FunctionDecl(f) => self.emit_function(f),
            _ => {}
        }
    }

    fn emit_function(&mut self, f: &FunctionDecl) {
        if f.is_extern {
            let params_str = f.params.iter()
                .map(|p| format!("{} {}", p.ty.lir_name(), p.name))
                .collect::<Vec<_>>()
                .join(", ");
            let ret_str = f.return_type.as_ref()
                .map(|t| t.lir_name())
                .unwrap_or_else(|| "unit".into());
            self.output.push_str(&format!("  extern fn {} : ({}) -> {}\n", f.name, params_str, ret_str));
            return;
        }
        let params_str = f.params.iter()
            .map(|p| format!("{} {}", p.ty.lir_name(), p.name))
            .collect::<Vec<_>>()
            .join(", ");
        let ret_str = f.return_type.as_ref()
            .map(|t| t.lir_name())
            .unwrap_or_else(|| "unit".into());

        self.output.push_str(&format!("  fn {} : ({}) -> {}\n", f.name, params_str, ret_str));
        self.output.push_str("  entry:\n");
        let has_return = self.block_has_return(&f.body);
        self.emit_block(&f.body);
        if !has_return {
            self.output.push_str("    ret unit\n");
        }
        self.output.push_str("  }\n");
    }

    fn emit_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.emit_statement(stmt);
        }
    }

    fn block_has_return(&self, block: &Block) -> bool {
        for stmt in &block.statements {
            match stmt {
                Statement::ReturnStmt(_) => return true,
                Statement::IfStmt(i) => {
                    if self.block_has_return(&i.then_block) {
                        return true;
                    }
                    if let Some(eb) = &i.else_block {
                        if self.block_has_return(eb) {
                            return true;
                        }
                    }
                }
                Statement::WhileStmt(w) => {
                    if self.block_has_return(&w.body) {
                        return true;
                    }
                }
                Statement::QuantumBlock(b) => {
                    if self.block_has_return(b) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn emit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VarDecl(v) => {
                if let Some(val) = &v.value {
                    let val_reg = self.emit_expr(val);
                    let reg = self.new_reg();
                    self.output.push_str(&format!("    {} = move {}\n", reg, val_reg));
                    self.var_map.insert(v.name.clone(), reg);
                }
            }
            Statement::ConstDecl(c) => {
                let val_reg = self.emit_expr(&c.value);
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = copy {}\n", reg, val_reg));
                self.var_map.insert(c.name.clone(), reg);
            }
            Statement::ExprStmt(e, _) => {
                let _ = self.emit_expr(e);
            }
            Statement::AsmStmt(code, _) => {
                self.output.push_str(&format!("    asm \"{}\"\n", code.replace('"', "\\\"")));
            }
            Statement::ReturnStmt(r) => {
                if let Some(v) = &r.value {
                    match v {
                        Expr::Literal(Literal::Unit, _) => {
                            self.output.push_str("    ret unit\n");
                        }
                        _ => {
                            let reg = self.emit_expr(v);
                            self.output.push_str(&format!("    ret {}\n", reg));
                        }
                    }
                } else {
                    self.output.push_str("    ret unit\n");
                }
            }
            Statement::IfStmt(i) => {
                let cond_reg = self.emit_expr(&i.cond);
                let then_label = self.new_label();
                let else_label = self.new_label();
                let end_label = self.new_label();
                // if cond then goto then_label else goto else_label
                self.output.push_str(&format!("    br.cond {} {}, {}\n", cond_reg, then_label, else_label));
                self.output.push_str(&format!("  {}:\n", then_label));
                self.emit_block(&i.then_block);
                self.output.push_str(&format!("    br {}\n", end_label));
                self.output.push_str(&format!("  {}:\n", else_label));
                if let Some(eb) = &i.else_block {
                    self.emit_block(eb);
                    self.output.push_str(&format!("    br {}\n", end_label));
                }
                self.output.push_str(&format!("  {}:\n", end_label));
            }
            Statement::WhileStmt(w) => {
                let start_label = self.new_label();
                let end_label = self.new_label();
                self.output.push_str(&format!("  {}:\n", start_label));
                let cond_reg = self.emit_expr(&w.cond);
                self.output.push_str(&format!("    br.cond {} {}, {}\n", cond_reg, end_label, start_label));
                self.emit_block(&w.body);
                self.output.push_str(&format!("    br {}\n", start_label));
                self.output.push_str(&format!("  {}:\n", end_label));
            }
            Statement::QuantumBlock(b) => {
                self.output.push_str("    effect.enter quantum\n");
                self.emit_block(b);
                self.output.push_str("    effect.exit quantum\n");
            }
            _ => {}
        }
    }

    fn emit_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::Literal(Literal::Int(v), _) => {
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = const.int {}\n", reg, v));
                reg
            }
            Expr::Literal(Literal::Float(v), _) => {
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = const.float {}\n", reg, v));
                reg
            }
            Expr::Literal(Literal::Bool(b), _) => {
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = const.bool {}\n", reg, if *b { 1 } else { 0 }));
                reg
            }
            Expr::Literal(Literal::String(s), _) => {
                let reg = self.new_reg();
                // Escape special characters for LIR (LEX-079: bidi controls → \\u{XXXX})
                let escaped = s.chars().map(|c| {
                    match c {
                        '"' => "\\\"".to_string(),
                        '\\' => "\\\\".to_string(),
                        '\n' => "\\n".to_string(),
                        '\t' => "\\t".to_string(),
                        '\r' => "\\r".to_string(),
                        '\0' => "\\0".to_string(),
                        c if (c as u32) >= 0x200E && (c as u32) <= 0x200F => format!("\\u{{{:04X}}}", c as u32),
                        c if (c as u32) >= 0x202A && (c as u32) <= 0x202E => format!("\\u{{{:04X}}}", c as u32),
                        c if (c as u32) >= 0x2066 && (c as u32) <= 0x2069 => format!("\\u{{{:04X}}}", c as u32),
                        c if (c as u32) == 0x2028 || (c as u32) == 0x2029 => format!("\\u{{{:04X}}}", c as u32),
                        c => c.to_string(),
                    }
                }).collect::<String>();
                self.output.push_str(&format!("    {} = const.str \"{}\"\n", reg, escaped));
                reg
            }
            Expr::Literal(Literal::Char(c), _) => {
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = const.char '{}'\n", reg, c));
                reg
            }
            Expr::Literal(Literal::Byte(b), _) => {
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = const.byte {}\n", reg, b));
                reg
            }
            Expr::Literal(Literal::Null, _) => {
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = const.null\n", reg));
                reg
            }
            Expr::Literal(Literal::Unit, _) => {
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = const.unit\n", reg));
                reg
            }
            Expr::Ident(name, _) => {
                // Use canonical register from var_map if available
                if let Some(reg) = self.var_map.get(name) {
                    reg.clone()
                } else {
                    // Unknown identifier — create a new register reference
                    format!("%{}", name)
                }
            }
            Expr::Assign { target, value, .. } => {
                let val_reg = self.emit_expr(value);
                match target.as_ref() {
                    Expr::Ident(name, _) => {
                        let reg = self.new_reg();
                        self.output.push_str(&format!("    {} = move {}\n", reg, val_reg));
                        self.var_map.insert(name.clone(), reg.clone());
                        reg
                    }
                    Expr::Index { object, index, .. } => {
                        let obj_reg = self.emit_expr(object);
                        let idx_reg = self.emit_expr(index);
                        let reg = self.new_reg();
                        self.output.push_str(&format!("    {} = store {}, {}, {}\n", reg, obj_reg, idx_reg, val_reg));
                        reg
                    }
                    _ => {
                        let _ = self.emit_expr(target);
                        val_reg
                    }
                }
            }
            Expr::BinaryOp { op, left, right, .. } => {
                let l = self.emit_expr(left);
                let r = self.emit_expr(right);
                let reg = self.new_reg();
                let op_str = match op {
                    BinaryOp::Add => "add",
                    BinaryOp::Sub => "sub",
                    BinaryOp::Mul => "mul",
                    BinaryOp::Div => "div",
                    BinaryOp::Rem => "rem",
                    BinaryOp::Eq => "eq",
                    BinaryOp::Ne => "ne",
                    BinaryOp::Lt => "lt",
                    BinaryOp::Le => "le",
                    BinaryOp::Gt => "gt",
                    BinaryOp::Ge => "ge",
                    BinaryOp::And => "and",
                    BinaryOp::Or => "or",
                    BinaryOp::BitAnd => "bitand",
                    BinaryOp::BitOr => "bitor",
                    BinaryOp::BitXor => "bitxor",
                    BinaryOp::Shl => "shl",
                    BinaryOp::Shr => "shr",
                };
                self.output.push_str(&format!("    {} = {} {}, {}\n", reg, op_str, l, r));
                reg
            }
            Expr::UnaryOp { op, operand, .. } => {
                let o = self.emit_expr(operand);
                let reg = self.new_reg();
                let op_str = match op {
                    UnaryOp::Not => "not",
                    UnaryOp::Neg => "neg",
                    UnaryOp::BitNot => "bitnot",
                };
                self.output.push_str(&format!("    {} = {} {}\n", reg, op_str, o));
                reg
            }
            Expr::Call { callee, args, .. } => {
                let callee_str = self.emit_expr(callee);
                let arg_strs: Vec<String> = args.iter().map(|a| self.emit_expr(a)).collect();
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = call {}({})\n", reg, callee_str, arg_strs.join(", ")));
                reg
            }
            Expr::FieldAccess { object, field, .. } => {
                let obj = self.emit_expr(object);
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = getfield {}, {}\n", reg, obj, field));
                reg
            }
            Expr::Index { object, index, .. } => {
                let obj = self.emit_expr(object);
                let idx = self.emit_expr(index);
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = index {}, {}\n", reg, obj, idx));
                reg
            }
            Expr::Cast { expr, target_type, .. } => {
                let e = self.emit_expr(expr);
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = cast {} as {}\n", reg, e, target_type.lir_name()));
                reg
            }
            _ => {
                let reg = self.new_reg();
                self.output.push_str(&format!("    {} = const.unit\n", reg));
                reg
            }
        }
    }

    fn new_reg(&mut self) -> String {
        self.reg_counter += 1;
        format!("%r{}", self.reg_counter)
    }

    fn new_label(&mut self) -> String {
        self.label_counter += 1;
        format!("L{}", self.label_counter)
    }
}