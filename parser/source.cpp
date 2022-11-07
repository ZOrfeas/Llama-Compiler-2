#include "source.hpp"
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
Source::Source(std::string_view filename)
    : text({}), filenames({}), included_filenames({}) {
    // cannot be in the initializer list cause preprocess uses the class fields
    text = preprocess(filename);
}

static auto match_with_str(Source::const_iterator it,
                           Source::const_iterator end, std::string_view str)
    -> bool {
    return std::distance(it, end) >= str.size() &&
           std::equal(str.begin(), str.end(), it);
};
static auto eat_until_non_whitespace(Source::const_iterator it,
                                     Source::const_iterator end)
    -> Source::const_iterator {
    while (it != end && std::isspace(*it)) {
        ++it;
    }
    return it;
};
static auto eat_until_whitespace(Source::const_iterator it,
                                 Source::const_iterator end)
    -> Source::const_iterator {
    while (it != end && !std::isspace(*it)) {
        ++it;
    }
    return it;
};
static auto eat_until_newline(Source::const_iterator it,
                              Source::const_iterator end)
    -> Source::const_iterator {
    while (it != end && *it != '\n') {
        ++it;
    }
    return it;
}
// static auto find_all_directives(Source::const_iterator it,
//                                 Source::const_iterator end)
//     -> std::vector<Source::const_iterator> {
//     std::vector<Source::const_iterator> occurrences;
//     while ((it = std::find(it, end, '#')) != end) {
//         if (*(it - 1) != '\n') {
//             throw parse_error{it_to_src_pos(it),
//                               "directives must be at the beginning of a
//                               line"};
//         }
//         occurrences.push_back(it);
//         ++it;
//     }
//     return occurrences;
// }
// auto Source::preprocess() -> void {
//     if (this->is_preprocessed) {
//         return;
//     }

//     std::string last_filename{std::move(this->filenames.back())};
//     included_filenames.insert(last_filename);
//     this->text = preprocess(std::move(this->text));
//     this->filenames.push_back(std::move(last_filename));

//     this->is_preprocessed = true;
// }

auto Source::preprocess(std::string_view filename) -> std::vector<char> {
    auto [it, success] = included_filenames.insert({std::string(filename), {}});
    if (!success) {
        throw parse_error{source_position{}, "include cycle detected"};
    }
    auto &[f_name, f_info] = *it; // this remains valid after insertions
    auto text = file_to_char_vec(filename); // this will be altered "in-place"

    filenames.push_back(f_name);
    return text;
}

auto Source::begin() const -> const_iterator { return this->text.begin(); }
auto Source::end() const -> const_iterator { return this->text.end(); }
auto Source::get_filename(const_iterator it) const -> std::string_view {
    // TODO: Implement
    return this->filenames.back();
}
auto Source::it_to_src_pos(const_iterator it) const -> source_position {
    source_position pos{1, 1, this->filenames[0]};
    auto src_it = this->text.begin();
    for (; src_it != this->text.end() && src_it != it; ++src_it) {
        if (*src_it == '\n') {
            ++pos.lineno;
            pos.colno = 1;
        } else {
            ++pos.colno;
        }
    }
    if (src_it == this->text.end() && it != this->text.end()) {
        throw parse_error{
            pos, "tried to get source position of out of bounds iterator",
            true};
    }
    pos.filename = this->get_filename(it);
    return pos;
}
