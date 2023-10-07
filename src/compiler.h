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

class OpcodeWithMetadata
{
public:
    struct Metadata
    {
        std::string jmp_to_label = "";
        std::string this_label = "";
    };
    Opcode op;
    Metadata md;

public:
    OpcodeWithMetadata(Opcode op, Metadata md);
    OpcodeWithMetadata(Opcode op);

    Opcode get_op() const;
    Opcode &get_op();
    const std::string &get_label() const;
    const std::string &get_jmp_to_label() const;
};

class FactoryCompiler
{
    std::vector<OpcodeWithMetadata> code;

    std::unique_ptr<ConstantTable> const_table;

    void add_op(Opcode op);
    void add_op(OpcodeWithMetadata op);

    OpcodeParamType register_const(int64_t value);

    std::stack<LayoutTracker> _layouts;

    uint32_t current_label_index = 0;
    std::string generate_unique_label();

    void link_jumps();

public:
    FactoryCompiler();
    ~FactoryCompiler() = default;

    void compile_stmt(const Statement *stmt) { compile_stmt(stmt, ""); }
    void compile_stmt(const Statement *stmt, const std::string &top_label);
    void compile_expr(const Expression *expr) { compile_expr(expr, ""); }
    void compile_expr(const Expression *expr, const std::string &top_label);
    void link()
    {
        link_jumps();
    }
    std::vector<Opcode> get_code();
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