#include "cli/cli.hpp"
#include "config.hpp"
#include "parser/lexer.hpp"

#define STR_HELPER(x) #x
#define STR(x) STR_HELPER(x)

static constexpr auto version = STR(LLAMAC_VERSION_MAJOR) "." STR(
    LLAMAC_VERSION_MINOR) "." STR(LLAMAC_VERSION_PATCH);

auto main(int argc, char **argv) -> int {
    auto args = lla::cli::Args(argc, argv, version);
    if (args.result) {
        return args.result;
    }

    lla::Lexer lexer(args.source_file);
    const auto src = lexer.src;
    for (auto c : src)
        std::cout << c;
    return 0;
}