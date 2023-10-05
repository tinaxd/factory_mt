#pragma once

/*
    Constant table for Factory language.
    This is referenced by opcodes.
    Generated during compilation, and provided to the VM.
*/

#include "opcode.h"
#include <stdint.h>
#include <vector>

enum ConstantKind
{
    CONSTKIND_INT,
    CONSTKIND_STRING,
};

using ConstantKindType = uint32_t;
using ConstantAddress = uint32_t;

class ConstantTable
{
    std::vector<int64_t> consts;

public:
    ConstantTable();
    ConstantTable(const ConstantTable &) = default;

    ConstantTable &operator=(const ConstantTable &) = default;

    ConstantAddress add_int(int64_t value);
    void get_by_address(OpcodeParamType index, ConstantKindType kind, void *target);
    inline ConstantAddress get_size() const
    {
        return static_cast<ConstantAddress>(consts.size());
    }
};
