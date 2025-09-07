#include "graphs/graph.hpp"
#include <array>

constexpr std::array<Direction, 4> QuadDirs{
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
};

constexpr std::array<Direction, 8> AllDirs{
    Direction::UpLeft, Direction::Up,        Direction::UpRight,
    Direction::Left,   Direction::Right,     Direction::DownLeft,
    Direction::Down,   Direction::DownRight,
};

std::vector<std::string> AsciiGraph::split_lines(const std::string &input) {

  std::vector<std::string> lines;

  std::string current;

  for (char character : input) {
    if (character == '\n') {
      lines.push_back(std::move(current));
    } else {
      current.push_back(character);
    }
  }

  if (!current.empty()) {
    lines.push_back(std::move(current));
  }

  return lines;
}

AsciiGraph::AsciiGraph(std::string input, EmptySet empty,
                       ObstructionSet obstructions)
    : graph_(split_lines(input)), empty_(std::move(empty)),
      obstructions_(std::move(obstructions)) {}

bool AsciiGraph::in_bounds(Coord c) const {
  return c.row >= 0 && c.row < static_cast<int>(height()) && c.col >= 0 &&
         c.col < static_cast<int>(width());
}

bool AsciiGraph::is_empty(Coord c) const {
  return in_bounds(c) && empty_.chars.contains(graph_[c.row][c.col]);
}

bool AsciiGraph::is_obstruction(Coord c) const {
  return in_bounds(c) && obstructions_.chars.contains(graph_[c.row][c.col]);
}

CoordSet AsciiGraph::quad_neighbors(Coord c) const {
  CoordSet result;

  for (auto direction : QuadDirs) {
    Coord next = c + direction;
    if (in_bounds(next) && !is_obstruction(next)) {
      result.insert(next);
    }
  }
  return result;
}

CoordSet AsciiGraph::all_neighbors(Coord c) const {
  CoordSet result;

  for (auto direction : AllDirs) {
    Coord next = c + direction;
    if (in_bounds(next) && !is_obstruction(next)) {
      result.insert(next);
    }
  }
  return result;
}
