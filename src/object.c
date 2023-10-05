#include "object.h"
#include <stdlib.h>

FactoryObject *fo_int_const(int64_t value)
{
    FactoryValue *v = (FactoryValue *)malloc(sizeof(FactoryValue));
    v->kind = FVAL_INT;
    v->data.int_value = value;

    FactoryObject *o = (FactoryObject *)malloc(sizeof(FactoryObject));
    o->value = v;
    o->refcount = 1;

    return o;
}
