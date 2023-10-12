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
    While(WhileStatement),
    FuncDef(FuncDefStatement),
    Return(ReturnStatement),
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
pub struct WhileStatement {
    cond: Box<Expression>,
    body: Box<Statement>,
}

impl WhileStatement {
    pub fn new(cond: Expression, body: Statement) -> Self {
        Self {
            cond: Box::new(cond),
            body: Box::new(body),
        }
    }

    pub fn cond(&self) -> &Expression {
        &self.cond
    }

    pub fn body(&self) -> &Statement {
        &self.body
    }
}

#[derive(Debug, Clone)]
pub struct FuncDefStatement {
    name: String,
    params: Vec<String>,
    body: Box<Statement>,
}

impl FuncDefStatement {
    pub fn new(name: String, params: Vec<String>, body: Statement) -> Self {
        Self {
            name,
            params,
            body: Box::new(body),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn params(&self) -> &[String] {
        &self.params
    }

    pub fn body(&self) -> &Statement {
        &self.body
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    expression: Option<Box<Expression>>,
}

impl ReturnStatement {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression: Some(Box::new(expression)),
        }
    }

    pub fn new_null() -> Self {
        Self { expression: None }
    }

    pub fn expression(&self) -> Option<&Expression> {
        self.expression.as_ref().map(|e| &**e)
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(BinaryExpression),
    Literal(LiteralExpression),
    FunCall(FunCallExpression),
    Index(IndexExpression),
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
pub struct FunCallExpression {
    callee: Box<Expression>,
    args: Vec<Expression>,
}

impl FunCallExpression {
    pub fn new(callee: Expression, args: Vec<Expression>) -> Self {
        Self {
            callee: Box::new(callee),
            args,
        }
    }

    pub fn callee(&self) -> &Expression {
        &self.callee
    }

    pub fn args(&self) -> &[Expression] {
        &self.args
    }
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

#[derive(Debug, Clone)]
pub struct IndexExpression {
    callee: Box<Expression>,
    args: Vec<Expression>,
}

impl IndexExpression {
    pub fn new(callee: Expression, args: Vec<Expression>) -> Self {
        Self {
            callee: Box::new(callee),
            args,
        }
    }

    pub fn callee(&self) -> &Expression {
        &self.callee
    }

    pub fn args(&self) -> &[Expression] {
        &self.args
    }
}
