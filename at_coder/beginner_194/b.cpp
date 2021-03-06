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

	intfl a, b, combined, min_a, min_b, min_a_second, min_b_second, min_combined, min_a_index, min_b_index;
	min_a = min_b = min_combined = 1e10;
	min_a_index = min_b_index = -1;
	for(intfl i = 0; i < n; i++){
		cin >> a >> b;
		combined = a + b;
		if(combined < min_combined) min_combined = combined;
		if(a < min_a){
			min_a_second = min_a;
			min_a = a;
			min_a_index = i;
		} else if (a < min_a_second){
			min_a_second = a;
		}
		if(b < min_b){
			min_b_second = min_b;
			min_b = b;
			min_b_index = i;
		} else if (b < min_b_second){
			min_b_second = b;
		}
	}

	if(min_a_index == min_b_index) cout << min(min(max(min_a, min_b_second), max(min_b, min_a_second)), min_combined);
	else cout << min(max(min_a, min_b), min_combined);

	return 0;
}
