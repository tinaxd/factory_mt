// runtime object used in Factory interpreter
#[derive(Debug, Clone)]
pub struct Object {
    value: Value,
}

#[derive(Debug, Clone)]
pub enum Value {
    Invalid,
    Integer(i64),
    Boolean(bool),
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

    pub fn make_invalid() -> Self {
        Object {
            value: Value::Invalid,
        }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
