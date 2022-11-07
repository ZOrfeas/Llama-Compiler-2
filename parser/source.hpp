#ifndef PARSE_SOURCE_HPP
#define PARSE_SOURCE_HPP

#include "common.hpp"
#include <string_view>
#include <unordered_map>
#include <vector>

namespace lla {
    class Source {
    public:
        using const_iterator = std::vector<char>::const_iterator;

        Source(std::string_view);
        // auto preprocess() -> void;
        [[nodiscard]] auto begin() const -> const_iterator;
        [[nodiscard]] auto end() const -> const_iterator;
        [[nodiscard]] auto get_filename(const_iterator) const
            -> std::string_view;
        [[nodiscard]] auto it_to_src_pos(const_iterator) const
            -> source_position;

    private:
        struct file_info {
            std::vector<char>::size_type size;
            std::vector<colno_t> line_lengths;
        };

        std::vector<char> text;
        // std::vector<std::vector<char>::size_type> src_file_indexes;
        std::vector<std::string_view> //! NOTE: possibly obsolete
            filenames; // topologically sorted D.A.G. - aka include-order
        std::unordered_map<std::string, file_info>
            included_filenames; // used to detect include cycles
        //! NOTE: Replace directive lines with empty ones to not mess up
        //! source_position elsewhere
        auto preprocess(std::string_view) -> std::vector<char>;
    };
} // namespace lla

#endif // PARSE_SOURCE_HPP