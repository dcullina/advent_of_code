#include "part_2.hpp"

#include <charconv>
#include <ranges>
#include <stdexcept>

std::string part_2(const std::string &input) {

  int sum = 0;

  std::vector<std::string> lines;
  for (auto &&line_range : input | std::views::split('\n')) {
    lines.emplace_back(&*line_range.begin(),
                       static_cast<size_t>(std::ranges::distance(line_range)));
  }

  std::unordered_map<int, int> card_copies;
  int highest_original_card = 0;
  for (const std::string &line : lines) {
    if (line.empty())
      continue;
    highest_original_card = process_line(line, card_copies);
  }

  for (auto &[card_number, num_copies] : card_copies) {
    if (card_number > highest_original_card) continue;
    sum += num_copies;
  }

  return std::to_string(sum);
}

int process_line(std::string_view line,
                  std::unordered_map<int, int> &card_copies) {

  int card_number = parse_card_number(line);
  card_copies[card_number] += 1;

  std::unordered_set<int> winning_numbers = parse_winning_numbers(line);
  std::vector<int> player_numbers = parse_player_numbers(line);

  int future_card_number = card_number;
  for (int player_number : player_numbers) {
    if (!winning_numbers.contains(player_number)) continue;
    card_copies[++future_card_number] += card_copies[card_number];
  }

  return card_number;
}

std::unordered_set<int> parse_winning_numbers(std::string_view line) {
  std::unordered_set<int> winning_numbers;

  auto colon_pos = line.find(':');
  auto pipe_pos = line.find('|', colon_pos);
  if (colon_pos == std::string_view::npos ||
      pipe_pos == std::string_view::npos) {
    throw std::runtime_error("Malformed line: missing ':' or '|'");
  }

  std::string_view numbers =
      line.substr(colon_pos + 1, pipe_pos - (colon_pos + 1));

  size_t pos = 0;
  while (pos < numbers.size()) {
    while (pos < numbers.size() && numbers[pos] == ' ')
      ++pos;
    if (pos >= numbers.size())
      break;

    size_t end = pos;
    while (end < numbers.size() && numbers[end] != ' ')
      ++end;

    int value{};
    auto [ptr, ec] =
        std::from_chars(numbers.data() + pos, numbers.data() + end, value);

    if (ec == std::errc{}) {
      winning_numbers.insert(value);
    }

    pos = end;
  }

  return winning_numbers;
}

std::vector<int> parse_player_numbers(std::string_view line) {
  std::vector<int> player_numbers;

  auto pipe_pos = line.find('|');
  if (pipe_pos == std::string_view::npos) {
    throw std::runtime_error("Malformed line: missing '|'");
  }

  std::string_view numbers =
      line.substr(pipe_pos + 1);

  size_t pos = 0;
  while (pos < numbers.size()) {
    while (pos < numbers.size() && numbers[pos] == ' ')
      ++pos;
    if (pos >= numbers.size())
      break;

    size_t end = pos;
    while (end < numbers.size() && numbers[end] != ' ')
      ++end;

    int value{};
    auto [ptr, ec] =
        std::from_chars(numbers.data() + pos, numbers.data() + end, value);

    if (ec == std::errc{}) {
      player_numbers.push_back(value);
    }

    pos = end;
  }

  return player_numbers;
}

int parse_card_number(std::string_view line) {
  constexpr std::string_view prefix = "Card ";

  auto pos = line.find(prefix);
  if (pos == std::string_view::npos) {
    throw std::runtime_error("No 'Card ' prefix found");
  }

  auto start = pos + prefix.size();
  auto end = line.find(':', start);
  if (end == std::string_view::npos) {
    throw std::runtime_error("No ':' character found");
  }

  std::string_view card_number = line.substr(start, end - start);

  while (!card_number.empty() && card_number.front() == ' ')
    card_number.remove_prefix(1);
  while (!card_number.empty() && card_number.back() == ' ')
    card_number.remove_suffix(1);

  int card_number_int{};
  auto [ptr, ec] =
      std::from_chars(card_number.begin(), card_number.end(), card_number_int);
  if (ec != std::errc{}) {
    throw std::runtime_error("Failed to parse card number");
  }

  return card_number_int;
}