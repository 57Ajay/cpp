#include <iostream>
#include <string>
using namespace std;

int charIndex(string str, char s){
	int index;
	for (int i = 0; i < str.length(); i++){
		if (str.at(i) == s){
			index = i;		
	    };
	};
	return index;
};

int main (){
	string str;
	char s;
	cout << "Enter the string: ";
	cin >> str;
    cout << "Enter the charecter to find it's index: ";
	cin >> s;
	int idx = charIndex(str, s);
	cout << "The index of " << s << " is at " << idx << endl;
	return 0;
}
