foreach(day RANGE 1 24)
    if(day LESS 10)
        set(day_padded "0${day}")
    else()
        set(day_padded "${day}")
    endif()

    set(day_dir ${CMAKE_CURRENT_SOURCE_DIR}/${day_padded})

    if(EXISTS ${day_dir})
        aoc_add_day(${day_dir})
    endif()
endforeach()
