#include "part_2.hpp"
#include "graphs/graph.hpp"

#include <numeric>
#include <print>
#include <ranges>
#include <vector>

std::string part_2(const std::string &input) {

  int sum = 0;

  AsciiGraph input_graph(input, EmptySet{{'.'}}, ObstructionSet{});

  bool new_number = true;

  CoordMap gear_map;

  input_graph.for_each_coord([&](Coord coordinate, char character) {
    if (!std::isdigit(character)) {
      new_number = true;
      return;
    }

    if (!new_number) {
      return;
    }

    int part_number = character - '0';
    CoordSet neighbors = input_graph.all_neighbors(coordinate);

    Coord next_coordinate = coordinate + Direction::Right;

    while (input_graph.in_bounds(next_coordinate)) {
      char next_character = input_graph.at(next_coordinate);
      if (!std::isdigit(next_character)) {
        break;
      }
      neighbors.merge(input_graph.all_neighbors(next_coordinate));

      part_number *= 10;
      part_number += next_character - '0';

      next_coordinate = next_coordinate + Direction::Right;
    }

    new_number = false;

    for (Coord const &neighbor_coord : neighbors) {
      char neighbor_char = input_graph.at(neighbor_coord);

      if (neighbor_char != GEAR_SYMBOL)
        continue;

      gear_map[neighbor_coord].push_back(part_number);
    }
  });

  auto sum_view =
      gear_map | std::views::values |
      std::views::filter([](auto const &vec) { return vec.size() == 2; }) |
      std::views::transform([](auto const &vec) { return vec[0] * vec[1]; });

  sum = std::accumulate(sum_view.begin(), sum_view.end(), 0);

  return std::to_string(sum);
}
