#include "source.hpp"
#include "common.hpp"
#include "fmt/core.h"
#include <__iterator/concepts.h>
#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iterator>
#include <optional>
#include <type_traits>
#include <utility>

using namespace lla::parse;

template <typename Iter>
concept char_forward_iterator =
    std::forward_iterator<Iter> &&
    std::same_as<typename std::iterator_traits<Iter>::value_type, char>;

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
    : f_infos({{std::string(filename)}}), f_name_map({}), text({}) {
    auto &f_info = f_infos.back();
    auto [it, success] = f_name_map.insert({f_info.name, f_info});
    if (!success) {
        throw parse_error{source_position{0, 0, ""},
                          "failed to insert initial file", true};
    }

    preprocess(f_info.name, f_info);
}
auto Source::begin() const -> const_iterator { return this->text.begin(); }
auto Source::end() const -> const_iterator { return this->text.end(); }
auto Source::get_filename(const_iterator it) const -> std::string_view {
    // TODO: Implement
    return this->f_infos.back().name;
}
auto Source::it_to_src_pos(const_iterator it) const -> source_position {
    // TODO: Implement
}

static auto match_iter_with_str(std::random_access_iterator auto it,
                                std::random_access_iterator auto end,
                                std::string_view str) -> bool {
    return std::distance(it, end) >= str.size() &&
           std::equal(str.begin(), str.end(), it);
};
static auto find_non_whitespace(char_forward_iterator auto it,
                                char_forward_iterator auto end) {
    while (it != end && std::isspace(*it)) {
        ++it;
    }
    return it;
};
static auto find_whitespace(char_forward_iterator auto it,
                            char_forward_iterator auto end) {
    while (it != end && !std::isspace(*it)) {
        ++it;
    }
    return it;
};
static auto find_line_end(char_forward_iterator auto it,
                          char_forward_iterator auto end) {
    while (it != end && *it != '\n') {
        ++it;
    }
    return it;
}

auto Source::preprocess(std::string_view f_name, file_info &f_info) -> void {
    // Possible implementation notes:
    // 1. insert into this->text (or a vec passed by ref.) up until an include
    //      directive is found.
    // 2. check if include is valid -- if not, throw
    // 3. repeat from step 1.
    // that way everything is inserted past the end and expensive extra copying
    //      is avoided
    //? REMINDER: Do not count the size of directive lines into f_info.size

    //! NOTE: Enforce includes to only be at the beginning of a file to make
    //! dependencies into a D.A.G.
    auto cur_text = file_to_char_vec(f_name);
    const_iterator prev_it{cur_text.begin()}, it{cur_text.begin()},
        copy_it{cur_text.begin()};

    const auto save_line_length = [&](auto len) {
        f_info.line_lengths.push_back(len);
        f_info.size += len;
    };

    while (it != cur_text.end()) {
        if (*it == '#' && (it == cur_text.begin() || *(it - 1) == '\n')) {
            // '#' found after newline, this is a directive line

            // copy up to the character before the directive.
            this->text.insert(text.end(), copy_it, it);
            copy_it = find_line_end(it, cur_text.end());

            // TODO: Try-catch here to allow recovery from errors
            handle_directive(
                {static_cast<lineno_t>(f_info.line_lengths.size() + 1), 1,
                 f_info.name},
                {it + 1, copy_it});

            it = copy_it + 1; // this will be '\n' or text.end()
            prev_it = it;
            save_line_length(0);
        } else if (*it == '\n') {
            // save line-length
            save_line_length(std::distance(prev_it, it));
            prev_it = ++it;
        } else {
            ++it;
        }
    }
    save_line_length(std::distance(prev_it, it)); // last line-length
    this->text.insert(text.end(), copy_it, it);

    f_infos.emplace_back(f_info);
}

static auto match_str_with_str(std::string_view str, std::string_view match) {
    return match_iter_with_str(str.begin(), str.end(), match);
}
auto Source::handle_directive(const source_position &dir_pos,
                              std::string_view dir_line) -> void {
    struct dir_t {
        std::string_view name;
        std::optional<std::string> (Source::*handler)(std::string_view);
    };
    static const std::array directives = {
        dir_t{"include", &Source::handle_include}};

    for (auto &pair : directives) {
        if (match_str_with_str(dir_line, pair.name)) {
            if (auto err_msg =
                    (this->*pair.handler)(dir_line.substr(pair.name.size()))) {
                throw parse_error{dir_pos, *err_msg};
            }
            return;
        }
    }
    throw parse_error{dir_pos, "unknown directive"};
}
auto Source::handle_include(std::string_view dir_body)
    -> std::optional<std::string> {
    auto f_name_start = find_non_whitespace(dir_body.begin(), dir_body.end());
    auto f_name_end = find_whitespace(f_name_start, dir_body.end());
    if (f_name_start == f_name_end) {
        return "empty include filename";
    }
    if (find_non_whitespace(f_name_end, dir_body.end()) != dir_body.end()) {
        return "trailing characters after include filename";
    }
    if (*f_name_start != '"' || *(std::prev(f_name_end)) != '"') {
        return "include filename not enclosed in double quotes";
    }

    std::string_view f_name{std::next(f_name_start), std::prev(f_name_end)};
    if (!std::filesystem::exists(f_name)) {
        return fmt::format("file \"{}\" not found", f_name);
    }
    // fmt::print("include: {}\n", f_name);

    return std::nullopt;
}
