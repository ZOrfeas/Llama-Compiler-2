add_library(coro INTERFACE)
target_include_directories(coro INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/include)
add_library(external::coro ALIAS coro)
