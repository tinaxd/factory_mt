// ExpressionType enum
enum ExpressionType {
    Binary,
    Literal,
    Name,
}

// LiteralType enum
enum LiteralType {
    Integer,
}

// BinaryOperator enum
enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}

// StatementType enum
enum StatementType {
    Assign,
    Expr,
    Block,
    Conditional,
}

pub struct AssignmentStatement {
    name: String,
    expression: Expression,
}
