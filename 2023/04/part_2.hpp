#pragma once
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

std::string part_2(const std::string &input);
int process_line(std::string_view line, std::unordered_map<int, int> &card_copies);
int parse_card_number(std::string_view line);
std::unordered_set<int> parse_winning_numbers(std::string_view line);
std::vector<int> parse_player_numbers(std::string_view line);