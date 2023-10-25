use derive_builder::Builder;
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

    fn collect_functions(&self) -> Vec<crate::object::FunctionInfo> {
        self.register()
            .into_iter()
            .map(|f| {
                crate::object::FunctionInfo::new(
                    crate::object::FunctionAddress::Native(f.address().clone()),
                    f.n_params(),
                )
            })
            .collect()
    }
}
