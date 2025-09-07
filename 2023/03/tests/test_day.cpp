#include "parsing/input.hpp"
#include "part_1.hpp"
#include "part_2.hpp"

#include <gtest/gtest.h>

TEST(Day03, Part1) {
  std::string input = load_file("example_input.txt");
  EXPECT_EQ(part_1(input), "4361");
}

TEST(Day03, Part2) {
  std::string input = load_file("example_input.txt");
  EXPECT_EQ(part_2(input), "467835");
}
