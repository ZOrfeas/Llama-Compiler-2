#include "common.hpp"

class Lexer 
{
public:
    Lexer(std::string& text)
    : text(text) 
    {
        tokens = std::vector<token>();
    }
    void lex()
    {
        int cur_index = 0;
        position cur_pos = {0, 0};
        
        token cur_token;
        while(true) 
        {
            cur_token = next_token(cur_index, cur_pos);
            tokens.push_back(cur_token);
            if (cur_token.t == STOP)
                break;
        }
    }
    std::vector<token> get_tokens() 
    { 
        return tokens; 
    }
private:
    /* Finds the next token in the sequence and adjusts the cursor */
    token next_token(int& index, position& pos);
    std::string text;
    std::vector<token> tokens;
};

//?NOTE: No regex needed, I can easily match keywords with one function, and do custom stuff for the rest