#ifndef __ERROR_HPP__
#define __ERROR_HPP__

#include <iostream>
#include <string_view>
#include <concepts>
#include <type_traits>

namespace error {
    template<typename T>
    concept ErrorTy = requires (T t) {
        { T::NAME } -> std::convertible_to<const char *>;
    };
    template<ErrorTy E>
    void crash(std::string_view msg, int exit_code = 1) {
        std::cerr << E::NAME << " error: " << msg << '\n';
        std::exit(exit_code);
    }
    class Internal {
    public: static constexpr const char* NAME = "Internal";
    };
    class Runtime {
    public: static constexpr const char* NAME = "Runtime";
    };
    class Parsing {
    public: static constexpr const char* NAME = "Parsing";
    };
}

#endif // __ERROR_HPP__