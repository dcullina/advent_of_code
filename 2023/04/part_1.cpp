#include "part_1.hpp"

#include <ranges>
#include <unordered_set>
#include <charconv>
#include <print>

std::string part_1(const std::string &input) {

  int sum = 0;

  auto lines = input
    | std::views::split('\n')
    | std::views::filter([](auto&& line) { return !std::ranges::empty(line); });

  for (const auto &line : lines) {
    if (line.empty()) continue;
    sum += process_line(line);
  }

  return std::to_string(sum);
}

int process_line(const auto &line_range) {

  auto numbers = line_range
    | std::views::drop_while([](char c){ return c != ':'; })
    | std::views::drop(1)
    | std::views::split('|');

  auto parse_numbers = [](auto &&rng) {
    std::unordered_set<int> out;
    for (auto&& token : rng | std::views::split(' ')) {
      if (token.empty()) continue;
        int value{};
        auto sv = std::string_view(token.begin(), token.end());
        std::from_chars(sv.data(), sv.data() + sv.size(), value);
        out.insert(value);
    }
    return out;
  };

  auto it = numbers.begin();
  auto winning_numbers = parse_numbers(*it);

  int sum = 0;
  for (auto&& token : *std::next(it) | std::views::split(' ')) {
    if (token.empty()) continue;
    int value{};
    auto sv = std::string_view(token.begin(), token.end());
    std::from_chars(sv.data(), sv.data() + sv.size(), value);
    if (!winning_numbers.contains(value)) continue;
    sum = std::max(1, sum * 2);
  }

  // std::println("Sum: {}", sum);
  return sum;
}