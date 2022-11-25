#ifndef UTILS_HPP
#define UTILS_HPP

#include <cstdio>
#include <memory>
#include <string>

namespace lla::utils {
/**
 * @brief Wrapper around cstdio's fopen using RAII.
 * @param filename Can be "stdout" or "stderr", or any other valid filename.
 * @return An owning unique_ptr to a FILE, or nullptr.
 */
[[nodiscard]] auto make_file(const std::string &filename) noexcept
    -> std::unique_ptr<std::FILE, int (*)(std::FILE *)>;

} // namespace lla::utils

#endif // UTILS_HPP