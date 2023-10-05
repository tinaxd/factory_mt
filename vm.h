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
#include <vector>
#include <memory>
#include <stack>

class StackFrame;

class FactoryVM
{
    std::vector<FactoryObject *> stack;
    size_t stack_top = 0;

    std::vector<Opcode> code;
    uint64_t pc; // program counter

    std::unique_ptr<ConstantTable> ct; // constant table

    std::stack<StackFrame> _stack_frames;

public:
    FactoryVM(uint64_t stack_size);

    void set_code(std::vector<Opcode> code);
    void set_const_table(std::unique_ptr<ConstantTable> ct);
    void step_code();

    FactoryObject *get_stack_top() const;
};

class StackFrame
{
    std::vector<FactoryObject *> memory;

public:
    void store(uint32_t address, FactoryObject *value);
    FactoryObject *load(uint32_t address);
};
