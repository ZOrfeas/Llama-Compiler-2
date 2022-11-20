#include "cli/cli.hpp"
#include "config.hpp"
#include "parser/lexer.hpp"

#include <iostream>

#define STR_HELPER(x) #x
#define STR(x) STR_HELPER(x)

static constexpr auto version =
    STR(LLAMAC_VERSION_MAJOR) "." STR(LLAMAC_VERSION_MINOR) "." STR(
        LLAMAC_VERSION_PATCH) "(" STR(LLAMAC_BUILD_TYPE) ")";

auto handle_args(const lla::cli::Args &args) -> void {
    auto src = lla::parse::Source(args.source_file);
    if (args.preprocessed_outfile) src.print_text(*args.preprocessed_outfile);
    if (*args.only_preprocess) return;
    auto lexer = lla::parse::Lexer(src);
    lexer.lex();
    if (args.tokens_outfile) lexer.pretty_print_tokens(*args.tokens_outfile);
    if (*args.only_lex) return;
    // TODO: parsing
    // TODO: ast-printing
    if (*args.only_parse) return;
    // TODO: semantic analysis
    // TODO: type-printing
    if (*args.only_sem) return;
    // TODO: ir generation (in memory)
    // TODO: ir-printing
    if (*args.only_ir) return;
    // TODO: asm generation (this will only be necessary if print requested)
    // TODO: asm-printing
    if (*args.only_asm) return;
    // TODO: produce and output executable
    return;
}

auto main(int argc, char **argv) -> int {
    auto args = lla::cli::Args(argc, argv, version);
    if (args.result) {
        return args.result;
    }
    handle_args(args);

    return 0;
}