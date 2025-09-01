add_library(aoc_cpp_lib
    graphs/src/graph.cpp
    parsing/src/input.cpp
)

target_include_directories(aoc_cpp_lib PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/graphs/include
    ${CMAKE_CURRENT_SOURCE_DIR}/parsing/include
)
