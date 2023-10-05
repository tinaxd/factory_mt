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

ConstantTable *consttable_new();
void consttable_free(ConstantTable *ct);
void consttable_get_by_index(const ConstantTable *ct, OpcodeParamType index, ConstantKindType kind, void *target);
