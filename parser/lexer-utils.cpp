#include "lexer.hpp"
#include <fstream>
using namespace lla;

static auto file_to_char_vec(std::string_view filename) -> std::vector<char> {
    // std::ios::ate places the file pointer at the end of the file
    std::ifstream file(filename, std::ios::binary | std::ios::ate);

    // Weird way to read file in one string, should be fast
    auto size = file.tellg();
    std::vector<char> text(size);
    file.seekg(0);
    file.read(text.data(), size);
    return text;
}
///////////////////
Lexer::Source::Source(std::string_view filename)
    : filenames({filename}), text(preprocess(Source_file(filename))) {}
///////////////////
Lexer::Source::Source_file::Source_file(std::string_view filename)
    : filename(std::string(filename)), text(file_to_char_vec(filename)) {}
auto Lexer::Source::Source_file::cbegin() const
    -> std::vector<char>::const_iterator {
    return text.cbegin();
}
auto Lexer::Source::Source_file::cend() const
    -> std::vector<char>::const_iterator {
    return text.cend();
}
auto Lexer::Source::create_reader() const -> Reader { return {*this}; }
///////////////////
Lexer::Source::Reader::Reader(Source const &src)
    : src(src), cur_file_it(src.text.cbegin()), it(cur_file_it->cbegin()) {}
auto Lexer::Source::Reader::advance_it() -> void {
    if (++it == cur_file_it->cend()) {          //  end of current file
        if (++cur_file_it == src.text.cend()) { //  end of all files
            return;
        }
        it = cur_file_it->cbegin();
    }
}
auto Lexer::Source::Reader::rewind_it() -> void {
    if (it == cur_file_it->cbegin()) {          // beginning of current file
        if (cur_file_it == src.text.cbegin()) { // beginning of all files
            return;
        }
        --cur_file_it;
        it = (cur_file_it->cend());
    }
    --it;
}
auto Lexer::Source::Reader::get_cur_filename() const -> std::string_view {
    return cur_file_it->filename;
}
auto Lexer::Source::Reader::is_end() const -> bool {
    return cur_file_it == src.text.cend() && it == cur_file_it->cend();
}
auto Lexer::Source::Reader::is_start() const -> bool {
    return cur_file_it == src.text.cbegin() && it == cur_file_it->cbegin();
}
auto Lexer::Source::Reader::operator*() -> char { return *it; }
auto Lexer::Source::Reader::operator++() -> char {
    advance_it();
    return *it;
}
auto Lexer::Source::Reader::operator++(int) -> char {
    auto tmp = *it;
    advance_it();
    return tmp;
}
auto Lexer::Source::Reader::operator--() -> char {
    rewind_it();
    return *it;
}
auto Lexer::Source::Reader::operator--(int) -> char {
    auto tmp = *it;
    rewind_it();
    return tmp;
}
