#include "graphs/graph.hpp"

void Graph::add_edge(int u, int v) {
  adj_[u].push_back(v);
  adj_[v].push_back(u);
}

const std::vector<int> &Graph::neighbors(int u) const { return adj_.at(u); }
