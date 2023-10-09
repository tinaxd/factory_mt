use std::collections::HashMap;

#[derive(Debug)]
pub struct GlobalTable {
    table: HashMap<String, usize>,
}

impl GlobalTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn register_global(&mut self, name: &str) -> usize {
        if let Some(i) = self.table.get(name) {
            return *i;
        }

        let i = self.table.len();
        self.table.insert(name.to_string(), i);
        i
    }

    pub fn get_global(&self, name: &str) -> Option<usize> {
        self.table.get(name).map(|i| *i)
    }
}
