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

	string x; intfl m;
	cin >> x >> m;

	int d = 0;
	for(char c : x){
		string s{c};
		int new_d = stoi(s);
		if(new_d > d) d = new_d;
	}

	set<intfl> nums;
	intfl count = 0;

	long long xi = stoll(x);

	while(++d){
		intfl power = 1;
		intfl decimal = 0;
		for(auto c = x.rbegin(); c < x.rend(); c++){
			string s{*c};
			int digit = stoi(s);
			decimal += digit * power;
			if(decimal > m) break;
			power *= d;
		}
		if(decimal > 0 && decimal <= m && !nums.count(decimal)) count++;
		else break;

		nums.insert(decimal);
	}

	cout << count;

	return 0;
}
