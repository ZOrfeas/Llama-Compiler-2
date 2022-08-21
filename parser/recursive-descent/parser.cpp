#include "parser.hpp"

Parser::Parser(Lexer& lexer) {
    tokens = lexer.get_tokens();
}
void Parser::parse() {
    it = tokens.begin();

    if(!parse_program())
    {
        std::cout << "parse error" << std::endl;
        exit(1);
    }

    std::cout << "parse success" << std::endl;
}
token Parser::peek_consume_token() {
    auto tok = peek_token();
    consume_token();
    return tok;
}
token Parser::peek_token() {
    return *(this->it);
}
void Parser::consume_token() {
    this->it++;
}
bool Parser::parse_program() {
    while (true)
    {
        if (!parse_type())
        {
            return false;
        }
    }
}
bool Parser::parse_type() {
    token curr = peek_consume_token();
    
    switch (curr.t) {
        case token_kind::UNIT:
        case token_kind::INT:
        case token_kind::FLOAT:
        case token_kind::CHAR:
        case token_kind::stringliteral:
        case token_kind::TRUE:
        case token_kind::FALSE:
        case token_kind::idlower:
        {
            // Stop the fallthrough of the single token matches and go to the helper
            if(!parse_type_helper()) {
                return false;
            }

            return true;
        }
        case token_kind::LPAREN:
        {
            if(!parse_type()) {
                return false;
            }

            token next = peek_consume_token();
            if (next.t != token_kind::RPAREN) {
                return false;
            }

            if(!parse_type_helper()) {
                return false;
            }

            return true;
        }
        case token_kind::ARRAY:
        {
            curr = peek_consume_token();
            switch (curr.t) {
                case token_kind::LBRACKET: 
                {
                    curr = peek_consume_token();
                    if (curr.t != token_kind::STAR)
                    {
                        return false;
                    }

                    // Check "," "*" pairs
                    while (true)
                    {
                        curr = peek_consume_token();
                        if (curr.t != token_kind::COMMA)
                        {
                            // No more COMMA STAR pairs
                            break;
                        }
                        curr = peek_consume_token();
                        if (curr.t != token_kind::STAR)
                        {
                            // COMMA without STAR following it
                            return false;
                        }
                    }

                    // Token has already been consumed
                    if (curr.t != token_kind::RBRACKET)
                    {
                        return false;
                    }

                    curr = peek_consume_token();
                    if (curr.t != token_kind::OF)
                    {
                        return false;
                    }
                }
                case token_kind::OF:
                {
                    // Fallthrough helps because OF is mandatory 
                    if(!parse_type()) {
                        return false;
                    }

                    return true;
                }
                default:
                    return false;

            }

            if(!parse_type_helper()) {
                return false;
            }

            return true;
        }
        default:
            return false;
    }
}
bool Parser::parse_type_helper() {
    //! Must not consume token because empty sequence is valid

    switch (peek_token().t) {
        case token_kind::DASHGREATER:
        {
            consume_token();
            
            if (!parse_type()) {
                return false;
            }

            if (!parse_type_helper()) {
                return false;
            }

            return true;
        }
        case token_kind::REF:
            consume_token();

            if (!parse_type_helper()) {
                return false;
            }

            return true;
        default:
            // This rule matches epsilon
            return true;
    }
}

//!NOTE: Maintain positions of matched rules (basically passed to ast)

int main() 
{
    std::string filename = "../test_parser.lla";
    std::ifstream file(filename);
    if(!file) {
        std::cout << "File not found" << std::endl;
        return 1;
    }
    
    // Weird way to read file in one string, should be fast
    file.seekg(0, std::ios::end);
    size_t size = file.tellg();
    std::string text(size, ' ');
    file.seekg(0);
    file.read(&text[0], size);

    Lexer lexer(text);
    lexer.lex();

    //std::cout << "Printing tokens" << std::endl;
    //lexer.print_tokens();
    //std::cout << "Finished printing tokens" << std::endl;

    Parser parser(lexer);
    parser.parse();
}