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

	int t;
	cin >> t;

	while(t--){
		intfl n;
		cin >> n;

		vector<pair<intfl, intfl>> a(n);
		for(intfl i = 0; i < n; i++){
			cin >> a[i].first;
			a[i].second = i;
		}

		sort(a.begin(), a.end(), [](pair<intfl, intfl> a, pair<intfl, intfl> b){return a.first < b.first;});
		vector<intfl> sums(n);
		intfl last_sum = 0;
		for(intfl i = 0; i < n; i++){
			last_sum += a[i].first;
			sums[i] = last_sum;
		}

		vector<bool> has_chance(n, false);
		// The one who has most tokens always has a chance to win
		auto last = a[n - 1];
		has_chance[last.second] = true;
		intfl chances = 1;
		for(intfl i = n - 2; i >= 0; i--){
			if(sums[i] >= a[i + 1].first) has_chance[a[i].second] = true, chances++;
			else break;
		}

		cout << chances << "\n";
		for(intfl i = 0; i < n; i++){
			if(has_chance[i]) cout << (i + 1) << " ";
		}
		cout << "\n";
	}

	return 0;
}
