#include "cli/cli.hpp"
#include "config.hpp"
#include <iostream>

auto main(int argc, char **argv) -> int {
    using namespace lla;
    auto args = lla::cli::Args(argc, argv);
    if (args.result) {
        return args.result;
    }
    return 0;
}