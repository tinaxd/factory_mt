#include "vm.h"
#include <memory>
#include <cstdint>
#include <iostream>

FactoryVM::FactoryVM(uint64_t stack_size)
    : stack(std::vector<FactoryObject *>(stack_size)), ct(new ConstantTable()), pc(0)
{
    _stack_frames.push(StackFrame());
}

void FactoryVM::set_code(std::vector<Opcode> code)
{
    this->code = std::move(code);
}

void FactoryVM::set_const_table(std::unique_ptr<ConstantTable> ct)
{
    this->ct = std::move(ct);
}

void FactoryVM::step_code()
{
    // std::cout << "pc: " << pc << std::endl;
    // std::cout << "code.size(): " << this->code.size() << std::endl;
    if (pc >= code.size())
        return;
    Opcode op = code[pc];
    std::cout << "executing opcode " << op.tag << std::endl;
    switch (op.tag)
    {
    case OPC_CONST_INT:
    {
        OpcodeParamType ct_index = op.param;
        int64_t const_value;
        this->ct->get_by_address(ct_index, CONSTKIND_INT, &const_value);
        stack[stack_top++] = fo_int_const(const_value);
        break;
    }
    case OPC_ADD2:
    {
        FactoryObject *right = stack[--stack_top];
        FactoryObject *left = stack[--stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        stack[stack_top++] = fo_int_const(left_value + right_value);
        break;
    }
    case OPC_SUB2:
    {
        FactoryObject *right = stack[--stack_top];
        FactoryObject *left = stack[--stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        stack[stack_top++] = fo_int_const(left_value - right_value);
        break;
    }
    case OPC_MUL2:
    {
        FactoryObject *right = stack[--stack_top];
        FactoryObject *left = stack[--stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        stack[stack_top++] = fo_int_const(left_value * right_value);
        break;
    }
    case OPC_DIV2:
    {
        FactoryObject *right = stack[--stack_top];
        FactoryObject *left = stack[--stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        stack[stack_top++] = fo_int_const(left_value / right_value);
        break;
    }
    case OPC_MOD2:
    {
        FactoryObject *right = stack[--stack_top];
        FactoryObject *left = stack[--stack_top];
        // TODO: check if int
        int64_t left_value = left->value->data.int_value;
        int64_t right_value = right->value->data.int_value;
        stack[stack_top++] = fo_int_const(left_value % right_value);
        break;
    }
    case OPC_EXIT:
    {
        FactoryObject *exit_code = stack[--stack_top];
        // TODO: check if exit_code is integer
        std::exit(exit_code->value->data.int_value);
        break;
    }
    case OPC_DISCARD:
    {
        stack_top--;
        break;
    }
    case OPC_STORE:
    {
        FactoryObject *value = stack[--stack_top];
        uint32_t address = op.param;
        _stack_frames.top().store(address, value);
        break;
    }
    case OPC_LOAD:
    {
        uint32_t address = op.param;
        FactoryObject *value = _stack_frames.top().load(address);
        stack[stack_top++] = value;
        break;
    }
    default:
        std::cout << "unknown opcode " << op.tag << std::endl;
        break;
    }

    pc++;
}

FactoryObject *FactoryVM::get_stack_top() const
{
    if (stack_top > 0)
    {
        return stack[stack_top - 1];
    }
    return nullptr;
}

void StackFrame::store(uint32_t address, FactoryObject *value)
{
    if (address >= memory.size())
    {
        memory.resize(address + 1);
    }

    memory[address] = value;
}

FactoryObject *StackFrame::load(uint32_t address)
{
    if (address >= memory.size())
    {
        return nullptr;
    }

    return memory[address];
}
