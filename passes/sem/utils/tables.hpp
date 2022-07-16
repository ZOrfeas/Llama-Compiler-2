#ifndef __SEM_TABLES_HPP__
#define __SEM_TABLES_HPP__

#include <unordered_map>
#include <string_view>
#include <vector>
#include <utility>

#include <spdlog/spdlog.h>

// cannot only include ast/forward due to inheritance.
#include "../../../ast/ast.hpp"

namespace sem::tables {
    using ast::core::Node;
    template<bool is_member_of_table = false>
    class Scope {
    private:
        std::unordered_map<std::string_view, Node*> entries;
    public:
        Scope() {}
        Node* lookup(std::string_view name) const {
            if constexpr (!is_member_of_table)
                spdlog::trace("[sem::tables::Scope::lookup] looking-up {}", name);
            if (const auto it = entries.find(name); it != entries.end()) {
                return it->second;
            }
            return nullptr;
        }
        bool insert(std::string_view name, Node* entry) {
            if constexpr (!is_member_of_table)
                spdlog::trace("[sem::tables::Scope::insert] inserting {}", name);
            if (auto it = entries.find(name); it != entries.end()) {
                if constexpr (!is_member_of_table) {
                    spdlog::trace("[sem::tables::Scope::insert] name '{}' redeclared", name);
                    return false;
                }
                it->second = entry;
            } else {
                entries.insert({name, entry});
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
        Node* lookup(std::string_view name) const {
            spdlog::trace("[sem::tables::Table::lookup] looking-up {}", name);
            for (auto it = scopes.rbegin(); it != scopes.rend(); ++it) {
                if (const auto entry = it->lookup(name); entry != nullptr) {
                    return entry;
                }
            }
            return nullptr;
        }
        bool insert(std::string_view name , Node* entry) {
            spdlog::trace("[sem::tables::Table::insert] inserting {}", name);
            return scopes.back().insert(name, entry);
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