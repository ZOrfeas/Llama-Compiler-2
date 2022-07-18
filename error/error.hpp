#ifndef ERROR_HPP
#define ERROR_HPP

#include "spdlog/spdlog.h"
#include <iostream>
#include <string_view>
#include <concepts>
#include <type_traits>

namespace error {
    enum ErrorTy {
        INTERNAL = 0, RUNTIME, PARSING, SEMANTIC,
    };
    inline auto err_to_str(ErrorTy err) {
        static const char *err_str[] = {
            "Internal", "Runtime", "Parsing", "Semantic"
        };
        return err_str[static_cast<int>(err)];
    }
    template<ErrorTy E, typename... T>
    void crash(std::string_view msg) {
        spdlog::error("{} error: {}", err_to_str(E), msg);
        std::exit(1);
    }
}

#endif // ERROR_HPP