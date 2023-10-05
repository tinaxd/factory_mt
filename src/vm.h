#pragma once
/*
    Virtual machine for Factory language.
    It is a stack machine whose elements are pointers to
    objects allocated on the heap.

    Various opcodes to manipulate the stack are defined in opcode.h.
*/

#include "object.h"
#include "opcode.h"
#include "consttable.h"
#include <stdint.h>

struct StackFrame;

typedef struct FactoryVM
{
    FactoryObject **stack;
    size_t stack_top;
    size_t stack_capacity;

    Opcode *code;
    size_t code_size;
    uint64_t pc; // program counter

    ConstantTable *ct; // constant table

    struct StackFrame *stack_frames;
    size_t stack_frames_top;
    size_t stack_frames_capacity;
} FactoryVM;

FactoryVM *vm_new(uint64_t stack_size);
void vm_free(FactoryVM *vm);
void vm_set_code(FactoryVM *vm, Opcode *code, size_t code_size);
void vm_set_const_table(FactoryVM *vm, ConstantTable *ct);
void vm_step_code(FactoryVM *vm);
FactoryObject *vm_get_stack_top(FactoryVM *vm);

typedef struct StackFrame
{
    FactoryObject **memory;
    size_t memory_size;
    size_t memory_capacity;
} StackFrame;

StackFrame *sf_new();
void sf_init(StackFrame *sf);
void sf_free(StackFrame *sf);
void sf_store(StackFrame *sf, uint32_t address, FactoryObject *value);
FactoryObject *sf_load(StackFrame *sf, uint32_t address);
