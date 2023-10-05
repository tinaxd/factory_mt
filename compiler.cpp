#include "compiler.h"

FactoryCompiler::FactoryCompiler()
    : const_table(new ConstantTable())
{
}

void FactoryCompiler::add_op(Opcode op)
{
    code.push_back(std::move(op));
}

OpcodeParamType FactoryCompiler::register_const(int64_t value)
{
    return const_table->add_int(value);
}

void FactoryCompiler::compile_expr(const Expression *expr)
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
        compile_expr(left);
        compile_expr(right);
        // now we have the two operands on the stack
        Opcode op;
        op.tag = binop;
        op.param = 0;
        add_op(op);
        break;
    }
    case EXPR_LITERAL:
    {
        switch (expr->expr.lit->type)
        {
        case LIT_INTEGER:
        {
            Opcode op;
            op.tag = OPC_CONST_INT;
            op.param = register_const(expr->expr.lit->value.int_value);
            add_op(op);
        };
        }
        break;
    }
    }
}

const std::vector<Opcode> &FactoryCompiler::get_code()
{
    return code;
}

const ConstantTable &FactoryCompiler::get_const_table()
{
    return *const_table;
}
