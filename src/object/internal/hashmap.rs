use crate::object::{ObjectPtr, Value};

const DEFAULT_HASHMAP_SIZE: usize = 16;

#[derive(Debug)]
pub struct HashMap {
    map_size: usize,
    map: Vec<ObjectPtr>,
    values: Vec<ObjectPtr>,
}

fn hash_string(s: &str) -> usize {
    let mut hash = 5381;
    for c in s.chars() {
        hash = ((hash << 5) + hash) + (c as usize);
    }
    hash
}

fn objectptr_string_eq(p1: ObjectPtr, p2: ObjectPtr) -> bool {
    match (p1.get().value(), p2.get().value()) {
        (Value::String(s1), Value::String(s2)) => s1 == s2,
        _ => false,
    }
}

impl HashMap {
    pub fn new(initial_size: usize) -> Self {
        Self {
            map_size: initial_size,
            map: vec![ObjectPtr::null(); initial_size],
            values: vec![ObjectPtr::null(); initial_size],
        }
    }

    pub fn new_default() -> Self {
        Self::new(DEFAULT_HASHMAP_SIZE)
    }

    pub fn put(&mut self, key: ObjectPtr, value: ObjectPtr) {
        match key.get().value() {
            Value::String(s) => {
                let hash = hash_string(s);
                let index = hash % self.map_size;
                if self.map[index].is_null() {
                    self.map[index] = key;
                    self.values[index] = value;
                } else {
                    let current_key = self.map[index].clone();
                    if objectptr_string_eq(current_key, key.clone()) {
                        self.values[index] = value;
                    } else {
                        let initial_index = index;
                        let mut index = index;
                        loop {
                            index = (index + 1) % self.map_size;
                            if index == initial_index {
                                panic!("hashmap is full");
                            }
                            if self.map[index].is_null() {
                                self.map[index] = key;
                                self.values[index] = value;
                                break;
                            } else {
                                let current_key = self.map[index].clone();
                                if objectptr_string_eq(current_key, key.clone()) {
                                    self.values[index] = value;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                panic!("key must be string");
            }
        }
    }

    fn find_index(&self, key: ObjectPtr) -> Option<usize> {
        match key.get().value() {
            Value::String(s) => {
                let hash = hash_string(s);
                let index = hash % self.map_size;
                if self.map[index].is_null() {
                    None
                } else {
                    let current_key = self.map[index].clone();
                    if objectptr_string_eq(current_key, key.clone()) {
                        Some(index)
                    } else {
                        let initial_index = index;
                        let mut index = index;
                        loop {
                            index = (index + 1) % self.map_size;
                            if index == initial_index {
                                return None;
                            }
                            if self.map[index].is_null() {
                                return None;
                            } else {
                                let current_key = self.map[index].clone();
                                if objectptr_string_eq(current_key, key.clone()) {
                                    return Some(index);
                                }
                            }
                        }
                    }
                }
            }
            _ => panic!("key must be string"),
        }
    }

    pub fn get(&self, key: ObjectPtr) -> Option<ObjectPtr> {
        self.find_index(key).map(|i| self.values[i].clone())
    }

    pub fn exists(&self, key: ObjectPtr) -> bool {
        self.find_index(key).is_some()
    }
}
