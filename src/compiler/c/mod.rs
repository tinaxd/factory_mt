pub fn convert_to_c_fun_name(name: &str) -> String {
    let mut result = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c);
        } else {
            result.push('_');
        }
    }
    result
}

#[derive(Debug)]
pub struct UniqueNameGenerator {
    function_count: usize,
    var_count: usize,
    label_count: usize,
}

impl UniqueNameGenerator {
    pub fn new() -> Self {
        Self {
            function_count: 0,
            var_count: 0,
            label_count: 0,
        }
    }

    pub fn next_function_name(&mut self) -> String {
        let result = format!("fun_{}", self.function_count);
        self.function_count += 1;
        result
    }

    pub fn next_var_name(&mut self) -> String {
        let result = format!("var_{}", self.var_count);
        self.var_count += 1;
        result
    }

    pub fn next_label_name(&mut self) -> String {
        let result = format!("L{}", self.label_count);
        self.label_count += 1;
        result
    }
}
