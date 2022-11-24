add_library(lla-cli cli.cpp)
target_include_directories(lla-cli INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
add_library(LLAMAC::CLI ALIAS lla-cli)

target_link_libraries(lla-cli PUBLIC CLI11::CLI11)
