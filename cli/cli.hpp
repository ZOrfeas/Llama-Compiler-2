
#include <CLI/CLI.hpp>
#include <string>

namespace cli {
    class Args {
    private:
        void setup_frontend();
        void setup_print_options_flags(CLI::App*);
        void setup_compilation_step_flags(CLI::App*);
    public:
        CLI::App the_app;
        CLI::Option *only_parse, *only_sem, *only_ir, *only_asm;
        std::string source_file, ast_outfile, types_outfile, ir_outfile, asm_outfile;
        int result;
        Args(int argc, char **argv);
    };
}