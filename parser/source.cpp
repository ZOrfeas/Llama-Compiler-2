#include "source.hpp"
#include "common.hpp"
#include "fmt/core.h"
#include "fmt/format.h"
#include "utils.hpp"
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
Source::Source(std::string_view filename, bool crash_on_error)
    : filemap({}), f_order({}), text({}), crash_on_error(crash_on_error) {
    auto [it, success] = this->filemap.insert({std::string(filename), {}});
    if (!success) {
        throw parse_error{source_position{0, 0, ""},
                          "failed to insert initial file", true};
    }
    this->preprocess(it->first);
}
auto Source::print_text(const std::string &outfilename) const -> void {
    const auto file = utils::make_file(outfilename);
    fmt::print(file.get(), "{}", fmt::join(this->text, ""));
    fmt::print(file.get(), "\n");
}
auto Source::begin() const -> const_iterator { return this->text.begin(); }
auto Source::end() const -> const_iterator { return this->text.end(); }
auto Source::get_filename(const_iterator it) const -> std::string_view {
    // TODO: Implement
    return this->filemap.begin()->first;
}
auto Source::idx_pair_to_str(const idx_pair_t &f_line) const
    -> std::string_view {
    return {std::next(this->text.begin(), f_line.first),
            std::next(this->text.begin(), f_line.second)};
}
auto Source::f_name_to_f_info(std::string_view f_name)
    -> std::vector<idx_pair_t> & {
    if (auto it = this->filemap.find(std::string(f_name));
        it != this->filemap.end()) {
        return it->second;
    } else {
        throw parse_error{source_position{0, 0, ""},
                          "failed to find file in filemap", true};
    }
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

auto Source::preprocess(std::string_view f_name) -> void {
    auto cur_text = file_to_char_vec(f_name);
    const_iterator prev_it{cur_text.begin()}, it{cur_text.begin()},
        copy_it{cur_text.begin()};
    const_iterator::difference_type copy_backlog_cnt{0};
    auto &f_lines = this->f_name_to_f_info(f_name);

    const auto save_line_length = [&]() {
        if (prev_it == cur_text.end()) return;

        const auto char_cnt = std::distance(prev_it, it);
        const auto from_diff = std::distance(
            this->text.begin(), std::next(this->text.end(), copy_backlog_cnt));
        const auto to_diff = from_diff + char_cnt;
        f_lines.emplace_back(from_diff, to_diff);
    };
    const auto save_text = [&]() {
        this->text.insert(this->text.end(), copy_it, it);
        copy_backlog_cnt = 0;
        copy_it = find_line_end(it, cur_text.end());

        if (copy_it == it) return;
        if (f_order.empty() || f_order.back().data() != f_name.data()) {
            f_order.push_back(f_name);
        }
    };

    while (it != cur_text.end()) {
        if (*it == '#' && (it == cur_text.begin() || *(it - 1) == '\n')) {
            // '#' found after newline, this is a directive line

            // copy up to the character before the directive.
            save_text();

            const source_position dir_pos{
                static_cast<lineno_t>(f_lines.size() + 1), 1, f_name};
            const std::string_view dir_str{it, copy_it};
            try {
                handle_directive(dir_pos, {it, copy_it});
            } catch (const parse_error &e) {
                if (this->crash_on_error) {
                    throw;
                }
                fmt::print(stderr, "{} at {}", e.what(), dir_pos.to_string());
            }

            it = copy_it; // this will be '\n' or text.end()
            prev_it = it;
        } else if (*it == '\n') { // save line-length
            ++it;
            save_line_length();
            copy_backlog_cnt += std::distance(prev_it, it);
            prev_it = it;
        } else {
            ++it;
        }
    }
    save_line_length(); // last line-length
    save_text();
}

static auto match_str_with_str(std::string_view str, std::string_view match) {
    return match_iter_with_str(str.begin(), str.end(), match);
}
auto Source::handle_directive(const source_position &dir_pos,
                              std::string_view dir_line) -> void {
    using std::string_view_literals::operator""sv;
    static const std::array directives = {
        std::pair{"#include"sv, &Source::handle_include}};

    for (auto &pair : directives) {
        if (match_str_with_str(dir_line, pair.first)) {
            if (auto err_msg =
                    (this->*pair.second)(dir_line.substr(pair.first.size()))) {
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
    if (*f_name_start != '"' || *(std::prev(f_name_end)) != '"') {
        return "include filename not enclosed in double quotes";
    }
    if (find_non_whitespace(f_name_end, dir_body.end()) != dir_body.end()) {
        return "trailing characters after include filename";
    }
    std::string_view f_name{std::next(f_name_start), std::prev(f_name_end)};
    if (!std::filesystem::exists(f_name)) {
        return fmt::format("file \"{}\" not found", f_name);
    }
    auto [it, success] = this->filemap.insert({std::string(f_name), {}});
    if (!success) {
        return fmt::format("file \"{}\" already included", f_name);
    }
    this->preprocess(it->first);

    return std::nullopt;
}
