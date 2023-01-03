#include <iostream>
#include <vector>
#include <queue>

struct node {
  int point;
  int distance;
  
  bool operator<(const node &rhs) const {
    if (distance > rhs.distance) {
      return true;
    } else if (distance < rhs.distance) {
      return false;
    } else {
      return point < rhs.point;
    }
  }
};

int main() {
  int n;
  std::cin >> n;
  
  std::vector<std::vector<int>> neighbours(n);
  for (int i = 0; i < n; i++) {
    int a, b, c;
    std::cin >> a >> b >> c;
    
    neighbours[i].push_back(a - 1);
    neighbours[i].push_back(b - 1);
    neighbours[i].push_back(c - 1);
    neighbours[i].push_back(i + 1);
  }
  
  std::vector<int> distance(n, INT_MAX);
  std::priority_queue<node> pq;
  distance[0] = 0;
  pq.push(node { 0, 0 });
  
  while (!pq.empty()) {
    auto top = pq.top();
    pq.pop();
    
    if (top.distance > distance[top.point]) {
      continue;
    }
    
    if (top.point == n - 1) {
      std::cout << top.distance << '\n';
      return 0;
    }
    
    for (auto neighbour : neighbours[top.point]) {
      if (top.distance + 1 < distance[neighbour]) {
        distance[neighbour] = top.distance + 1;
        pq.push(node { neighbour, top.distance + 1 });
      }
    }
  }
}
