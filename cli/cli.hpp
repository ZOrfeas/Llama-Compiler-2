#ifndef LLA_CLI_HPP
#define LLA_CLI_HPP

#include <CLI/CLI.hpp>
#include <optional>
#include <string>

namespace lla::cli {
    class Args {
    private:
        CLI::App the_app;
        constexpr static auto COLUMN_WIDTH = 90;

        auto setup_frontend() -> void;
        auto setup_print_options_flags(CLI::App *) -> void;
        auto setup_compilation_step_flags(CLI::App *) -> void;

    public:
        CLI::Option *only_preprocess, *only_lex, *only_parse, *only_sem,
            *only_ir, *only_asm;
        std::string source_file;
        std::optional<std::string> preprocessed_outfile, tokens_outfile,
            ast_outfile, types_outfile, ir_outfile, asm_outfile;
        int result;

        constexpr static auto CALL_FOR_HELP = 99;
        constexpr static auto CALL_FOR_ALL_HELP = 98;
        Args(int argc, char **argv, std::string_view version = "0.0.0");
    };
} // namespace lla::cli

#endif // LLA_CLI_HPP