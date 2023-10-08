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
    pub fn new(name: String, expression: Expression) -> Self {
        Self {
            name,
            expression: Box::new(expression),
        }
    }
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

    pub fn new_no_else(cond: Expression, then: Statement) -> Self {
        Self {
            cond: Box::new(cond),
            then: Box::new(then),
            otherwise: None,
        }
    }

    pub fn new(cond: Expression, then: Statement, otherwise: Statement) -> Self {
        Self {
            cond: Box::new(cond),
            then: Box::new(then),
            otherwise: Some(Box::new(otherwise)),
        }
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

impl BinaryExpression {
    pub fn new(op: BinaryOperator, left: Expression, right: Expression) -> Self {
        Self {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn op(&self) -> &BinaryOperator {
        &self.op
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
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
