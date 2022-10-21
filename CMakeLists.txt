cmake_minimum_required(VERSION 3.24)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_EXPORT_COMPILE_COMMANDS True)

project(LLAMAC VERSION 0.1)
configure_file(config.hpp.in config.hpp)

add_subdirectory(cli)

add_executable(llamac main.cpp)
target_include_directories(llamac PUBLIC ${LLAMAC_BINARY_DIR})
target_link_libraries(llamac PUBLIC lla-cli)
