#include "lexer.hpp"

using namespace lla;
// // // // // // // //
auto Lexer::Source::preprocess(Source_file src) -> std::vector<Source_file> {
    // TODO: Implement #include statement lookup, recursively apply while
    // checking for cycles with the filenames set. Should not be too hard.
}

// Lexer::Lexer(std::string_view filename)
//     : tokens({}), errors({}), source(), source_it(), cur_pos({}) {}
