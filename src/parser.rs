use nom::bytes::complete::tag;
use nom::character::complete as cp;
use nom::multi::separated_list0;
use nom::sequence as seq;
use nom::{branch, combinator as comb, IResult};

use crate::ast::{
    AssignmentStatement, BinaryExpression, BinaryOperator, ConditionalStatement, Expression,
    FunCallExpression, FuncDefStatement, LiteralExpression, NameExpression, Statement,
    WhileStatement,
};

type Result<'a, T> = IResult<&'a str, T>;

fn white1(input: &str) -> Result<&str> {
    comb::recognize(cp::multispace1)(input)
}

fn white_no_newline1(input: &str) -> Result<&str> {
    comb::recognize(cp::space1)(input)
}

fn is_keyword(input: &str) -> bool {
    let keywords = vec![
        "if", "else", "end", "do", "while", "for", "in", "break", "continue", "return",
    ];
    keywords.contains(&input)
}

pub fn ident(input: &str) -> Result<&str> {
    let (new_input, o) = cp::alphanumeric1(input)?;
    if is_keyword(o) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((new_input, o))
}

pub fn literal_expression(input: &str) -> Result<Expression> {
    comb::map(cp::digit1, |s: &str| {
        Expression::Literal(LiteralExpression::Integer(s.parse::<i64>().unwrap()))
    })(input)
}

pub fn elementary_expression(input: &str) -> Result<Expression> {
    let paren = seq::delimited(tag("("), expression, tag(")"));
    let literal = literal_expression;
    let call = comb::map(
        seq::tuple((
            expression,
            white1,
            tag("("),
            white1,
            arg_list,
            white1,
            tag(")"),
        )),
        |(callee, _, _, _, args, _, _)| Expression::FunCall(FunCallExpression::new(callee, args)),
    );
    let name = comb::map(ident, |s| {
        Expression::Name(NameExpression::new(s.to_string()))
    });
    branch::alt((paren, literal, call, name))(input)
}

pub fn product_operator(input: &str) -> Result<BinaryOperator> {
    branch::alt((
        comb::map(tag("*"), |_| BinaryOperator::Times),
        comb::map(tag("/"), |_| BinaryOperator::Divide),
        comb::map(tag("%"), |_| BinaryOperator::Modulo),
    ))(input)
}

pub fn product_expression(input: &str) -> Result<Expression> {
    let p1 = comb::map(
        seq::tuple((elementary_expression, product_operator, product_expression)),
        |(left, op, right)| Expression::Binary(BinaryExpression::new(op, left, right)),
    );

    branch::alt((p1, elementary_expression))(input)
}

pub fn add_operator(input: &str) -> Result<BinaryOperator> {
    branch::alt((
        comb::map(tag("+"), |_| BinaryOperator::Plus),
        comb::map(tag("-"), |_| BinaryOperator::Minus),
    ))(input)
}

pub fn add_expression(input: &str) -> Result<Expression> {
    let p1 = comb::map(
        seq::tuple((product_expression, add_operator, add_expression)),
        |(left, op, right)| Expression::Binary(BinaryExpression::new(op, left, right)),
    );

    branch::alt((p1, product_expression))(input)
}

pub fn cmp_operator(input: &str) -> Result<BinaryOperator> {
    branch::alt((
        comb::map(tag("=="), |_| BinaryOperator::Eq),
        comb::map(tag("!="), |_| BinaryOperator::Neq),
        comb::map(tag("<"), |_| BinaryOperator::Lt),
        comb::map(tag("<="), |_| BinaryOperator::Le),
        comb::map(tag(">"), |_| BinaryOperator::Gt),
        comb::map(tag(">="), |_| BinaryOperator::Ge),
    ))(input)
}

pub fn cmp_expression(input: &str) -> Result<Expression> {
    let p1 = comb::map(
        seq::tuple((add_expression, cmp_operator, cmp_expression)),
        |(left, op, right)| Expression::Binary(BinaryExpression::new(op, left, right)),
    );

    branch::alt((p1, add_expression))(input)
}

pub fn expression(input: &str) -> Result<Expression> {
    cmp_expression(input)
}

pub fn arg_list(input: &str) -> Result<Vec<Expression>> {
    let sep = seq::tuple((white1, tag(","), white1));
    comb::map(separated_list0(sep, expression), |params| {
        params.into_iter().collect()
    })(input)
}

pub fn expression_stmt(input: &str) -> Result<Statement> {
    comb::map(expression, |e| Statement::Expression(e))(input)
}

pub fn assignment(input: &str) -> Result<Statement> {
    comb::map(
        seq::tuple((ident, tag("="), expression)),
        |(name, _, expr)| Statement::Assignment(AssignmentStatement::new(name.to_string(), expr)),
    )(input)
}

pub fn block_stmt(input: &str) -> Result<Statement> {
    comb::map(
        seq::tuple((tag("do"), white1, stmt_list, white1, tag("end"))),
        |(_, _, stmts, _, _)| Statement::Block(stmts),
    )(input)
}

pub fn stmt_list(input: &str) -> Result<Vec<Statement>> {
    comb::map(separated_list0(cp::multispace1, statement), |stmts| {
        stmts.into_iter().filter_map(|s| Some(s)).collect()
    })(input)
}

pub fn conditional_stmt(input: &str) -> Result<Statement> {
    let no_else = comb::map(
        seq::tuple((
            tag("if"),
            white_no_newline1,
            expression,
            white_no_newline1,
            block_stmt,
        )),
        |(_, _, cond, _, body)| {
            Statement::Conditional(ConditionalStatement::new_no_else(cond, body))
        },
    );

    let has_else = comb::map(
        seq::tuple((
            tag("if"),
            white_no_newline1,
            expression,
            white_no_newline1,
            block_stmt,
            white_no_newline1,
            tag("else"),
            white_no_newline1,
            block_stmt,
        )),
        |(_, _, cond, _, body, _, _, _, else_body)| {
            Statement::Conditional(ConditionalStatement::new(cond, body, else_body))
        },
    );

    branch::alt((has_else, no_else))(input)
}

pub fn while_stmt(input: &str) -> Result<Statement> {
    comb::map(
        seq::tuple((
            tag("while"),
            white_no_newline1,
            expression,
            white_no_newline1,
            block_stmt,
        )),
        |(_, _, cond, _, body)| Statement::While(WhileStatement::new(cond, body)),
    )(input)
}

pub fn funcdef_stmt(input: &str) -> Result<Statement> {
    let tup = seq::tuple((
        tag("def"),
        white_no_newline1,
        ident,
        white_no_newline1,
        tag("("),
        white_no_newline1,
        param_list,
        white_no_newline1,
        tag(")"),
        white1,
        block_stmt,
    ));
    comb::map(tup, |(_, _, name, _, _, _, params, _, _, _, body)| {
        Statement::FuncDef(FuncDefStatement::new(name.to_string(), params, body))
    })(input)
}

pub fn param_list(input: &str) -> Result<Vec<String>> {
    let sep = seq::tuple((white1, tag(","), white1));
    comb::map(separated_list0(sep, ident), |params| {
        params.into_iter().map(|s| s.to_string()).collect()
    })(input)
}

pub fn statement(input: &str) -> Result<Statement> {
    branch::alt((
        block_stmt,
        conditional_stmt,
        while_stmt,
        assignment,
        expression_stmt,
    ))(input)
}

pub fn program(input: &str) -> Result<Vec<Statement>> {
    comb::map(stmt_list, |stmts| {
        stmts.into_iter().filter_map(|s| Some(s)).collect()
    })(input)
}
