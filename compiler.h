#pragma once
/*
    AST to bytecode compiler for Factory language.
*/

#include <stdlib.h>
#include "ast.h"
#include "consttable.h"
#include "opcode.h"

typedef struct FactoryCompiler
{
    Opcode *code;
    size_t code_size;
    size_t code_i;

    ConstantTable *const_table;
} FactoryCompiler;

FactoryCompiler *FactoryCompiler_new();
void FactoryCompiler_free(FactoryCompiler *compiler);
void fc_compile_expr(FactoryCompiler *compiler, const Expression *expr);
