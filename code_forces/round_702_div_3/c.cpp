/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out
*/
#include <bits/stdc++.h>

using namespace std;

#define intfss int_fast8_t
#define intfs int_fast16_t
#define intf int_fast32_t
#define intfl int_fast64_t

int main(){
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	intfl t;
	cin >> t;

	outer_loop: while(t--){
		intfl n;
		cin >> n;

		for(intfl i = 1; i < cbrt(n); i++){
			intfl a = pow(i, 3);
			intfl b = n - a;
			intfl cbrt_of_b = cbrt(b);
			if(pow(cbrt_of_b, 3) == b){
				cout << "YES\n";
				goto outer_loop;
			}
		}
		cout << "NO\n";
	}

	return 0;
}
