#pragma once
#include <stdint.h>
#include <stdbool.h>

enum FactoryValueTag
{
    FVTagNull,
    FVTagInteger,
    FVTagBoolean,
};

struct FactoryValue
{
    FactoryValueTag tag;
    union
    {
        int32_t integer;
        bool boolean;
    } data;
};

struct FactoryObject
{
    struct FactoryValue value;
};

struct FactoryObject *factory_alloc_null(void);
struct FactoryObject *factory_alloc_integer(int32_t value);
struct FactoryObject *factory_alloc_boolean(bool value);
