#pragma once
#include <array>
#include <string>

enum class DiceColor { Red = 0, Green = 1, Blue = 2, Count };
using ColorCounts = std::array<int, static_cast<size_t>(DiceColor::Count)>;

std::string part_1(const std::string &input);
int get_id_from_line(const std::string &line);
bool check_if_possible(ColorCounts const &current_count);
ColorCounts parse_colors(const std::string &line);

constexpr auto COLOR_MAP =
    std::to_array<std::pair<std::string_view, DiceColor>>({
        {"red", DiceColor::Red},
        {"green", DiceColor::Green},
        {"blue", DiceColor::Blue},
    });

constexpr auto COLOR_LIMITS = std::to_array<std::pair<DiceColor, int>>({
    {DiceColor::Red, 12},
    {DiceColor::Green, 13},
    {DiceColor::Blue, 14},
});
