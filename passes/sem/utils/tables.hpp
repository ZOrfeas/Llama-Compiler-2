#ifndef __SEM_TABLES_HPP__
#define __SEM_TABLES_HPP__

#include <optional>
#include <unordered_map>
#include <string_view>
#include <vector>
#include <utility>

#include <spdlog/spdlog.h>

// cannot only include ast/forward due to inheritance.
#include "../../../ast/ast.hpp"
#include "../../../typesystem/types.hpp"

namespace sem::tables {
    using ast::core::Node;

    struct Entry {
        Node* node;
        typesys::Type type;
    };
    template<bool is_member_of_table = false>
    class Scope {
    public:
    private:
        std::unordered_map<std::string_view, Entry> entries;
    public:
        Scope() {}
        std::optional<Entry> lookup(std::string_view name) const {
            if constexpr (!is_member_of_table)
                spdlog::trace("[sem::tables::Scope::lookup] looking-up {}", name);
            if (const auto it = entries.find(name); it != entries.end()) {
                return it->second;
            }
            return std::nullopt;
        }
        bool insert(std::string_view name, Node* node, typesys::Type type) {
            if constexpr (!is_member_of_table)
                spdlog::trace("[sem::tables::Scope::insert] inserting {}", name);
            Entry newEntry{node, type};
            if (auto it = entries.find(name); it != entries.end()) {
                if constexpr (!is_member_of_table) {
                    spdlog::trace("[sem::tables::Scope::insert] name '{}' redeclared", name);
                    return false;
                }
                it->second = newEntry;
            } else {
                entries.insert({name, newEntry});
            }
            return true;
        }
    };
    class Table {
    public:
        using Scope = Scope<true>;
    private:
        std::vector<Scope> scopes;
    public:
        Table(): scopes({Scope()}) {}
        std::optional<Entry> lookup(std::string_view name) const {
            spdlog::trace("[sem::tables::Table::lookup] looking-up {}", name);
            for (auto it = scopes.rbegin(); it != scopes.rend(); ++it) {
                if (const auto entry = it->lookup(name)) {
                    return entry;
                }
            }
            return std::nullopt;
        }
        bool insert(std::string_view name , Node* node, typesys::Type type) {
            spdlog::trace("[sem::tables::Table::insert] inserting {}", name);
            return scopes.back().insert(name, node, type);
        }
        void open_scope() {
            spdlog::trace("[sem::tables::Table::open_scope]");
            scopes.emplace_back();
        }
        void close_scope() {
            spdlog::trace("[sem::tables::Table::close_scope]");
            scopes.pop_back();
        }
    }; 

}

#endif // __SEM_TABLES_HPP__