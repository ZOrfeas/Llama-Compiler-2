#ifndef PARSER_DRIVER_HPP
#define PARSER_DRIVER_HPP


#include <unordered_set>
#include <string_view>

#include "./parser.hpp"

#define YY_DECL \
  yy::parser::symbol_type yylex (ParseDriver& drv)
YY_DECL;

class IncludeStack {
private:
    std::unordered_set<std::string> filename_set;
    std::vector<std::string> filename_stack;
    ParseDriver& owning_driver;
public:
    IncludeStack(ParseDriver& drv);
    bool is_empty() const;
    bool has(std::string_view) const;
    void push(std::string_view);
    bool pop();
    std::string_view top() const;
};


class ParseDriver {
public:
    IncludeStack include_stack;
    yy::parser parser;
    ParseDriver(std::string_view source);
    int parse();
    void error(std::string_view msg) const;
};

#endif // PARSER_DRIVER_HPP