#include "cli/cli.hpp"
#include "config.hpp"
#include <iostream>

#define STR_HELPER(x) #x
#define STR(x) STR_HELPER(x)

static constexpr auto version = STR(LLAMAC_VERSION_MAJOR) "." STR(
    LLAMAC_VERSION_MINOR) "." STR(LLAMAC_VERSION_PATCH);

auto main(int argc, char **argv) -> int {
    using namespace lla;

    auto args = lla::cli::Args(argc, argv, version);

    if (args.result) {
        return args.result;
    }
    return 0;
}