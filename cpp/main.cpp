#include <cstddef>
#include <iostream>
#include <string>
using namespace std;

int main(){
	string name = "Ajay";
	cout << "This uses length() method: " << name.length()<< endl;
	cout << name.size();
	string fullname;
	getline(cin, fullname);
	cout << fullname;
	cout << "\nThis is c-style string, an array of chars: "<< endl;
	char greet[] = "Hello!";
	cout << greet << " "<< fullname << endl;
	return 0;
}
