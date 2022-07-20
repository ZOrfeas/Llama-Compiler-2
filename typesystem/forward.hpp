#ifndef TYPESYSTEM_FORWARD_HPP
#define TYPESYSTEM_FORWARD_HPP

namespace typesys {
    enum class TypeEnum {
        UNIT, INT, CHAR, BOOL, FLOAT,
        ARRAY, REF, FUNCTION, CUSTOM, CONSTRUCTOR, UNKNOWN
    };
    class Type;
    struct Unit;
    struct Int;
    struct Char;
    struct Bool;
    struct Float;
    class Array;
    class Ref;
    class Function;
    class Constructor;
    class Custom;
    class Unknown;
}

#endif // TYPESYSTEM_FORWARD_HPP