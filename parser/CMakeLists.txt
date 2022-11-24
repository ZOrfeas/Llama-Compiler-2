add_library(lla-parser )
target_include_directories(lla-parser INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
add_library(LLAMAC::PARSER ALIAS lla-parser)

target_link_libraries(lla-parser PRIVATE LLAMAC::UTILS)
target_link_libraries(lla-parser PUBLIC fmt::fmt)
