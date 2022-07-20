#include <fstream>
#include <string>

#include "generator.hpp"
#include "../ast/ast.hpp"

using namespace ast;

Generator::Generator(): scanner(*this), parser(scanner, *this) {}
int Generator::parse(std::string_view source) {
    std::ifstream filestream(source);
    scanner.include_stack.push(source);
    return parser.parse();
}
void Generator::error(std::string_view msg) const {
    log::crash(
        "Error in file {} token {} at line {}: {}\n",
        scanner.include_stack.top(),
        scanner.YYText(),
        "Unimplemented", msg
    );
}