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

fn str_impl(vm: &mut VM) -> Value {
    let arg = vm.get_function_argument_by_index(0);
    let arg = arg.get();
    let str_value = match arg.value() {
        Value::String(s) => s.clone(),
        Value::Integer(i) => i.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Invalid => panic!("access to uninitialized value"),
        Value::Null => "null".to_string(),
        Value::Function(_) => "<function object>".to_string(),
        Value::Instance(_) => "<instance object>".to_string(),
    };
    Value::String(str_value)
}

#[derive(Default, Debug)]
pub struct BasicFunctions {}

impl RegisterableExtension for &BasicFunctions {
    fn register(&self) -> Vec<NativeFunctionInfo> {
        vec![
            NativeFunctionInfoBuilder::default()
                .address(println_impl)
                .n_params(1)
                .name("println".to_string())
                .build()
                .unwrap(),
            NativeFunctionInfoBuilder::default()
                .address(str_impl)
                .n_params(1)
                .name("str".to_string())
                .build()
                .unwrap(),
        ]
    }
}
