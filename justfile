default: build

configure:
    cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Debug

build:
    cmake --build build

rebuild: clean configure build

test-lib: build
    ./build/tests/aoc_lib_tests

run-day day: build
    ./build/y2023/{{day}}/day{{day}}

test-day day: build
    ./build/y2023/{{day}}/test_day{{day}}

clean:
    rm -rf ./build
