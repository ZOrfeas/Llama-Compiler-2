#ifndef UTILS_CONCEPTS_HPP
#define UTILS_CONCEPTS_HPP

#include "fmt/format.h"
#include <__concepts/same_as.h>
#include <cstddef>
#include <cstdint>
#include <string>

namespace lla::utils {

using lineno_t = std::uint32_t;
using colno_t = std::uint32_t;
struct pos_t {
    lineno_t           lineno;
    colno_t            colno;
    std::string const *filename;
    [[nodiscard]] auto to_string() const -> std::string {
        return fmt::format("{}({},{})", *filename, lineno, colno);
    }
};
struct tok_t {
    std::string val;
    pos_t       from, to;
};

template<typename T>
concept IScanner = requires(T t, std::size_t n) {
                       { t.peek() } -> std::same_as<char>;
                       { t.peek(n) } -> std::same_as<char>;
                       { t.consume() } -> std::same_as<char>;
                       { t.consume(n) } -> std::same_as<char>;
                       { t.is_eof() } -> std::same_as<bool>;
                   };
template<typename T>
concept ILexer = requires(T t, std::size_t n) {
                     { t.peek() } ->
                 };

} // namespace lla::utils

#endif // UTILS_CONCEPTS_HPP