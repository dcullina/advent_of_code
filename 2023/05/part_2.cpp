#include "part_2.hpp"
#include "part_1.hpp"

#include <algorithm>
#include <print>
#include <ranges>
#include <string>

std::string part_2(const std::string &input) {

  std::vector<std::string> lines;
  for (auto &&line_range : input | std::views::split('\n')) {
    lines.emplace_back(line_range.begin(), line_range.end());
    // lines.emplace_back(&*line_range.begin(),
    //  static_cast<size_t>(std::ranges::distance(line_range)));
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
    if (colon == std::string::npos) {
      continue;
    }

    map_vector.push_back(parse_mapping_entry(lines, i));
  }

  if (seeds.size() % 2 != 0) {
    throw std::runtime_error(std::format(
        "Invalid pairing of seed & range, found an odd # of seeds + ranges: {}",
        seeds.size()));
  }

  int counter = 0;
  std::int64_t minimum = std::numeric_limits<std::int64_t>::max();

  for (size_t i = 0; i < seeds.size();
       i += 2) { // skip every other seed since it will be the range
    std::int64_t range = seeds[i + 1];

    // std::println("Starting process_seed_2 with side_a={} and side_b={}",
    //              seeds[i], seeds[i] + range - 1);
    process_seed_2(map_vector, minimum, size_t(0), seeds[i],
                   seeds[i] + range - 1);

    // std::println("Finished {} seed pair", ++counter);
  }

  return std::to_string(minimum);
} // part_2

void process_seed_2(const std::vector<std::vector<Mapping>> &map_vector,
                    std::int64_t &minimum, size_t depth, std::int64_t side_a,
                    std::int64_t side_b) {

  auto low_side = std::min(side_a, side_b);
  auto high_side = std::max(side_a, side_b);

  if (depth >= map_vector.size()) {
    minimum = std::min(minimum, low_side);
    return;
  }

  std::vector<std::pair<std::int64_t, std::int64_t>> covered_ranges;

  for (const auto &mapping : map_vector[depth]) {

    std::int64_t source_high_end = mapping.source + mapping.range - 1;

    if ((low_side > source_high_end) || (high_side < (mapping.source))) {
      continue;
    }

    std::int64_t overlap_low = std::max(low_side, mapping.source);
    std::int64_t overlap_high = std::min(high_side, source_high_end);

    std::int64_t destination_low_end =
        mapping.destination + (overlap_low - mapping.source);
    std::int64_t destination_high_end =
        mapping.destination + (overlap_high - mapping.source);

    // std::println("Going on to depth={} with low_dest={} and high_dest={}",
    //              depth, destination_low_end, destination_high_end);

    covered_ranges.emplace_back(overlap_low, overlap_high);
    process_seed_2(map_vector, minimum, depth + 1, destination_low_end,
                   destination_high_end);
  }

  std::int64_t new_low = low_side;
  std::sort(covered_ranges.begin(), covered_ranges.end());
  for (auto [covered_low, covered_high] : covered_ranges) {
    if (new_low < covered_low) {
      process_seed_2(map_vector, minimum, depth + 1, new_low, covered_low - 1);
    }
    new_low = covered_high + 1;
  }

  if (new_low <= high_side) {
    process_seed_2(map_vector, minimum, depth + 1, new_low, high_side);
  }
} // process_seed_2