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

	intfl sum = 0;
	intfl last = 0;
	intfl current = 0;
	while(n--){
		cin >> current;
		if(current > 0){
			if(last > 0) {
				if(current > last) last = current;
			}
			else {
				sum += last;
				last = current;
			}
		}
		if(current < 0){
			if(last < 0) {
				if(current > last) last = current;
			}
			else {
				sum += last;
				last = current;
			}
		}
	}

	sum += last;
	cout << sum;

	return 0;
}
