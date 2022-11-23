include(FetchContent)

add_library(lla-utils file-io.cpp)
target_include_directories(lla-utils INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
add_library(LLAMAC::UTILS ALIAS lla-utils)

FetchContent_Declare(
    fmt
    GIT_REPOSITORY https://github.com/fmtlib/fmt
    GIT_TAG c4ee726532178e556d923372f29163bd206d7732 # v9.0.0
)
FetchContent_MakeAvailable(fmt)
target_link_libraries(lla-utils PRIVATE fmt::fmt)

