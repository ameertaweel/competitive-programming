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

	intfl n;
	cin >> n;

	map<intfl, bool> found;
	for(intfl i = 2; i <= sqrt(n); i++){
		intfl p = 2;
		intfl r = pow(i, p);
		while(r <= n){
			found[r] = true;
			p++;
			r = pow(i, p);
		}
	}

	cout << n - found.size();

	return 0;
}
