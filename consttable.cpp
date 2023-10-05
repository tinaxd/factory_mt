#include "consttable.h"
#include <stdexcept>

ConstantTable::ConstantTable()
{
}

ConstantAddress ConstantTable::add_int(int64_t value)
{
    consts.push_back(value);
    return static_cast<ConstantAddress>(consts.size() - 1);
}

void ConstantTable::get_by_address(OpcodeParamType index, ConstantKindType kind, void *target)
{
    if (kind != CONSTKIND_INT)
    {
        throw std::runtime_error("ConstantTable::get_by_address: unsupported constant kind");
    }

    int64_t value = consts[index];
    *static_cast<int64_t *>(target) = value;
}
