#include "lexer.hpp"

using namespace lla;
// // // // // // // //
Lexer::Lexer(std::string_view filename) : src(filename), tokens(), errors() {}

// Lexer::Lexer(std::string_view filename)
//     : tokens({}), errors({}), source(), source_it(), cur_pos({}) {}
