#ifndef GENERATOR_HPP
#define GENERATOR_HPP


#include <unordered_set>
#include <string_view>

#include "scanner.hpp"

#include "parser.hpp"

// class IncludeStack {
// private:
//     std::unordered_set<std::string> filename_set;
//     std::vector<std::string> filename_stack;
//     ParseDriver& owning_driver;
// public:
//     IncludeStack(ParseDriver& drv);
//     bool is_empty() const;
//     bool has(std::string_view) const;
//     void push(std::string_view);
//     bool pop();
//     std::string_view top() const;
// };

namespace ast {
class Program;

class Generator {
private:
    Scanner scanner;
    Parser parser;
    // location
    // include_stack
public:
    Generator();
    int parse(std::string_view);
    void error(std::string_view) const;
    void clear();
    friend class Scanner;
    friend class Parser;
};

}

// class ParseDriver {
// public:
//     // IncludeStack include_stack;
//     ast::Parser parser;
//     ParseDriver(std::string_view source);
//     int parse();
//     void error(std::string_view msg) const;
// };

#endif // GENERATOR_HPP