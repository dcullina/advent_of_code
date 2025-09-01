add_library(day01_lib
    part_1.cpp
    part_2.cpp
)
target_include_directories(day01_lib
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(day01_lib
    PUBLIC
        aoc_cpp_lib
)

add_executable(day01
    main.cpp
)
target_link_libraries(day01
    PRIVATE
        day01_lib
)

add_executable(test_day01
    tests/test_day01.cpp
)
target_link_libraries(test_day01
    PRIVATE
        day01_lib
        GTest::gtest_main
)

add_test(NAME day01 COMMAND test_day01)
