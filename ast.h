#pragma once
#include <cstdint>

typedef enum ExpressionType
{
    EXPR_BINARY,
    EXPR_LITERAL,
} ExpressionType;

typedef enum LiteralType
{
    LIT_INTEGER,
} LiteralType;

typedef enum BinaryOperator
{
    BINOP_PLUS,
    BINOP_MINUS,
    BINOP_TIMES,
    BINOP_DIVIDE,
    BINOP_MODULO,
} BinaryOperator;

struct BinaryExpression;
struct LiteralExpression;

struct Expression
{
    int type;
    union
    {
        struct BinaryExpression *bin;
        struct LiteralExpression *lit;
    } expr;
};

struct BinaryExpression
{
    BinaryOperator op;
    Expression *left;
    Expression *right;
};

struct LiteralExpression
{
    int type;
    union
    {
        int64_t int_value;
        double float_value;
    } value;
};

Expression *make_int_literal(int value);
Expression *make_bin_expr(Expression *left, BinaryOperator op, Expression *right);

void print_expr(Expression *expr);