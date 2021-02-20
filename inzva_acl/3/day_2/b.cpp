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

	intfl n;
	cin >> n;

	intfl max = 1;
	for(intfl q = n; q > 2 * max; q--){
		for(intfl i = 1; i <= sqrt(q); i++){
			intfl j = q / i;
			intfl product = i * j;
			if(product == q && j < q && j > max) max = j;
		}
	}
	cout << max;

	return 0;
}
