use nom::bytes::complete::tag;
use nom::character::complete as cp;
use nom::error::{context, ParseError};
use nom::multi::{many0, separated_list0, separated_list1};
use nom::sequence as seq;
use nom::{branch, combinator as comb, IResult};
use nom_locate::LocatedSpan;

use crate::ast::{
    AssignmentStatement, BinaryExpression, BinaryOperator, ConditionalStatement, Expression,
    FunCallExpression, FuncDefStatement, IndexExpression, LiteralExpression, NameExpression,
    ObjectAssignmentStatement, ReturnStatement, Statement, WhileStatement,
};

type Span<'a> = LocatedSpan<&'a str>;

type Result<'a, T> = IResult<Span<'a>, T, nom::error::VerboseError<Span<'a>>>;

fn white1(input: Span) -> Result<Span> {
    comb::recognize(cp::multispace1)(input)
}

fn white_no_newline1(input: Span) -> Result<Span> {
    comb::recognize(cp::space1)(input)
}

fn white_no_newline0(input: Span) -> Result<Span> {
    comb::recognize(cp::space0)(input)
}

fn is_keyword(input: Span) -> bool {
    let keywords = vec![
        "if", "else", "end", "do", "while", "for", "in", "break", "continue", "return", "def",
    ];
    keywords.contains(&input)
}

