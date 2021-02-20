/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out && ./out < in
*/
#include <bits/stdc++.h>

using namespace std;

#define intfss int_fast8_t
#define intfs int_fast16_t
#define intf int_fast32_t
#define intfl int_fast64_t

#define FAST_IO \
ios::sync_with_stdio(0);\
cin.tie(0);\
cout.tie(0);

int main(){
	FAST_IO;

	string s;
	cin >> s;

	bool is_even = false;
	for(char c : s){
		if((is_even && !isupper(c)) || (!is_even && isupper(c))){
			cout << "No";
			return 0;
		}
		is_even = !is_even;
	}
	cout << "Yes";

	return 0;
}
