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

typedef struct FactoryVM
{
    FactoryObject **stack; // array of pointers to FactoryObject
    uint64_t stack_size;   // size of stack
    uint64_t stack_top;    // index of the top of the stack

    Opcode *code;       // array of opcodes
    uint64_t code_size; // size of code
    uint64_t pc;        // program counter

    ConstantTable *ct; // constant table
} FactoryVM;

FactoryVM *vm_init_new(uint64_t stack_size);
void vm_set_code(FactoryVM *vm, Opcode *code, uint64_t code_size);
void vm_step_code(FactoryVM *vm);
