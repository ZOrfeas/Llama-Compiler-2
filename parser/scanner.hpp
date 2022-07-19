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
        bool is_empty() const;
        bool has(std::string_view) const;
        void push(std::string_view);
        bool pop();
        std::string_view top() const;
    };
    Generator& gen;
protected:
    friend class Generator;
    IncludeStack include_stack;
public:
    Scanner(Generator &gen): gen(gen), include_stack(*this) {}
	virtual ~Scanner() {}
	virtual ast::Parser::symbol_type get_next_token();
};
}

#endif // SCANNER_HPP