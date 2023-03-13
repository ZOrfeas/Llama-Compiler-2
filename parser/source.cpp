#include "source.hpp"
#include "common.hpp"
#include "expected.hpp"
#include "utils.hpp"
#include <fstream>
#include <memory>
#include <optional>
#include <string>
#include <string_view>

namespace lla::parse {

    auto file_lines(std::string source) -> unique_generator<Line> {
        // could use C file API, maybe faster, benchmark later.
        // https://stackoverflow.com/a/51572325/13537527
        std::ifstream file(source, std::ios::binary);
        if (file.is_open()) {
            std::string line;
            lineno_t cur_lineno = 0;
            while (std::getline(file, line)) {
                cur_lineno++;
                co_yield Line{std::move(line), cur_lineno};
            }
        }
    }
    auto handle_include(std::string_view line)
        -> tl::expected<std::optional<std::string>, std::string> {
        constexpr static std::string_view include_directive = "#include ";
        constexpr static auto include_directive_len = include_directive.size();

        const auto directive_start = line.find_first_not_of(" \t\n");
        if (directive_start == std::string_view::npos ||
            line.at(directive_start) != '#') {
            return std::nullopt;
        }
        if (line.substr(directive_start, include_directive_len) !=
            include_directive) {
            return tl::unexpected(fmt::format("Unrecognized directive: {}",
                                              line.substr(directive_start)));
        }
        const auto filename_start = line.find_first_not_of(
            " \t\n", directive_start + include_directive_len);
        if (filename_start == std::string_view::npos) {
            return tl::unexpected(std::string("Filename not specified"));
        }
        const auto filename_end = [&]() {
            if (auto retval = line.find_first_of(" \t\n", filename_start);
                retval != std::string_view::npos) {
                return retval;
            } else {
                return line.size();
            }
        }();
        const auto filename = line.substr(filename_start, filename_end);
        if (filename.at(0) != '"' || filename.at(filename.size() - 1) != '"') {
            return tl::unexpected(fmt::format(
                "Invalid filename: {} (remember to wrap in '\"')", filename));
        }
        return std::string(filename.substr(1, filename.size() - 2));
    }

    struct Buffer {
        FilenamePtr filename;
        unique_generator<Line> line_gen;
        Buffer(std::string source)
            : filename(std::make_shared<std::string>(source)),
              line_gen(file_lines(std::move(source))) {}
    };
    auto all_files_lines(std::string source, bool crash_on_error)
        -> unique_generator<ScanEvent> {
        std::vector<Buffer> buffer_stack;

        buffer_stack.emplace_back(std::move(source));
        co_yield buffer_stack.back().filename;

        while (!buffer_stack.empty()) {
            auto &buf = buffer_stack.back();

            if (auto line = buf.line_gen.next(); line) {
                auto included_file = handle_include(line->text);
                included_file.map_error([&](auto &&err) {
                    fmt::print(stderr, "Error at line {} in {}: {}\n",
                               line->lineno, *buf.filename, err);
                    if (crash_on_error) {
                        std::exit(1);
                    }
                });
                if (included_file && *included_file) {
                    buffer_stack.emplace_back(std::move(**included_file));
                    co_yield buffer_stack.back().filename;
                    continue;
                }
                co_yield *line;
            } else {
                buffer_stack.pop_back();
                if (!buffer_stack.empty()) {
                    co_yield buffer_stack.back().filename;
                }
            }
        }
    }
} // namespace lla::parse
