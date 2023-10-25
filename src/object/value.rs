use super::internal::hashmap::{HashMap as MyHashMap, DEFAULT_HASHMAP_SIZE};
use super::{gc::Object, ObjectPtr};

#[derive(Debug)]
pub enum Value {
    Invalid,
    Null,
    Integer(i64),
    Boolean(bool),
    Function(Box<FunctionInfo>),
    String(String),
    Instance(Instance),
    // Dict()
}

impl Value {
    pub fn const_int(value: i64) -> Self {
        Value::Integer(value)
    }

    pub fn const_bool(value: bool) -> Self {
        Value::Boolean(value)
    }

    pub fn const_string(value: String) -> Self {
        Value::String(value)
    }

    pub fn children(&self) -> Vec<ObjectPtr> {
        match self {
            Value::Invalid => vec![],
            Value::Null => vec![],
            Value::Integer(_) => vec![],
            Value::Boolean(_) => vec![],
            Value::Function(_) => vec![],
            Value::String(_) => vec![],
            Value::Instance(i) => i.children(),
        }
    }

    pub fn children_mut(&mut self) -> Vec<ObjectPtr> {
        match self {
            Value::Invalid => vec![],
            Value::Null => vec![],
            Value::Integer(_) => vec![],
            Value::Boolean(_) => vec![],
            Value::Function(_) => vec![],
            Value::String(_) => vec![],
            Value::Instance(i) => i.children(),
        }
    }
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

#[derive(Debug)]
pub struct Instance {
    class: Option<ObjectPtr>,
    fields: MyHashMap,
}

impl Instance {
    pub fn new(class: Option<ObjectPtr>) -> Self {
        Self {
            class,
            fields: MyHashMap::new(DEFAULT_HASHMAP_SIZE),
        }
    }

    pub fn class(&self) -> Option<ObjectPtr> {
        self.class.clone()
    }

    pub fn set_field(&mut self, key: ObjectPtr, value: ObjectPtr) {
        self.fields.put(key, value);
    }

    pub fn get_field(&self, key: &ObjectPtr) -> Option<ObjectPtr> {
        self.fields.get(key.clone())
    }

    pub fn children(&self) -> Vec<ObjectPtr> {
        let mut pointers = self.class.clone().map_or_else(|| vec![], |c| vec![c]);
        pointers.extend(self.fields.pointer());
        pointers
    }
}
