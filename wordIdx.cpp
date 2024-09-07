#include <iostream>
#include <string>
using namespace std;

pair <int, int> findWordIndex(const string& sentence, const string& word){
    size_t startIndex = sentence.find(word);
    if (startIndex == string::npos){
        return { -1, -1 };
    };
    size_t endIndex = startIndex + word.length()-1;
    return { static_cast<int>(startIndex), static_cast<int>(endIndex) };
};

int main(){
    string sentence, word;
    cout << "Please input the sentence:=> \n";
    getline(cin, sentence); 
    cout <<  "Please input the word:=> \n";
    getline(cin, word);
    cout << "The start and end index of word in sentence are: " << findWordIndex(sentence, word).first << " and " << findWordIndex(sentence, word).second;
    return 0;
}
