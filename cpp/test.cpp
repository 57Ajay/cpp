#include <iostream>
#include <vector>
using namespace std;

int reverse_num(int num){
	int num_copy = num;
	int rev_num = 0;
	while (num){
		rev_num = rev_num*10 + num%10;
		num /= 10;
	}
	cout << "The reverse on the number " << num_copy << " is ";
	return rev_num;
}
int main(){
	
	int num;
	cout << "Enter the number to be reversed: ";
	cin >> num;
	cout << reverse_num(num) << endl;
	int arr[] = {2, 4, 6, 7, 1, 9};
   	vector<int> new_arr;

        for (int i : arr) {
        	cout << i / 7 << endl;
        	new_arr.push_back(i * 7);
    	}

   
  	cout << "new_arr: "<< "[";
   	for (int i : new_arr) {
        	cout << i << " ";
    	}
	cout << "]";
   	cout << endl;
	return 0;
}
