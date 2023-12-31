#[derive(Debug, Clone)]
pub enum Opcode {
    Nop,
    ConstNull,
    ConstInt(i64),
    ConstString(String),
    Add2,
    Sub2,
    Mul2,
    Div2,
    Mod2,
    Rot2,
    Eq2,
    Neq2,
    Lt2,
    Le2,
    Gt2,
    Ge2,
    Exit,
    Discard,
    Store(usize),
    Load(usize),
    StoreGlobal(String),
    LoadGlobal(String),
    JmpIfTrue(usize),
    JmpAlways(usize),
    JmpIfFalse(usize),
    CallNoKw(usize),              // count of arguments
    CallKw(usize),                // count of arguments (excluding the last kwarg)
    CreateFunction(usize, usize), // address, n_params
    Return,
}
