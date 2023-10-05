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
    CONSTKIND_STRING,
};

typedef uint32_t ConstantKindType;
typedef uint32_t ConstantAddress;

typedef struct ConstantTable
{
    int64_t *consts;
    size_t consts_size;
    size_t consts_capacity;
} ConstantTable;

ConstantTable *ct_new();
void ct_free(ConstantTable *ct);
ConstantAddress ct_add_int(ConstantTable *ct, int64_t value);
void ct_get_by_address(ConstantTable *ct, OpcodeParamType index, ConstantKindType kind, void *target);
ConstantAddress ct_get_size(ConstantTable *ct);
