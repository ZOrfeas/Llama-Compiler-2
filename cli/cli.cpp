#include "./cli.hpp"

using namespace lla::cli;

constexpr auto comp_step_grp_name = "Compilation-steps";
constexpr auto print_opts_grp_name = "Print-options";

Args::Args(int argc, char **argv, std::string_view version)
    : the_app("Compiler for the Llama language v" + std::string(version),
              "llamac"),
      only_preprocess(nullptr), only_lex(nullptr), only_parse(nullptr),
      only_sem(nullptr), only_ir(nullptr), only_asm(nullptr), result(0) {
    the_app.get_formatter()->column_width(60);
    the_app.set_help_all_flag("--help-all", "More detailed help");

    the_app.add_option("source", source_file, "The source file to compile")
        ->check(CLI::ExistingFile)
        ->required();

    setup_frontend();
    try {
        the_app.parse(argc, argv);
    } catch (const CLI::CallForHelp &e) {
        the_app.exit(e);
        result = 99;
    } catch (const CLI::CallForAllHelp &e) {
        the_app.exit(e);
        result = 98;
    } catch (const CLI::Error &e) {
        result = the_app.exit(e);
    }
}

auto Args::setup_frontend() -> void {
    auto frontend =
        the_app.add_subcommand("frontend", "Compiler frontend options");

    setup_compilation_step_flags(frontend);
    the_app.group(comp_step_grp_name)->require_option(-1);

    setup_print_options_flags(frontend);
    the_app.group(print_opts_grp_name)->require_option(0);
}

auto Args::setup_print_options_flags(CLI::App *frontend) -> void {
    using namespace cli;
    frontend
        ->add_flag("--print-preprocessed{stdout}", preprocessed_outfile,
                   "Print the preprocessed source")
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend
        ->add_flag("--print-tokens{stdout}", tokens_outfile, "Print the tokens")
        ->excludes(only_preprocess)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend->add_flag("--print-ast{stdout}", ast_outfile, "Print the AST")
        ->excludes(only_preprocess, only_lex)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend
        ->add_flag("--print-types{stdout}", types_outfile,
                   "Print inferred types")
        ->excludes(only_preprocess, only_lex, only_parse)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend->add_flag("--print-ir{stdout}", ir_outfile, "Print the IR")
        ->excludes(only_preprocess, only_lex, only_parse, only_sem)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
    frontend->add_flag("--print-asm{stdout}", asm_outfile, "Print assembly")
        ->excludes(only_preprocess, only_lex, only_parse, only_sem, only_ir)
        ->group(print_opts_grp_name)
        ->expected(0, 1);
}

auto Args::setup_compilation_step_flags(CLI::App *frontend) -> void {
    using namespace cli;
    only_asm = frontend->add_flag("--asm", "Stop after assembly generation")
                   ->group(comp_step_grp_name);
    only_ir = frontend->add_flag("--ir", "Stop after IR generation")
                  ->group(comp_step_grp_name);
    only_sem = frontend->add_flag("--sem", "Stop after semantic analysis")
                   ->group(comp_step_grp_name);
    only_parse = frontend->add_flag("--parse", "Stop after parsing")
                     ->group(comp_step_grp_name);
    only_lex = frontend->add_flag("--lex", "Stop after lexical analysis")
                   ->group(comp_step_grp_name);
    only_preprocess =
        frontend->add_flag("--preprocess", "Stop after preprocessing")
            ->group(comp_step_grp_name);
}
