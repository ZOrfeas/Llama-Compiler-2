#include <CLI/CLI.hpp>
#include <string>

namespace lla::cli {
    class Args {
    private:
        auto setup_frontend() -> void;
        auto setup_print_options_flags(CLI::App *) -> void;
        auto setup_compilation_step_flags(CLI::App *) -> void;

    public:
        CLI::App the_app;
        CLI::Option *only_parse, *only_sem, *only_ir, *only_asm;
        std::string source_file, ast_outfile, types_outfile, ir_outfile,
            asm_outfile;
        int result;
        Args(int argc, char **argv);
    };
} // namespace lla::cli