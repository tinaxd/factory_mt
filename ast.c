#include "ast.h"
#include <stdlib.h>
#include <stdio.h>

Expression *make_int_literal(int value)
{
    LiteralExpression *lit = malloc(sizeof(LiteralExpression));
    lit->type = 0;
    lit->value.int_value = value;

    Expression *expr = malloc(sizeof(Expression));
    expr->type = EXPR_LITERAL;
    expr->expr.lit = lit;
    return expr;
}

Expression *make_bin_expr(Expression *left, BinaryOperator op, Expression *right)
{
    BinaryExpression *bin = malloc(sizeof(BinaryExpression));
    bin->left = left;
    bin->op = op;
    bin->right = right;

    Expression *expr = malloc(sizeof(Expression));
    expr->type = EXPR_BINARY;
    expr->expr.bin = bin;
    return expr;
}

static void print_binary_expr(BinaryExpression *bin)
{
    char *op;
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
        printf("%d", lit->value.int_value);
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
    }
}
