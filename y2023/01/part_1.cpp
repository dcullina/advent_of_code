#include "part_1.hpp"
#include <ranges>

std::string part_1(const std::string &input) {

  int sum = 0;

  auto lines =
      input | std::views::split('\n') | std::views::transform([](auto &&r) {
        return std::string(r.begin(), r.end());
      });

  for (auto const &line : lines) {
    for (char current_character : line) {
      if (std::isdigit(static_cast<unsigned char>(current_character))) {
        sum += (current_character - '0') * 10;
        break;
      }
    }

    for (char current_character : line | std::views::reverse) {
      if (std::isdigit(static_cast<unsigned char>(current_character))) {
        sum += (current_character - '0');
        break;
      }
    }
  }

  return std::to_string(sum);
}
