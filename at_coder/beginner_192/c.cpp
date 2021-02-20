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

	intfl n, k;
	cin >> n >> k;

	intfl ai = n;
	while(k--){
		string asc_str = to_string(ai);
		sort(asc_str.begin(), asc_str.end());
		string desc_str = to_string(ai);
		sort(desc_str.rbegin(), desc_str.rend());

		long long asc = stoll(asc_str);
		long long desc = stoll(desc_str);

		ai = desc - asc;
		if(ai == 0) break;
	}

	cout << ai;

	return 0;
}
