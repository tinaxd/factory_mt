#include <cstdlib>
#include <cstdio>
#include <iostream>
extern "C"
{
#include "vm.h"
#include "ast.h"
}
#include "compiler.h"

extern "C" int yyparse(void);
extern "C" int yydebug;

// Expression *top_expr = nullptr;
Statement *top_stmt = nullptr;

int main(void)
{
    extern FILE *yyin;
    yydebug = 1;

    yyin = stdin;
    if (yyparse())
    {
        fprintf(stderr, "Error\n");
        return 1;
    }

    // print_expr(top_expr);
    print_stmt(top_stmt);
    std::cout << std::endl;

    FactoryCompiler compiler;
    compiler.compile_stmt(top_stmt);
    auto code = compiler.get_code();
    auto consts = compiler.get_const_table();
    std::cout << "operations: " << code.size() << std::endl;
    std::cout << "constants: " << ct_get_size(&consts) << std::endl;

    FactoryVM *vm = vm_new(1024);
    vm_set_code(vm, code.data(), code.size());
    vm_set_const_table(vm, &consts);

    for (int i = 0; i < 50; i++)
    {
        vm_step_code(vm);
        auto *stack_top = vm_get_stack_top(vm);
        if (stack_top == nullptr)
            continue;
        std::cout << "stack top: " << stack_top->value->data.int_value << std::endl;
    }

    return 0;
}
