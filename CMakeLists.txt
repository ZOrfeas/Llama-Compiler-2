cmake_minimum_required(VERSION 3.24)
include(FetchContent)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_EXPORT_COMPILE_COMMANDS True)

project(LLAMAC VERSION 0.0.1)
configure_file(config.hpp.in config.hpp)

FetchContent_Declare(fmt
    GIT_REPOSITORY https://github.com/fmtlib/fmt
    GIT_TAG c4ee726532178e556d923372f29163bd206d7732 # v9.0.0
)
FetchContent_Declare(cli11
    GIT_REPOSITORY https://github.com/CLIUtils/CLI11
    GIT_TAG a66ae4145779c56dc0f9f98a631656417dd77de8 # v2.3.0
)
FetchContent_MakeAvailable(fmt)
FetchContent_MakeAvailable(cli11)

add_subdirectory(utils)
add_subdirectory(cli)
add_subdirectory(parser)

add_executable(llamac main.cpp)
target_include_directories(llamac PUBLIC ${LLAMAC_BINARY_DIR})

target_link_libraries(llamac PUBLIC LLAMAC::CLI LLAMAC::PARSER)
