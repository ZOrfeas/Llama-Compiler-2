#include "ast/forward.hpp"
#include "parser.hpp"
#include "passes/print/ast-print.hpp"

int main() {
    /* yydebug = 1; // default val is zero so just comment this to disable */
    ast::core::Program *program = nullptr;
    int result = yyparse(program);
    /* if (program == nullptr) std::cout << "Test"; */
    auto v = PrintVisitor();
    program->accept(v);
    if (result == 0) std::cout << "Success\n";
    return result;
}
