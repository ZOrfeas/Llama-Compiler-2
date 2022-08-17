#include <regex>

#include "common.hpp"

auto split_words_from_string(std::string s) -> std::vector<std::string> {
    std::vector<std::string> words = {};
    std::regex wbound(R"(\b\w+\b)");
    std::regex nwbound("[[:punct:]]+");
    std::sregex_iterator words_begin(s.begin(), s.end(), nwbound);
    std::sregex_iterator words_end;

    // Save the locations of boundaries between words
    for (std::sregex_iterator i = words_begin; i != words_end; ++i) {
        std::smatch match = *i;
        std::string match_str = match.str();
        std::cout << match_str << std::endl;
    }

    return words;
}

/* Consumes spaces, newlines */
auto eat_white_space(std::vector<std::string> &text, position &start) -> void {
    bool flag = false;
    for (auto line = start.line; line < text.size(); line++) {
        for (auto column = start.column; column < text[line].size(); column++) {
            if (std::isspace(text[start.line][start.column])) {
                flag = true;
                break;
            }
        }
        if (flag) {
            start.line = line;
            break;
        }
    }
}

auto parse_type_helper(std::vector<std::string> &text, position &start,
                       position &end) -> bool;

// start must point to the first character of non terminal
// end must be modified to point to the end of non terminal
auto parse_type(std::vector<std::string> &text, position &start, position &end)
    -> bool {
    token current_token = get_next_token(text, start);
    end = current_token.end;

    switch (current_token.t) {
    case token_kind::UNIT:          // fallthrough
    case token_kind::INT:           // fallthrough
    case token_kind::FLOAT:         // fallthrough
    case token_kind::CHAR:          // fallthrough
    case token_kind::stringliteral: // fallthrough
    case token_kind::TRUE:          // fallthrough
    case token_kind::FALSE:         // fallthrough
    case token_kind::ARRAY:         // fallthrough
    case token_kind::idlower:
        return parse_type_helper(text, current_token.end, end);
    case token_kind::LPAREN:
        return true;
    case token_kind::RPAREN:
        return false;
    default:
        return false;
    }
}

auto parse_type_helper(std::vector<std::string> &text, position &start,
                       position &end) -> bool {
    token current_token = get_next_token(text, start);
    end = current_token.end;

    switch (current_token.t) {
    case token_kind::REF:
        return true;
    case token_kind::DASHGREATER:
        return parse_type(text, current_token.end, end);
    default:
        return false;
    }
}

auto main(int argc, char **argv) -> int {
    std::string filename = "test.lla";
    std::ifstream file;
    file.open(filename);
    std::string line;
    std::vector<std::string> text = {};

    while (std::getline(file, line)) {
        text.push_back(line);
        auto words = split_words_from_string(line);
        for (auto &s : words) {
            std::cout << s << std::endl;
        }
    }
    file.close();

    position start = {0, 0};
    position end = {0, 0};
}