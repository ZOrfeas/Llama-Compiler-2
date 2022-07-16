
#include "../passes/sem/utils/tables.hpp"
#include "../ast/ast.hpp"
#include "spdlog/spdlog.h"
#include <iostream>

int main(int argc, char** argv) {
    using namespace sem::tables;
    // std::unordered_map<std::string, int> m;
    spdlog::set_level(spdlog::level::trace);
    Table t;

    t.insert("a", new ast::core::Program());
    t.insert("b", new ast::core::Program());
    t.insert("c", new ast::core::Program());

    std::cout << std::boolalpha << !!t.lookup("a") << std::endl;
    std::cout << std::boolalpha << !!t.lookup("b") << std::endl;

    t.open_scope();
    t.insert("a", new ast::core::Program());

    std::cout << std::boolalpha << !!t.lookup("a") << std::endl;
    std::cout << std::boolalpha << !!t.lookup("b") << std::endl;

    return 0;
}
