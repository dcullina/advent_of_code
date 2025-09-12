#include "part_2.hpp"
#include "part_1.hpp"

#include <print>
#include <ranges>
#include <set>
#include <string>
#include <vector>

std::string part_2(const std::string &input) {

  std::vector<std::string> lines;
  for (auto &&line_range : input | std::views::split('\n')) {
    lines.emplace_back(&*line_range.begin(),
                       static_cast<size_t>(std::ranges::distance(line_range)));
  }

  std::multiset<Hand> hands;
  bool use_jokers = true;

  for (size_t line = 0; line < lines.size(); line++) {

    if (lines[line].empty())
      continue;

    hands.insert(Hand(lines[line], use_jokers));
  }

  int rank = 1;
  int sum = 0;
  for (auto hand : hands) {
    sum += (rank++) * hand.wager;
  }

  return std::to_string(sum);
} // part_2