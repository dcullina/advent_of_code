#include "part_1.hpp"

#include <cassert>
#include <charconv>
#include <format>
#include <print>
#include <ranges>
#include <string>

std::string part_1(const std::string &input) {

  std::vector<std::string> lines;
  for (auto &&line_range : input | std::views::split('\n')) {
    lines.emplace_back(&*line_range.begin(),
                       static_cast<size_t>(std::ranges::distance(line_range)));
  }

  std::vector<std::int64_t> times = parse_line(lines[0]);
  std::vector<std::int64_t> distances = parse_line(lines[1]);
  assert(times.size() == distances.size());

  // std::println("Times: {}", times);
  // std::println("Distances: {}", distances);

  std::int64_t number_of_ways = 1;

  for (size_t race = 0; race < times.size(); race++) {
    number_of_ways *= process_race(times[race], distances[race]);
  }

  return std::to_string(number_of_ways);
} // part_1

std::int64_t process_race(std::int64_t time, std::int64_t distance_to_beat) {

  std::int64_t first = find_first(time, distance_to_beat);
  if (first < 0) {
    throw std::runtime_error(std::format(
        "Cannot find any way to beat distance_to_beat={} using time={}",
        distance_to_beat, time));
  }

  int64_t last = time - first;

  return last - first + 1;
} // process_race

std::int64_t find_first(std::int64_t time, std::int64_t distance_to_beat) {

  for (std::int64_t button_time = 0; button_time <= time; button_time++) {
    if (compute_distance(time, button_time) > distance_to_beat) {
      return button_time;
    }
  }

  return -1;

} // find_first

std::int64_t compute_distance(std::int64_t total_time,
                              std::int64_t button_time) {

  if (button_time > total_time) {
    throw std::runtime_error(
        std::format("Cannot have button_time={} longer than total_time={}",
                    button_time, total_time));
  }

  if (button_time < 0) {
    throw std::runtime_error(
        std::format("button_time={} cannot be negative", button_time));
  }

  std::int64_t run_time = total_time - button_time;
  std::int64_t distance = run_time * button_time;

  return distance;
} // compute_distance

std::vector<std::int64_t> parse_line(std::string_view line) {

  if (line.empty()) {
    throw std::runtime_error("Line is empty");
  }

  auto colon_pos = line.find(':');
  if (colon_pos == std::string_view::npos) {
    throw std::runtime_error("Malformed line: missing ':'");
  }

  std::string_view numbers = line.substr(colon_pos + 1);

  return parse_numbers(numbers);
} // parse_line

std::vector<std::int64_t> parse_numbers(std::string_view numbers_string_view) {

  for (char c : numbers_string_view) {
    if (!(std::isdigit(static_cast<unsigned char>(c)) ||
          std::isspace(static_cast<unsigned char>(c)))) {
      throw std::runtime_error(
          std::format("Invalid character in input: {}", c));
    }
  }

  std::vector<std::int64_t> numbers;

  size_t pos = 0;
  while (pos < numbers_string_view.size()) {
    while (pos < numbers_string_view.size() && numbers_string_view[pos] == ' ')
      ++pos;
    if (pos >= numbers_string_view.size())
      break;

    size_t end = pos;
    while (end < numbers_string_view.size() && numbers_string_view[end] != ' ')
      ++end;

    std::int64_t value{};
    auto [ptr, ec] = std::from_chars(numbers_string_view.data() + pos,
                                     numbers_string_view.data() + end, value);

    if (ec == std::errc{}) {
      numbers.push_back(value);
    }

    pos = end;
  }

  return numbers;
} // parse_numbers