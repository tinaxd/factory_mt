use std::collections::HashMap;

use crate::{
    object::{FunctionAddress, FunctionInfo, Object, Value},
    opcode::Opcode,
};

use crate::object::ObjectPtr;

#[derive(Debug)]
pub struct VM {
    stack: Vec<ObjectPtr>,
    stack_top: usize,

    opcode: Vec<Opcode>,
    pc: usize,

    stack_frames: Vec<LinearMemory>,
    stack_frame_top: usize,

    globals: NameMemory,

    gc: crate::object::GCSystem,
}

impl VM {
    pub fn new(stack_size: usize) -> Self {
        let mut gc = crate::object::GCSystem::new(100);
        let invalid_obj = gc.new_object(Object::make_invalid(), &mut vec![]);
        let vm = Self {
            stack: vec![gc.new_object(Object::make_invalid(), &mut vec![]); stack_size],
            stack_top: 0,

            opcode: vec![],
            pc: 0,

            stack_frames: vec![LinearMemory::new(invalid_obj.clone())],
            stack_frame_top: 0,

            globals: NameMemory::new(invalid_obj),

            gc,
        };
        vm
    }

    pub fn gc_debug(&mut self) {
        let all_alloc = self.gc.get_all_allocated();
        let num_objects = self.gc.num_objects();
        println!(
            "gc: current objects: {:?}, total allocated: {:?}",
            num_objects,
            all_alloc.len()
        );
    }

    pub fn gc_mark(&mut self) {
        let mut roots = self.collect_objptr();
        for sf in self.stack_frames.iter_mut() {
            roots.extend(sf.collect_objptr().iter().cloned().collect::<Vec<_>>());
        }
        for root in roots {
            self.gc.mark_all(root);
        }
    }

    pub fn gc_sweep(&mut self) {
        self.gc.sweep();
    }

    fn collect_objptr(&mut self) -> Vec<ObjectPtr> {
        self.stack.clone()
    }

    pub fn alloc_object(&mut self, object: Object) -> ObjectPtr {
        let mut roots = self.collect_objptr();
        for sf in self.stack_frames.iter_mut() {
            roots.extend(sf.collect_objptr());
        }
        roots.extend(self.globals.collect_objptr());
        self.gc.new_object(object, &mut roots)
    }

    pub fn stack_top(&self) -> Option<&Object> {
        self.stack
            .get(((self.stack_top as isize) - 1) as usize)
            .map(|w| w.get())
    }

    pub fn dump_stack(&self) {
        println!("stack:");
        for i in 0..self.stack_top {
            println!("\t{}: {:?}", i, self.stack[i].get().value());
        }
    }

    fn push_stackframe(&mut self, return_pc: usize) {
        self.stack_frame_top += 1;
        let invalid_obj = self.alloc_object(Object::make_invalid());
        self.stack_frames
            .push(LinearMemory::new_with_return(invalid_obj, return_pc));
    }

    fn pop_stackframe(&mut self) {
        self.stack_frame_top -= 1;
        self.stack_frames.pop();
    }

    pub fn set_code(&mut self, code: Vec<Opcode>) {
        self.opcode = code;
    }

    fn current_stack_frame(&mut self) -> &mut LinearMemory {
        &mut self.stack_frames[self.stack_frame_top]
    }

    fn opcode_arithmetic(&mut self, op: Opcode) {
        let right = self.stack[self.stack_top - 1].clone();
        let left = self.stack[self.stack_top - 2].clone();
        self.stack_top -= 2;

        let result = match (left.get().value(), right.get().value()) {
            (Value::Integer(left), Value::Integer(right)) => {
                self.alloc_object(Object::const_int(match op {
                    Opcode::Add2 => left + right,
                    Opcode::Sub2 => left - right,
                    Opcode::Mul2 => left * right,
                    Opcode::Div2 => left / right,
                    Opcode::Mod2 => left % right,
                    _ => panic!("invalid operands for arithmetic"),
                }))
            }
            _ => panic!("invalid operands for arithmetic"),
        };

        self.stack[self.stack_top] = result;
        self.stack_top += 1;
    }

    fn opcode_add(&mut self) {
        let right = self.stack[self.stack_top - 1].clone();
        let left = self.stack[self.stack_top - 2].clone();
        self.stack_top -= 2;

        let result = match (left.get().value(), right.get().value()) {
            (Value::Integer(left), Value::Integer(right)) => {
                self.alloc_object(Object::const_int(left + right))
            }
            (Value::String(left), Value::String(right)) => {
                self.alloc_object(Object::const_string(format!("{}{}", left, right)))
            }
            _ => panic!("invalid operands for arithmetic"),
        };

        self.stack[self.stack_top] = result;
        self.stack_top += 1;
    }

