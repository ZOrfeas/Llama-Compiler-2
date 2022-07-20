#include "cli/cli.hpp"
#include "ast/forward.hpp"
#include "parser/parser.hpp"

#include <vector>
#include <unordered_set>

// void print_ast_if_enabled(ast::core::Program& ast) {
//     if (cli::ast_outfile.length() > 0) {
//         if (cli::ast_outfile == "stdout") {
//             output_ast(ast);
//         } else {
//             std::ofstream os(cli::ast_outfile);
//             output_ast(ast, os);
//         }
//     }
// }
int main(int argc, char** argv) {
    auto args = cli::Args(argc, argv);
    if(args.result) {
        return args.result;
    }
    
    // ast::Generator().parse(args.source_file)
    // print_ast_if_enabled(ast);
    if (*args.only_parse) return 0;
    return 0;
}
