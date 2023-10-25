use crate::vm::VM;

use super::internal::hashmap::HashMap as MyHashMap;
use super::ObjectPtr;

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
    address: FunctionAddress,
    n_params: usize,
}

impl FunctionInfo {
    pub fn new(address: FunctionAddress, n_params: usize) -> Self {
        Self { address, n_params }
    }

    pub fn address(&self) -> &FunctionAddress {
        &self.address
    }

    pub fn n_params(&self) -> usize {
        self.n_params
    }
}

pub type NativeFunction = fn(&mut VM);

#[derive(Debug, Clone)]
pub enum FunctionAddress {
    Native(NativeFunction),
    Bytecode(usize),
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
            fields: MyHashMap::new_default(),
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
