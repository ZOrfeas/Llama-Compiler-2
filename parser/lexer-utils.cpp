#include "lexer.hpp"
#include <fstream>
#include <iostream>
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
auto Lexer::Source::preprocess(Source_file src) -> std::vector<Source_file> {
    // TODO: Implement #include statement lookup, recursively apply while
    // checking for cycles with the filenames set. Should not be too hard.
    // temp
    return {src};
}
auto Lexer::Source::create_reader() const -> const_iterator { return {*this}; }
auto Lexer::Source::begin() const -> const_iterator { return create_reader(); }
auto Lexer::Source::end() const -> const_iterator {
    auto reader = create_reader();
    reader.cur_file_it = text.cend();
    reader.it = (reader.cur_file_it - 1)->cend();
    return reader;
}
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
///////////////////
Lexer::Source::const_iterator::const_iterator(Source const &src)
    : src(src), cur_file_it(src.text.cbegin()), it(cur_file_it->cbegin()) {}
auto Lexer::Source::const_iterator::advance_it() -> void {
    if (++it == cur_file_it->cend()) {          //  end of current file
        if (++cur_file_it == src.text.cend()) { //  end of all files
            return;
        }
        it = cur_file_it->cbegin();
    }
}
auto Lexer::Source::const_iterator::rewind_it() -> void {
    if (it == cur_file_it->cbegin()) {          // beginning of current file
        if (cur_file_it == src.text.cbegin()) { // beginning of all files
            return;
        }
        --cur_file_it;
        it = (cur_file_it->cend());
    }
    --it;
}
auto Lexer::Source::const_iterator::get_cur_filename() const
    -> std::string_view {
    return cur_file_it->filename;
}
auto Lexer::Source::const_iterator::is_end() const -> bool {
    return cur_file_it == src.text.cend() && it == ((cur_file_it - 1)->cend());
}
auto Lexer::Source::const_iterator::is_start() const -> bool {
    return cur_file_it == src.text.cbegin() && it == cur_file_it->cbegin();
}
auto Lexer::Source::const_iterator::operator*() -> char { return *it; }
auto Lexer::Source::const_iterator::operator++() -> const_iterator & {
    advance_it();
    return *this;
}
auto Lexer::Source::const_iterator::operator++(int) -> const_iterator {
    auto tmp = const_iterator(*this);
    advance_it();
    return tmp;
}
auto Lexer::Source::const_iterator::operator--() -> const_iterator & {
    rewind_it();
    return *this;
}
auto Lexer::Source::const_iterator::operator--(int) -> const_iterator {
    auto tmp = const_iterator(*this);
    rewind_it();
    return tmp;
}