mod global;
mod layout;

use std::collections::HashMap;

use crate::{
    ast::{Expression, LiteralExpression, Statement},
    opcode::Opcode,
};

use self::layout::LayoutTracker;

#[derive(Debug)]
pub struct Compiler {
    codes: Vec<Vec<OpcodeWithMetadata>>,
}

impl Compiler {
    pub fn new() -> Self {
        Self { codes: Vec::new() }
    }

    pub fn compile_top(&mut self, top_stmt: &Statement) {
        let mut unit_compiler = UnitCompiler::new(true);
        unit_compiler.compile_stmt(top_stmt, &vec![]);
        self.codes.extend(unit_compiler.collect_codes());
    }

    fn link_jumps(&mut self, orig_codes: &Vec<OpcodeWithMetadata>) -> Vec<OpcodeWithMetadata> {
        let mut codes = orig_codes.clone();

        let mut label_map: HashMap<String, u32> = HashMap::new();
        // first pass: collect labels and their addresses
        for (i, op) in codes.iter().enumerate() {
            let labels = op.get_labels();
            for label in labels.iter() {
                label_map.insert(label.clone(), i as u32);
            }
        }

        println!("ops before link: {:#?}", codes);

        // second pass: link jumps
        for op in codes.iter_mut() {
            let jmp_to_label = op.get_jmp_to_label();
            if let Some(jmp_to_label) = jmp_to_label {
                println!("processing: {}", jmp_to_label.as_str());
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

        codes
    }

    pub fn link(&mut self) -> Vec<Opcode> {
        let concat_codes = self.codes.concat();

        let linked = self.link_jumps(&concat_codes);
        linked.iter().map(|op| op.op.clone()).collect()
    }
}

#[derive(Debug)]
pub struct UnitCompiler {
    is_global: bool,
    code: Vec<OpcodeWithMetadata>,
    layout: LayoutTracker,

    ext_codes: Vec<Vec<OpcodeWithMetadata>>,

    current_label_index: u32,
}

impl UnitCompiler {
    fn add_op(&mut self, op: Opcode) {
        self.code.push(OpcodeWithMetadata::new_op(op));
    }

    fn add_op_md(&mut self, op: Opcode, md: Metadata) {
        self.code.push(OpcodeWithMetadata::new(op, md));
    }

    pub fn new(is_global: bool) -> UnitCompiler {
        Self {
            is_global,
            code: Vec::new(),
            layout: LayoutTracker::new(),
            ext_codes: Vec::new(),
            current_label_index: 0,
        }
    }

    fn current_layout_mut(&mut self) -> &mut LayoutTracker {
        &mut self.layout
    }

    pub fn compile_expr(&mut self, expr: &Expression, top_labels: &Vec<String>) {
        match expr {
            Expression::Binary(bin) => {
                let op = bin.op();
                let left = bin.left();
                let right = bin.right();
                // generate operands
                self.compile_expr(left, top_labels);
                self.compile_expr(right, &vec![]);

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
                    let md = Metadata {
                        this_label: top_labels.to_owned(),
                        jmp_to_label: None,
                    };
                    self.add_op_md(op, md);
                }
                LiteralExpression::String(s) => {
                    let op = Opcode::ConstString(s.to_string());
                    let md = Metadata {
                        this_label: top_labels.to_owned(),
                        jmp_to_label: None,
                    };
                    self.add_op_md(op, md);
                }
                _ => unimplemented!(),
            },
            Expression::Name(name) => {
                let var_name = name.get_name();
                if !self.is_global {
                    let var_index = self.current_layout_mut().get_local(var_name);
                    match var_index {
                        None => {
                            // fall back to global

                            let op = Opcode::LoadGlobal(var_name.to_owned());

                            let md = Metadata {
                                this_label: top_labels.to_owned(),
                                jmp_to_label: None,
                            };
                            self.add_op_md(op, md);
                        }
                        Some(var_index) => {
                            let op = Opcode::Load(var_index);

                            let md = Metadata {
                                this_label: top_labels.to_owned(),
                                jmp_to_label: None,
                            };
                            self.add_op_md(op, md);
                        }
                    };
                } else {
                    let op = Opcode::LoadGlobal(var_name.to_owned());

                    let md = Metadata {
                        this_label: top_labels.to_owned(),
                        jmp_to_label: None,
                    };
                    self.add_op_md(op, md);
                }
            }
            Expression::FunCall(func) => {
                let callee = func.callee();
                let args = func.args();

                // generate callee
                self.compile_expr(callee, top_labels);

                // generate args (from left to right)
                for arg in args.iter() {
                    self.compile_expr(arg, &vec![]);
                }

                // generate call
                let op = Opcode::CallNoKw(args.len());
                self.add_op(op);

                // generate return destination
                self.add_op(Opcode::Nop);
            }
            _ => unimplemented!(),
        }
    }

    pub fn compile_stmt(&mut self, stmt: &Statement, top_labels: &Vec<String>) {
        println!("compiling stmt: {:?}", stmt);
        match stmt {
            Statement::Expression(expr) => {
                self.compile_expr(expr, top_labels);
                let discard = Opcode::Discard;
                self.add_op(discard);
            }
            Statement::Assignment(assign) => {
                let name = assign.name();

                self.compile_expr(assign.expression(), top_labels);
                if !self.is_global {
                    let assigned_index = self.current_layout_mut().register_local(name.to_string());
                    let op = Opcode::Store(assigned_index);
                    self.add_op(op);
                } else {
                    let op = Opcode::StoreGlobal(name.to_owned());
                    self.add_op(op);
                }
            }
            Statement::Block(blk) => {
                for (i, stmt) in blk.iter().enumerate() {
                    let v = vec![];
                    self.compile_stmt(stmt, if i == 0 { top_labels } else { &v });
                }
            }
            Statement::Conditional(cond) => {
                // evaluate condition
                let cond_expr = cond.cond();
                self.compile_expr(cond_expr, top_labels);

                // jump if true
                let true_label = self.generate_unique_label();
                let true_jmp_op = {
                    let op = Opcode::JmpIfTrue(0);
                    let md = Metadata {
                        this_label: vec![],
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
                                this_label: vec![],
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
                self.compile_stmt(cond.then(), &vec![true_label]);
                self.add_op_md(
                    Opcode::JmpAlways(0),
                    Metadata {
                        this_label: vec![],
                        jmp_to_label: Some(branch_end_label.clone()),
                    },
                );

                // code for false branch
                if let Some(false_label) = false_label {
                    self.compile_stmt(cond.otherwise().unwrap(), &vec![false_label]);
                }

                // branch end label
                self.add_op_md(
                    Opcode::Nop,
                    Metadata {
                        this_label: vec![branch_end_label],
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
                self.compile_expr(cond, &vec![cond_label.clone()]);
                // jump if condition is not met
                self.add_op_md(
                    Opcode::JmpIfFalse(0),
                    Metadata {
                        this_label: vec![],
                        jmp_to_label: Some(body_end_label.clone()),
                    },
                );

                // generate body
                self.compile_stmt(body, &vec![]);
                // jump back to condition
                self.add_op_md(
                    Opcode::JmpAlways(0),
                    Metadata {
                        this_label: vec![],
                        jmp_to_label: Some(cond_label.clone()),
                    },
                );

                // end of while
                self.add_op_md(
                    Opcode::Nop,
                    Metadata {
                        this_label: vec![body_end_label],
                        jmp_to_label: None,
                    },
                );
            }
            Statement::FuncDef(def) => {
                let func_name = def.name();
                let func_params = def.params();
                let func_body = def.body();

                // register function earlier to handle recursive calls
                let func_register_op = if !self.is_global {
                    let func_index = self
                        .current_layout_mut()
                        .register_local(func_name.to_string());
                    Opcode::Store(func_index)
                } else {
                    // no need to register function
                    // because in global space, name lookup is done at runtime
                    Opcode::StoreGlobal(func_name.to_string())
                };

                let func_body_label = self.generate_func_label(func_name);

                // these codes are generated after the body of the currently compiling function
                // so these are not the first instructions in the function
                self.compile_fundef_body(
                    func_params,
                    func_body,
                    &vec![vec![func_body_label.clone()]].concat(),
                );

                self.add_op_md(
                    Opcode::CreateFunction(0, func_params.len()),
                    Metadata {
                        this_label: top_labels.to_owned(), // this is the first instruction in the function.
                        jmp_to_label: Some(func_body_label.clone()),
                    },
                );

                self.add_op(func_register_op);
                println!(
                    "registered function: {} (global: {})",
                    func_name, self.is_global
                );
            }
            Statement::Return(ret) => {
                match ret.expression() {
                    None => {
                        self.add_op(Opcode::ConstNull);
                    }
                    Some(e) => {
                        self.compile_expr(e, top_labels);
                    }
                }
                self.add_op(Opcode::Return);
            }
            _ => unimplemented!(),
        }
    }

    fn compile_fundef_body(
        &mut self,
        params: &[String],
        body: &Statement,
        top_labels: &Vec<String>,
    ) {
        let mut unit = UnitCompiler::new(false);

        // register params in order
        for param in params.iter() {
            unit.current_layout_mut().register_local(param.to_string());
        }

        unit.compile_stmt(body, top_labels);
        // these returns are redundant if the function already contains a return statement
        // but it's okay because the VM will never reach these returns in that case
        unit.add_op(Opcode::ConstNull);
        unit.add_op(Opcode::Return);

        let codes = unit.collect_codes();
        self.ext_codes.extend(codes);
    }

    // first code will always be a main code
    fn collect_codes(&self) -> Vec<Vec<OpcodeWithMetadata>> {
        let mut main_code = self.code.clone();
        // end of program
        if self.is_global {
            main_code.push(OpcodeWithMetadata::new_op(Opcode::ConstInt(0)));
            main_code.push(OpcodeWithMetadata::new_op(Opcode::Exit));
        }
        let mut codes = vec![main_code];
        codes.extend(self.ext_codes.clone());
        codes
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
                this_label: vec![],
            },
        }
    }

    pub fn get_labels(&self) -> &[String] {
        &self.md.this_label
    }

    pub fn get_jmp_to_label(&self) -> Option<String> {
        self.md.jmp_to_label.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub jmp_to_label: Option<String>,
    pub this_label: Vec<String>,
}
