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

	intfl sum_a;
	cin >> sum_a;

	intfl sum = (n - 1) * pow(sum_a, 2);

	intfl ai;
	for(intfl i = 1; i < n; i++){
		cin >> ai;
		sum += (n - 1) * pow(ai, 2);
		sum -= 2 * ai * sum_a;
		sum_a += ai;
	}

	cout << sum;

	return 0;
}
