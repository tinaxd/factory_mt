use derive_builder::Builder;

use crate::vm::VM;
pub mod basic;

#[derive(Debug, Clone, Builder)]
pub struct NativeFunctionInfo {
    address: crate::object::NativeFunction,
    n_params: usize,
    name: String,
}

impl NativeFunctionInfo {
    pub fn address(&self) -> &crate::object::NativeFunction {
        &self.address
    }

    pub fn n_params(&self) -> usize {
        self.n_params
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

pub trait RegisterableExtension {
    fn register(&self) -> Vec<NativeFunctionInfo>;
}

pub fn register_native(vm: &mut VM, extension: impl RegisterableExtension) {
    let functions = extension.register();
    for f in functions {
        let f_info = crate::object::FunctionInfo::new(
            crate::object::FunctionAddress::Native(f.address().clone()),
            f.n_params(),
        );
        vm.register_native(f.name(), &f_info);
    }
}
