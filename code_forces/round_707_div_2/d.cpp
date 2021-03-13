/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out && ./out < in
*/
#include <bits/stdc++.h>

using namespace std;

#define intfss int_fast8_t
#define intfs int_fast16_t
#define intf int_fast32_t
#define intfl int_fast64_t
#define ll long long

#define FAST_IO \
ios::sync_with_stdio(0);\
cin.tie(0);\
cout.tie(0);

int main(){
	FAST_IO;

	int n, m; intfl k;
	cin >> n >> m >> k;

	vector<int> a(n);
	vector<int> b(m);

	for(auto& ai : a) cin >> ai;
	for(auto& bi : b) cin >> bi;

	for(int i = 0; n < m; i++, n++) a.push_back(a[i]);
	for(int i = 0; m < n; i++, m++) b.push_back(b[i]);

	intfl day = 0;
	intfl diff = 0;
	int i = 0;

	while(diff < k){
		if(a[i] != b[i]) diff++;
		day++, i++;
		if(day == n) {
			i = 0;
			intfl remaining = k / diff;
			diff *= remaining;
			day *= remaining;
		}
	}

	cout << day;

	return 0;
}
