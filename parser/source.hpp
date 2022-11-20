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
        [[nodiscard]] auto it_to_src_pos(const_iterator) const
            -> source_position;
        auto print_text(const std::string &) const -> void;

    private:
        struct file_info {
            std::vector<char>::size_type size{};
            std::vector<colno_t> line_lengths{};
            auto operator==(const file_info &other) const -> bool {
                return this == &other;
            }
        };
        struct f_bound {
            std::vector<char>::size_type idx;
            std::string_view f_name;
        };

        // owning container for file_info structs
        std::vector<std::string_view> f_name_dag;
        // used to detect include cycles
        std::unordered_map<std::string, file_info> filemap;
        std::vector<f_bound> f_bounds;

        std::vector<char> text;

        auto f_name_to_f_info(std::string_view) -> file_info &;
        //! NOTE: Replace directive lines with empty ones to not mess up
        //! source_position elsewhere
        auto preprocess(std::string_view) -> void;
        auto handle_directive(const source_position &, std::string_view)
            -> void;
        auto handle_include(std::string_view) -> std::optional<std::string>;
    };
} // namespace lla::parse

#endif // PARSE_SOURCE_HPP