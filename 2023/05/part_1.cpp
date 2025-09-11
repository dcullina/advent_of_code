#include "part_1.hpp"

#include <algorithm>
#include <charconv>
#include <cstdint>
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

  llVector seeds = parse_seeds(lines[0]);
  // std::println("Seeds {}", seeds);

  std::vector<std::vector<Mapping>> map_vector;

  for (size_t i = 1; i < lines.size(); ++i) {
    // if a line is empty we mark that we want to start a new entry for our
    // map_vector
    if (lines[i].empty()) {
      continue;
    }

    auto colon = lines[i].find(':');
    if (colon == std::string_view::npos) {
      continue;
    }

    map_vector.push_back(parse_mapping_entry(lines, i));
  }

  std::int64_t minimum = std::numeric_limits<std::int64_t>::max();

  for (const auto &seed : seeds) {
    minimum = std::min(minimum, process_seed_1(map_vector, seed));
  }

  return std::to_string(minimum);
} // part_1

std::int64_t process_seed_1(const std::vector<std::vector<Mapping>> &map_vector,
                            std::int64_t seed) {

  for (const auto &map_entry : map_vector) {
    for (const auto &mapping : map_entry) {
      if ((seed < mapping.source) ||
          (seed > (mapping.source + mapping.range - 1))) {
        continue;
      }
      std::int64_t offset = seed - mapping.source;
      seed = mapping.destination + offset;
      break;
    }
  }

  return seed;
} // process_seed_1

llVector parse_seeds(std::string_view seed_line) {
  llVector seeds;

  auto colon_pos = seed_line.find(':');
  if (colon_pos == std::string_view::npos) {
    throw std::runtime_error("Malformed line: missing ':'");
  }

  std::string_view numbers = seed_line.substr(colon_pos + 1);

  return parse_numbers(numbers);
} // parse_seeds

std::vector<Mapping> parse_mapping_entry(std::vector<std::string> lines,
                                         int line_number) {
  std::vector<Mapping> entry_mappings;

  for (size_t i = line_number + 1; i < lines.size(); ++i) {
    if (lines[i].empty())
      break;

    auto colon = lines[i].find(':');
    if (colon != std::string_view::npos) {
      continue;
    }

    llVector numbers = parse_numbers(lines[i]);

    if (numbers.size() != 3) {
      // std::println("Line: {}", lines[i]);
      throw std::runtime_error(std::format(
          "Invalid parsing of mapping line, found {} numbers", numbers.size()));
    }

    entry_mappings.push_back(Mapping{
        .destination = numbers[0],
        .source = numbers[1],
        .range = numbers[2],
    });
  }

  return entry_mappings;
} // parse_mapping_entry

llVector parse_numbers(std::string_view numbers_string_view) {

  for (char c : numbers_string_view) {
    if (!(std::isdigit(static_cast<unsigned char>(c)) ||
          std::isspace(static_cast<unsigned char>(c)))) {
      throw std::runtime_error(
          std::format("Invalid character in input: {}", c));
    }
  }

  llVector numbers;

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