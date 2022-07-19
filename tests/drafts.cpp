#include "../parser/generator.hpp"
#include "../cli/cli.hpp"
#include "../log/log.hpp"
#include <iostream>

int main(int argc, char** argv) {
    auto args = cli::Args(argc, argv);
    if(args.result) {
        return args.result;
    }
    // log::log<log::Error>("This {} went wrong\n", "Hello,");
    // log::log<log::Warning>("This {} went wrong\n", "Hello,");
    // log::log<log::Info>("This {} went wrong\n", "Hello,");
    // log::log<log::Debug>("This {} went wrong\n", "Hello,");
    
    // std::cout << ast::Generator().parse(args.source_file) << '\n';
    return 0;
}
