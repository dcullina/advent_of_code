#include "part_1.hpp"

#include <algorithm>
#include <print>
#include <ranges>
#include <set>
#include <unordered_map>
#include <vector>

std::string part_1(const std::string &input) {

  std::vector<std::string> lines;
  for (auto &&line_range : input | std::views::split('\n')) {
    lines.emplace_back(&*line_range.begin(),
                       static_cast<size_t>(std::ranges::distance(line_range)));
  }

  std::multiset<Hand> hands;
  bool use_jokers = false;

  for (size_t line = 0; line < lines.size(); line++) {

    if (lines[line].empty())
      continue;

    hands.insert(Hand(lines[line], use_jokers));
  }

  int rank = 1;
  int sum = 0;
  for (auto hand : hands) {
    sum += (rank++) * hand.wager;
  }

  return std::to_string(sum);
} // part_1

HandClassification Hand::evaluate_classification() const {
  std::unordered_map<CamelCard, int> counts;

  for (auto card : cards) {
    counts[card]++;
  }

  std::vector<int> frequency;
  frequency.reserve(counts.size());
  for (auto [card, count] : counts) {
    if (jokers && card == CamelCard::Joker)
      continue;
    frequency.push_back(count);
  }
  std::sort(frequency.begin(), frequency.end(), std::greater<>());

  if (jokers && counts[CamelCard::Joker] > 0) {
    if (frequency.empty()) {
      frequency.push_back(counts[CamelCard::Joker]);
    } else {
      frequency[0] += counts[CamelCard::Joker];
    }
  }

  if (frequency[0] == 5) {
    return HandClassification::FiveOfKind;
  }

  if (frequency[0] == 4) {
    return HandClassification::FourOfKind;
  }

  if (frequency[0] == 3 && frequency.size() > 1 && frequency[1] == 2) {
    return HandClassification::FullHouse;
  }

  if (frequency[0] == 3) {
    return HandClassification::ThreeOfKind;
  }

  if (frequency[0] == 2 && frequency.size() > 1 && frequency[1] == 2) {
    return HandClassification::TwoPair;
  }

  if (frequency[0] == 2) {
    return HandClassification::OnePair;
  }

  return HandClassification::HighCard;
} // Hand::evaluate_classification