// ExpressionType enum
#[derive(Debug, Clone)]
pub enum ExpressionType {
    Binary,
    Literal,
    Name,
}

// LiteralType enum
#[derive(Debug, Clone)]
pub enum LiteralType {
    Integer,
}

// BinaryOperator enum
#[derive(Debug, Clone)]
pub enum BinaryOperator {
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
#[derive(Debug, Clone)]
pub enum StatementType {
    Assign,
    Expr,
    Block,
    Conditional,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(AssignmentStatement),
    Expression(Expression),
    Block(Vec<Statement>),
    Conditional(ConditionalStatement),
}

#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    name: String,
    expression: Box<Expression>,
}

impl AssignmentStatement {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}

#[derive(Debug, Clone)]
pub struct ConditionalStatement {
    cond: Box<Expression>,
    then: Box<Statement>,
    otherwise: Option<Box<Statement>>,
}

impl ConditionalStatement {
    pub fn cond(&self) -> &Expression {
        &self.cond
    }

    pub fn then(&self) -> &Statement {
        &self.then
    }

    pub fn otherwise(&self) -> Option<&Statement> {
        self.otherwise.as_ref().map(|s| &**s)
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(BinaryExpression),
    Literal(LiteralExpression),
    Name(NameExpression),
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    op: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum LiteralExpression {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct NameExpression {
    name: String,
}

impl NameExpression {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
