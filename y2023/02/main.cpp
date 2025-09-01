#include "parsing/input.hpp"
#include "part_1.hpp"
#include "part_2.hpp"
#include <print>

int main() {

  std::string input = load_file("y2023/02/input.txt");

  std::println("Part 1: {}", part_1(input));
  std::println("Part 2: {}", part_2(input));

  return 0;
}
