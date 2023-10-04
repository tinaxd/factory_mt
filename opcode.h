#pragma once
#include <stdint.h>
/*
    Opcodes for the virtual machine.
    Some opcodes contain a parameter, which is a 32-bit integer.
    All opcodes are represented by 64-bit integers
    (32-bit for the opcode tag, 32-bit for the parameter).
*/

typedef enum
{
    OPC_CONST_INT, // (address to integer on the const table)
    OPC_ADD2,
    OPC_SUB2,
    OPC_MUL2,
    OPC_DIV2,
    OPC_MOD2,
    OPC_ROT2,
} OpcodeKind;

typedef struct Opcode
{
    uint32_t tag;
    uint32_t param;
};