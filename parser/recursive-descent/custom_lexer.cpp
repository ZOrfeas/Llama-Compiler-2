#include "custom_lexer.hpp"



Lexer::Lexer(std::string& text)
:   text(text) 
{
    tokens = std::vector<token>();
}
std::vector<token> Lexer::get_tokens() 
{ 
    return tokens; 
}
void Lexer::lex()
    {
        this->it = this->text.begin();
        this->pos = {0, 0};

        token cur_token;
        while(true) 
        {
            cur_token = next_token();
            tokens.push_back(cur_token);
            if (cur_token.t == token_kind::STOP)
            {
                break;
            }
        }
    }
token Lexer::next_token()
{
    
    // Consume all white space (\n, \t, \r, space)
    match_whitespace(); // no token just skip ahead
    
    // Token could be a ...
    token ans = {token_kind::UNMATCHED, "", this->pos, this->pos};

    // EOF
    if (match_end())
    {
        ans.t = token_kind::STOP;
        ans.value = this->cur_s;
        ans.end = pos;
        return ans;
    }

    // Whole comment
    if (match_single_line_comment() || match_multi_line_comment())
    {
        ans.t = token_kind::COMMENT;
        ans.value = this->cur_s;
        ans.end = pos;
        return ans;
    }
        
    // Keyword
    std::vector<reserved> keywords = {
    {"and",     token_kind::AND},
    {"array",   token_kind::ARRAY},
    {"begin",   token_kind::BEGIN},
    {"bool",    token_kind::BOOL},
    {"char",    token_kind::CHAR},
    {"delete",  token_kind::DELETE},
    {"dim",     token_kind::DIM},
    {"do",      token_kind::DO},
    {"done",    token_kind::DONE},
    {"downto",  token_kind::DOWNTO},
    {"else",    token_kind::ELSE},
    {"end",     token_kind::END},
    {"false",   token_kind::FALSE},
    {"float",   token_kind::FLOAT},
    {"for",     token_kind::FOR},
    {"if",      token_kind::IF},
    {"in",      token_kind::IN},
    {"int",     token_kind::INT},
    {"let",     token_kind::LET},
    {"match",   token_kind::MATCH},
    {"mod",     token_kind::MOD},
    {"mutable", token_kind::MUTABLE},
    {"new",     token_kind::NEW},
    {"not",     token_kind::NOT},
    {"of",      token_kind::OF},
    {"rec",     token_kind::REC},
    {"ref",     token_kind::REF},
    {"then",    token_kind::THEN},
    {"to",      token_kind::TO},
    {"true",    token_kind::TRUE},
    {"type",    token_kind::TYPE},
    {"unit",    token_kind::UNIT},
    {"while",   token_kind::WHILE},
    {"with",    token_kind::WITH}
};
    for (auto& k : keywords)
    {
        if (match_prefix_word_with(k.name)) 
        {
            ans.t = k.t;
            ans.value = k.name;
            ans.end = this->pos;
            return ans;
        }
    }

    // Identifiers
    if (match_id()) 
    {
        ans.t = token_kind::idlower;
        ans.value = cur_s;
        ans.end = this->pos;
        return ans;
    }
    if (match_Id())
    {
        ans.t = token_kind::idupper;
        ans.value = cur_s;
        ans.end = this->pos;
        return ans;
    } 
    
    // Literal float
    if (match_literal_float())
    {
        ans.t = token_kind::floatconst;
        ans.value = this->cur_s;
        ans.end = this->pos;
        return ans;
    }

    // Literal int
    if (match_literal_int())
    {
        ans.t = token_kind::intconst;
        ans.value = this->cur_s;
        ans.end = this->pos;
        return ans;
    }

    // Literal char (+ escape sequences)
    if (match_literal_char())
    {
        ans.t = token_kind::charconst;
        ans.value = this->cur_s;
        ans.end = this->pos;
        return ans;
    }

    // Literal string (can't span more than one string in the code + stray \)
    if (match_literal_string())
    {
        ans.t = token_kind::stringliteral;
        ans.value = this->cur_s;
        ans.end = this->pos;
        return ans;
    } 
    
    // Symbolic operators (multiple chars)
    std::vector<reserved> symops = {
    { "->", token_kind::DASHGREATER },
    { "+.", token_kind::PLUSDOT },
    { "-.", token_kind::MINUSDOT },
    { "*.", token_kind::STARDOT },
    { "/.", token_kind::SLASHDOT },
    { "**", token_kind::DBLSTAR },
    { "&&", token_kind::DBLAMPERSAND },
    { "||", token_kind::DBLBAR },
    { "<>", token_kind::LTGT },
    { "<=", token_kind::LEQ },
    { ">=", token_kind::GEQ },
    { "==", token_kind::DBLEQ },
    { "!=", token_kind::EXCLAMEQ },
    { ":=", token_kind::COLONEQ }
    };
    for (auto& k : symops)
    {
        if (match_prefix_word_with(k.name)) 
        {
            ans.t = k.t;
            ans.value = k.name;
            ans.end = this->pos;
            return ans;
        }
    }

    // Separators and single char operators
    std::vector<reserved> single_char = {
    { "=", token_kind::EQ },
    { "|", token_kind::BAR },
    { "+", token_kind::PLUS },
    { "-", token_kind::MINUS },
    { "*", token_kind::STAR },
    { "/", token_kind::SLASH },
    { "!", token_kind::EXCLAM},
    { ";", token_kind::SEMICOLON },
    { "<", token_kind::LT },
    { ">", token_kind::GT },
    { "(", token_kind::LPAREN },
    { ")", token_kind::RPAREN },
    { "[", token_kind::LBRACKET },
    { "]", token_kind::RBRACKET },
    { ",", token_kind::COMMA },
    { ":", token_kind::COLON }
    };
    for (auto& k : single_char)
    {
        if (match_prefix_word_with(k.name)) 
        {
            ans.t = k.t;
            ans.value = k.name;
            ans.end = this->pos;
            return ans;
        }
    }

    // Can't match token
    match_unmatched();
    ans.t = token_kind::UNMATCHED;
    ans.value = this->cur_s;
    ans.end = this->pos;
    return ans;
}
bool Lexer::match_prefix_word_with(std::string s)
{
    std::string::iterator it_temp = this->it;
    std::string::iterator s_temp = s.begin();

    // Find s in prefix
    while(it_temp != this->text.end() && s_temp != s.end())
    {
        if (*it_temp != *s_temp)
        {
            return false;
        }
        it_temp++;
        s_temp++;
    }

    // Check word boundary to make sure the word ends
    if (it_temp != this->text.end() && (std::isalnum(*it_temp) || *it_temp == '_'))
    {
        return false;
    }
 
    this->pos.column += s.size();
    this->it = it_temp;
    return true;
}
void Lexer::match_whitespace()
{
    std::string::iterator it_temp = this->it;
    
    while (it_temp != this->text.end() && std::isspace(*it_temp))
    {
        if (*it_temp == '\n' || *it_temp == '\r') 
        {
            this->pos.line++;
            this->pos.column = 0;
        }
        else if (*it_temp == '\t')
        {
            //!NOTE: how many spaces is a tab in current editor?
            this->pos.column += TAB_SIZE - (this->pos.column % TAB_SIZE);
        }
        else if (*it_temp == ' ')
        {
            this->pos.column++;
        }

        it_temp++;
    }
    
    this->it = it_temp;
    return;
}
bool Lexer::match_id()
{
    // Make a cope of the iterator
    std::string::iterator it_temp = this->it;

    // Check first character, must be lower
    if (!std::islower(*it_temp))
    {
        return false;
    }

    // Find how many characters it takes up
    while(++it_temp != text.end())
    {
        if (!std::isalnum(*it_temp) && *it_temp != '_')
        {
            break;
        }
    }

    // The id lies between the two iterators
    this->cur_s = std::string(this->it, it_temp);
    this->pos.column += std::distance(this->it, it_temp);
    this->it = it_temp;
    return true;
}
bool Lexer::match_Id()
{
    // Copy text iterator
    std::string::iterator it_temp = this->it;

    // Check first character, must be upper
    if (!std::isupper(*it_temp))
    {
        return false;
    }

    // Find how many characters it takes up
    while(++it_temp != text.end())
    {
        if (!std::isalnum(*it_temp) && *it_temp != '_')
        {
            break;
        }
    }

    // The id lies between the two iterators
    this->cur_s = std::string(this->it, it_temp);
    this->pos.column += std::distance(this->it, it_temp);
    this->it = it_temp;
    return true;
}
bool Lexer::match_single_line_comment() 
{
    // Copy text iterator
    std::string::iterator it_temp = this->it;

    // Single line comment --
    if (*it_temp != '-' || *(it_temp + 1) != '-')
    {
        return false;
    }

    // Skip the --
    it_temp += 2;

    // Skip until \n or \r
    while (it_temp != this->text.end() && *it_temp != '\n' && *it_temp != '\r')
    {
        it_temp++;
    }

    this->pos.column += std::distance(this->it, it_temp);
    this->cur_s = std::string(it, it_temp);
    this->it = it_temp;
    return true;
}
bool Lexer::match_multi_line_comment()
{
    std::string::iterator it_temp = this->it;
    int balance = 0;

    // Match (* to start comment
    if (*it_temp != '(' || *(it_temp + 1) != '*')
    {
        return false;
    }

    // Keep count of balanced parentheses and iterator
    balance++;
    it_temp += 2;

    while (it_temp != this->text.end() || balance != 0)
    {
        if (*it_temp == '(' && *(it_temp + 1) == '*')
        {
            // Found (*
            balance++;
            it_temp += 2;
            this->pos.column += 2;
        }
        else if (*it_temp == '*' && *(it_temp + 1) == ')') 
        {
            // Found *)
            balance--;
            it_temp += 2;
            this->pos.column += 2;
        }
        else if (*it_temp == '\n' || *it_temp == '\r') 
        {
            // Beware end of line
            it_temp++;
            this->pos.line++;
            this->pos.column = 0;
        }
        else 
        {
            // Skip comment character
            it_temp++;
            this->pos.column++;

        }
    }

    //! If EOF was reached before closing comment generate error
    if (balance != 0)
    {

    }

    this->cur_s = std::string(it, it_temp);
    this->it = it_temp;
    return true;
}
bool Lexer::match_end()
{
    if (this->it == this->text.end())
    {
        this->cur_s = "$";
        return true;
    }

    return false;
}
void Lexer::match_unmatched()
{
    //! This is where I can make it error tolerant by matching whatever until next whitespace for instance
    this->cur_s = std::string(this->it, this->text.end());
    this->it = this->text.end();
}
bool Lexer::match_literal_float()
{
    std::string::iterator it_temp = this->it;

    // Consume at least one digit for the integer part
    if (!std::isdigit(*it_temp)) 
    {
        return false;
    }
    it_temp++;

    // Consume all remaining consecutive digits
    while (std::isdigit(*it_temp)) 
    {
        it_temp++;
    }

    // Check . after integer part
    if (it_temp == this->text.end() || *it_temp != '.') 
    {
        return false;
    }
    it_temp++;

    // Consume at least one digit for the decimal part
    if (!std::isdigit(*it_temp)) 
    {
        //! This is a good place for an error since we know it is a float written without decimal part
        return false;
    }
    it_temp++;
    
    // Consume all remaining consecutive digits
    while (std::isdigit(*it_temp))
    {
        it_temp++;
    } 

    // Check e after integer part
    if (it_temp == this->text.end() || *it_temp != 'e') 
    {
        // So far we have a valid float without exponential part
        this->pos.column += std::distance(this->it, it_temp);
        this->cur_s = std::string(this->it, it_temp);
        this->it = it_temp;
        return true;
    }
    it_temp++;

    // Consume +/- if found
    if (*it_temp == '+' || *it_temp == '-')
    {
        it_temp++;
    }
    
    // Consume at least one digit for exponential part
    if (! std::isdigit(*it_temp)) 
    {
        return false;
    }
    it_temp++;

    // Consume all remaining digits in exponential part
    while (std::isdigit(*it_temp))
    {
        it_temp++;
    }

    // So far we have a valid float with exponential part
    this->pos.column += std::distance(this->it, it_temp);
    this->cur_s = std::string(this->it, it_temp);
    this->it = it_temp;
    return true;
}
bool Lexer::match_literal_int()
{
    std::string::iterator it_temp = this->it;

    // Consume at least one digit of the integer
    if (!std::isdigit(*it_temp)) 
    {
        return false;
    }
    it_temp++;

    // Consume all remaining consecutive digits
    while (std::isdigit(*it_temp)) 
    {
        it_temp++;
    }

    // Check . after integer part
    if (*it_temp == '.') 
    {
        // Dot can't be part of an integer 
        return false;
    }
    
    // Matched the literal integer
    this->pos.column += std::distance(it, it_temp);
    this->cur_s = std::string(it, it_temp);
    this->it = it_temp;
    return true;
}
bool Lexer::match_literal_char()
{
    std::string::iterator it_temp = this->it;

    // Check starting single quote
    if (*it_temp != '\'')
    {
        return false;
    }
    it_temp++;

    // Single character case
    if (*it_temp == '\'' ||
        *it_temp == '\"' ||
        *it_temp == '\n' ||
        *it_temp == '\r' ||
        *it_temp == '\0')
    {
        //! Error illegar literal char
        return false;
    }

    // Escape sequence
    if (*it_temp == '\\')
    {
        it_temp++;
        if (*it_temp != 'n' &&
            *it_temp != 't' && 
            *it_temp != 'r' && 
            *it_temp != '0' &&
            *it_temp != '\\' &&
            *it_temp != '\'' &&
            *it_temp != '\"' &&
            *it_temp != 'x')
        {
            return false;
        }

        // Hex
        if (*it_temp == 'x')
        {
            it_temp++;
            
            if(!std::isdigit(*it_temp) && !(*it_temp >= 'a' && *it_temp <= 'f'))
            {
                //! Error invalid hex
                return false;
            }
            it_temp++;
            
            if(!std::isdigit(*it_temp) && !(*it_temp >= 'a' && *it_temp <= 'f'))
            {
                //! Error invalid hex
                return false;
            }
            it_temp++;
        }

        // Other escape sequence 
        else 
        {
            it_temp++;
        }
    }

    // Check ending single quote
    if (*it_temp != '\'')
    {
        //! Error expected single quote
        return false;
    }
    it_temp++;

    this->pos.column += std::distance(this->it, it_temp);
    this->cur_s = std::string(this->it, it_temp);
    this->it = it_temp;
    return true;
}
bool Lexer::match_literal_string()
{
    std::string::iterator it_temp = it;

    // Check doublequotes
    if (*it_temp != '\"')
    {
        return false;
    }
    it_temp++;

    // Consume the rest of the string (at most the rest of the line)
    while (it_temp != this->text.end() && *it_temp != '\"')
    {
        // String only spans one string
        if (*it_temp == '\n')
        {
            //! Error string spans more than one line
            return false;
        }
        
        // Escape sequence
        if (*it_temp == '\\')
        {
            it_temp++;

            // Newline is not covered by escape sequence
            if (*it_temp == '\n')
            {
                //! Error string spans more than one line
                return false;
            }
        }

        // Consume the next character
        it_temp++;
    }

    it_temp++;

    this->pos.column += std::distance(this->it, it_temp);
    this->cur_s = std::string(this->it, it_temp);
    this->it = it_temp;
    return true;
}
void Lexer::print_tokens()
{
    for (auto& t: this->tokens)
    {
        std::cout   << token_kind_string[t.t] 
                    << "(" << t.value << ")"
                    << " (" << t.start.line << ", " << t.start.column << "),"
                    << " (" << t.end.line << ", " << t.end.column << ")" 
                    << std::endl;
    }
}

//?NOTE: No regex needed, I can easily match keywords with one function, and do custom stuff for the rest

int main() 
{
    std::string filename = "test.lla";
    std::ifstream file(filename);
    
    // Weird way to read file in one string, should be fast
    file.seekg(0, std::ios::end);
    size_t size = file.tellg();
    std::string text(size, ' ');
    file.seekg(0);
    file.read(&text[0], size);

    Lexer lexer(text);
    lexer.lex();
    lexer.print_tokens();
}