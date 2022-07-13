#include "ast/forward.hpp"
#include "ast/parts/core.hpp"
#include "lexer.hpp"
#include "parser.hpp"
#include "passes/print/ast-print.hpp"
#include "cli/cli.hpp"

#include <vector>
#include <unordered_set>

// TODO(ORF): Hide filename utilities and only provide push/pop/contains functionality.
// TODO(ORF): Use cli-args to specify initial input file.
// TODO(ORF): Think on how you want cwd to work. (Currently cwd is the active working directory when the compiler was invoked)

void print_ast(ast::core::Program& p, std::ostream& os = std::cout) {
    auto v = PrintVisitor(os);
    p.accept(&v);
}
void handle_prints(ast::core::Program& ast) {
    if (cli::ast_outfile.length() > 0) {
        if (cli::ast_outfile == "stdout") {
            print_ast(ast);
        } else {
            std::ofstream os(cli::ast_outfile);
            print_ast(ast, os);
        }
    }
}

int main(int argc, char** argv) {
    if(auto exit_code = cli::parse_cli(argc, argv); exit_code != 0) {
        return exit_code;
    }
    ast::core::Program ast;
    if (int parse_result = parser::parse(ast, cli::source_file)) {
        return parse_result;
    };
    handle_prints(ast);
    
    
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
