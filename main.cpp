#include "ast/forward.hpp"
#include "parser.hpp"
#include "passes/print/ast-print.hpp"

#include "typesystem/types.hpp"

void print_ast(ast::core::Program& p, std::ostream& os = std::cout) {
    auto v = PrintVisitor(os);
    p.accept(&v);
}

int main() {
    /* yydebug = 1; // default val is zero so just comment this to disable */
    ast::core::Program program;
    int result = yyparse(program);
    print_ast(program);
    if (result == 0) std::cout << "Success\n";
    return result;
    // using namespace typesys;
    // Type t1 = Type::get<Unit>();
    // Type t2 = Type::get<Unit>();
    // Type t3 = Type::get<Int>();
    // Type arr = Type::get<Array>(t1, 2);
    // Type arr2 = Type::get<Array>(t2, 2);
    // std::cout << arr << '\n';
    // std::cout << arr2 << '\n';
    // std::cout << "Equal: " <<
    //     std::boolalpha << (arr == arr2) << '\n';
}
