use crate::opcode::Opcode;

#[derive(Debug)]
pub struct Compiler {
    code: Vec<OpcodeWithMetadata>,
    layouts: Vec<LayoutTracker>,
}

impl Compiler {}

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
