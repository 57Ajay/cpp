#include <iostream>
#include <string>
using namespace std;

int charIndex(string str, char s){
	int index;
	for (int i = 0; i < str.length(); i++){
		if (str.at(i) == s){
			index = i;
			break;
	    };
	};
	return index;
};

string charIndexEfficient(string str, char s){
	size_t index = str.find(s);
	if (index == string::npos) {
        return "Character '" + string(1, s) + "' not found in the string.";
    };
	return "The index of '" + string(1,s) + "' is at " + to_string(index);
}

int main (){
	string str;
	char s;
	cout << "Enter the string: ";
	cin >> str;
    cout << "Enter the charecter to find it's index: ";
	cin >> s;
	int idx = charIndex(str, s);
	cout << "charIndex: => The index of " << s << " is at " << idx << endl;
	cout << "chatIndexEfficient: =>" << charIndexEfficient(str, s);
	return 0;
}
