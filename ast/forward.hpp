#ifndef AST_FORWARD_HPP
#define AST_FORWARD_HPP

namespace ast {
namespace stmts {
    struct DefStmt;
    
    struct LetStmt;
    struct TypeStmt;
}
namespace annotation {
    struct TypeAnnotation;

    struct BasicType;
    struct FunctionType;
    struct ArrayType;
    struct RefType;
    struct CustomType;
}
namespace defs {
    struct LetDef;
    struct TypeDef;

    struct Constant;
    struct Param;
    struct Function;
    struct Array;
    struct Variable;
    struct Constructor;
}
namespace exprs {
    struct Expression;

    struct LetIn;
    namespace literals {
        struct Int;
        struct Char;
        struct Bool;
        struct Float;
        struct String;
    }
    struct Literal;
    struct UnaryOp;
    struct BinaryOp;
    struct NewOp;
    struct While;
    struct For;
    struct If;
    struct Dim;
    struct IdCall;
    struct FuncCall;
    struct ConstrCall;
    struct ArrayAccess;
    namespace match {
        struct IdPattern;
        struct ConstructorPattern;
        struct Clause;
        struct Pattern;
    }
    struct Match;
}
}

#endif // AST_FORWARD_HPP