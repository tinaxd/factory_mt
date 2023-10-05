#pragma once
#include <cstdint>

typedef enum ExpressionType
{
    EXPR_BINARY,
    EXPR_LITERAL,
    EXPR_NAME,
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

typedef enum StatementType
{
    STMT_ASSIGN,
    STMT_EXPR,
    STMT_BLOCK,
    STMT_CONDITIONAL,
} StatementType;

struct BinaryExpression;
struct LiteralExpression;
struct NameExpression;
struct Statement;
struct AssignStatement;
struct ConditionalStatement;

struct Statement
{
    int type;
    union
    {
        struct AssignStatement *assign;
        struct Expression *expr;
        struct Statement *blk_start;
        struct ConditionalStatement *cond;
    } stmt;
    struct Statement *blk_next; // for statements in a block
};

struct AssignStatement
{
    char *name;
    struct Expression *expr;
};

struct ConditionalStatement
{
    struct Expression *cond;
    struct Statement *then;
    struct Statement *otherwise;
};

struct Expression
{
    int type;
    union
    {
        struct BinaryExpression *bin;
        struct LiteralExpression *lit;
        struct NameExpression *name;
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

struct NameExpression
{
    char *name;
};

Expression *make_int_literal(int value);
Expression *make_bin_expr(Expression *left, BinaryOperator op, Expression *right);
Expression *make_name_expr(char *name);

Statement *make_expr_statement(Expression *expr);
Statement *make_assign_statement(char *name, Expression *expr);
Statement *make_block_statement(Statement *first);
void append_block_statement(Statement *block, Statement *stmt);
Statement *make_cond2(Expression *cond, Statement *then);
Statement *make_cond3(Expression *cond, Statement *then, Statement *otherwise);

void print_expr(Expression *expr);
void print_stmt(Statement *stmt);