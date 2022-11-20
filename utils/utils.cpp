#include "./utils.hpp"
#include "fmt/format.h"
#include <memory>

namespace lla::utils {
    auto make_file(const std::string &name)
        -> std::unique_ptr<std::FILE, int (*)(std::FILE *)> {
        if (name == "stdout")
            return {stdout, [](std::FILE *) -> int { return 0; }};
        if (name == "stderr")
            return {stderr, [](std::FILE *) -> int { return 0; }};
        std::unique_ptr<std::FILE, int (*)(std::FILE *)> file(
            std::fopen(name.c_str(), "w"), &std::fclose);
        if (!file) {
            throw std::runtime_error(
                fmt::format("failed to open file {}", name));
        }
        return file;
    }

} // namespace lla::utils