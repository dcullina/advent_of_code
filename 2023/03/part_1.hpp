#pragma once
#include <string>
#include <unordered_set>

std::string part_1(const std::string &input);

const std::unordered_set<char> SPECIAL_SYMBOLS = {
    '#', '$', '%', '&', '*', '+', '-', '/', '=', '@',
};
