use crate::{
    object::{Object, Value},
    opcode::Opcode,
};

#[derive(Debug)]
pub struct VM {
    stack: Vec<Object>,
    stack_top: usize,

    opcode: Vec<Opcode>,
    pc: usize,

    stack_frames: Vec<StackFrame>,
    stack_frame_top: usize,
}

impl VM {
    pub fn new(stack_size: usize) -> Self {
        Self {
            stack: vec![Object::make_invalid(); stack_size],
            stack_top: 0,

            opcode: vec![],
            pc: 0,

            stack_frames: vec![StackFrame::new()],
            stack_frame_top: 0,
        }
    }

    pub fn stack_top(&self) -> Option<&Object> {
        self.stack.get(((self.stack_top as isize) - 1) as usize)
    }

    pub fn set_code(&mut self, code: Vec<Opcode>) {
        self.opcode = code;
    }

    fn current_stack_frame(&mut self) -> &mut StackFrame {
        &mut self.stack_frames[self.stack_frame_top]
    }

    pub fn step_code(&mut self) {
        // early return if pc is larger than code size
        if self.pc >= self.opcode.len() {
            return;
        }

        // fetch opcode
        let op = &self.opcode[self.pc].clone();
        println!("executing {:?}", op);
        match op {
            Opcode::ConstInt(const_value) => {
                self.stack[self.stack_top] = Object::const_int(*const_value);
                self.stack_top += 1;
            }
            Opcode::Add2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_int(left + right)
                    }
                    _ => panic!("invalid operands for add"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Sub2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_int(left - right)
                    }
                    _ => panic!("invalid operands for sub"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Mul2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_int(left * right)
                    }
                    _ => panic!("invalid operands for mul"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Div2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_int(left / right)
                    }
                    _ => panic!("invalid operands for div"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Mod2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_int(left % right)
                    }
                    _ => panic!("invalid operands for mod"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Eq2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_bool(left == right)
                    }
                    _ => panic!("invalid operands for eq"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Neq2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_bool(left != right)
                    }
                    _ => panic!("invalid operands for neq"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Lt2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_bool(left < right)
                    }
                    _ => panic!("invalid operands for lt"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Gt2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_bool(left > right)
                    }
                    _ => panic!("invalid operands for gt"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Le2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_bool(left <= right)
                    }
                    _ => panic!("invalid operands for le"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Ge2 => {
                let right = self.stack[self.stack_top - 1].clone();
                let left = self.stack[self.stack_top - 2].clone();
                self.stack_top -= 2;

                let result = match (left.value(), right.value()) {
                    (Value::Integer(left), Value::Integer(right)) => {
                        Object::const_bool(left >= right)
                    }
                    _ => panic!("invalid operands for ge"),
                };

                self.stack[self.stack_top] = result;
                self.stack_top += 1;
            }
            Opcode::Exit => {
                let exit_code = self.stack[self.stack_top - 1].clone();
                self.stack_top -= 1;
                match exit_code.value() {
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
            Opcode::JmpAlways(address) => {
                self.pc = *address;
                return; // avoid incrementing pc
            }
            Opcode::JmpIfTrue(address) => {
                let cond = self.stack[self.stack_top - 1].clone();
                self.stack_top -= 1;

                match cond.value() {
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

                match cond.value() {
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
            _ => unimplemented!("opcode not implemented"),
        }

        self.pc += 1;
    }
}

#[derive(Debug)]
struct StackFrame {
    memory: Vec<Object>,
}

impl StackFrame {
    pub fn new() -> Self {
        StackFrame { memory: Vec::new() }
    }

    pub fn store(&mut self, address: usize, object: Object) {
        if address >= self.memory.len() {
            self.memory.resize(address + 1, Object::make_invalid());
        }

        self.memory[address] = object;
    }

    pub fn load(&self, address: usize) -> Object {
        if address >= self.memory.len() {
            Object::make_invalid()
        } else {
            self.memory[address].clone()
        }
    }
}
