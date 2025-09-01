#pragma once
#include <array>
#include <string>

constexpr auto NUMBER_MAP = std::to_array<std::pair<std::string_view, int>>({
    {"one", 1},
    {"two", 2},
    {"three", 3},
    {"four", 4},
    {"five", 5},
    {"six", 6},
    {"seven", 7},
    {"eight", 8},
    {"nine", 9},
    {"ten", 10},
});

std::string part_2(const std::string &input);
