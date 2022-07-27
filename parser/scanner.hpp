#ifndef SCANNER_HPP
#define SCANNER_HPP

#if ! defined(yyFlexLexerOnce)
#undef yyFlexLexer
#define yyFlexLexer AST_FlexLexer // the trick with prefix; no namespace here :(
#include <FlexLexer.h>
#endif

#undef YY_DECL
#define YY_DECL ast::Parser::symbol_type ast::Scanner::get_next_token()

#include "parser.hpp" // this is needed for symbol_type
#include <unordered_set>
#include <fstream>
#include <memory>
#include <utility>
#include <string>

namespace ast {
class Generator;

class Scanner : public yyFlexLexer {
private:
    class IncludeStack {
    private:
        std::unordered_set<std::string> filename_set;
        std::vector<std::pair<std::string, std::unique_ptr<std::ifstream>>> filename_stack;
        Scanner& scanner;
    public:
        IncludeStack(Scanner& scanner);
        auto is_empty() const -> bool;
        auto has(std::string_view) const -> bool;
        auto push(std::string_view) -> void;
        auto pop() -> bool;
        auto top() const -> std::string_view;
    };
    Generator& gen;
    auto single_char_token_switch(char c) -> ast::Parser::symbol_type;
protected:
    friend class Generator;
    friend class Parser;
    IncludeStack include_stack;
    // auto extract_char(std::string_view str) const -> char;
    // auto extract_string(std::string_view str) const -> std::string;
public:
    Scanner(Generator &gen): gen(gen), include_stack(*this) {}
	virtual ~Scanner() {}
	virtual auto get_next_token() -> ast::Parser::symbol_type;
};
}

#endif // SCANNER_HPP