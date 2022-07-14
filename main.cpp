#include "ast/forward.hpp"
#include "parser/lexer.hpp"
#include "parser/parser.hpp"
#include "passes/print/ast-print.hpp"
#include "cli/cli.hpp"

#include <vector>
#include <unordered_set>

// TODO(ORF): Think on how you want cwd to work. (Currently cwd is the active working directory when the compiler was invoked)

void print_ast_if_enabled(ast::core::Program& ast) {
    if (cli::ast_outfile.length() > 0) {
        if (cli::ast_outfile == "stdout") {
            output_ast(ast);
        } else {
            std::ofstream os(cli::ast_outfile);
            output_ast(ast, os);
        }
    }
}
int main(int argc, char** argv) {
    if(auto exit_code = cli::parse_cli(argc, argv)) {
        return exit_code;
    }
    
    ast::core::Program ast{parser::parse(cli::source_file)};
    print_ast_if_enabled(ast);
    if (*cli::only_parse) return 0;

    return 0;
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
