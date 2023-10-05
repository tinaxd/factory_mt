#pragma once
/*
    AST to bytecode compiler for Factory language.
*/

#include "ast.h"
#include "consttable.h"
#include "opcode.h"
#include <vector>
#include <memory>

class FactoryCompiler
{
    std::vector<Opcode> code;

    std::unique_ptr<ConstantTable> const_table;

    void add_op(Opcode op);

    OpcodeParamType register_const(int64_t value);

public:
    FactoryCompiler();
    ~FactoryCompiler() = default;

    void compile_expr(const Expression *expr);
    const std::vector<Opcode> &get_code();
    const ConstantTable &get_const_table();
};
