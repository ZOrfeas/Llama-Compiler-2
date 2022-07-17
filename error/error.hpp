#ifndef __ERROR_HPP__
#define __ERROR_HPP__

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
    void crash(std::string_view msg, T&&... args) {
        const auto fmt_msg = 
            spdlog::fmt_lib::format(msg, std::forward<T>(args)...);
        spdlog::error("{} error: {}", err_to_str(E), fmt_msg);
        std::exit(1);
    }
}

#endif // __ERROR_HPP__