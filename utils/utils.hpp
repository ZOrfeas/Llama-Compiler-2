#ifndef UTILS_HPP
#define UTILS_HPP

// #include <__tuple>
#include <algorithm>
#include <memory>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>
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

    template<typename T, typename... Ts>
    concept is_one_of = std::disjunction_v<std::is_same<T, Ts>...>;

    template<typename... Ts>
    class Variant : public std::variant<Ts...> {
    public:
        using type = Variant;
        using base_std_variant_t = std::variant<Ts...>;
        Variant() = delete;
        using base_std_variant_t::base_std_variant_t; // inherit constructors
        using base_std_variant_t::operator=; // inherit assignment operator
        
        template<typename T> requires (is_one_of<T, Ts...>)
        auto is() const -> bool { return std::holds_alternative<T>(*this); }
        
        template<typename T> requires (is_one_of<T, Ts...>)
        auto as(std::string_view msg) const -> T& {
            if (auto ptr = std::get_if<T>(this)) return *ptr;
            log::crash("{}\n", msg);
        }
    };
    
    template<typename T>
    concept IsVariant = is_specialization_of_v<T, Variant>;
    template<typename T>
    concept HasTypeFieldVariant = IsVariant<typename T::type>;
    template<typename T, typename VARIANT_T> struct is_variant_member_helper : std::false_type {};
    template<typename T, typename... Ts>
    struct is_variant_member_helper<T, Variant<Ts...>> 
        : public std::disjunction<std::is_same<T, Ts>...> {};
    template<typename T, typename Var>
    concept IsVariantMember = 
        HasTypeFieldVariant<Var> && 
        is_variant_member_helper<T, typename Var::type>::value;

    template<typename T>
    struct enable_make_variant {
        template<typename O, typename... Args>
        static auto make(Args&&... args) -> std::unique_ptr<T> {
            static_assert(IsVariantMember<O, T>);
            return std::unique_ptr<T>(new T{
                std::in_place_type<O>, std::forward<Args>(args)...
            });
        }
    };

    template<IsVariant... Vars> struct concat_variants;
    template<typename... Ts1, typename... Ts2>
    struct concat_variants<Variant<Ts1...>, Variant<Ts2...>> {
        using type = Variant<Ts1..., Ts2...>;
    };
    template<IsVariant Var1, IsVariant Var2, IsVariant... Vars>
    requires (sizeof...(Vars) > 0)
    struct concat_variants<Var1, Var2, Vars...> {
        using type = typename concat_variants<
            typename concat_variants<
                //!Note: Using Var1::Variant produced a warning in clang.
                typename Var1::type,
                typename Var2::type
            >::type, Vars...
        >::type;
    };
    template<HasTypeFieldVariant...Vars>
    using concat_variants_t = 
        typename concat_variants<typename Vars::type...>::type;
}

#endif // UTILS_HPP