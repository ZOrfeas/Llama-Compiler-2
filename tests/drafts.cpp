// #include "../parser/scanner.hpp"
#include <fstream>
#include <iostream>
#include <memory>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include "../utils/utils.hpp"
#include "../ast/ast.hpp"

// struct Var1 : public utils::Variant<int, double> {};
// struct Var2 : public utils::Variant<float, std::string> {};
// struct Var3 : public utils::Variant<std::string_view, char> {};

// struct Var : public utils::concat_variants_t<Var1, Var2, Var3> {};

auto main(int argc, char** argv) -> int {

    // auto some_int = ast::exprs::literals::Int{{{}, "kostas"}, 42};

    // static_assert(std::is_same_v<
    //         Var1::Variant, utils::Variant<int, double>
    //     >, "should be the same");

    // static_assert(std::is_same_v<
    //         Var::Variant, utils::Variant<
    //             int, double, float, 
    //             std::string, std::string_view,
    //             char
    //         >
    //     >, "should be the same");

    // using namespace typesys;
    // Type t1 = Type::get<Unit>();
    // Type t2 = Type::get<Unit>();
    // Type t3 = Type::get<Int>();
    // Type arr = Type::get<Array>(t1, 2);
    // Type arr2 = Type::get<Array>(t2, 2);
    // Type ref = Type::get<Ref>(t3);
    // std::cout << arr  << " == " << arr2 << ' ' << std::boolalpha << (arr == arr2) << '\n';
    // std::cout << arr << " == " << ref << ' ' << std::boolalpha << (arr == ref) << '\n';


    return 0;
}
