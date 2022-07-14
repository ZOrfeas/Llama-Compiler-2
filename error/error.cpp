#include "./error.hpp"

namespace error {
    void internal(std::string_view msg) {
        std::cerr << "Internal error: " << msg << '\n';
        std::exit(1);
    }
    void runtime(std::string_view msg) {
        std::cerr << "Runtime error: " << msg << '\n';
        std::exit(1);
    }
    void parse(std::string_view msg) {
        std::cerr << "Parse error: " << msg << '\n';
        std::exit(1);
    }
}