include(FetchContent)

add_library(lla-cli cli.cpp)
target_include_directories(lla-cli INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
add_library(LLAMAC::CLI ALIAS lla-cli)

# FetchContent_Declare(
#     cli11
#     GIT_REPOSITORY https://github.com/CLIUtils/CLI11
#     GIT_TAG a66ae4145779c56dc0f9f98a631656417dd77de8 # v2.3.0
# )
# FetchContent_MakeAvailable(cli11)
target_link_libraries(lla-cli PUBLIC CLI11::CLI11)
