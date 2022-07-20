#include "./cli.hpp"

using namespace cli;

constexpr auto comp_step_grp_name = "Compilation-steps";
constexpr auto print_opts_grp_name = "Print-options";

Args::Args(int argc, char **argv):
    the_app("Compiler for the Llama language", "llamac") {
    the_app.get_formatter()->column_width(60);
    the_app.set_help_all_flag("--help-all", "More detailed help");

    the_app.add_option("source", source_file, "The source file to compile")
        ->check(CLI::ExistingFile)
        ->required();

    setup_frontend();
    result = 0;
    try {
        the_app.parse(argc, argv);
    } catch (const CLI::CallForHelp& e) {
        the_app.exit(e);
        result = 99;
    } catch (const CLI::CallForAllHelp& e) {
        the_app.exit(e);
        result =  98;
    } catch (const CLI::Error& e) {
        result = the_app.exit(e);
    }
}


void Args::setup_frontend() {    
    auto frontend = the_app.add_subcommand("frontend", "Compiler frontend options");

    setup_compilation_step_flags(frontend);
    the_app.group(comp_step_grp_name)->require_option(-1);
    
    setup_print_options_flags(frontend);
}

void Args::setup_print_options_flags(CLI::App* frontend) {
    using namespace cli;
    frontend->add_flag("--print-ast{stdout}", ast_outfile, "Print the AST")
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend->add_flag("--print-types{stdout}", types_outfile, "Print inferred types")
        ->excludes(only_parse)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend->add_flag("--print-ir{stdout}", ir_outfile, "Print the IR")
        ->excludes(only_parse, only_sem)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend->add_flag("--print-asm{stdout}", asm_outfile, "Print assembly")
        ->excludes(only_parse, only_sem, only_ir)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
}

void Args::setup_compilation_step_flags(CLI::App *frontend) {
    using namespace cli;
    only_asm = frontend->add_flag("--asm", "Stop after assembly generation")
        ->group(comp_step_grp_name);
    only_ir = frontend->add_flag("--ir", "Stop after IR generation")
        ->group(comp_step_grp_name);
    only_sem = frontend->add_flag("--sem", "Stop after semantic analysis")
        ->group(comp_step_grp_name);
    only_parse = frontend->add_flag("--parse", "Stop after parsing")
        ->group(comp_step_grp_name);
}
