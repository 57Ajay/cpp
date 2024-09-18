#include<iostream>
#include<cmath>
using namespace std;

void simple_math(){
	cout << "Enter a number to find a sqrt: " << endl;
	int num;
	cin >> num;
	double sq = sqrt(num);
	cout << sq << endl;
	cout << "This is rounded value of founded sqrt: "<< round(sq) << endl;
	cout << "This is the log of founded sqrt: "<< log(sq) <<endl;	
}

void ter_ope(){
	int time = 20;
	string res = (time > 12) ? "This is now good Evening.": "THis is good morning.";
       cout << res;	
}

int main(){
	simple_math();
	ter_ope();
	return 0;
}
