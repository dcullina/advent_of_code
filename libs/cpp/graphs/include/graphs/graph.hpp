#pragma once
#include <unordered_map>
#include <vector>

class Graph {
public:
  void add_edge(int u, int v);

  const std::vector<int> &neighbors(int u) const;

private:
  std::unordered_map<int, std::vector<int>> adj_;
};
