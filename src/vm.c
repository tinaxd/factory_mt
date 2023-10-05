#include "vm.h"
#include <memory.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

FactoryVM *vm_new(uint64_t stack_size)
{
    FactoryVM *vm = (FactoryVM *)malloc(sizeof(FactoryVM));
    vm->stack = (FactoryObject **)malloc(sizeof(FactoryObject *) * stack_size);
    vm->stack_top = 0;
    vm->stack_capacity = stack_size;

    vm->code = NULL;
    vm->pc = 0;

    vm->ct = NULL;

    vm->stack_frames = (StackFrame *)malloc(sizeof(StackFrame));
    sf_init(&vm->stack_frames[0]);
    vm->stack_frames_top = 0;
    vm->stack_frames_capacity = 1;
    return vm;
}

void vm_free(FactoryVM *vm)
{
    free(vm->stack);
    vm->stack_top = 0;
    vm->stack_capacity = 0;

    free(vm->code);
    vm->pc = 0;

    free(vm->ct);

    for (size_t i = 0; i < vm->stack_frames_top; i++)
    {
        sf_free(&vm->stack_frames[i]);
    }
    free(vm->stack_frames);
    vm->stack_frames_top = 0;
    vm->stack_frames_capacity = 0;
}

void vm_set_code(FactoryVM *vm, Opcode *code, size_t code_size)
{
    vm->code = code;
    vm->code_size = code_size;
}

void vm_set_const_table(FactoryVM *vm, ConstantTable *ct)
{
    vm->ct = ct;
}

static StackFrame *vm_get_current_stack_frame(FactoryVM *vm)
{
    return &vm->stack_frames[vm->stack_frames_top];
}

void vm_step_code(FactoryVM *vm)
{
    // std::cout << "pc: " << pc << std::endl;
    // std::cout << "code.size(): " << this->code.size() << std::endl;
    if (vm->pc >= vm->code_size)
        return;
    Opcode op = vm->code[vm->pc];
    printf("executing opcode %d\n", op.tag);
    switch (op.tag)
    {
    case OPC_CONST_INT:
    {
        OpcodeParamType ct_index = op.param;
        int64_t const_value;
        ct_get_by_address(vm->ct, ct_index, CONSTKIND_INT, &const_value);
        vm->stack[vm->stack_top++] = fo_int_const(const_value);
        break;
    }
    case OPC_ADD2:
    {
        FactoryObject *right = vm->stack[--vm->stack_top];
        FactoryObject *left = vm->stack[--vm->stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        vm->stack[vm->stack_top++] = fo_int_const(left_value + right_value);
        break;
    }
    case OPC_SUB2:
    {
        FactoryObject *right = vm->stack[--vm->stack_top];
        FactoryObject *left = vm->stack[--vm->stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        vm->stack[vm->stack_top++] = fo_int_const(left_value - right_value);
        break;
    }
    case OPC_MUL2:
    {
        FactoryObject *right = vm->stack[--vm->stack_top];
        FactoryObject *left = vm->stack[--vm->stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        vm->stack[vm->stack_top++] = fo_int_const(left_value * right_value);
        break;
    }
    case OPC_DIV2:
    {
        FactoryObject *right = vm->stack[--vm->stack_top];
        FactoryObject *left = vm->stack[--vm->stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        vm->stack[vm->stack_top++] = fo_int_const(left_value / right_value);
        break;
    }
    case OPC_MOD2:
    {
        FactoryObject *right = vm->stack[--vm->stack_top];
        FactoryObject *left = vm->stack[--vm->stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        vm->stack[vm->stack_top++] = fo_int_const(left_value % right_value);
        break;
    }
    case OPC_EXIT:
    {
        FactoryObject *exit_code = vm->stack[--vm->stack_top];
        // TODO: check if exit_code is integer
        exit(exit_code->value->data.int_value);
        break;
    }
    case OPC_DISCARD:
    {
        vm->stack_top--;
        break;
    }
    case OPC_STORE:
    {
        FactoryObject *value = vm->stack[--vm->stack_top];
        uint32_t address = op.param;
        sf_store(vm_get_current_stack_frame(vm), address, value);
        break;
    }
    case OPC_LOAD:
    {
        uint32_t address = op.param;
        FactoryObject *value = sf_load(vm_get_current_stack_frame(vm), address);
        vm->stack[vm->stack_top++] = value;
        break;
    }
    default:
        fprintf(stderr, "unknown opcode %d\n", op.tag);
        break;
    }

    vm->pc++;
}

FactoryObject *vm_get_stack_top(FactoryVM *vm)
{
    if (vm->stack_top > 0)
    {
        return vm->stack[vm->stack_top - 1];
    }
    return NULL;
}

StackFrame *sf_new()
{
    StackFrame *sf = (StackFrame *)malloc(sizeof(StackFrame));
    sf_init(sf);
    return sf;
}

void sf_init(StackFrame *sf)
{
    sf->memory_capacity = 32;
    sf->memory_size = 0;
    sf->memory = (FactoryObject **)malloc(sizeof(FactoryObject *) * sf->memory_capacity);
}

void sf_free(StackFrame *sf)
{
    free(sf->memory);
    sf->memory_capacity = 0;
    sf->memory_size = 0;
}

void sf_store(StackFrame *sf, uint32_t address, FactoryObject *value)
{
    if (address >= sf->memory_capacity)
    {
        sf->memory_capacity = address + 1;
        sf->memory = (FactoryObject **)realloc(sf->memory, sizeof(FactoryObject *) * sf->memory_capacity);
    }

    sf->memory[address] = value;
}

FactoryObject *sf_load(StackFrame *sf, uint32_t address)
{
    if (address >= sf->memory_capacity)
    {
        return NULL;
    }

    return sf->memory[address];
}
