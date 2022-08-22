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
        if (peek_token().t == token_kind::STOP) {
            break;
        }

        if (!parse_typedef())
        {
            return false;
        }
    }

    return true;
}
bool Parser::parse_letdef() {
    // let
    if (peek_token().t != token_kind::LET) {
        return false;
    }
    consume_token();
    
    // [rec]
    if (peek_token().t == token_kind::REC) {
        consume_token();
    }
    
    // def
    if (!parse_def) {
        return false;
    }

    // (and def)*
    while (true) {
        if (peek_token().t == token_kind::AND) {
            break;
        }
        consume_token();

        if (!parse_def()) {
            return false;
        }
    }
    return true;
}
bool Parser::parse_typedef() {
    // type
    if (peek_token().t != token_kind::TYPE) {
        return false;
    }
    consume_token();

    // tdef
    if (!parse_tdef()) {
        return false;
    }
    
    // (and tdef)*
    while (true) {
        if (peek_token().t != token_kind::AND) {
            break;
        }

        consume_token();
        if (!parse_tdef()) {
            return false;
        }
    }

    return true;
}
bool Parser::parse_tdef() {
    // id
    if (peek_token().t != token_kind::idlower) {
        return false;
    }
    consume_token();

    // =
    if (peek_token().t != token_kind::EQ) {
        return false;
    }
    consume_token();

    // constr
    if (!parse_constr()) {
        return false;
    }

    // (| constr)*
    while (true) {
        if (peek_token().t != token_kind::BAR) {
            break;
        }
        consume_token();
        
        if (!parse_constr()) {
            return false;
        }
    }
    return true;
}
bool Parser::parse_constr() {
    // id
    if (peek_token().t != token_kind::idupper) {
        return false;
    }
    consume_token();

    // of (optional)
    if (peek_token().t != token_kind::OF) {
        return true;
    }
    consume_token();
    
    // type
    if (!parse_type()) {
        return false;
    }

    // type*
    while(parse_type()) {}

    return true;
}
bool Parser::parse_type() {
    switch (peek_token().t) {
        case token_kind::UNIT:
        case token_kind::INT:
        case token_kind::FLOAT:
        case token_kind::CHAR:
        case token_kind::stringliteral:
        case token_kind::TRUE:
        case token_kind::FALSE:
        case token_kind::idlower:
        {
            consume_token();
            
            // Stop the fallthrough of the single token matches and go to the helper
            if(!parse_type_helper()) {
                return false;
            }
            
            return true;
        }
        case token_kind::LPAREN:
        {
            consume_token();

            // type
            if(!parse_type()) {
                return false;
            }

            // )
            if (peek_token().t != token_kind::RPAREN) {
                return false;
            }
            consume_token();

            if(!parse_type_helper()) {
                return false;
            }

            return true;
        }
        case token_kind::ARRAY:
        {
            consume_token();

            switch (peek_token().t) {
                case token_kind::LBRACKET: 
                {
                    consume_token();
                    
                    // *
                    if (peek_token().t != token_kind::STAR) {
                        return false;
                    }
                    consume_token();

                    // Check "," "*" pairs
                    while (true)
                    {
                        if (peek_token().t != token_kind::COMMA) {
                            // No more COMMA STAR pairs
                            break;
                        }
                        consume_token();
                        
                        if (peek_token().t != token_kind::STAR) {
                            // COMMA without STAR following it
                            return false;
                        }
                        consume_token();
                    }

                    // ]
                    if (peek_token().t != token_kind::RBRACKET) {
                        return false;
                    }
                    consume_token();

                    // of
                    if (peek_token().t != token_kind::OF) {
                        return false;
                    }

                    //? Consumption will happen after fallthrough
                }
                case token_kind::OF:
                {
                    consume_token();

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
    // Exists only to eliminate left recursion
    
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

//? Each rule must peek for first token to make sure we are inside the rule
//!NOTE: Maintain positions of matched rules (basically passed to ast)

int main() 
{
    std::string filename = "../tests/typedef.lla";
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