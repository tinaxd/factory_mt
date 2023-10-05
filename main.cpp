#include <cstdlib>
#include <cstdio>
#include "ast.h"
#include "compiler.h"
#include "vm.h"
#include <iostream>

Expression *top_expr = nullptr;

int main(void)
{
    extern int yyparse(void);
    extern FILE *yyin;

    yyin = stdin;
    if (yyparse())
    {
        fprintf(stderr, "Error\n");
        return 1;
    }

    print_expr(top_expr);
    std::cout << std::endl;

    FactoryCompiler compiler;
    compiler.compile_expr(top_expr);
    const auto code = compiler.get_code();
    const auto consts = compiler.get_const_table();
    std::cout << "operations: " << code.size() << std::endl;
    std::cout << "constants: " << consts.get_size() << std::endl;

    FactoryVM vm(100);
    vm.set_code(code);
    vm.set_const_table(std::make_unique<ConstantTable>(consts));

    for (int i = 0; i < 50; i++)
    {
        vm.step_code();
        auto *stack_top = vm.get_stack_top();
        if (stack_top == nullptr)
            continue;
        std::cout << "stack top: " << stack_top->value->data.int_value << std::endl;
    }

    return 0;
}
