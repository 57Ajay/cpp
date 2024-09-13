#include <iostream>
#include <vector>
using namespace std;

int alter_sum(const vector<int> &numbers) {
  int total_sum = 0;
  for (int i = 0; i < numbers.size(); i++) {
    if (i % 2 == 0) {
      total_sum += numbers[i];
    } else {
      total_sum -= numbers[i];
    };
  };
  return total_sum;
};

int main() {
  int t;
  cin >> t;
  while (t--) {
    int n;
    cin >> n;
    vector<int> numbers(n);
    for (int i = 0; i < n; i++) {
      cin >> numbers[i];
    };
    cout << alter_sum(numbers) << endl;
  }
  return 0;
}
