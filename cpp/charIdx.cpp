#include <chrono>
#include <iostream>
#include <string>
using namespace std;

int charIndex(string str, char s) {
    int index;
    for (int i = 0; i < str.length(); i++) {
        if (str.at(i) == s) {
            index = i;
            break;
        }
    }
    return index;
}

string charIndexEfficient(string str, char s) {
    size_t index = str.find(s);
    if (index == string::npos) {
        return "Character '" + string(1, s) + "' not found in the string.";
    }
    return "The index of '" + string(1, s) + "' is at " + to_string(index);
}

int main() {
    string str;
    char s;

    cout << "Enter the string: ";
    cin >> str;
    cout << "Enter the character to find its index: ";
    cin >> s;

    // Execution time for charIndex
    auto start_time_charIndex = chrono::high_resolution_clock::now();
    int idx = charIndex(str, s);
    auto end_time_charIndex = chrono::high_resolution_clock::now();
    double elapsed_time_charIndex =
        chrono::duration_cast<chrono::microseconds>(end_time_charIndex -
                                                    start_time_charIndex)
            .count() /
        1e6;

    // Execution time for charIndexEfficient
    auto start_time_efficient = chrono::high_resolution_clock::now();
    string result = charIndexEfficient(str, s);
    auto end_time_efficient = chrono::high_resolution_clock::now();
    double elapsed_time_efficient =
        chrono::duration_cast<chrono::microseconds>(end_time_efficient -
                                                    start_time_efficient)
            .count() /
        1e6;

    cout << "charIndex: => The index of " << s << " is at " << idx << endl;
    cout << "charIndex time: " << elapsed_time_charIndex << " milliseconds"
         << endl;
    cout << "charIndexEfficient: => " << result << endl;
    cout << "charIndexEfficient time: " << elapsed_time_efficient
         << " milliseconds" << endl;

    return 0;
}
