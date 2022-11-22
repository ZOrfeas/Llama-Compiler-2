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

        Source(std::string_view);
        [[nodiscard]] auto begin() const -> const_iterator;
        [[nodiscard]] auto end() const -> const_iterator;
        [[nodiscard]] auto get_filename(const_iterator) const
            -> std::string_view;
        auto print_text(const std::string &) const -> void;

    private:
        struct file_info {
            std::vector<char>::size_type size{};
            std::vector<colno_t> line_lengths{};
        };
        struct f_bound {
            std::string_view f_name;
            std::vector<char>::size_type idx;
            // std::vector<colno_t>::size_type line_cnt;
        };

        std::vector<std::string_view> f_name_dag;
        std::unordered_map<std::string, file_info> filemap;
        std::vector<f_bound> f_bounds;

        std::vector<char> text;

        auto f_name_to_f_info(std::string_view) -> file_info &;

        auto preprocess(std::string_view) -> void;

        auto handle_directive(const source_position &, std::string_view)
            -> void;
        auto handle_include(std::string_view) -> std::optional<std::string>;
    };
} // namespace lla::parse

#endif // PARSE_SOURCE_HPP