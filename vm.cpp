#include "vm.h"
#include <memory>
#include <cstdint>
#include <iostream>

FactoryVM::FactoryVM(uint64_t stack_size)
    : stack(std::vector<FactoryObject *>(stack_size)), ct(new ConstantTable())
{
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
