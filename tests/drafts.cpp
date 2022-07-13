
#include <CLI/CLI.hpp>

#include "../ast/forward.hpp"
#include "../parser.hpp"
#include "../passes/print/ast-print.hpp"

#include <tuple>
#include <vector>
#include <unordered_set>

IncludeStack include_stack{};

namespace clivars {
    constexpr auto comp_step_grp_name = "Compilation-steps";
    constexpr auto print_opts_grp_name = "Print-options";

    CLI::Option *only_parse, *only_sem, *only_ir, *only_asm; 

    std::string ast_outfile, types_outfile, ir_outfile, asm_outfile;
}

void setup_compilation_step_flags(CLI::App *frontend) {
    using namespace clivars;
    only_asm = frontend->add_flag("--asm", "Stop after assembly generation")
        ->group(comp_step_grp_name);
    only_ir = frontend->add_flag("--ir", "Stop after IR generation")
        ->group(comp_step_grp_name);
    only_sem = frontend->add_flag("--sem", "Stop after semantic analysis")
        ->group(comp_step_grp_name);
    only_parse = frontend->add_flag("--parse", "Stop after parsing")
        ->group(comp_step_grp_name);
    frontend->group(comp_step_grp_name)->require_option(-1);
}
auto setup_print_options_flags(CLI::App *frontend) {
    using namespace clivars;
    frontend->add_flag("--print-ast{out.ast}", ast_outfile, "Print the AST")
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend->add_flag("--print-types{out.types}", types_outfile, "Print inferred types")
        ->excludes(only_parse)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend->add_flag("--print-ir{out.ll}", ir_outfile, "Print the IR")
        ->excludes(only_parse, only_sem)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend->add_flag("--print-asm{out.asm}", asm_outfile, "Print assembly")
        ->excludes(only_parse, only_sem, only_ir)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
}
void setup_frontend(CLI::App &compiler) {
    using namespace clivars;
    
    auto frontend = compiler.add_subcommand("frontend", "Compiler frontend options");

    setup_compilation_step_flags(frontend);
    setup_print_options_flags(frontend);
}


// TODO(ORF): Consider making `print` into a subcommand that performs
// TODO(ORF):   the requested action in a callback. 
int main(int argc, char** argv) {
    using namespace clivars;
    CLI::App compiler{"Compiler for the Llama language", "llamac"};
    compiler.get_formatter()->column_width(60);
    compiler.set_help_all_flag("--help-all", "More detailed help");

    std::string source_file;
    compiler.add_option("source", source_file, "The source file to compile")
        ->check(CLI::ExistingFile)
        ->required();

    setup_frontend(compiler);
    CLI11_PARSE(compiler, argc, argv);

    return 0;
}
