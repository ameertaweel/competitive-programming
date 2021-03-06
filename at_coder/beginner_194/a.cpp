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

	intfl a, b;
	cin >> a >> b;

	intfl milk_solids = a + b;
	intfl milk_fat = b;

	if(milk_solids >= 15 && milk_fat >= 8) cout << 1;
	else if(milk_solids >= 10 && milk_fat >= 3) cout << 2;
	else if(milk_solids >= 3) cout << 3;
	else cout << 4;

	return 0;
}
