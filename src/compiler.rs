use crate::{
    ast::{BinaryExpression, Expression},
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

    pub fn compile_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Binary(bin) => {
                let BinaryExpression { op, left, right } = bin;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct LayoutTracker {
    locals: Vec<(String, usize)>,
}

impl LayoutTracker {
    pub fn register_local(&mut self, name: String) {
        self.locals.push((name, self.locals.len()));
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
