#include "part_2.hpp"
#include "part_1.hpp"

#include <ranges>

std::string part_2(const std::string &input) {

  int sum = 0;

  auto lines =
      input | std::views::split('\n') | std::views::transform([](auto &&r) {
        return std::string(r.begin(), r.end());
      });

  for (auto const &line : lines) {

    if (line.empty())
      continue;

    ColorCounts counts = parse_colors(line);

    int power = 1;

    for (size_t color = 0; color < static_cast<size_t>(DiceColor::Count);
         color++) {

      power *= counts[color];
    }

    sum += power;
  }

  return std::to_string(sum);
}
