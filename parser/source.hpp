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
        auto print_text() const -> void;

    private:
        struct file_info {
            file_info(std::string name) : name(std::move(name)) {}
            std::string name;
            std::vector<char>::size_type size{};
            std::vector<colno_t> line_lengths{};
            auto operator==(const file_info &other) const -> bool {
                return this == &other || this->name == other.name;
            }
        };
        struct f_bound {
            const_iterator idx;
            std::vector<file_info>::size_type f_info_idx{};
        };

        // owning container for file_info structs
        std::vector<file_info> f_infos;
        // used to detect include cycles
        std::unordered_map<std::string, std::vector<file_info>::size_type>
            f_name_map;
        std::vector<f_bound> f_bounds;

        std::vector<char> text;

        //! NOTE: Replace directive lines with empty ones to not mess up
        //! source_position elsewhere
        auto preprocess(std::vector<file_info>::size_type) -> void;
        auto handle_directive(const source_position &, std::string_view)
            -> void;
        auto handle_include(std::string_view) -> std::optional<std::string>;
    };
} // namespace lla::parse

#endif // PARSE_SOURCE_HPP