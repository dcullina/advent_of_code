#pragma once
#include <charconv>
#include <format>
#include <stdexcept>
#include <string>

const size_t HAND_SIZE = 5;

enum class CamelCard : int {
  Joker = 1,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace
}; // CamelCard

inline CamelCard char_to_card(char c, bool use_jokers) {
  switch (c) {
  case 'J':
    return use_jokers ? CamelCard::Joker : CamelCard::Jack;
  case '2':
    return CamelCard::Two;
  case '3':
    return CamelCard::Three;
  case '4':
    return CamelCard::Four;
  case '5':
    return CamelCard::Five;
  case '6':
    return CamelCard::Six;
  case '7':
    return CamelCard::Seven;
  case '8':
    return CamelCard::Eight;
  case '9':
    return CamelCard::Nine;
  case 'T':
    return CamelCard::Ten;
  case 'Q':
    return CamelCard::Queen;
  case 'K':
    return CamelCard::King;
  case 'A':
    return CamelCard::Ace;
  default:
    throw std::runtime_error(std::format("Invalid card character: c={}", c));
  }
} // char_to_card

constexpr auto operator<=>(CamelCard lhs, CamelCard rhs) {
  return static_cast<int>(lhs) <=> static_cast<int>(rhs);
}

enum class HandClassification : int {
  HighCard = 1,
  OnePair,
  TwoPair,
  ThreeOfKind,
  FullHouse,
  FourOfKind,
  FiveOfKind
}; // HandClassification

constexpr auto operator<=>(HandClassification lhs, HandClassification rhs) {
  return static_cast<int>(lhs) <=> static_cast<int>(rhs);
}

// big assumption: no two Hand's will be the same since sorting is done by
// cards' ranking against one another. Any two Hand's with the same cards could
// result in undefined behavior if one Hand is placed before another since the
// final calculation depends on the wager * rank.
struct Hand {
  std::array<CamelCard, HAND_SIZE> cards;
  int wager;
  bool jokers;

  Hand() = delete;

  explicit Hand(std::string_view input, bool use_jokers) : jokers(use_jokers) {

    if (input.empty())
      throw std::runtime_error("Line is empty, cannot construct Hand");

    auto space = input.find(' ');
    if (space == std::string_view::npos) {
      throw std::runtime_error("Malformed line, ' ' character not found");
    }

    for (size_t i = 0; i < HAND_SIZE; i++) {
      cards[i] = char_to_card(input[i], jokers);
    }

    auto wager_substr = input.substr(space + 1);
    auto [ptr, ec] =
        std::from_chars(wager_substr.begin(), wager_substr.end(), wager);

    if (ec != std::errc{}) {
      throw std::runtime_error(
          std::format("Failed to parse wager number: wager={}", wager));
    }

    classification = evaluate_classification();
  }

  auto operator<=>(const Hand &other) const {
    if (auto cmp = classification <=> other.classification; cmp != 0) {
      return cmp;
    }

    return cards <=> other.cards;
  }

private:
  HandClassification classification;
  HandClassification evaluate_classification() const;
}; // Hand

std::string part_1(const std::string &input);