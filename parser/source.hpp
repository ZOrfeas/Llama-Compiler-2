#ifndef PARSE_SOURCE_HPP
#define PARSE_SOURCE_HPP

#include "common.hpp"
#include <functional>
#include <string_view>
#include <unordered_map>
#include <vector>

namespace lla::parse {
    class Source {
    public:
        using const_iterator = std::vector<char>::const_iterator;

        Source(std::string_view, bool = true);
        [[nodiscard]] auto begin() const -> const_iterator;
        [[nodiscard]] auto end() const -> const_iterator;
        [[nodiscard]] auto get_filename(const_iterator) const
            -> std::string_view;
        auto print_text(const std::string &) const -> void;

    private:
        using f_line_t = std::pair<std::vector<char>::difference_type,
                                   std::vector<char>::difference_type>;
        [[nodiscard]] auto f_line_to_str(const f_line_t &) const
            -> std::string_view;

        std::unordered_map<std::string, std::vector<f_line_t>> filemap;
        std::vector<std::string_view> f_order;
        std::vector<char> text;
        bool crash_on_error;

        auto f_name_to_f_info(std::string_view) -> std::vector<f_line_t> &;

        auto preprocess(std::string_view) -> void;
        auto handle_directive(const source_position &, std::string_view)
            -> void;
        auto handle_include(std::string_view) -> std::optional<std::string>;
    };
} // namespace lla::parse

#endif // PARSE_SOURCE_HPP