#include "consttable.h"
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

ConstantTable *ct_new()
{
    ConstantTable *ct = (ConstantTable *)malloc(sizeof(ConstantTable));
    ct->consts = (int64_t *)malloc(sizeof(int64_t) * 128);
    ct->consts_size = 0;
    ct->consts_capacity = 128;
    return ct;
}

void ct_free(ConstantTable *ct)
{
    free(ct->consts);
}

static void ct_realloc_if_necessary(ConstantTable *ct)
{
    if (ct->consts_size == ct->consts_capacity)
    {
        ct->consts_capacity *= 2;
        ct->consts = (int64_t *)realloc(ct->consts, sizeof(int64_t) * ct->consts_capacity);
    }
}

ConstantAddress ct_add_int(ConstantTable *ct, int64_t value)
{
    ct_realloc_if_necessary(ct);
    ct->consts[ct->consts_size] = value;
    ct->consts_size++;
    return ct->consts_size - 1;
}

void ct_get_by_address(ConstantTable *ct, OpcodeParamType index, ConstantKindType kind, void *target)
{
    if (kind != CONSTKIND_INT)
    {
        fprintf(stderr, "ConstantTable::get_by_address: unsupported constant kind\n");
        abort();
    }

    int64_t value = ct->consts[index];
    *(int64_t *)target = value;
}

ConstantAddress ct_get_size(const ConstantTable *ct)
{
    return ct->consts_size;
}
