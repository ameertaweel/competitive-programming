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

	intfl t;
	cin >> t;

	while(t--){
		intfl n, m;
		cin >> n >> m;

		vector<intfl> a(n);
		intfl max_sum = a[0];
		intfl rotation_sum = 0;
		vector<pair<intfl, intfl>> sums;
		intfl i = 1;
		for(auto& ai : a){
			cin >> ai;
			rotation_sum += ai;
			if(max_sum < rotation_sum) {
				max_sum = rotation_sum;
				if(rotation_sum > 0) sums.push_back({i, rotation_sum});
			}
			i++;
		}

		while(m--){
			intfl x;
			cin >> x;

			intfl sec = -1;
			if(max_sum <= 0 || (x > max_sum && rotation_sum <= 0)){
				cout << -1 << " ";
				continue;
			}

			intfl current_sum = 0;
			if(x > max_sum){
				intfl rotations = (x - max_sum) / rotation_sum + ((x - max_sum) % rotation_sum ? 1 : 0);
				sec += rotations * n;
				current_sum += rotations * rotation_sum;
			}

			auto low = lower_bound(sums.begin(), sums.end(), x - current_sum, [](pair<intfl, intfl> a, intfl val){
					return a.second < val;
			});
			auto position = (low- sums.begin());
			sec += sums[position].first;
			cout << sec << " ";
		}
		cout << "\n";
	}

	return 0;
}
