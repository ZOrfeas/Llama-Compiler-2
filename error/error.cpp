#include <iostream>

#include "./error.hpp"

namespace error {
    void internal(std::string_view msg) {
        std::cerr << "Internal error: " << msg << std::endl;
        std::exit(1);
    }
}