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
    OPC_EQ2,
    OPC_NEQ2,
    OPC_LT2,
    OPC_LE2,
    OPC_GT2,
    OPC_GE2,
    OPC_EXIT,    // exit program with exit code
    OPC_DISCARD, // discard top of stack
    OPC_STORE,   // store top of stack to memory indexed by parameter
    OPC_LOAD,    // load from memory indexed by parameter
    JMP_IF_TRUE, // jump to address if top of stack is true (FVAL_BOOL)
    JMP_ALWAYS,  // jump to address
} OpcodeKind;

typedef uint32_t OpcodeTagType;
typedef uint32_t OpcodeParamType;

typedef struct Opcode
{
    OpcodeTagType tag;
    OpcodeParamType param;
} Opcode;
