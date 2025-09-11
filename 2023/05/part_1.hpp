#pragma once
#include <format>
#include <string>
#include <vector>

struct Mapping {
  int64_t destination;
  int64_t source;
  int64_t range;
};

template <> struct std::formatter<Mapping> : std::formatter<std::string> {
  auto format(const Mapping &mapping, auto &ctx) const {
    return std::format_to(ctx.out(), "d: {}, s: {}, r: {}", mapping.destination,
                          mapping.source, mapping.range);
  }
};

using llVector = std::vector<std::int64_t>;
std::string part_1(const std::string &input);
std::int64_t process_seed_1(const std::vector<std::vector<Mapping>> &map_vector,
                            std::int64_t seed);
llVector parse_seeds(std::string_view seed_line);
std::vector<Mapping> parse_mapping_entry(std::vector<std::string> input,
                                         int line_number);
llVector parse_numbers(std::string_view numbers_string_view);