#ifndef __AST_FORWARD_HPP__
#define __AST_FORWARD_HPP__

namespace ast {
namespace core {
    class Node;
    class DefStmt;
    class Program;
    class TypeAnnotation;
    class Expression;
}
namespace stmt {
    class LetStmt;
    class TypeStmt;
}
namespace def {
    class Def;
    class TypeDef;
    class Constant;
    class Function;
    class Mutable;
    class Array;
    class Variable;
}
namespace expr {
    class LetIn;
    class Literal;
    namespace literal {
        class Unit;
        class Int;
        class Char;
        class Bool;
        class Float;
        class String;
    }
    namespace op {
        class Binary;
        class Unary;
        class New;
    }
    class While;
    class For;
    class If;
    class Dim;
    class IdCall;
    class FuncCall;
    class ConstrCall;
    class ArrayAccess;
    class Match;
}
namespace annotation {
    class BasicType;
    class FunctionType;
    class ArrayType;
    class RefType;
    class CustomType;
}
namespace utils::def {
    class Constructor;
    class Param;
}
namespace utils::match {
    class Pattern;
    class PatLiteral;
    class PatId;
    class PatConstr;
    class Clause;
}
}

#endif // __AST_FORWARD_HPP__