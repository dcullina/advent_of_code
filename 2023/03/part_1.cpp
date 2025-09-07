#include "part_1.hpp"
#include "graphs/graph.hpp"

#include <print>

std::string part_1(const std::string &input) {

  int sum = 0;

  AsciiGraph input_graph(input, EmptySet{{'.'}}, ObstructionSet{});

  bool new_number = true;

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

    for (Coord const &neighbor_coord : neighbors) {
      char neighbor_char = input_graph.at(neighbor_coord);
      if (SPECIAL_SYMBOLS.contains(neighbor_char)) {
        sum += part_number;
        new_number = false;
        break;
      }
    }
  });

  return std::to_string(sum);
}
