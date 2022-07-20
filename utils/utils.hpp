#ifndef UTILS_HPP
#define UTILS_HPP

#include <__tuple>
#include <algorithm>
#include <memory>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <variant>
#include <concepts>

#include "../log/log.hpp"

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

    template<typename... Ts>
    class Variant : public std::variant<Ts...> {
    protected:
    public:
        using base_std_variant_t = std::variant<Ts...>;
        using type = Variant;
        Variant() = delete;
        using base_std_variant_t::base_std_variant_t; // inherit constructors
        using base_std_variant_t::operator=; // inherit assignment operator

        template<typename T> requires (std::disjunction_v<std::is_same<T, Ts>...>)
        bool is() const { return std::holds_alternative<T>(*this); }
        
        template<typename T> requires (std::disjunction_v<std::is_same<T, Ts>...>)
        T& as(std::string_view msg) const {
            if (auto ptr = std::get_if<T>(this)) return *ptr;
            log::crash("{}\n", msg);
        }
    };

    template<typename T>
    concept IsVariant = is_specialization_of_v<T, Variant>;

    template <IsVariant Var1, IsVariant Var2> struct concat_variants;
    template <typename... Ts1, typename... Ts2>
    struct concat_variants<Variant<Ts1...>, Variant<Ts2...>> {
        using type = Variant<Ts1..., Ts2...>;
    };
    
}

#endif // UTILS_HPP