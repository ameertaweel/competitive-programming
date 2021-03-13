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

const intfl OPEN = -1;
const intfl CLOSE = -2;
const intfl POINT = -3;

int main(){
	FAST_IO;

	intfl t;
	cin >> t;

	while(t--){
		intfl n;
		cin >> n;

		intfl current_added_layers = 0;
		intfl cream_layers;
		unordered_map<intfl, vector<intfl>> intervals;
		while(current_added_layers < n){
			cin >> cream_layers;
			if(cream_layers){
				intfl reach = current_added_layers - cream_layers + 1;
				if(reach < 0) reach = 0;
				if(reach == current_added_layers) intervals[current_added_layers].push_back(POINT);
				else {
					intervals[reach].push_back(OPEN);
					intervals[current_added_layers].push_back(CLOSE);
				}
			}
			current_added_layers++;
		}
		intfl open = 0;
		for(intfl i = 0; i < n; i++){
			bool printed_1 = false;
			if(open) {cout << 1 << " "; printed_1 = true;}

			for(auto s : intervals[i]){
				switch(s){
					case OPEN: open++; if(!printed_1) {cout << 1 << " "; printed_1 = true;} break;
					case CLOSE: open--; break;
					default: if(!printed_1) {cout << 1 << " "; printed_1 = true;}
				}
			}

			if(!printed_1) cout << 0 << " ";
		}
		cout << "\n";
	}

	return 0;
}
