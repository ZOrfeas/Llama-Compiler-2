add_library(lla-utils file-io.cpp)
target_include_directories(lla-utils INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
add_library(LLAMAC::UTILS ALIAS lla-utils)

target_link_libraries(lla-utils PRIVATE fmt::fmt)

