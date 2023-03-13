#include "./utils.hpp"
#include "expected.hpp"
#include "fmt/format.h"
#include <memory>

namespace lla::utils {
    auto make_file(const std::string &name)
        -> tl::expected<std::unique_ptr<std::FILE, int (*)(std::FILE *)>,
                        std::string> {
        if (name == "stdout")
            return std::unique_ptr<std::FILE, int (*)(std::FILE *)>(
                stdout, [](std::FILE *) -> int { return 0; });
        if (name == "stderr")
            return std::unique_ptr<std::FILE, int (*)(std::FILE *)>(
                stderr, [](std::FILE *) -> int { return 0; });
        std::unique_ptr<std::FILE, int (*)(std::FILE *)> file(
            std::fopen(name.c_str(), "w"), &std::fclose);
        if (!file) {
            tl::unexpected(fmt::format("Failed to open file: {}", name));
        }
        return file;
    }

} // namespace lla::utils