add_library(expected INTERFACE)
target_include_directories(expected INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/include)
add_library(external::expected ALIAS expected)