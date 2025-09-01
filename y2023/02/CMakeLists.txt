add_library(day02_lib
    part_1.cpp
    part_2.cpp
)
target_include_directories(day02_lib
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(day02_lib
    PUBLIC
        aoc_cpp_lib
)

add_executable(day02
    main.cpp
)
target_link_libraries(day02
    PRIVATE
        day02_lib
)

add_executable(test_day02
    tests/test_day02.cpp
)
target_link_libraries(test_day02
    PRIVATE
        day02_lib
        GTest::gtest_main
)

add_test(NAME day02 COMMAND test_day02)