    fn opcode_compare(&mut self, op: Opcode) {
        let right = self.stack[self.stack_top - 1].clone();
        let left = self.stack[self.stack_top - 2].clone();
        self.stack_top -= 2;

        let result = match (left.get().value(), right.get().value()) {
            (Value::Integer(left), Value::Integer(right)) => {
                self.alloc_object(Object::const_bool(match op {
                    Opcode::Eq2 => left == right,
                    Opcode::Neq2 => left != right,
                    Opcode::Lt2 => left < right,
                    Opcode::Gt2 => left > right,
                    Opcode::Le2 => left <= right,
                    Opcode::Ge2 => left >= right,
                    _ => panic!("invalid operands for arithmetic"),
                }))
            }
            _ => panic!("invalid operands for arithmetic"),
        };

        self.stack[self.stack_top] = result;
        self.stack_top += 1;
    }

    pub fn register_native(&mut self, name: &str, f: &FunctionInfo) {
        let fun_object =
            self.alloc_object(Object::new_from_value(Value::Function(Box::new(f.clone()))));
        self.globals.store(name, fun_object);
    }

    pub fn step_code(&mut self) {
        // early return if pc is larger than code size
        if self.pc >= self.opcode.len() {
            return;
        }

        // fetch opcode
        let op = &self.opcode[self.pc].clone();
        // println!("pc: {:?}, executing {:?}", self.pc, op);
        match op {
            Opcode::ConstInt(const_value) => {
                self.stack[self.stack_top] = self.alloc_object(Object::const_int(*const_value));
                self.stack_top += 1;
            }
            Opcode::ConstNull => {
                self.stack[self.stack_top] = self.alloc_object(Object::const_null());
                self.stack_top += 1;
            }
            Opcode::ConstString(const_value) => {
                self.stack[self.stack_top] =
                    self.alloc_object(Object::const_string(const_value.clone()));
                self.stack_top += 1;
            }
            Opcode::Add2 => {
                self.opcode_add();
            }
            Opcode::Sub2 => {
                self.opcode_arithmetic(op.clone());
            }
            Opcode::Mul2 => {
                self.opcode_arithmetic(op.clone());
            }
            Opcode::Div2 => {
                self.opcode_arithmetic(op.clone());
            }
            Opcode::Mod2 => {
                self.opcode_arithmetic(op.clone());
            }
            Opcode::Eq2 => {
                self.opcode_compare(op.clone());
            }
            Opcode::Neq2 => {
                self.opcode_compare(op.clone());
            }
            Opcode::Lt2 => {
                self.opcode_compare(op.clone());
            }
            Opcode::Gt2 => {
                self.opcode_compare(op.clone());
            }
            Opcode::Le2 => {
                self.opcode_compare(op.clone());
            }
            Opcode::Ge2 => {
                self.opcode_compare(op.clone());
            }
            Opcode::Exit => {
                let exit_code = self.stack[self.stack_top - 1].clone();
                self.stack_top -= 1;
                match exit_code.get().value() {
                    Value::Integer(exit_code) => {
                        std::process::exit(*exit_code as i32);
                    }
                    _ => panic!("invalid exit code"),
                }
            }
            Opcode::Discard => {
                self.stack_top -= 1;
            }
            Opcode::Store(address) => {
                let value = self.stack[self.stack_top - 1].clone();
                self.stack_top -= 1;

                self.current_stack_frame().store(*address, value);
            }
            Opcode::Load(address) => {
                let value = self.current_stack_frame().load(*address);
                self.stack[self.stack_top] = value;
                self.stack_top += 1;
            }
            Opcode::StoreGlobal(address) => {
                let value = self.stack[self.stack_top - 1].clone();
                self.stack_top -= 1;

                self.globals.store(address, value);
            }
            Opcode::LoadGlobal(address) => {
                let value = self.globals.load(address);
                self.stack[self.stack_top] = value;
                self.stack_top += 1;
            }
            Opcode::JmpAlways(address) => {
                self.pc = *address;
                return; // avoid incrementing pc
            }
            Opcode::JmpIfTrue(address) => {
                let cond = self.stack[self.stack_top - 1].clone();
                self.stack_top -= 1;

                match cond.get().value() {
                    Value::Boolean(cond) => {
                        if *cond {
                            self.pc = *address;
                            return; // avoid incrementing pc
                        }
                    }
                    _ => panic!("invalid condition"),
                }
            }
            Opcode::JmpIfFalse(address) => {
                let cond = self.stack[self.stack_top - 1].clone();
                self.stack_top -= 1;

                match cond.get().value() {
                    Value::Boolean(cond) => {
                        if !*cond {
                            self.pc = *address;
                            return; // avoid incrementing pc
                        }
                    }
                    _ => panic!("invalid condition"),
                }
            }
            Opcode::Nop => {}
            Opcode::CreateFunction(address, n_params) => {
                let func_info = FunctionInfo::new(FunctionAddress::Bytecode(*address), *n_params);
                let func_value = Value::Function(Box::new(func_info));
                let func_object = Object::new_from_value(func_value);
                self.stack[self.stack_top] = self.alloc_object(func_object);
                self.stack_top += 1;
            }
            Opcode::CallNoKw(n_args) => {
                let fun_object = self.stack[self.stack_top - n_args - 1].clone();
                let fun_value = fun_object.get().value();
                let fun_info = match fun_value {
                    Value::Function(fun_info) => fun_info,
                    e => panic!("invalid function {:?}", e),
                };

                if fun_info.n_params() != *n_args {
                    panic!("invalid number of arguments");
                }

                self.push_stackframe(self.pc + 1);
                for i in (0..(*n_args)).rev() {
                    // println!("storing arg {} in stack frame", i);
                    let arg = self.stack[self.stack_top - 1].clone();
                    self.stack_top -= 1;
                    self.current_stack_frame().store(i, arg);
                }
                self.stack_top -= 1; // consume function object

                match fun_info.address() {
                    FunctionAddress::Bytecode(pc) => {
                        self.pc = *pc;
                        return; // avoid incrementing pc
                    }
                    FunctionAddress::Native(f) => {
                        let return_val = f(self);
                        self.stack[self.stack_top] =
                            self.alloc_object(Object::new_from_value(return_val));
                        self.stack_top += 1;
                        let return_to_pc = self.current_stack_frame().return_pc;
                        self.pop_stackframe();
                        match return_to_pc {
                            Some(return_to_pc) => {
                                self.pc = return_to_pc;
                                return; // avoid incrementing pc
                            }
                            None => panic!("return without call"),
                        }
                    }
                }
            }
            Opcode::Return => {
                let return_to_pc = self.current_stack_frame().return_pc;
                self.pop_stackframe();
                match return_to_pc {
                    Some(return_to_pc) => {
                        self.pc = return_to_pc;
                        return; // avoid incrementing pc
                    }
                    None => panic!("return without call"),
                }
            }
            _ => unimplemented!("opcode not implemented"),
        }

        self.pc += 1;
    }

