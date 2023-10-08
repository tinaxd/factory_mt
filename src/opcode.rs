#[derive(Debug, Clone)]
pub enum Opcode {
    Nop,
    ConstInt(i64),
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
    JmpIfTrue(usize),
    JmpAlways(usize),
    JmpIfFalse(usize),
    CallNoKw(usize), // count of arguments
    CallKw(usize),   // count of arguments (excluding the last kwarg)
}
