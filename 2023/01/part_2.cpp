#include "part_2.hpp"

#include <ranges>
#include <vector>

std::string part_2(const std::string &input) {

  int sum = 0;

  auto lines =
      input | std::views::split('\n') | std::views::transform([](auto &&r) {
        return std::string(r.begin(), r.end());
      });

  for (auto const &line : lines) {

    std::vector<int> detected_numbers;

    for (size_t i = 0; i < line.size(); i++) {
      char current_character = line[i];

      if (std::isdigit(static_cast<unsigned char>(current_character))) {
        detected_numbers.push_back(current_character - '0');
      }

      for (auto const &[word, value] : NUMBER_MAP) {
        if (line.compare(i, word.size(), word) == 0) {
          detected_numbers.push_back(value);
          break;
        }
      }
    }

    if (!detected_numbers.empty()) {
      sum += detected_numbers.front() * 10 + detected_numbers.back();
    }
  }

  return std::to_string(sum);
}
