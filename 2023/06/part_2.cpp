#include "part_2.hpp"
#include "part_1.hpp"

#include <charconv>
#include <print>
#include <ranges>
#include <stdexcept>
#include <string>

std::string part_2(const std::string &input) {
  std::vector<std::string> lines;
  for (auto &&line_range : input | std::views::split('\n')) {
    lines.emplace_back(&*line_range.begin(),
                       static_cast<size_t>(std::ranges::distance(line_range)));
  }

  std::int64_t time = parse_line_2(lines[0]);
  std::int64_t distance = parse_line_2(lines[1]);

  // std::println("Time: {}, Distance: {}", time, distance);

  std::int64_t number_of_ways = process_race(time, distance);
  return std::to_string(number_of_ways); // part_2
}

std::int64_t parse_line_2(std::string_view line) {

  if (line.empty()) {
    throw std::runtime_error("Line is empty");
  }

  auto colon_pos = line.find(':');
  if (colon_pos == std::string_view::npos) {
    throw std::runtime_error("Malformed line: missing ':'");
  }

  std::string_view number = line.substr(colon_pos + 1);

  return parse_numbers_2(number);
} // parse_line

std::int64_t parse_numbers_2(std::string_view numbers_string_view) {

  for (char c : numbers_string_view) {
    if (!(std::isdigit(static_cast<unsigned char>(c)) ||
          std::isspace(static_cast<unsigned char>(c)))) {
      throw std::runtime_error(
          std::format("Invalid character in input: {}", c));
    }
  }

  std::string digits;
  digits.reserve(numbers_string_view.size());

  for (char character : numbers_string_view) {
    if (character != ' ')
      digits.push_back(character);
  }

  std::int64_t number{};
  auto [ptr, ec] =
      std::from_chars(digits.data(), digits.data() + digits.size(), number);

  if (ec != std::errc{}) {
    throw std::runtime_error(std::format("Invalid number: {}", number));
  }

  // std::println("Parsed number: {}", number);
  return number;
} // parse_numbers