#include <iostream>
#include <string>
using namespace std;

int main() {
  cout << "This is  main file. ";
  string name;
  cin >> name;
  cout << name;
  int sum;
  for (int i = 0; i < 2732; i++) {
    sum += i;
  };
  cout << sum;
  return 0;
}
