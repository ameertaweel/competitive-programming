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

	int n;
	cin >> n;

	vector<int> a(n);
	unordered_map<int, vector<pair<int, int>>> k;
	for(intfl w = 0; w < n; w++){
		cin >> a[w];
		for(int x = 0; x < w; x++){
			int sum = a[w] + a[x];
			k[sum].push_back(make_pair(w, x));
			if(k[sum].size() > 1){
				for(auto [y, z] : k[sum]){
					if(w != y && w != z && x != y && x != z) {
						cout << "YES\n";
						cout << (w + 1) << " " << (x + 1) << " " << (y + 1) << " " << (z + 1) << "\n";
						return 0;
					}
				}
			}
		}
	}

	cout << "NO\n";

	return 0;
}
