cmake_minimum_required(VERSION 4.1.1)
project(AdventOfCode LANGUAGES CXX)

set(CMAKE_C_COMPILER ${CC})
set(CMAKE_CXX_COMPILER ${CXX})

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON CACHE INTERNAL "")

include(FetchContent)
FetchContent_Declare(
    googletest
    GIT_REPOSITORY https://github.com/google/googletest.git
    GIT_TAG v1.17.0
)
FetchContent_MakeAvailable(googletest)

include(cmake/AocDay.cmake)

enable_testing()

add_compile_options(
    -Wall
    -Wextra
    -Wpedantic
    # -Werror
    -Wno-error=unused-variable
    -Wno-error=unused-parameter
)

add_subdirectory(libs/cpp)

add_subdirectory(2023)
add_subdirectory(tests)