pub fn ident(input: Span) -> Result<Span> {
    let (new_input, o) = context("ident", cp::alphanumeric1)(input)?;
    if is_keyword(o) {
        return Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
            o,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((new_input, o))
}

fn list_literal(input: Span) -> Result<Expression> {
    context(
        "list_literal",
        comb::map(
            seq::tuple((
                tag("["),
                cp::multispace0,
                // TODO: This should be a list of expressions
                tag("]"),
            )),
            |(_, _, _)| Expression::Literal(LiteralExpression::List),
        ),
    )(input)
}

pub fn literal_expression(input: Span) -> Result<Expression> {
    let int_lit = comb::map(cp::digit1, |s: Span| {
        Expression::Literal(LiteralExpression::Integer(s.parse::<i64>().unwrap()))
    });
    let name = comb::map(ident, |s| {
        Expression::Name(NameExpression::new(s.to_string()))
    });
    context(
        "literal_expression",
        branch::alt((
            list_literal,
            context("int literal", int_lit),
            context("name literal", name),
        )),
    )(input)
}

fn elementary_expression(input: Span) -> Result<Expression> {
    let paren_expr = seq::delimited(tag("("), expression, tag(")"));
    context(
        "elementary_expression",
        branch::alt((literal_expression, context("paren expr", paren_expr))),
    )(input)
}

fn indexing_expression(input: Span) -> Result<IndexExpression> {
    let index_paren = context(
        "index_paren",
        comb::map(
            seq::tuple((
                cp::multispace0,
                tag("["),
                cp::space0,
                expression,
                cp::space0,
                tag("]"),
            )),
            |(_, _, _, arg, _, _)| arg,
        ),
    );

    let indexing = comb::map(
        seq::tuple((elementary_expression, index_paren)),
        |(callee, arg)| IndexExpression::new(callee, arg),
    );

    context("indexing", indexing)(input)
}

pub fn call_expression(input: Span) -> Result<Expression> {
    let call_paren = context(
        "call_paren",
        comb::map(
            seq::tuple((cp::multispace0, tag("("), arg_list0, tag(")"))),
            |(_, _, args, _)| args,
        ),
    );

    let call = comb::map(
        seq::tuple((elementary_expression, call_paren)),
        |(callee, args)| Expression::FunCall(FunCallExpression::new(callee, args)),
    );

    context(
        "call_expression",
        branch::alt((
            context("call", call),
            comb::map(indexing_expression, |e| Expression::Index(e)),
            elementary_expression,
        )),
    )(input)
}

pub fn product_operator(input: Span) -> Result<BinaryOperator> {
    context(
        "product_operator",
        branch::alt((
            comb::map(tag("*"), |_| BinaryOperator::Times),
            comb::map(tag("/"), |_| BinaryOperator::Divide),
            comb::map(tag("%"), |_| BinaryOperator::Modulo),
        )),
    )(input)
}

pub fn product_expression(input: Span) -> Result<Expression> {
    let p = seq::tuple((
        call_expression,
        many0(seq::tuple((product_operator, product_expression))),
    ));
    context(
        "product_expression",
        comb::map(p, |(first, rest)| {
            rest.into_iter().fold(first, |acc, (op, expr)| {
                Expression::Binary(BinaryExpression::new(op, acc, expr))
            })
        }),
    )(input)
}

pub fn add_operator(input: Span) -> Result<BinaryOperator> {
    context(
        "add_operator",
        branch::alt((
            comb::map(tag("+"), |_| BinaryOperator::Plus),
            comb::map(tag("-"), |_| BinaryOperator::Minus),
        )),
    )(input)
}

pub fn add_expression(input: Span) -> Result<Expression> {
    let p = seq::tuple((
        product_expression,
        many0(seq::tuple((add_operator, add_expression))),
    ));
    context(
        "add_expression",
        comb::map(p, |(first, rest)| {
            rest.into_iter().fold(first, |acc, (op, expr)| {
                Expression::Binary(BinaryExpression::new(op, acc, expr))
            })
        }),
    )(input)
}

pub fn cmp_operator(input: Span) -> Result<BinaryOperator> {
    context(
        "cmp_operator",
        branch::alt((
            comb::map(tag("=="), |_| BinaryOperator::Eq),
            comb::map(tag("!="), |_| BinaryOperator::Neq),
            comb::map(tag("<"), |_| BinaryOperator::Lt),
            comb::map(tag("<="), |_| BinaryOperator::Le),
            comb::map(tag(">"), |_| BinaryOperator::Gt),
            comb::map(tag(">="), |_| BinaryOperator::Ge),
        )),
    )(input)
}

pub fn cmp_expression(input: Span) -> Result<Expression> {
    let p = seq::tuple((
        add_expression,
        many0(seq::tuple((cmp_operator, cmp_expression))),
    ));
    context(
        "cmp_expression",
        comb::map(p, |(first, rest)| {
            rest.into_iter().fold(first, |acc, (op, expr)| {
                Expression::Binary(BinaryExpression::new(op, acc, expr))
            })
        }),
    )(input)
}

pub fn expression(input: Span) -> Result<Expression> {
    context("expression", cmp_expression)(input)
}

pub fn arg_list(input: Span) -> Result<Vec<Expression>> {
    let sep = seq::tuple((cp::space0, tag(","), cp::space0));
    context(
        "arg_list",
        comb::map(separated_list0(sep, expression), |params| {
            params.into_iter().collect()
        }),
    )(input)
}

pub fn arg_list0(input: Span) -> Result<Vec<Expression>> {
    // arg_list surrounded by spaces
    let arg_list_spaced = comb::map(
        seq::tuple((cp::space0, arg_list, cp::space0)),
        |(_, args, _)| args,
    );
    // empty list
    let empty_list = comb::map(cp::space0, |_| vec![]);
    context("arg_list0", branch::alt((arg_list_spaced, empty_list)))(input)
}

pub fn expression_stmt(input: Span) -> Result<Statement> {
    context(
        "expression_stmt",
        comb::map(expression, |e| Statement::Expression(e)),
    )(input)
}

pub fn assignment(input: Span) -> Result<Statement> {
    let assign = context(
        "assignment",
        comb::map(
            seq::tuple((ident, cp::space0, tag("="), cp::space0, expression)),
            |(name, _, _, _, expr)| {
                Statement::Assignment(AssignmentStatement::new(name.to_string(), expr))
            },
        ),
    );

    let indexed_assign = context(
        "indexed_assignment",
        comb::map(
            seq::tuple((
                indexing_expression,
                cp::space0,
                tag("="),
                cp::space0,
                expression,
            )),
            |(indexing_exp, _, _, _, expr)| {
                Statement::ObjectAssignment(ObjectAssignmentStatement::new(
                    indexing_exp.callee().clone(),
                    indexing_exp.arg().clone(),
                    expr,
                ))
            },
        ),
    );

    branch::alt((assign, indexed_assign))(input)
}

pub fn block_stmt(input: Span) -> Result<Statement> {
    context(
        "block_stmt",
        comb::map(
            seq::tuple((tag("do"), white1, stmt_list, white1, tag("end"))),
            |(_, _, stmts, _, _)| Statement::Block(stmts),
        ),
    )(input)
}

pub fn return_stmt(input: Span) -> Result<Statement> {
    let value_return = comb::map(
        seq::tuple((tag("return"), white1, expression)),
        |(_, _, expr)| Statement::Return(ReturnStatement::new(expr)),
    );
    let no_value_return = comb::map(tag("return"), |_| {
        Statement::Return(ReturnStatement::new_null())
    });
    context("return_stmt", branch::alt((value_return, no_value_return)))(input)
}

pub fn stmt_list(input: Span) -> Result<Vec<Statement>> {
    context(
        "stmt_list",
        comb::map(separated_list0(cp::multispace1, statement), |stmts| {
            stmts.into_iter().filter_map(|s| Some(s)).collect()
        }),
    )(input)
}

pub fn stmt_list1(input: Span) -> Result<Vec<Statement>> {
    context(
        "stmt_list1",
        comb::map(separated_list1(cp::multispace1, statement), |stmts| {
            stmts.into_iter().filter_map(|s| Some(s)).collect()
        }),
    )(input)
}

pub fn conditional_stmt(input: Span) -> Result<Statement> {
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

    context("conditional_stmt", branch::alt((has_else, no_else)))(input)
}

pub fn while_stmt(input: Span) -> Result<Statement> {
    context(
        "while_stmt",
        comb::map(
            seq::tuple((
                tag("while"),
                white_no_newline1,
                expression,
                white_no_newline1,
                block_stmt,
            )),
            |(_, _, cond, _, body)| Statement::While(WhileStatement::new(cond, body)),
        ),
    )(input)
}

pub fn funcdef_stmt(input: Span) -> Result<Statement> {
    let tup = seq::tuple((
        tag("def"),
        white_no_newline1,
        ident,
        white_no_newline0,
        tag("("),
        cp::multispace0,
        param_list,
        cp::multispace0,
        tag(")"),
        white1,
        block_stmt,
    ));
    context(
        "funcdef_stmt",
        comb::map(tup, |(_, _, name, _, _, _, params, _, _, _, body)| {
            Statement::FuncDef(FuncDefStatement::new(name.to_string(), params, body))
        }),
    )(input)
}

pub fn param_list(input: Span) -> Result<Vec<String>> {
    let sep = seq::tuple((cp::multispace0, tag(","), cp::multispace0));
    context(
        "param_list",
        comb::map(separated_list0(sep, ident), |params| {
            params.into_iter().map(|s| s.to_string()).collect()
        }),
    )(input)
}

pub fn statement(input: Span) -> Result<Statement> {
    context(
        "statement",
        branch::alt((
            block_stmt,
            funcdef_stmt,
            conditional_stmt,
            while_stmt,
            assignment,
            return_stmt,
            expression_stmt,
        )),
    )(input)
}

pub fn program(input: Span) -> Result<Vec<Statement>> {
    // comb::map(stmt_list1, |stmts| {
    //     stmts.into_iter().filter_map(|s| Some(s)).collect()
    // })(input)
    context("program", comb::map(block_stmt, |stmt| vec![stmt]))(input)
}
