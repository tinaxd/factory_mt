#pragma once
/*
    Object type for Factory language.
    It is the only type that can exist as element of the stack of the VM.
    It consists of
    - an address to a heap-allocated object
    - a reference count

    The reference count is manipulated by non-atomic operations.
    This makes VM not thread-safe.
    In later versions, we will add a tag whether the value may be accessed by multiple threads, and if so, we will use atomic operations.
    Or we will fully depend on the multi-process model to achieve concurrency.

    Value type is the value which is pointed to by FactoryObjects.
    It is a tagged union of possible values.
    The tag is a 32-bit integer.
    Currently, the only possible type is the Integer.
*/

#include <stdint.h>

struct FactoryObject;
struct FactoryValue;
typedef uint32_t FactoryValueKind;

typedef struct FactoryObject
{
    uint64_t refcount;
    struct FactoryValue *value;
} FactoryObject;

enum FactoryValueKind
{
    FVAL_INT,
};

typedef struct FactoryValue
{
    FactoryValueKind kind;
    union
    {
        int64_t int_value;
    } data;
} FactoryValue;

FactoryObject *fo_int_const(int64_t value);
