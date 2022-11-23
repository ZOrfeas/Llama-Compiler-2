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
        using idx_pair_t = std::pair<std::vector<char>::difference_type,
                                     std::vector<char>::difference_type>;
        [[nodiscard]] auto idx_pair_to_str(const idx_pair_t &) const
            -> std::string_view;

        // TODO: Consider a BST to allow fast lookup from iterator to filename
        // TODO:    and lineno. Maybe a simple iterator on f_order and the chunk
        // TODO:    sizes will be enough though

        std::unordered_map<std::string, std::vector<idx_pair_t>> filemap;
        std::vector<std::pair<std::string_view, idx_pair_t>> f_order;
        std::vector<char> text;
        bool crash_on_error;

        auto f_name_to_f_info(std::string_view) -> std::vector<idx_pair_t> &;

        auto preprocess(std::string_view) -> void;
        auto handle_directive(const source_position &, std::string_view)
            -> void;
        auto handle_include(std::string_view) -> std::optional<std::string>;
    };
} // namespace lla::parse

#endif // PARSE_SOURCE_HPP