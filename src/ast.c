#include "ast.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

Expression *make_int_literal(int value)
{
    LiteralExpression *lit = (LiteralExpression *)malloc(sizeof(LiteralExpression));
    lit->type = LIT_INTEGER;
    lit->value.int_value = value;

    Expression *expr = (Expression *)malloc(sizeof(Expression));
    expr->type = EXPR_LITERAL;
    expr->expr.lit = lit;
    return expr;
}

Expression *make_bin_expr(Expression *left, BinaryOperator op, Expression *right)
{
    BinaryExpression *bin = (BinaryExpression *)malloc(sizeof(BinaryExpression));
    bin->left = left;
    bin->op = op;
    bin->right = right;

    Expression *expr = (Expression *)malloc(sizeof(Expression));
    expr->type = EXPR_BINARY;
    expr->expr.bin = bin;
    return expr;
}

Expression *make_name_expr(char *name)
{
    NameExpression *name_expr = (NameExpression *)malloc(sizeof(NameExpression));
    name_expr->name = strdup(name);

    Expression *expr = (Expression *)malloc(sizeof(Expression));
    expr->type = EXPR_NAME;
    expr->expr.name = name_expr;
    return expr;
}

Statement *make_expr_statement(Expression *expr)
{
    Statement *stmt = (Statement *)malloc(sizeof(Statement));
    stmt->type = STMT_EXPR;
    stmt->stmt.expr = expr;
    stmt->blk_next = NULL;
    return stmt;
}

Statement *make_assign_statement(char *name, Expression *expr)
{
    AssignStatement *stmt = (AssignStatement *)malloc(sizeof(AssignStatement));
    stmt->name = strdup(name);
    stmt->expr = expr;

    Statement *stm = (Statement *)malloc(sizeof(Statement));
    stm->type = STMT_ASSIGN;
    stm->stmt.assign = stmt;
    stm->blk_next = NULL;
    return stm;
}

Statement *make_block_statement(Statement *first)
{
    Statement *stmt = (Statement *)malloc(sizeof(Statement));
    stmt->type = STMT_BLOCK;
    stmt->stmt.blk_start = first;
    return stmt;
}

void append_block_statement(Statement *block, Statement *stmt)
{
    while (block->blk_next != NULL)
    {
        block = block->blk_next;
    }
    block->blk_next = stmt;
}

Statement *make_cond2(Expression *cond, Statement *then)
{
    ConditionalStatement *c = (ConditionalStatement *)malloc(sizeof(ConditionalStatement));
    c->cond = cond;
    c->then = then;
    c->otherwise = NULL;

    Statement *stmt = (Statement *)malloc(sizeof(Statement));
    stmt->type = STMT_CONDITIONAL;
    stmt->stmt.cond = c;
    stmt->blk_next = NULL;
    return stmt;
}

Statement *make_cond3(Expression *cond, Statement *then, Statement *otherwise)
{
    ConditionalStatement *c = (ConditionalStatement *)malloc(sizeof(ConditionalStatement));
    c->cond = cond;
    c->then = then;
    c->otherwise = otherwise;

    Statement *stmt = (Statement *)malloc(sizeof(Statement));
    stmt->type = STMT_CONDITIONAL;
    stmt->stmt.cond = c;
    stmt->blk_next = NULL;
    return stmt;
}

static void print_binary_expr(BinaryExpression *bin)
{
    const char *op;
    switch (bin->op)
    {
    case BINOP_PLUS:
        op = "+";
        break;
    case BINOP_MINUS:
        op = "-";
        break;
    case BINOP_TIMES:
        op = "*";
        break;
    case BINOP_DIVIDE:
        op = "/";
        break;
    case BINOP_MODULO:
        op = "%";
        break;
    case BINOP_EQ:
        op = "==";
        break;
    case BINOP_NEQ:
        op = "!=";
        break;
    case BINOP_LT:
        op = "<";
        break;
    case BINOP_LE:
        op = "<=";
        break;
    case BINOP_GT:
        op = ">";
        break;
    case BINOP_GE:
        op = ">=";
        break;
    }
    printf("(");
    print_expr(bin->left);
    printf(" %s ", op);
    print_expr(bin->right);
    printf(")");
}

static void print_literal_expr(LiteralExpression *lit)
{
    switch (lit->type)
    {
    case 0:
        printf("%ld", lit->value.int_value);
        break;
    }
}

void print_expr(Expression *expr)
{
    switch (expr->type)
    {
    case EXPR_LITERAL:
        print_literal_expr(expr->expr.lit);
        break;
    case EXPR_BINARY:
        print_binary_expr(expr->expr.bin);
        break;
    case EXPR_NAME:
        printf("%s", expr->expr.name->name);
        break;
    }
}

void print_stmt(Statement *stmt)
{
    switch (stmt->type)
    {
    case STMT_EXPR:
        print_expr(stmt->stmt.expr);
        break;
    case STMT_ASSIGN:
        printf("%s = ", stmt->stmt.assign->name);
        print_expr(stmt->stmt.assign->expr);
        break;
    case STMT_BLOCK:
        printf("{\n");
        Statement *block = stmt->stmt.blk_start;
        while (block != NULL)
        {
            print_stmt(block);
            printf(";\n");
            block = block->blk_next;
        }
        printf("}");
        break;
    case STMT_CONDITIONAL:
        printf("if (");
        print_expr(stmt->stmt.cond->cond);
        printf(") ");
        print_stmt(stmt->stmt.cond->then);
        if (stmt->stmt.cond->otherwise != NULL)
        {
            printf(" else ");
            print_stmt(stmt->stmt.cond->otherwise);
        }
        break;
    }
}
