#pragma once
/*
    AST to bytecode compiler for Factory language.
*/

extern "C"
{
#include "ast.h"
#include "consttable.h"
#include "opcode.h"
}
#include <vector>
#include <memory>
#include <string>
#include <tuple>
#include <stack>

class LayoutTracker;

class FactoryCompiler
{
    std::vector<Opcode> code;

    std::unique_ptr<ConstantTable> const_table;

    void add_op(Opcode op);

    OpcodeParamType register_const(int64_t value);

    std::stack<LayoutTracker> _layouts;

public:
    FactoryCompiler();
    ~FactoryCompiler() = default;

    void compile_stmt(const Statement *stmt);
    void compile_expr(const Expression *expr);
    const std::vector<Opcode> &get_code();
    const ConstantTable &get_const_table();
};

class LayoutTracker
{
    std::vector<std::tuple<std::string, uint32_t>> locals;

public:
    LayoutTracker();

    uint32_t register_local(const std::string &name);
    uint32_t get_local(const std::string &name);
};