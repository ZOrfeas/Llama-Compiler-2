#ifndef __TYPES__HPP__
#define __TYPES__HPP__

#include <string_view>

namespace types {
    enum class Builtin {
        UNIT, INT, CHAR, BOOL, FLOAT
    };
    static const char* builtin_name[] = {
        "unit", "int", "char", "bool", "float"
    };
    // TODO: Move implementation to a .cpp file
    inline const char* type_name(Builtin b) {
        return builtin_name[static_cast<int>(b)];
    }

}

#endif // __TYPES__HPP__