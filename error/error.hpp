#ifndef __ERROR_HPP__
#define __ERROR_HPP__

#include <iostream>
#include <string_view>

namespace error {
    void internal(std::string_view msg);
    void runtime(std::string_view msg);
    void parse(std::string_view msg);
}

#endif // __ERROR_HPP__