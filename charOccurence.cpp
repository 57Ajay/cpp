#include <iostream>
#include <string>
using namespace std;

int charOcr(string str, char c) {
  int count = 0;
  for (int i = 0; i < str.length(); i++) {
    if (str[i] == c) {
      count++;
    }
  }
  return count;
}

int main() {
  string str;
  char c;

  cout << "Please enter a word or a sentence: => \n";
  getline(cin, str);

  if (str.empty()) {
    cout << "Please enter a word or a sentence." << endl;
    return 1;
  }

  cout << "Please enter a character: => \n";
  cin >> c;

  if (cin.fail() || cin.peek() != '\n') {
    cout << "Please enter a single character." << endl;
    return 1;
  }

  int result = charOcr(str, c);
  cout << "The character '" << c << "' occurs " << result
       << " times in the string." << endl;

  return 0;
}
