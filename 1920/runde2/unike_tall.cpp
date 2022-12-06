#include <iostream>
#include <map>

int main() {
  int n;
  std::cin >> n;
  
  std::map<int, int> count;
  
  for (int i = 0; i < n; i++) {
    int x;
    std::cin >> x;
    
    count[x] += 1;
  }
  
  for (auto pair : count) {
    auto number = pair.first;
    auto c = pair.second;
    if (c == 1) {
      std::cout << number;
      return 0;
    }
  }
  
  std::cout << -1;
}
