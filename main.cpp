#include "cli/cli.hpp"
#include "common.hpp"
#include "config.hpp"
#include "fmt/core.h"
#include "source.hpp"
#include "utils/utils.hpp"
// #include "parser/lexer.hpp"

#include <__concepts/invocable.h>
#include <cstdio>
#include <iostream>

#define STR_HELPER(x) #x
#define STR(x) STR_HELPER(x)

static constexpr auto version =
    STR(LLAMAC_VERSION_MAJOR) "." STR(LLAMAC_VERSION_MINOR) "." STR(
        LLAMAC_VERSION_PATCH) "(" STR(LLAMAC_BUILD_TYPE) ")";

auto unroll(auto gen) -> void {
    for (auto i : gen) {
    }
}

auto handle_args(const lla::cli::Args &args) -> void {
    using lla::utils::match;

    auto lines = lla::utils::adapt_gen_with_writer(
        lla::parse::all_files_lines(args.source_file),
        args.preprocessed_outfile,
        [](std::FILE *f, const lla::parse::ScanEvent &event) {
            event | match{[f](const lla::parse::Line &line) {
                              fmt::print(f, "{}\n", line.text);
                          },
                          [f](auto) {}};
        });
    if (*args.only_preprocess) {
        unroll(std::move(lines));
        return;
    }
    // src.print_text(*args.preprocessed_outfile); if (*args.only_preprocess)
    // return; auto lexer = lla::parse::Lexer(src); lexer.lex(); if
    // (args.tokens_outfile) lexer.pretty_print_tokens(*args.tokens_outfile); if
    // (*args.only_lex) return;
    // // TODO: parsing
    // // TODO: ast-printing
    // if (*args.only_parse) return;
    // // TODO: semantic analysis
    // // TODO: type-printing
    // if (*args.only_sem) return;
    // // TODO: ir generation (in memory)
    // // TODO: ir-printing
    // if (*args.only_ir) return;
    // // TODO: asm generation (this will only be necessary if print requested)
    // // TODO: asm-printing
    // if (*args.only_asm) return;
    // // TODO: produce and output executable
    // return;
}

auto main(int argc, char **argv) -> int {
    // using lla::utils::match;
    // // // auto line_gen =
    // // lla::parse::all_files_lines("./tests/test_includes.lla");
    // for (auto event :
    //      lla::parse::all_files_lines("./tests/test_includes.lla")) {
    //     event | match{
    //                 [](const lla::parse::FilenamePtr &filename) {
    //                     fmt::print("File: {}\n", *filename);
    //                 },
    //                 [](const lla::parse::Line &line) {
    //                     fmt::print("Line {}: {}\n", line.lineno, line.text);
    //                 },
    //             };
    // }
    auto args = lla::cli::Args(argc, argv, version);
    if (auto res = args.result) {
        return res;
    }
    handle_args(args);

    return 0;
}