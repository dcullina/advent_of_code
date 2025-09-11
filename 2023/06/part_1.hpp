#pragma once
#include <string>
#include <vector>

std::string part_1(const std::string &input);
std::int64_t process_race(std::int64_t time, std::int64_t distance_to_beat);
std::int64_t find_first(std::int64_t time, std::int64_t distance_to_beat);
std::int64_t compute_distance(std::int64_t total_time,
                              std::int64_t button_time);
std::vector<std::int64_t> parse_line(std::string_view line);
std::vector<std::int64_t> parse_numbers(std::string_view numbers_string_view);