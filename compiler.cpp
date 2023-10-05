#include "compiler.h"

FactoryCompiler *FactoryCompiler_new()
{
    FactoryCompiler *c = (FactoryCompiler *)malloc(sizeof(FactoryCompiler));
    c->code = (Opcode *)malloc(sizeof(Opcode) * 1024);
    c->code_size = 1024;
    c->code_i = 0;

    c->const_table = consttable_new();
}

void FactoryCompiler_free(FactoryCompiler *compiler)
{
    free(compiler->code);

    consttable_free(compiler->const_table);
    free(compiler->const_table);
}

static void fc_add_op(FactoryCompiler *c, Opcode op)
{
    if (c->code_i >= c->code_size)
    {
        c->code_size *= 2;
        c->code = (Opcode *)realloc(c->code, sizeof(Opcode) * c->code_size);
    }
    c->code[c->code_i++] = op;
}

void fc_compile_expr(FactoryCompiler *compiler, const Expression *expr)
{
    switch (expr->type)
    {
    case EXPR_BINARY:
    {
        const Expression *left = expr->expr.bin->left;
        const Expression *right = expr->expr.bin->right;
        OpcodeKind binop;
        switch (expr->expr.bin->op)
        {
        case BINOP_PLUS:
            binop = OPC_ADD2;
            break;
        case BINOP_MINUS:
            binop = OPC_SUB2;
            break;
        case BINOP_TIMES:
            binop = OPC_MUL2;
            break;
        case BINOP_DIVIDE:
            binop = OPC_DIV2;
            break;
        case BINOP_MODULO:
            binop = OPC_MOD2;
            break;
        }
        fc_compile_expr(compiler, left);
        fc_compile_expr(compiler, right);
        // now we have the two operands on the stack
        Opcode op;
        op.tag = binop;
        op.param = 0;
        fc_add_op(compiler, op);
        break;
    }
    case EXPR_LITERAL:
    {

        break;
    }
    }
}
