#ifndef ERROR_HPP
#define ERROR_HPP

#include "fmt/core.h"
#include "fmt/color.h"
#include <cstdio>
#include <iostream>
#include <string_view>
#include <concepts>
#include <type_traits>

namespace log {
    enum Level {
        Info, Warning, Error, Debug
    };
    constexpr inline auto level_to_string(Level level) -> std::string_view {
        switch (level) {
            case Level::Info: return "Info";
            case Level::Warning: return "Warn";
            case Level::Error: return "Error";
            case Level::Debug: return "Debug";
        }
    }
    constexpr inline auto level_to_outfile(Level level) -> FILE* {
        switch (level) {
            case Level::Info: return stdout;
            case Level::Warning: return stderr;
            case Level::Error: return stderr;
            case Level::Debug: return stderr;
        }
    }
    constexpr inline auto style(Level level) -> fmt::text_style {
        const auto color = [](Level level) {
            switch(level) {
                case Level::Info: return fmt::color::green;
                case Level::Warning: return fmt::color::yellow;
                case Level::Error: return fmt::color::red;
                case Level::Debug: return fmt::color::cyan;
            }
        }(level);
        return fmt::emphasis::bold | fmt::fg(color);
    }
    template<Level l>
    auto print_preamble() -> void {
        fmt::print(level_to_outfile(l), style(l), "[{}] ", level_to_string(l));
    }
    template<Level l, typename... Args>
    auto log(fmt::format_string<Args...> s, Args&&... args) -> void {
        print_preamble<l>();
        fmt::print(level_to_outfile(l), s, std::forward<Args>(args)...);
    }
    template<typename... Args>
    auto crash(fmt::format_string<Args...> s, Args&&... args) -> void {
        log<Error>(s, std::forward<Args>(args)...);
        std::exit(1);
    }

}
#endif // ERROR_HPP