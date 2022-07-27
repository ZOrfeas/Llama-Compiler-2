#ifndef GENERATOR_HPP
#define GENERATOR_HPP


#include <unordered_set>
#include <string_view>

#include "scanner.hpp"
#include "parser.hpp"

namespace ast {
struct Program;

class Generator {
private:
    Scanner scanner;
    Parser parser;
    // location
    // include_stack
    std::unique_ptr<ast::Program> ast;
public:
    Generator();
    auto parse(std::string_view) -> int;
    auto error(std::string_view) const -> void;
    auto set_ast(std::unique_ptr<Program> ast) -> void;
    auto extract_ast() && -> std::unique_ptr<Program>;
    // friend class Scanner;
    // friend class Parser;
};
} // namespace ast

#endif // GENERATOR_HPP