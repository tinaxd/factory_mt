#pragma once

/*
    Constant table for Factory language.
    This is referenced by opcodes.
    Generated during compilation, and provided to the VM.
*/

#include "opcode.h"
#include <stdint.h>

enum ConstantKind
{
    CONSTKIND_INT,
};

typedef uint32_t ConstantKindType;

typedef struct ConstantTable
{

} ConstantTable;

void consttable_get_by_index(const ConstantTable *ct, OpcodeParamType index, ConstantKind kind, void *target);
