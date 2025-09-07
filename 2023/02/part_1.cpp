#include "part_1.hpp"

#include <charconv>
#include <ranges>

int get_id_from_line(const std::string &line) {

  constexpr std::string_view game_keyword = "Game ";

  size_t game_key_length = game_keyword.size();

  size_t colon_pos = line.find(':', game_key_length);

  std::string_view id_view(line.data() + game_key_length,
                           colon_pos - game_key_length);

  int id = 0;
  std::from_chars(id_view.begin(), id_view.end(), id);

  return id;
}

bool check_if_possible(ColorCounts const &current_count) {

  for (auto const &[color, limit] : COLOR_LIMITS) {

    auto index = static_cast<size_t>(color);

    if (current_count[index] > limit)
      return false;
  }

  return true;
}

ColorCounts parse_colors(const std::string &line) {
  ColorCounts counts{0, 0, 0};

  auto extract_number_before = [&](size_t position) {
    int value = 0;
    while (position > 0 && std::isspace(line[position - 1]))
      --position;
    size_t end = position;
    while (position > 0 && std::isdigit(line[position - 1]))
      --position;
    value = std::stoi(line.substr(position, end - position));
    return value;
  };

  for (auto const &[color_str, color] : COLOR_MAP) {

    size_t position = line.find(color_str);

    while (position != std::string::npos) {

      int value = extract_number_before(position);

      counts[static_cast<size_t>(color)] =
          std::max(value, counts[static_cast<size_t>(color)]);

      position = line.find(color_str, position + color_str.size());
    }
  }

  return counts;
}

std::string part_1(const std::string &input) {

  int sum = 0;

  auto lines =
      input | std::views::split('\n') | std::views::transform([](auto &&r) {
        return std::string(r.begin(), r.end());
      });

  for (auto const &line : lines) {

    if (line.empty())
      continue;

    ColorCounts counts = parse_colors(line);
    int id = get_id_from_line(line);

    if (check_if_possible(counts))
      sum += id;
  }

  return std::to_string(sum);
}
