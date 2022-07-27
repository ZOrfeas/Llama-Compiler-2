#include <fstream>
#include <iostream>
#include <string>

#include "generator.hpp"
#include "../ast/ast.hpp"

using namespace ast;

Generator::Generator(): scanner(*this), parser(scanner, *this) {}
auto Generator::parse(std::string_view source) -> int {
    std::ifstream filestream(source);
    scanner.include_stack.push(source);
    return parser.parse();
}
auto Generator::error(std::string_view msg) const -> void {
    log::crash(
        "Error in file {} token {} at line {}: {}\n",
        scanner.include_stack.top(),
        scanner.YYText(),
        "Unimplemented", msg
    );
}
auto Generator::set_ast(std::unique_ptr<ast::Program> ast) -> void {
    this->ast = std::move(ast);
}
auto Generator::extract_ast() && -> std::unique_ptr<ast::Program> {
    return std::move(ast);
}