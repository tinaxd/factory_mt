use std::collections::HashMap;

use crate::{
    ast::{Expression, LiteralExpression, Statement},
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
                let op = bin.op();
                let left = bin.left();
                let right = bin.right();
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
            Expression::FunCall(func) => {
                let callee = func.callee();
                let args = func.args();

                // generate callee
                self.compile_expr(callee, None);

                // generate args (from left to right)
                for arg in args.iter() {
                    self.compile_expr(arg, None);
                }

                // generate call
                let op = Opcode::CallNoKw(args.len());
                self.add_op(op);

                // generate return destination
                self.add_op(Opcode::Nop);
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

                // branch end label
                let branch_end_label = self.generate_unique_label();

                // code for true branch
                self.compile_stmt(cond.then(), Some(true_label.as_str()));
                self.add_op_md(
                    Opcode::JmpAlways(0),
                    Metadata {
                        this_label: None,
                        jmp_to_label: Some(branch_end_label.clone()),
                    },
                );

                // code for false branch
                if let Some(false_label) = false_label {
                    self.compile_stmt(cond.otherwise().unwrap(), Some(false_label.as_str()));
                }

                // branch end label
                self.add_op_md(
                    Opcode::Nop,
                    Metadata {
                        this_label: Some(branch_end_label),
                        jmp_to_label: None,
                    },
                );
            }
            Statement::While(wh) => {
                let cond = wh.cond();
                let body = wh.body();

                let cond_label = self.generate_unique_label();
                let body_end_label = self.generate_unique_label();

                // evaluate condition
                self.compile_expr(cond, Some(cond_label.as_str()));
                // jump if condition is not met
                self.add_op_md(
                    Opcode::JmpIfFalse(0),
                    Metadata {
                        this_label: None,
                        jmp_to_label: Some(body_end_label.clone()),
                    },
                );

                // generate body
                self.compile_stmt(body, None);
                // jump back to condition
                self.add_op_md(
                    Opcode::JmpAlways(0),
                    Metadata {
                        this_label: None,
                        jmp_to_label: Some(cond_label.clone()),
                    },
                );

                // end of while
                self.add_op_md(
                    Opcode::Nop,
                    Metadata {
                        this_label: Some(body_end_label),
                        jmp_to_label: None,
                    },
                );
            }
            Statement::FuncDef(def) => {
                let func_name = def.name();
                let func_params = def.params();
                let func_body = def.body();

                let func_def_end_label = self.generate_unique_label();
                self.add_op_md(
                    Opcode::JmpAlways(0),
                    Metadata {
                        this_label: None,
                        jmp_to_label: Some(func_def_end_label.clone()),
                    },
                );

                let func_body_label = self.generate_func_label(func_name);

                self.push_layout();
                {
                    let lay = self.layouts.first_mut().unwrap();
                    for param in func_params.iter() {
                        lay.register_local(param.to_string());
                    }
                }
                self.compile_stmt(func_body, Some(&func_body_label.as_str()));
                self.add_op(Opcode::ConstNull);
                self.add_op(Opcode::Return);
                self.pop_layout();

                self.add_op_md(
                    Opcode::CreateFunction(0, func_params.len()),
                    Metadata {
                        this_label: Some(func_def_end_label),
                        jmp_to_label: Some(func_body_label.clone()),
                    },
                );
                let func_index = self
                    .layouts
                    .first_mut()
                    .unwrap()
                    .register_local(func_name.to_string());
                self.add_op(Opcode::Store(func_index));
            }
        }
    }

    fn generate_unique_label(&mut self) -> String {
        let label = format!("L{}", self.current_label_index);
        self.current_label_index += 1;
        label
    }

    fn generate_func_label(&mut self, func_name: &str) -> String {
        let label = format!("F{}", func_name);
        label
    }

    fn link_jumps(&mut self) {
        let mut label_map: HashMap<String, u32> = HashMap::new();
        // first pass: collect labels and their addresses
        for (i, op) in self.code.iter().enumerate() {
            let label = op.get_label();
            if let Some(label) = label {
                label_map.insert(label.clone(), i as u32);
            }
        }

        // second pass: link jumps
        for op in self.code.iter_mut() {
            let jmp_to_label = op.get_jmp_to_label();
            if let Some(jmp_to_label) = jmp_to_label {
                match op.op {
                    Opcode::JmpIfTrue(_) => {
                        let jmp_to_addr = label_map.get(&jmp_to_label).unwrap();
                        op.op = Opcode::JmpIfTrue(*jmp_to_addr as usize);
                    }
                    Opcode::JmpAlways(_) => {
                        let jmp_to_addr = label_map.get(&jmp_to_label).unwrap();
                        op.op = Opcode::JmpAlways(*jmp_to_addr as usize);
                    }
                    Opcode::JmpIfFalse(_) => {
                        let jmp_to_addr = label_map.get(&jmp_to_label).unwrap();
                        op.op = Opcode::JmpIfFalse(*jmp_to_addr as usize);
                    }
                    Opcode::CreateFunction(_, n) => {
                        let jmp_to_addr = label_map.get(&jmp_to_label).unwrap();
                        op.op = Opcode::CreateFunction(*jmp_to_addr as usize, n);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn link(&mut self) {
        self.link_jumps();
    }

    pub fn code(&self) -> Vec<Opcode> {
        self.code.iter().map(|op| op.op.clone()).collect()
    }

    fn push_layout(&mut self) {
        self.layouts.push(LayoutTracker::new());
    }

    fn pop_layout(&mut self) {
        self.layouts.pop();
    }
}

#[derive(Debug, Clone)]
struct LayoutTracker {
    locals: Vec<(String, usize)>,
}

impl LayoutTracker {
    pub fn register_local(&mut self, name: String) -> usize {
        // if already exists, return index
        if let Some(i) = self.get_local(&name) {
            return i;
        }

        // otherwise, register and return index
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
