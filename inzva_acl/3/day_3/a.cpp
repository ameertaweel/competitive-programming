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

	intfl t;
	cin >> t;

	while(t--){
		intfl a, b;
		cin >> a >> b;

		intfl close = min(a, b);
		intfl far = max(a, b);

		intfl time_to_even = far - close;

		if(time_to_even > close) {
			cout << "NO\n";
			continue;
		}

		intfl dist = close - time_to_even;

		cout << ((dist % 3 == 0) ? "YES" : "NO") << "\n";
	}

	return 0;
}
