include(FetchContent)

add_library(lla-parser lexer.cpp parser.cpp lexer-utils.cpp)
FetchContent_Declare(
    fmt
    GIT_REPOSITORY https://github.com/fmtlib/fmt
    GIT_TAG c4ee726532178e556d923372f29163bd206d7732 # v9.0.0
)
FetchContent_MakeAvailable(fmt)
target_link_libraries(lla-parser PUBLIC fmt::fmt)