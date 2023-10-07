#include "compiler.h"
#include <stdexcept>
#include <map>
#include <iostream>

std::string FactoryCompiler::generate_unique_label()
{
    return std::string("L") + std::to_string(current_label_index++);
}

FactoryCompiler::FactoryCompiler()
    : const_table(new ConstantTable())
{
    _layouts.push(LayoutTracker());
}

void FactoryCompiler::add_op(Opcode op)
{
    code.push_back(OpcodeWithMetadata(std::move(op)));
}

void FactoryCompiler::add_op(OpcodeWithMetadata op)
{
    code.push_back(std::move(op));
}

OpcodeParamType FactoryCompiler::register_const(int64_t value)
{
    return ct_add_int(const_table.get(), value);
}

void FactoryCompiler::compile_expr(const Expression *expr, const std::string &top_label)
{
    const auto need_label = top_label != "";

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
        case BINOP_EQ:
            binop = OPC_EQ2;
            break;
        case BINOP_NEQ:
            binop = OPC_NEQ2;
            break;
        case BINOP_LT:
            binop = OPC_LT2;
            break;
        case BINOP_LE:
            binop = OPC_LE2;
            break;
        case BINOP_GT:
            binop = OPC_GT2;
            break;
        case BINOP_GE:
            binop = OPC_GE2;
            break;
        }
        compile_expr(left, top_label);
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
            if (need_label)
            {
                OpcodeWithMetadata::Metadata op_md;
                op_md.this_label = top_label;
                add_op(OpcodeWithMetadata(op, op_md));
            }
            else
            {
                add_op(op);
            }
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
        if (need_label)
        {
            OpcodeWithMetadata::Metadata op_md;
            op_md.this_label = top_label;
            add_op(OpcodeWithMetadata(op, op_md));
        }
        else
        {
            add_op(op);
        }
        break;
    }
    default:
    {
        throw std::runtime_error("compiler: unimplemented expr type");
        break;
    }
    }
}

void FactoryCompiler::link_jumps()
{
    // first pass: collect all labels and their addresses
    std::map<std::string, uint32_t> labels;
    for (uint32_t i = 0; i < code.size(); i++)
    {
        const auto &op = code[i];
        const auto label = op.get_label();
        if (label != "")
        {
            std::cout << "label: " << label << std::endl;
            labels.insert(std::make_pair(std::string(label), i));
        }
    }

    // second pass: apply the addresses to the jump instructions
    for (uint32_t i = 0; i < code.size(); i++)
    {
        const auto &jmp_to_label = code[i].get_jmp_to_label();
        auto &op = code[i].get_op();
        switch (op.tag)
        {
        case JMP_IF_TRUE:
        case JMP_ALWAYS:
            if (jmp_to_label != "")
            {
                op.param = labels.at(jmp_to_label);
            }
            break;
        default:
            break;
        }
    }
}

void FactoryCompiler::compile_stmt(const Statement *stmt, const std::string &top_label)
{
    const auto need_label = top_label != "";

    switch (stmt->type)
    {
    case STMT_EXPR:
    {
        compile_expr(stmt->stmt.expr);
        Opcode op;
        op.tag = OPC_DISCARD;
        if (need_label)
        {
            OpcodeWithMetadata::Metadata op_md;
            op_md.this_label = top_label;
            add_op(OpcodeWithMetadata(op, op_md));
        }
        else
        {
            add_op(op);
        }
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
        if (need_label)
        {
            OpcodeWithMetadata::Metadata op_md;
            op_md.this_label = top_label;
            add_op(OpcodeWithMetadata(op, op_md));
        }
        else
        {
            add_op(op);
        }
        break;
    }
    case STMT_BLOCK:
    {
        const Statement *orig_stmt = stmt->stmt.blk_start;
        for (const Statement *stmt = orig_stmt; stmt != nullptr; stmt = stmt->blk_next)
        {
            compile_stmt(stmt, top_label);
        }
        break;
    }
    case STMT_CONDITIONAL:
    {
        // first evaluate the condition expression
        compile_expr(stmt->stmt.cond->cond, top_label); // FVAL_BOOL should be on the stack top

        // jump if true
        const auto true_label = generate_unique_label();
        Opcode jmp_true_op;
        jmp_true_op.tag = JMP_IF_TRUE;
        jmp_true_op.param = 0; // will be filled later
        OpcodeWithMetadata::Metadata jmp_true_op_md;
        jmp_true_op_md.jmp_to_label = true_label;
        const auto jmp_true_op_ = OpcodeWithMetadata(jmp_true_op, jmp_true_op_md);
        add_op(jmp_true_op_);

        // jump if false
        std::string false_label = "";
        bool need_false = stmt->stmt.cond->otherwise != NULL;
        if (need_false)
        {
            false_label = generate_unique_label();
            Opcode jmp_false_op;
            jmp_false_op.tag = JMP_ALWAYS;
            jmp_false_op.param = 0; // will be filled later
            OpcodeWithMetadata::Metadata jmp_false_op_md;
            jmp_false_op_md.jmp_to_label = false_label;
            const auto jmp_false_op_ = OpcodeWithMetadata(jmp_false_op, jmp_false_op_md);
            add_op(jmp_false_op_);
        }

        // generate code for the true branch
        compile_stmt(stmt->stmt.cond->then, true_label);

        // generate code for the false branch
        if (need_false)
        {
            compile_stmt(stmt->stmt.cond->otherwise, false_label);
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

std::vector<Opcode> FactoryCompiler::get_code()
{
    std::vector<Opcode> result;
    result.reserve(code.size());
    for (const auto &op : code)
    {
        result.push_back(op.get_op());
    }
    return result;
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

OpcodeWithMetadata::OpcodeWithMetadata(Opcode op, Metadata md)
    : op(op), md(std::move(md))
{
}

OpcodeWithMetadata::OpcodeWithMetadata(Opcode op)
    : op(op), md(Metadata())
{
}

Opcode OpcodeWithMetadata::get_op() const
{
    return op;
}

Opcode &OpcodeWithMetadata::get_op()
{
    return op;
}

const std::string &OpcodeWithMetadata::get_label() const
{
    return md.this_label;
}

const std::string &OpcodeWithMetadata::get_jmp_to_label() const
{
    return md.jmp_to_label;
}
