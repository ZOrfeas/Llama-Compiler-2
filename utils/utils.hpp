#ifndef LLA_UTILS_HPP
#define LLA_UTILS_HPP
#include "expected.hpp"
#include "fmt/core.h"
#include "unique_generator.h"
#include <string>

namespace lla::utils {
    auto make_file(const std::string &)
        -> tl::expected<std::unique_ptr<std::FILE, int (*)(std::FILE *)>,
                        std::string>;

    template <typename... Fs> struct match : Fs... {
        using Fs::operator()...;
    };
    template <class... Ts> match(Ts...) -> match<Ts...>;

    template <typename... Ts, typename... Fs>
    constexpr auto operator|(std::variant<Ts...> const &v,
                             match<Fs...> const &match) -> decltype(auto) {
        return std::visit(match, v);
    }
    template <typename Item, typename Writer>
        requires std::invocable<Writer, std::FILE *, Item>
    auto adapt_gen_with_writer(unique_generator<Item> gen,
                               const std::optional<std::string> &destination,
                               Writer writer) -> unique_generator<Item> {
        if (!destination) {
            return std::move(gen);
            // for (auto i : gen) {
            //     co_yield i;
            // }
        } else {
            auto dest_file = lla::utils::make_file(*destination);
            if (!dest_file) {
                fmt::print(stderr, "{}", dest_file.error());
                std::exit(1);
            }
            return map_gen(std::move(gen), [f = std::move(*dest_file),
                                            w = std::move(writer)](Item i) {
                w(f.get(), i);
                return i;
            });
            // for (auto i : gen) {
            //     writer((*dest_file).get(), i);
            //     co_yield i;
            // }
        }
    }

} // namespace lla::utils

#endif // LLA_UTILS_HPP