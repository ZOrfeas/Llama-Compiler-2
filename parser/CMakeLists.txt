include(FetchContent)

add_library(lla-parser lexer.cpp parser.cpp source.cpp)
target_include_directories(lla-parser INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
add_library(LLAMAC::PARSER ALIAS lla-parser)

FetchContent_Declare(
    fmt
    GIT_REPOSITORY https://github.com/fmtlib/fmt
    GIT_TAG c4ee726532178e556d923372f29163bd206d7732 # v9.0.0
)
FetchContent_MakeAvailable(fmt)
# FetchContent_Declare(
#     expected
#     GIT_REPOSITORY https://github.com/TartanLlama/expected
#     GIT_TAG v1.0.0
# )
# FetchContent_MakeAvailable(expected)
target_link_libraries(lla-parser PRIVATE LLAMAC::UTILS)
target_link_libraries(lla-parser PUBLIC fmt::fmt external::coro external::expected)
