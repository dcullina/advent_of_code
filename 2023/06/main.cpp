#include "parsing/input.hpp"
#include "part_1.hpp"
#include "part_2.hpp"

#include <print>

namespace fs = std::filesystem;

int main(int argc, char *argv[]) {

  if (argc < 2) {
    std::println(stderr, "Usage: {} <input_file>", argv[0]);
    return 1;
  }

  std::string_view input_path{argv[1]};

  if (!fs::exists(input_path)) {
    std::println(stderr, "Error: file not found -> {}", input_path);
    return 2;
  }

  std::ifstream in{std::string(input_path)};
  if (!in.is_open()) {
    std::println(stderr, "Error: could not open file -> {}", input_path);
    return 3;
  }

  auto input = load_file(std::string(input_path));

  std::println("Part 1: {}", part_1(input));
  std::println("Part 2: {}", part_2(input));

  return 0;
}
