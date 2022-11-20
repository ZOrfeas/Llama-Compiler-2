#ifndef LLA_UTILS_HPP
#define LLA_UTILS_HPP
#include <string>

namespace lla::utils {
    auto make_file(const std::string &)
        -> std::unique_ptr<std::FILE, int (*)(std::FILE *)>;

}

#endif // LLA_UTILS_HPP