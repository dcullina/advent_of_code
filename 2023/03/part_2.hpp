#pragma once
#include "graphs/graph.hpp"

#include <string>
#include <unordered_map>

const char GEAR_SYMBOL = '*';

std::string part_2(const std::string &input);

using CoordMap =
    std::unordered_map<Coord, std::vector<int>, CoordHash, CoordEq>;
