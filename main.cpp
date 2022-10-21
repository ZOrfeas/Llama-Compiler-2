#include "ast/ast.hpp"
#include "cli/cli.hpp"
#include "parser/generator.hpp"

#include <iostream>
#include <unordered_set>
#include <vector>

auto print_ast_if_enabled(std::shared_ptr<ast::Program> ast,
                          cli::Args const &args) -> void {
    if (args.ast_outfile.length() > 0) {
        if (args.ast_outfile == "stdout") {
            // output_ast(ast);
        } else {
            std::ofstream os(args.ast_outfile);
            // output_ast(ast, os);
        }
    }
}
auto main(int argc, char **argv) -> int {
    auto args = cli::Args(argc, argv);
    if (args.result) {
        return args.result;
    }
    auto generator = ast::Generator();
    if (int parse_res = generator.parse(args.source_file)) {
        return parse_res;
    }
    auto const &ast = std::move(generator).extract_ast();
    std::cout << ast->def_stmts.size() << '\n';
    // print_ast_if_enabled(ast, args);
    if (*args.only_parse)
        return 0;
    return 0;
}
