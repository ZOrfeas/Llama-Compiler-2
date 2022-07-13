
#include <CLI/CLI.hpp>
#include <string>

namespace cli {
    extern CLI::Option 
        *only_parse, *only_sem,
        *only_ir, *only_asm;
    extern std::string
        source_file,
        ast_outfile, types_outfile,
        ir_outfile, asm_outfile;
    
    int parse_cli(int argc, char **argv);
}