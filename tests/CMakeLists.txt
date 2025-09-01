add_executable(aoc_lib_tests
    graph_tests.cpp
    parsing_tests.cpp
)

target_link_libraries(aoc_lib_tests
    PRIVATE
        aoc_cpp_lib
        GTest::gtest_main
)

include(GoogleTest)
gtest_discover_tests(aoc_lib_tests
    PROPERTIES LABELS "lib-tests"
)