    pub fn get_function_argument_by_index(&mut self, index: usize) -> ObjectPtr {
        self.current_stack_frame().load(index)
    }
}

#[derive(Debug)]
struct LinearMemory {
    memory: Vec<ObjectPtr>,
    return_pc: Option<usize>,

    invalid_obj: ObjectPtr,
}

impl LinearMemory {
    pub fn new(invalid_obj: ObjectPtr) -> Self {
        LinearMemory {
            memory: Vec::new(),
            return_pc: None,
            invalid_obj,
        }
    }

    pub fn new_with_return(invalid_obj: ObjectPtr, return_pc: usize) -> Self {
        LinearMemory {
            memory: Vec::new(),
            return_pc: Some(return_pc),
            invalid_obj,
        }
    }

    pub fn store(&mut self, address: usize, object: ObjectPtr) {
        if address >= self.memory.len() {
            self.memory.resize(address + 1, self.invalid_obj.clone());
        }

        self.memory[address] = object;
    }

    pub fn load(&mut self, address: usize) -> ObjectPtr {
        if address >= self.memory.len() {
            self.invalid_obj.clone()
        } else {
            self.memory[address].clone()
        }
    }

    pub fn collect_objptr(&mut self) -> Vec<ObjectPtr> {
        self.memory.clone()
    }
}

#[derive(Debug)]
struct NameMemory {
    memory: HashMap<String, ObjectPtr>,
    invalid_obj: ObjectPtr,
}

impl NameMemory {
    pub fn new(invalid_obj: ObjectPtr) -> Self {
        NameMemory {
            memory: HashMap::new(),
            invalid_obj,
        }
    }

    pub fn store(&mut self, address: &str, object: ObjectPtr) {
        self.memory.insert(address.to_string(), object);
    }

    pub fn load(&mut self, address: &str) -> ObjectPtr {
        self.memory
            .get(address)
            .map_or_else(|| self.invalid_obj.clone(), |w| w.clone())
    }

    pub fn collect_objptr(&mut self) -> Vec<ObjectPtr> {
        self.memory.values().cloned().collect()
    }
}
