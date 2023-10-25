use crate::{extension::NativeFunctionInfoBuilder, object::Value, vm::VM};

use super::{NativeFunctionInfo, RegisterableExtension};

fn println_impl(vm: &mut VM) -> Value {
    let arg = vm.get_function_argument_by_index(0);
    let arg = arg.get();
    match arg.value() {
        Value::String(s) => println!("{}", s),
        Value::Integer(i) => println!("{}", i),
        Value::Boolean(b) => println!("{}", b),
        Value::Invalid => panic!("access to uninitialized value"),
        Value::Null => println!("null"),
        Value::Function(_) => println!("<function object>"),
        Value::Instance(_) => println!("<instance object>"),
    }
    Value::Null
}

#[derive(Default, Debug)]
pub struct BasicFunctions {}

impl RegisterableExtension for &BasicFunctions {
    fn register(&self) -> Vec<NativeFunctionInfo> {
        vec![NativeFunctionInfoBuilder::default()
            .address(println_impl)
            .n_params(1)
            .name("println".to_string())
            .build()
            .unwrap()]
    }
}
