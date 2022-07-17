#ifndef __UTILS_HPP__
#define __UTILS_HPP__

#include <__tuple>
#include <algorithm>
#include <memory>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <variant>
#include <concepts>

#include "../error/error.hpp"

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

    template<typename... Ts>
    class SharedPtrVariant {
    protected:
        using shared_ptr_pack = std::tuple<std::shared_ptr<Ts>...>;
        static constexpr auto type_cnt = sizeof...(Ts);

        std::variant<std::shared_ptr<Ts>...> v;
        std::shared_ptr<void> raw_data;
    public:
        SharedPtrVariant() = delete; // TODO: think this through
        template<typename T> requires (std::disjunction_v<std::is_same<T, Ts>...>)
        SharedPtrVariant(T t) : v(t), raw_data(t) {}

        template<typename T> requires (std::disjunction_v<std::is_same<T, Ts>...>)
        static SharedPtrVariant wrap(std::shared_ptr<T> t) {
            return SharedPtrVariant(t);
        }
        template<typename T> requires (std::disjunction_v<std::is_same<T, Ts>...>)
        bool is() const {
            return std::holds_alternative<std::shared_ptr<T>>(v);
        }
        template<typename T> requires (std::disjunction_v<std::is_same<T, Ts>...>)
        std::shared_ptr<T> unsafe_as() const {
            return std::static_pointer_cast<T>(raw_data);
        }
        template<typename T> requires (std::disjunction_v<std::is_same<T, Ts>...>)
        std::shared_ptr<T> as() const {
            if (auto ptr_to_inner = std::get_if<std::shared_ptr<T>>(v)) {
                return *ptr_to_inner;
            }
            return std::shared_ptr<T>();
        }
        template<typename T> requires (std::disjunction_v<std::is_same<T, Ts>...>)
        std::shared_ptr<T> safe_as(std::string_view msg) const {
            if (auto inner = as<T>(); inner) {
                return inner;
            }
            error::crash<error::INTERNAL>(msg);
        }
    };

    // TODO: If feeling adventurous, try and make this template
    // TODO:  able to be instantiated by combining many of itself
}

#endif // __UTILS_HPP__