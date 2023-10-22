use crate::object::value::Value;

#[derive(Debug)]
pub struct Object {
    marked: bool,
    next_value: Option<ObjectPtr>, // existence of object is guaranteed by GCSystem
    value: Value,
}

impl Object {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            marked: false,
            next_value: None,
        }
    }

    pub fn new_from_value(value: Value) -> Self {
        Self::new(value)
    }

    pub fn const_int(value: i64) -> Self {
        Self::new_from_value(Value::const_int(value))
    }

    pub fn const_bool(value: bool) -> Self {
        Self::new_from_value(Value::const_bool(value))
    }

    pub fn const_string(value: String) -> Self {
        Self::new_from_value(Value::const_string(value))
    }

    pub fn const_null() -> Self {
        Self::new_from_value(Value::Null)
    }

    pub fn make_invalid() -> Self {
        Self::new_from_value(Value::Invalid)
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Debug)]
pub struct ObjectPtr {
    object: *mut Object,
}

impl ObjectPtr {
    pub fn wrap(object: Object) -> Self {
        let object_ptr = Box::into_raw(Box::new(object));
        Self { object: object_ptr }
    }

    pub fn get(&self) -> &Object {
        unsafe { &*self.object }
    }

    pub fn get_mut(&mut self) -> &mut Object {
        unsafe { &mut *self.object }
    }

    pub fn dispose(&mut self) {
        let object = unsafe { Box::from_raw(self.object) };
        drop(object);
        self.object = std::ptr::null_mut();
    }
}

impl Clone for ObjectPtr {
    fn clone(&self) -> Self {
        Self {
            object: self.object,
        }
    }
}

#[derive(Debug)]
pub struct GCSystem {
    head: Option<ObjectPtr>,
    tail: Option<ObjectPtr>,
    num_objects: usize,
    max_objects: usize,

    _all_allocated: Vec<ObjectPtr>,
}

impl GCSystem {
    pub fn new(max_objects: usize) -> Self {
        Self {
            head: None,
            tail: None,
            num_objects: 0,
            max_objects,
            _all_allocated: vec![],
        }
    }

    pub fn get_all_allocated(&self) -> &Vec<ObjectPtr> {
        &self._all_allocated
    }

    pub fn num_objects(&self) -> usize {
        self.num_objects
    }

    // pub fn clone_object(&mut self, object: &Object) -> Object {
    //     let value = unsafe { &*object.value };
    //     let value = value.clone();
    //     let value_ptr = Box::into_raw(Box::new(value));
    //     Object::new(value_ptr)
    // }

    pub fn new_object(&mut self, obj: Object, roots: &mut Vec<ObjectPtr>) -> ObjectPtr {
        let gc_value = obj;
        if self.num_objects == self.max_objects {
            self.collect_garbage(roots);
        }
        self.num_objects += 1;
        let ptr = ObjectPtr::wrap(gc_value);
        if self.head.is_none() {
            self.head = Some(ptr.clone());
            self.tail = Some(ptr.clone());
        } else {
            let mut tail = self.tail.take().unwrap();
            tail.get_mut().next_value = Some(ptr.clone());
            self.tail = Some(tail);
        }

        self._all_allocated.push(ptr.clone());

        ptr
    }

    pub fn new_object_from_value(&mut self, value: Value, roots: &mut Vec<ObjectPtr>) -> ObjectPtr {
        self.new_object(Object::new(value), roots)
    }

    pub fn collect_garbage(&mut self, roots: &mut Vec<ObjectPtr>) {
        for root in roots {
            self.mark_all(root.clone());
        }
        let num_objects = self.sweep();
        self.num_objects = num_objects;
    }

    pub fn mark_all(&mut self, root: ObjectPtr) {
        let head = root;

        // dfs
        let mut stack = vec![head];
        while let Some(mut object) = stack.pop() {
            if object.get().marked {
                continue;
            }

            object.get_mut().marked = true;

            // find children
            let value = &object.get().value;
            let children = value.children();
            for child in children {
                stack.push(child);
            }
        }
    }

    pub fn sweep(&mut self) -> usize {
        // sweep marked objects
        let mut object = self.head.take();
        let mut last_marked: Option<ObjectPtr> = None;
        let mut head_candidate = None;
        let mut allocated_objects = 0;
        while let Some(mut object_inner) = object {
            if object_inner.get().marked {
                allocated_objects += 1;
                object_inner.get_mut().marked = false;
                if head_candidate.is_none() {
                    head_candidate = Some(object_inner.clone());
                }
                object = object_inner.get_mut().next_value.take();

                if let Some(mut last_marked) = last_marked {
                    last_marked.get_mut().next_value = Some(object_inner.clone());
                }
                last_marked = Some(object_inner);
            } else {
                let next = object_inner.get_mut().next_value.take();

                // dealloc value
                object_inner.dispose();

                object = next;
            }
        }

        // set last object next to null
        if let Some(mut last_marked) = last_marked {
            last_marked.get_mut().next_value = None;
        }

        self.head = head_candidate;

        allocated_objects
    }
}
