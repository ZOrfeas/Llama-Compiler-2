#include "./cli.hpp"


void setup_compilation_step_flags(CLI::App*);
void setup_print_options_flags(CLI::App*);
void setup_frontend(CLI::App&);

namespace cli {
    CLI::Option
        *only_parse, *only_sem,
        *only_ir, *only_asm;
    std::string
        source_file,
        ast_outfile, types_outfile,
        ir_outfile, asm_outfile;

    int parse_cli(int argc, char **argv) {
        CLI::App compiler{"Compiler for the Llama language", "llamac"};
        compiler.get_formatter()->column_width(60);
        compiler.set_help_all_flag("--help-all", "More detailed help");

        compiler.add_option("source", source_file, "The source file to compile")
            ->check(CLI::ExistingFile)
            ->required();

        setup_frontend(compiler);
        try {
            compiler.parse(argc, argv);
        } catch (const CLI::CallForHelp& e) {
            compiler.exit(e);
            return 99;
        } catch (const CLI::CallForAllHelp& e) {
            compiler.exit(e);
            return 98;
        } catch (const CLI::Error& e) {
            return compiler.exit(e);
        }

        // CLI11_PARSE(compiler, argc, argv);
        return 0;
    }
}
constexpr auto comp_step_grp_name = "Compilation-steps";
constexpr auto print_opts_grp_name = "Print-options";

void setup_frontend(CLI::App &compiler) {    
    auto frontend = compiler.add_subcommand("frontend", "Compiler frontend options");

    setup_compilation_step_flags(frontend);
    compiler.group(comp_step_grp_name)->require_option(-1);
    
    setup_print_options_flags(frontend);
}

void setup_print_options_flags(CLI::App *frontend) {
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

void setup_compilation_step_flags(CLI::App *frontend) {
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
