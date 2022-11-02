#include "cli/cli.hpp"
#include "config.hpp"
#include "parser/lexer.hpp"

#include <iostream>

#define STR_HELPER(x) #x
#define STR(x) STR_HELPER(x)

static constexpr auto version = STR(LLAMAC_VERSION_MAJOR) "." STR(
    LLAMAC_VERSION_MINOR) "." STR(LLAMAC_VERSION_PATCH);

auto main(int argc, char **argv) -> int {
    auto args = lla::cli::Args(argc, argv, version);
    if (args.result) {
        return args.result;
    }

    auto src = lla::Source(args.source_file);
    // std::cout << "Read source file is:"
    //           << "\n```\n";
    // for (const auto c : src) {
    //     std::cout << c;
    // }
    // std::cout << "\n```\n";

    auto lexer = lla::Lexer(src);
    lexer.lex();
    lexer.pretty_print_tokens();
    return 0;
}