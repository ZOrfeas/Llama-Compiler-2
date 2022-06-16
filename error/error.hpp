#ifndef __ERROR_HPP__
#define __ERROR_HPP__

#include <string_view>

namespace error {
    void internal(std::string_view msg);
}

#endif // __ERROR_HPP__