#pragma once
#include "part_1.hpp"
#include <string>

std::string part_2(const std::string &input);
void process_seed_2(const std::vector<std::vector<Mapping>> &map_vector,
                    std::int64_t &minimum, size_t depth, std::int64_t side_a,
                    std::int64_t side_b);