#include "ast/forward.hpp"
#include "parser.hpp"
#include "passes/print/ast-print.hpp"

#include <vector>
#include <unordered_set>

// TODO(ORF): Hide filename utilities and only provide push/pop/contains functionality.
// TODO(ORF): Use cli-args to specify initial input file.
// TODO(ORF): Think on how you want cwd to work. (Currently cwd is the active working directory when the compiler was invoked)

std::unordered_set<std::string> filename_set;
std::vector<std::string> filename_stack;


void print_ast(ast::core::Program& p, std::ostream& os = std::cout) {
    auto v = PrintVisitor(os);
    p.accept(&v);
}

int main() {
    /* yydebug = 1; // default val is zero so just comment this to disable */
    ast::core::Program program;
    filename_set.insert("dummy_filename_for_now");
    filename_stack.push_back("dummy_filename_for_now");
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
    // Type ref = Type::get<Ref>(t3);
    // std::cout << arr  << " == " << arr2 << ' ' << std::boolalpha << (arr == arr2) << '\n';
    // std::cout << arr << " == " << ref << ' ' << std::boolalpha << (arr == ref) << '\n';
}
