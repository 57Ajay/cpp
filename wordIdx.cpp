#include <iostream>
#include <string>
#include <string_view>
#include <chrono>

using namespace std;

pair<int, int> findWordIndex(const string& sentence, const string& word) {
    size_t startIndex = sentence.find(word);
    if (startIndex == string::npos) {
        return {-1, -1};
    }
    size_t endIndex = startIndex + word.length() - 1;
    return {static_cast<int>(startIndex), static_cast<int>(endIndex)};
}

pair<size_t, size_t> findWordIndexEfficient(string_view sentence, string_view word) {
    static pair<size_t, size_t> result;
    size_t startIndex = sentence.find(word);
    if (startIndex == string_view::npos) {
        result = {string_view::npos, string_view::npos};
    } else {
        result = {startIndex, startIndex + word.length() - 1};
    }
    return result;
}

template<typename Func>
double measureExecutionTime(Func func) {
    auto start_time = chrono::high_resolution_clock::now();
    func();
    auto end_time = chrono::high_resolution_clock::now();
    return chrono::duration_cast<chrono::microseconds>(end_time - start_time).count() / 1e6;
}

int main() {
    string sentence, word;
    cout << "Please input the sentence:=> \n";
    getline(cin, sentence);
    cout << "Please input the word:=> \n";
    getline(cin, word);

    double time_original = measureExecutionTime([&]() {
        auto [start, end] = findWordIndex(sentence, word);
        cout << "findWordIndex: => The start and end index of word in sentence are: " << start << " and " << end << endl;
    });

    double time_efficient = measureExecutionTime([&]() {
        auto [start, end] = findWordIndexEfficient(sentence, word);
        cout << "findWordIndexEfficient: => The start and end index of word in sentence are: " << start << " and " << end << endl;
    });

    cout << "findWordIndex time: " << time_original * 1000 << " milliseconds" << endl;
    cout << "findWordIndexEfficient time: " << time_efficient * 1000 << " milliseconds" << endl;

    return 0;
}
