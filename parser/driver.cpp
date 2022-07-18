#include <string>

#include "driver.hpp"

ParseDriver::ParseDriver(std::string_view source)
: include_stack(*this), parser(*this) {
    include_stack.push(source);
}
int ParseDriver::parse() {
    // ast::core::Program ast;
    int parse_res = parser.parse();
    if (parse_res) {
        const std::string err_msg =
            "Parser failed with error code " +
            std::to_string(parse_res);
        error::crash<error::PARSING>(err_msg);
    } 
    // else {
    //     std::cout << "Success\n";
    // }
    return parse_res;
}
void ParseDriver::error(std::string_view msg) const {
    using std::string_literals::operator""s;
    const std::string err_msg =
        "Error in file "s +
        std::string(include_stack.top()) +
        " at line "s + "(not implemented yet)" +
        // std::to_string(yylineno) +
        ": "s + std::string(msg);
    error::crash<error::PARSING>(err_msg);
}