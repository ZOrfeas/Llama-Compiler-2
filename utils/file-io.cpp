#include "./utils.hpp"

namespace lla::utils {
auto make_file(const std::string &filename) noexcept
    -> std::unique_ptr<std::FILE, int (*)(std::FILE *)> {
    if (filename == "stdout")
        return {stdout, [](std::FILE *) -> int { return 0; }};
    if (filename == "stderr")
        return {stderr, [](std::FILE *) -> int { return 0; }};
    std::unique_ptr<std::FILE, int (*)(std::FILE *)> file(
        std::fopen(filename.c_str(), "w"), &std::fclose);
    return file;
}

} // namespace lla::utils