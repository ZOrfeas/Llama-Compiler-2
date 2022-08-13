#include <common.hpp>

std::vector<std::string> split_words_from_string(std::string s) {
    std::vector<std::string> words = {};
    std::regex wbound("\\b\\w+\\b");
    std::regex nwbound("[[:punct:]]+");
    std::sregex_iterator words_begin(s.begin(), s.end(), nwbound);
    std::sregex_iterator words_end;
    
    // Save the locations of boundaries between words
    for(std::sregex_iterator i = words_begin; i != words_end; ++i) 
    {
        std::smatch match = *i;
        std::string match_str = match.str();
        std::cout << match_str << std::endl;
    }

    return words;
}

/* Fetches a token and sets the end of it (a lexer, to be honest) */
token get_next_token(std::vector<std::string>& text, position& start) 
{
    // Token coulde be a ...
    
    // Whole comment
    
    // Keyword
    
    // Identifier (id/Id)
    /* std::regex_search(token, std::regex("\\b[[:lower:]][[:alnum:]_]*\\b")); */
    /* std::regex_search(token, std::regex("\\b[[:upper:]][[:alnum:]_]*\\b")); */
    
    // Literal int, float, character, string (+escape sequences)
    
    // Symbolic operators
    
    // Separators
    
    // White space (\n, \t, \r, \f, \v, space)
}

/* Consumes spaces, newlines */
void eat_white_space(std::vector<std::string>& text, position& start)
{
    bool flag = false;
    for(int line = start.line; line < text.size(); line++) {
        for(int column = start.column; column < text[line].size(); column++) {
            if(std::isspace(text[start.line][start.column])){
                flag = true;
                break;
            }
        }
        if(flag) {
            start.line = line;
            break;
        }
    }

}

bool parse_type_helper(std::vector<std::string>& text, position& start, position& end);

// start must point to the first character of non terminal
// end must be modified to point to the end of non terminal
bool parse_type(std::vector<std::string>& text, position& start, position& end) 
{
    token current_token = get_next_token(text, start);
    end = current_token.end;
    
    switch (current_token.t) {
        case token_kind::UNIT:
            if(parse_type_helper(text, current_token.end, end)) {
                return true;
            }
        case token_kind::INT:
            if(parse_type_helper(text, current_token.end, end)) {
                return true;
            }
        case token_kind::FLOAT:
            if(parse_type_helper(text, current_token.end, end)) {
                return true;
            }
        case token_kind::CHAR:
            if(parse_type_helper(text, current_token.end, end)) {
                return true;
            }
        case token_kind::stringliteral:
            if(parse_type_helper(text, current_token.end, end)) {
                return true;
            }
        case token_kind::TRUE:
            if(parse_type_helper(text, current_token.end, end)) {
                return true;
            }  
        case token_kind::FALSE:
            if(parse_type_helper(text, current_token.end, end)) {
                return true;
            }
        case token_kind::LPAREN:
            return true;
        case token_kind::RPAREN:
            return false;
        case token_kind::ARRAY:
            if(parse_type_helper(text, current_token.end, end)) {
                return true;
            }
        case token_kind::idlower:
            if(parse_type_helper(text, current_token.end, end)) {
                return true;
            }
        default:
            return false;
    }
}  

bool parse_type_helper(std::vector<std::string>& text, position& start, position& end)
{
    token current_token = get_next_token(text, start);
    end = current_token.end;

    switch (current_token.t) {
        case token_kind::REF:
            return true;
        case token_kind::DASHGREATER:
            if(parse_type(text, current_token.end, end)) {
                return true;
            }
        default: 
            return false;
    }
}

int main(int argc, char** argv) 
{
    std::string filename = "test.lla";
    std::ifstream file;
    file.open(filename);
    std::string line;
    std::vector<std::string> text = {};

    while (std::getline(file, line)) {
        text.push_back(line);
        auto words = split_words_from_string(line);
        for(auto& s: words)
        {
            std::cout << s << std::endl;
        }
    }
    file.close();

    position start = {0, 0};
    position end = {0, 0};
}