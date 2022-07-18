#include "../parser/parser.hpp"
#include "../parser/driver.hpp"
#include <iostream>

int main(int argc, char** argv) {
    std::cout << ParseDriver("examples/test.lla").parse() << '\n';
    return 0;
}
