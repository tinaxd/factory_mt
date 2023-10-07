use crate::{
    ast::{BinaryExpression, Expression, LiteralExpression, Statement},
    opcode::Opcode,
};

#[derive(Debug)]
pub struct Compiler {
    code: Vec<OpcodeWithMetadata>,
    layouts: Vec<LayoutTracker>,

    current_label_index: u32,
}

impl Compiler {
    fn add_op(&mut self, op: Opcode) {
        self.code.push(OpcodeWithMetadata::new_op(op));
    }

    fn add_op_md(&mut self, op: Opcode, md: Metadata) {
        self.code.push(OpcodeWithMetadata::new(op, md));
    }

    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            layouts: vec![LayoutTracker::new()],
            current_label_index: 0,
        }
    }

    pub fn compile_expr(&mut self, expr: &Expression, top_label: Option<&str>) {
        match expr {
            Expression::Binary(bin) => {
                let BinaryExpression { op, left, right } = bin;
                // generate operands
                self.compile_expr(left, top_label);
                self.compile_expr(right, None);

                // generate operator
                use crate::ast::BinaryOperator::*;
                let op = match op {
                    Plus => Opcode::Add2,
                    Minus => Opcode::Sub2,
                    Times => Opcode::Mul2,
                    Divide => Opcode::Div2,
                    Modulo => Opcode::Mod2,
                    Eq => Opcode::Eq2,
                    Neq => Opcode::Neq2,
                    Lt => Opcode::Lt2,
                    Le => Opcode::Le2,
                    Gt => Opcode::Gt2,
                    Ge => Opcode::Ge2,
                };
                self.add_op(op);
            }
            Expression::Literal(lit) => match lit {
                LiteralExpression::Integer(i) => {
                    let op = Opcode::ConstInt(*i);
                    match top_label {
                        None => {
                            self.add_op(op);
                        }
                        Some(top_label) => {
                            let md = Metadata {
                                this_label: Some(top_label.to_string()),
                                jmp_to_label: None,
                            };
                            self.add_op_md(op, md);
                        }
                    }
                }
                _ => unimplemented!(),
            },
            Expression::Name(name) => {
                let var_name = name.get_name();
                let var_index = self.layouts.first().unwrap().get_local(var_name).unwrap();
                let op = Opcode::Load(var_index);

                match top_label {
                    None => {
                        self.add_op(op);
                    }
                    Some(top_label) => {
                        let md = Metadata {
                            this_label: Some(top_label.to_string()),
                            jmp_to_label: None,
                        };
                        self.add_op_md(op, md);
                    }
                }
            }
        }
    }

    pub fn compile_stmt(&mut self, stmt: &Statement, top_label: Option<&str>) {
        match stmt {
            Statement::Expression(expr) => {
                self.compile_expr(expr, top_label);
                let discard = Opcode::Discard;
                self.add_op(discard);
            }
            Statement::Assignment(assign) => {
                let name = assign.name();
                let assigned_index = self
                    .layouts
                    .first_mut()
                    .unwrap()
                    .register_local(name.to_string());
                self.compile_expr(assign.expression(), top_label);
                let op = Opcode::Store(assigned_index);
                self.add_op(op);
            }
            Statement::Block(blk) => {
                for (i, stmt) in blk.iter().enumerate() {
                    self.compile_stmt(stmt, if i == 0 { top_label } else { None });
                }
            }
            Statement::Conditional(cond) => {
                // evaluate condition
                let cond_expr = cond.cond();
                self.compile_expr(cond_expr, top_label);

                // jump if true
                let true_label = self.generate_unique_label();
                let true_jmp_op = {
                    let op = Opcode::JmpIfTrue(0);
                    let md = Metadata {
                        this_label: None,
                        jmp_to_label: Some(true_label.clone()),
                    };
                    (op, md)
                };
                self.add_op_md(true_jmp_op.0, true_jmp_op.1);

                // jump if false
                let false_label = match cond.otherwise() {
                    None => None,
                    Some(_) => {
                        let false_label = self.generate_unique_label();
                        let false_jmp_op = {
                            let op = Opcode::JmpAlways(0);
                            let md = Metadata {
                                this_label: None,
                                jmp_to_label: Some(false_label.clone()),
                            };
                            (op, md)
                        };
                        self.add_op_md(false_jmp_op.0, false_jmp_op.1);
                        Some(false_label)
                    }
                };

                // code for true branch
                self.compile_stmt(cond.then(), Some(true_label.as_str()));

                // code for false branch
                if let Some(false_label) = false_label {
                    self.compile_stmt(cond.otherwise().unwrap(), Some(false_label.as_str()));
                }
            }
        }
    }

    fn generate_unique_label(&mut self) -> String {
        let label = format!("L{}", self.current_label_index);
        self.current_label_index += 1;
        label
    }
}

#[derive(Debug, Clone)]
struct LayoutTracker {
    locals: Vec<(String, usize)>,
}

impl LayoutTracker {
    pub fn register_local(&mut self, name: String) -> usize {
        self.locals.push((name, self.locals.len()));
        self.locals.len() - 1
    }

    pub fn get_local(&self, name: &str) -> Option<usize> {
        for (n, i) in &self.locals {
            if n == name {
                return Some(*i);
            }
        }
        None
    }

    pub fn new() -> Self {
        Self { locals: Vec::new() }
    }
}

#[derive(Debug, Clone)]
pub struct OpcodeWithMetadata {
    op: Opcode,
    md: Metadata,
}

impl OpcodeWithMetadata {
    pub fn new(op: Opcode, md: Metadata) -> Self {
        Self { op, md }
    }

    pub fn new_op(op: Opcode) -> Self {
        Self {
            op,
            md: Metadata {
                jmp_to_label: None,
                this_label: None,
            },
        }
    }

    pub fn get_label(&self) -> Option<String> {
        self.md.this_label.clone()
    }

    pub fn get_jmp_to_label(&self) -> Option<String> {
        self.md.jmp_to_label.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub jmp_to_label: Option<String>,
    pub this_label: Option<String>,
}
