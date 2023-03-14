#ifndef PARSE_SOURCE_HPP
#define PARSE_SOURCE_HPP

#include "common.hpp"
#include "unique_generator.h"
#include <string>
#include <string_view>
#include <variant>

namespace lla::parse {
    struct Line {
        std::string text;
        lineno_t lineno;
    };

    using ScanEvent = std::variant<Line, FilenamePtr>;

    [[nodiscard]] auto all_files_lines(std::string source,
                                       bool crash_on_error = true)
        -> unique_generator<ScanEvent>;
} // namespace lla::parse

#endif // PARSE_SOURCE_HPP