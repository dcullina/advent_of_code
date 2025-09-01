#include "parsing/input.hpp"
#include "part_1.hpp"
#include "part_2.hpp"
#include <gtest/gtest.h>

TEST(Day02, Part1) {
  std::string input = load_file("y2023/02/tests/example_input_1.txt");
  EXPECT_EQ(part_1(input), "8");
}

TEST(Day02, Part2) {
  std::string input = load_file("y2023/02/tests/example_input_2.txt");
  EXPECT_EQ(part_2(input), "-1");
}
