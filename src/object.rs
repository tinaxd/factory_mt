// runtime object used in Factory interpreter
#[derive(Debug, Clone)]
pub struct Object {
    value: Value,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    address: usize,
    n_params: usize,
}

impl FunctionInfo {
    pub fn new(address: usize, n_params: usize) -> Self {
        Self { address, n_params }
    }

    pub fn address(&self) -> usize {
        self.address
    }

    pub fn n_params(&self) -> usize {
        self.n_params
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Invalid,
    Null,
    Integer(i64),
    Boolean(bool),
    Function(Box<FunctionInfo>),
    // Dict()
}

impl Value {
    pub fn const_int(value: i64) -> Self {
        Value::Integer(value)
    }

    pub fn const_bool(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl Object {
    pub fn const_int(value: i64) -> Self {
        Object {
            value: Value::const_int(value),
        }
    }

    pub fn const_bool(value: bool) -> Self {
        Object {
            value: Value::const_bool(value),
        }
    }

    pub fn const_null() -> Self {
        Object { value: Value::Null }
    }

    pub fn make_invalid() -> Self {
        Object {
            value: Value::Invalid,
        }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn new_from_value(value: Value) -> Self {
        Object { value }
    }
}
