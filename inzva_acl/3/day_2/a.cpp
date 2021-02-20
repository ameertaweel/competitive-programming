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

	int n;
	cin >> n;

	vector<int> till(n);
	for(auto& day : till) cin >> day;

	vector<int> need(n);
	for(auto& day : need) cin >> day;

	for(int i = 1; i < n; i++){
		int current = till[i] - need[i] + 1;
		if(current <= till[i - 1]){
			cout << "NO";
			return 0;
		}
	}

	cout << "YES";

	return 0;
}
