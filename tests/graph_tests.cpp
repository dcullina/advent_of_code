#include "graphs/graph.hpp"
#include <gtest/gtest.h>

TEST(GraphTest, AddEdge) {
  Graph g;
  g.add_edge(1, 2);
  auto neighbors = g.neighbors(1);
  EXPECT_EQ(neighbors.size(), 1);
  EXPECT_EQ(neighbors[0], 2);
}
