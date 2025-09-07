default: build

configure:
    cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

build: configure
    cmake --build build

rebuild: clean build

test-lib: build
    ./build/tests/aoc_lib_tests

run year day: build
    ./build/{{year}}/{{day}} {{year}}/{{day}}/input.txt

test year day: build
    ctest --test-dir build -R {{year}}_{{day}} -V

clean:
    rm -rf ./build
