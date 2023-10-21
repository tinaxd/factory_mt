mod gc;
mod internal;
mod value;

// runtime object used in Factory interpreter
pub use gc::GCSystem;
pub use gc::Object;
pub use gc::ObjectPtr;
pub use value::FunctionInfo;
pub use value::Value;

#[derive(Debug, Clone)]
pub struct ClassObject {}

pub struct DictObject {}
