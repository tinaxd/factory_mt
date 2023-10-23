use crate::object::{ObjectPtr, Value};

const DEFAULT_HASHMAP_SIZE: usize = 16;

#[derive(Debug, Clone)]
struct Entry {
    key: ObjectPtr,
    value: ObjectPtr,
    occupied: bool,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            key: ObjectPtr::null(),
            value: ObjectPtr::null(),
            occupied: false,
        }
    }
}
impl Entry {
    pub fn new(key: ObjectPtr, value: ObjectPtr) -> Self {
        Self {
            key,
            value,
            occupied: true,
        }
    }

    pub fn is_occupied(&self) -> bool {
        self.occupied
    }

    pub fn is_null(&self) -> bool {
        !self.occupied
    }

    pub fn key(&self) -> ObjectPtr {
        self.key.clone()
    }

    pub fn value(&self) -> ObjectPtr {
        self.value.clone()
    }

    pub fn replace(&mut self, key: ObjectPtr, value: ObjectPtr) {
        self.key = key;
        self.value = value;
        self.occupied = true;
    }

    pub fn set_value(&mut self, value: ObjectPtr) {
        self.value = value;
    }

    pub fn clear(&mut self) {
        self.key = ObjectPtr::null();
        self.value = ObjectPtr::null();
        self.occupied = false;
    }

    pub fn key_equals(&self, key: &ObjectPtr) -> bool {
        if self.is_null() {
            false
        } else {
            objectptr_string_eq(&self.key, key)
        }
    }
}

#[derive(Debug)]
pub struct HashMap {
    map_size: usize,
    data: Vec<Entry>,
}

fn hash_string(s: &str) -> usize {
    let mut hash = 5381;
    for c in s.chars() {
        hash = ((hash << 5) + hash) + (c as usize);
    }
    hash
}

fn objectptr_string_eq(p1: &ObjectPtr, p2: &ObjectPtr) -> bool {
    match (p1.get().value(), p2.get().value()) {
        (Value::String(s1), Value::String(s2)) => s1 == s2,
        _ => false,
    }
}

impl HashMap {
    pub fn new(initial_size: usize) -> Self {
        Self {
            map_size: initial_size,
            data: vec![Entry::default(); initial_size],
        }
    }

    pub fn new_default() -> Self {
        Self::new(DEFAULT_HASHMAP_SIZE)
    }

    pub fn put(&mut self, key: ObjectPtr, value: ObjectPtr) {
        match key.get().value() {
            Value::String(s) => {
                let hash = hash_string(s);
                let initial_index = hash % self.map_size;
                let mut index = initial_index;

                loop {
                    if index == initial_index {
                        panic!("hashmap is full");
                    }

                    if self.data[index].is_null() {
                        self.data[index].replace(key, value);
                        break;
                    } else {
                        if self.data[index].key_equals(&key) {
                            self.data[index].set_value(value);
                            break;
                        } else {
                            index = (index + 1) % self.map_size;
                            continue;
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
                let initial_index = hash % self.map_size;
                let mut index = initial_index;

                loop {
                    if index == initial_index {
                        panic!("hashmap is full");
                    }

                    if self.data[index].is_null() {
                        return None;
                    } else {
                        if self.data[index].key_equals(&key) {
                            return Some(index);
                        } else {
                            index = (index + 1) % self.map_size;
                            continue;
                        }
                    }
                }
            }
            _ => panic!("key must be string"),
        }
    }

    pub fn get(&self, key: ObjectPtr) -> Option<ObjectPtr> {
        self.find_index(key).map(|i| self.data[i].value().clone())
    }

    pub fn exists(&self, key: ObjectPtr) -> bool {
        self.find_index(key).is_some()
    }
}
