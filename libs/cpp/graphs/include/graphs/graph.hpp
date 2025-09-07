#pragma once
#include <format>
#include <string>
#include <unordered_set>
#include <vector>

enum class Direction {
  UpLeft,
  Up,
  UpRight,
  Left,
  Right,
  DownLeft,
  Down,
  DownRight
};

struct Coord {
  int row;
  int col;

  auto operator<=>(const Coord &) const = default;

  Coord operator+(const Coord &other) const {
    return {row + other.row, col + other.col};
  }

  Coord operator+(Direction dir) const {
    switch (dir) {
    case Direction::UpLeft:
      return {row - 1, col - 1};
    case Direction::Up:
      return {row - 1, col};
    case Direction::UpRight:
      return {row - 1, col + 1};
    case Direction::Left:
      return {row, col - 1};
    case Direction::Right:
      return {row, col + 1};
    case Direction::DownLeft:
      return {row + 1, col - 1};
    case Direction::Down:
      return {row + 1, col};
    case Direction::DownRight:
      return {row + 1, col + 1};
    }
    return *this;
  }
};

struct CoordHash {
  std::size_t operator()(const Coord &c) const noexcept {
    return (static_cast<std::size_t>(c.row) << 32) ^
           static_cast<std::size_t>(c.col);
  }
};

struct CoordEq {
  bool operator()(const Coord &a, const Coord &b) const noexcept {
    return a.row == b.row && a.col == b.col;
  }
};
using CoordSet = std::unordered_set<Coord, CoordHash, CoordEq>;

template <> struct std::formatter<Coord> : std::formatter<std::string> {
  auto format(const Coord &c, std::format_context &ctx) const {
    return std::formatter<std::string>::format(
        std::format("({}, {})", c.row, c.col), ctx);
  }
};

template <>
struct std::formatter<std::vector<Coord>> : std::formatter<std::string> {
  auto format(const std::vector<Coord> &vec, std::format_context &ctx) const {
    std::string output = "[";
    for (size_t i = 0; i < vec.size(); i++) {
      output += std::format("{}", vec[i]);
      if (i + 1 < vec.size())
        output += ", ";
    }
    output += "]";
    return std::formatter<std::string>::format(output, ctx);
  }
};

template <> struct std::formatter<CoordSet> : std::formatter<std::string> {
  auto format(const CoordSet &set, std::format_context &ctx) const {
    std::string output = "{";
    size_t i = 0;
    for (Coord const &c : set) {
      output += std::format("{}", c);
      if (++i < set.size())
        output += ", ";
    }
    output += "}";
    return std::formatter<std::string>::format(output, ctx);
  }
};

struct EmptySet {
  std::unordered_set<char> chars;
};

struct ObstructionSet {
  std::unordered_set<char> chars;
};

class AsciiGraph {
public:
  AsciiGraph(std::string input, EmptySet empty, ObstructionSet obstructions);

  [[nodiscard]] bool in_bounds(Coord c) const;
  [[nodiscard]] bool is_empty(Coord c) const;
  [[nodiscard]] bool is_obstruction(Coord c) const;

  [[nodiscard]] CoordSet quad_neighbors(Coord c) const;
  [[nodiscard]] CoordSet all_neighbors(Coord c) const;

  [[nodiscard]] const std::vector<std::string> &rows() const { return graph_; };

  template <typename Func> void for_each_coord(Func f) const {
    for (int row = 0; row < static_cast<int>(graph_.size()); row++) {
      for (int col = 0; col < static_cast<int>(graph_.size()); col++) {
        f(Coord{row, col}, graph_[row][col]);
      }
    }
  }

  size_t height() const { return graph_.size(); }
  size_t width() const { return graph_.empty() ? 0 : graph_[0].size(); }
  char at(Coord c) const { return graph_[c.row][c.col]; }

private:
  std::vector<std::string> graph_;
  EmptySet empty_;
  ObstructionSet obstructions_;

  static std::vector<std::string> split_lines(const std::string &input);
};
