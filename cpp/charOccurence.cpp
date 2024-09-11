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

int wordOcr(string str, string word) {
    int count = 0;
    size_t pos = str.find(word);

    while (pos != string::npos) {
        count++;
        pos = str.find(word, pos + word.length());
    }

    return count;
}

int main() {
    string str;
    string input;

    cout << "Please enter a word or a sentence: => \n";
    getline(cin, str);

    if (str.empty()) {
        cout << "Please enter a word or a sentence." << endl;
        return 1;
    }

    cout << "Please enter a character or a word: => \n";
    getline(cin, input);

    if (input.length() == 1) {
        char c = input[0];
        int result = charOcr(str, c);
        cout << "The character '" << c << "' occurs " << result
             << " times in the string." << endl;
    } else {
        int result = wordOcr(str, input);
        cout << "The word '" << input << "' occurs " << result
             << " times in the string." << endl;
    }

    return 0;
}
