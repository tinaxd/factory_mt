#include "vm.h"

FactoryVM *vm_init_new(uint64_t stack_size)
{
    FactoryVM *vm = malloc(sizeof(FactoryVM));
    vm->stack = malloc(sizeof(FactoryObject *) * stack_size);
    vm->stack_size = stack_size;
    vm->stack_top = 0;
    return vm;
}

void vm_set_code(FactoryVM *vm, Opcode *code, uint64_t code_size)
{
    vm->code = code;
    vm->code_size = code_size;
}

void vm_step_code(FactoryVM *vm)
{
    Opcode op = vm->code[vm->pc];
    switch (op.tag)
    {
    OPC_CONST_INT:
    {
        OpcodeParamType ct_index = op.param;
        int64_t const_value;
        consttable_get_by_index(vm->ct, ct_index, CONSTKIND_INT, &const_value);
        vm->stack[vm->stack_top++] = fo_int_const(const_value);
        break;
    }
    OPC_ADD2:
    {
        FactoryObject *right = vm->stack[--vm->stack_top];
        FactoryObject *left = vm->stack[--vm->stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        vm->stack[vm->stack_top++] = fo_int_const(left_value + right_value);
        break;
    }
    OPC_SUB2:
    {
        FactoryObject *right = vm->stack[--vm->stack_top];
        FactoryObject *left = vm->stack[--vm->stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        vm->stack[vm->stack_top++] = fo_int_const(left_value - right_value);
        break;
    }
    OPC_MUL2:
    {
        FactoryObject *right = vm->stack[--vm->stack_top];
        FactoryObject *left = vm->stack[--vm->stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        vm->stack[vm->stack_top++] = fo_int_const(left_value * right_value);
        break;
    }
    OPC_DIV2:
    {
        FactoryObject *right = vm->stack[--vm->stack_top];
        FactoryObject *left = vm->stack[--vm->stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        vm->stack[vm->stack_top++] = fo_int_const(left_value / right_value);
        break;
    }
    OPC_MOD2:
    {
        FactoryObject *right = vm->stack[--vm->stack_top];
        FactoryObject *left = vm->stack[--vm->stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        vm->stack[vm->stack_top++] = fo_int_const(left_value % right_value);
        break;
    }
    }

    vm->pc++;
}
