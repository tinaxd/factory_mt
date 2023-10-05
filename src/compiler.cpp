#include "compiler.h"
#include <stdexcept>

FactoryCompiler::FactoryCompiler()
    : const_table(new ConstantTable())
{
    _layouts.push(LayoutTracker());
}

void FactoryCompiler::add_op(Opcode op)
{
    code.push_back(std::move(op));
}

OpcodeParamType FactoryCompiler::register_const(int64_t value)
{
    return ct_add_int(const_table.get(), value);
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
    case EXPR_NAME:
    {
        std::string name = expr->expr.name->name;
        uint32_t index = _layouts.top().get_local(name);
        Opcode op;
        op.tag = OPC_LOAD;
        op.param = index;
        add_op(op);
        break;
    }
    default:
    {
        throw std::runtime_error("compiler: unimplemented expr type");
        break;
    }
    }
}

void FactoryCompiler::compile_stmt(const Statement *stmt)
{
    switch (stmt->type)
    {
    case STMT_EXPR:
    {
        compile_expr(stmt->stmt.expr);
        Opcode op;
        op.tag = OPC_DISCARD;
        add_op(op);
        break;
    }
    case STMT_ASSIGN:
    {
        const char *name = stmt->stmt.assign->name;
        uint32_t assigned_index = _layouts.top().register_local(std::string(name));
        compile_expr(stmt->stmt.assign->expr);
        Opcode op;
        op.tag = OPC_STORE;
        op.param = assigned_index;
        add_op(op);
        break;
    }
    case STMT_BLOCK:
    {
        const Statement *orig_stmt = stmt->stmt.blk_start;
        for (const Statement *stmt = orig_stmt; stmt != nullptr; stmt = stmt->blk_next)
        {
            compile_stmt(stmt);
        }
        break;
    }
    default:
    {
        throw std::runtime_error("compiler: unimplemented stmt type");
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

LayoutTracker::LayoutTracker()
{
}

uint32_t LayoutTracker::register_local(const std::string &name)
{
    for (const auto &local : locals)
    {
        if (std::get<0>(local) == name)
            return std::get<1>(local);
    }

    uint32_t index = locals.size();
    locals.push_back(std::make_tuple(name, index));
    return index;
}

uint32_t LayoutTracker::get_local(const std::string &name)
{
    for (const auto &local : locals)
    {
        if (std::get<0>(local) == name)
            return std::get<1>(local);
    }

    throw std::runtime_error("local not found");
}
