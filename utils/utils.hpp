#ifndef __UTILS_HPP__
#define __UTILS_HPP__

#include <variant>

namespace utils {
    template <typename T, template <typename...> class Z>
    struct is_specialization_of : std::false_type {};
    template <typename... Args, template <typename...> class Z>
    struct is_specialization_of<Z<Args...>, Z> : std::true_type {};
    template <typename T, template <typename...> class Z>
    inline constexpr bool is_specialization_of_v = is_specialization_of<T,Z>::value;
    template <typename T>
    concept IsSharedPtr = is_specialization_of_v<T, std::shared_ptr>;
    
    // =====[ match ]=====
    // source: https://github.com/AVasK/vx/blob/main/vx.hpp
    template <typename... Fs>
    struct match : Fs... {using Fs::operator()...;};
    template<class... Ts> match(Ts...) -> match<Ts...>; // needed even though c++20... bad clang

    template <typename... Ts, typename... Fs>
    constexpr decltype(auto) operator| (std::variant<Ts...> const& v, match<Fs...> const& match) {
        return std::visit(match, v);
    }
    template <typename... Ts1, typename... Ts2, typename... Fs>
    constexpr decltype(auto) operator| (std::tuple<std::variant<Ts1...>, std::variant<Ts2...>> const& v, match<Fs...> const& match) {
        auto& [v1, v2] = v;
        return std::visit(match, v1, v2);
    }

}

#endif // __UTILS_HPP__