function(aoc_add_day day_dir)
    get_filename_component(day_name ${day_dir} NAME)
    get_filename_component(year_dir ${day_dir} DIRECTORY)
    get_filename_component(year_name ${year_dir} NAME)

    set(PART1 ${day_dir}/part_1.cpp)
    set(PART2 ${day_dir}/part_2.cpp)
    set(MAIN ${day_dir}/main.cpp)

    if(EXISTS ${PART1} AND EXISTS ${PART2})
        add_library(${day_name}_lib
            ${PART1}
            ${PART2}
        )
        target_include_directories(${day_name}_lib PUBLIC ${day_dir})
        target_link_libraries(${day_name}_lib PUBLIC aoc_cpp_lib)

        if(EXISTS ${MAIN})
            add_executable(${day_name} ${MAIN})
            target_link_libraries(${day_name} PRIVATE ${day_name}_lib)

            set_target_properties(${day_name} PROPERTIES WORKING_DIRECTORY ${day_dir})
        endif()
            
        set(TEST_FILE ${day_dir}/tests/test_day.cpp)

        if(EXISTS ${TEST_FILE})
            add_executable(${day_name}_test ${TEST_FILE})
            target_link_libraries(${day_name}_test PRIVATE ${day_name}_lib GTest::gtest_main)

            add_test(NAME ${year_name}_${day_name} COMMAND ${day_name}_test)
            set_tests_properties(${year_name}_${day_name} PROPERTIES WORKING_DIRECTORY ${day_dir}/tests)
        endif()
    else()
        message(WARNING "Skipping day ${day_name} in year ${year_name}: part_1.cpp or part_2.cpp missing")
    endif()
endfunction()
