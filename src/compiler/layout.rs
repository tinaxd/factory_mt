#[derive(Debug, Clone)]
pub struct LayoutTracker {
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
