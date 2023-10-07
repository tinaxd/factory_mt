#pragma once
#include <stdint.h>

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
    BINOP_EQ,
    BINOP_NEQ,
    BINOP_LT,
    BINOP_LE,
    BINOP_GT,
    BINOP_GE,
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

typedef struct Statement
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
} Statement;

typedef struct AssignStatement
{
    char *name;
    struct Expression *expr;
} AssignStatement;

typedef struct ConditionalStatement
{
    struct Expression *cond;
    struct Statement *then;
    struct Statement *otherwise;
} ConditionalStatement;

typedef struct Expression
{
    int type;
    union
    {
        struct BinaryExpression *bin;
        struct LiteralExpression *lit;
        struct NameExpression *name;
    } expr;
} Expression;

typedef struct BinaryExpression
{
    BinaryOperator op;
    Expression *left;
    Expression *right;
} BinaryExpression;

typedef struct LiteralExpression
{
    LiteralType type;
    union
    {
        int64_t int_value;
        double float_value;
    } value;
} LiteralExpression;

typedef struct NameExpression
{
    char *name;
} NameExpression;

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