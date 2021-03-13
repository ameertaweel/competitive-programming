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

	intfl t;
	cin >> t;

	while(t--){
		intfl n;
		cin >> n;

		vector<pair<intfl, intfl>> schedule(n);
		vector<intfl> delays(n);

		for(auto& time : schedule) cin >> time.first >> time.second;
		for(auto& delay : delays) cin >> delay;

		intfl time = 0;
		intfl b_prev = 0;
		for(intfl i = 0; i < n; i++){
			intfl ai = schedule[i].first;
			intfl bi = schedule[i].second;
			intfl ti = delays[i];

			// Arrived at current station
			time += ai- b_prev + ti;

			// Departure
			if(i < n - 1){
				intfl condition_1 = time + (bi - ai) / 2;
				if((bi - ai) % 2) condition_1 += 1;

				intfl condition_2 = bi;

				time = max(condition_1, condition_2);

				b_prev = bi;
			}

		}

		cout << time << "\n";
	}

	return 0;
}
